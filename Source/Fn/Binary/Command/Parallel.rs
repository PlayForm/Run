/// Executes a series of commands on a list of entries concurrently.
///
/// # Arguments
/// * `Option` - Struct with fields: `Entry` (Vec<String>), `Separator`
///   (String), `Pattern` (String), `Command` (Vec<String>).
///
/// # Details
/// Filters entries by pattern, processes them with commands in parallel using
/// workers, and prints outputs. Uses Rayon for filtering and Tokio for async
/// command execution, with a lock-free queue for work distribution.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }:Option) {
	// --- OPTIMIZATION: Check for GPG signing ONCE at the start ---
	let commands_require_signing:Vec<bool> = Command
		.iter()
		.map(|cmd_str| {
			let parts:Vec<String> = cmd_str.split(' ').map(String::from).collect();
			GPG::Fn(&parts)
		})
		.collect();

	// --- ARCHITECTURE CHANGE: Pure Tokio/Futures concurrency model ---
	let command_arc = Arc::new(Command);
	let signing_arc = Arc::new(commands_require_signing);

	// Filter the entries to find the directories we need to operate on.
	// This is fast enough to not need Rayon.
	let target_dirs:Vec<String> = Entry
		.into_iter()
		.filter_map(|entry_parts| {
			entry_parts
				.last()
				.filter(|last| *last == &Pattern)
				.map(|_| entry_parts[0..entry_parts.len() - 1].join(&Separator.to_string()))
		})
		.collect();

	// Determine a reasonable concurrency limit. Using num_cpus is a good default.
	// This prevents spawning thousands of processes at once.
	let concurrency_limit = num_cpus::get();

	// Turn our list of directories into a stream that can be processed
	// concurrently.
	stream::iter(target_dirs)
		.for_each_concurrent(concurrency_limit, |entry_path| {
			// Clone Arcs for the new async block. This is cheap.
			let local_commands = Arc::clone(&command_arc);
			let local_signing_info = Arc::clone(&signing_arc);

			async move {
				// For each directory, run all its commands.
				// We can run the commands for a *single* directory in parallel.
				let tasks = local_commands.iter().enumerate().map(|(i, cmd_str)| {
					let requires_signing = local_signing_info[i];
					let parts:Vec<String> = cmd_str.split(' ').map(String::from).collect();
					let entry = entry_path.clone();

					async move {
						if requires_signing {
							let _lock = GPG_MUTEX.lock().await;
							Process::Fn(&parts, &entry).await
						} else {
							Process::Fn(&parts, &entry).await
						}
					}
				});

				// Await all commands for the current directory and collect their output.
				let outputs = futures::future::join_all(tasks).await;

				// Print the results for this directory. `println!` is thread-safe.
				for output in outputs.into_iter().filter(|s| !s.is_empty()) {
					println!("{}", output);
				}
			}
		})
		.await;
}

use std::sync::Arc;

use futures::stream::{self, StreamExt};
// The GPG mutex is still necessary to serialize signing operations.
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::Struct::Binary::Command::Entry::Struct as Option;
static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub mod GPG;
pub mod Process;
