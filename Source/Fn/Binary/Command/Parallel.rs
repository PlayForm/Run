//! This module provides the core concurrent execution logic.
//! It is responsible for taking a list of entries (e.g., directories),
//! filtering them, and running a series of shell commands within each
//! one in parallel.

/// Executes a series of commands on a list of entries concurrently.
///
/// # Arguments
/// * `Option` - A struct with fields: `Entry` (Vec<String>), `Separator`
///   (String), `Pattern` (String), `Command` (Vec<String>).
///
/// # Details
/// This function is the heart of the application. It performs the following
/// steps:
/// 1. Determines which commands require GPG signing and need to be serialized.
/// 2. Filters the input `Entry` list based on the `Pattern`.
/// 3. For each valid entry (the "target"), it creates an asynchronous task.
/// 4. It runs a limited number of these tasks in parallel using
///    `buffer_unordered`.
/// 5. Inside each task, it executes all specified commands via `Process::Fn`.
/// 6. It collects all command outputs for a single target into a formatted
///    string.
/// 7. After all concurrent tasks are complete, it prints the collected results
///    sequentially, ensuring that output from different targets is not
///    interleaved.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }:Option) {
	// Step 1: Pre-calculate which commands require the GPG mutex.
	let CommandSign:Vec<bool> = Command
		.iter()
		.map(|cmd_str| {
			let parts:Vec<String> = cmd_str.split(' ').map(String::from).collect();
			GPG::Fn(&parts)
		})
		.collect();

	// Wrap Commands and Signing info in Arcs for safe, cheap sharing across
	// threads.
	let CommandArc = Arc::new(Command);
	let SignArc = Arc::new(CommandSign);

	// Step 2: Filter the entry list to get the final list of targets.
	let Target:Vec<String> = Entry
		.into_iter()
		.filter_map(|Part| {
			Part.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Part[0..Part.len() - 1].join(&Separator.to_string()))
		})
		.collect();

	// Use the number of CPU cores as the concurrency limit for parallel tasks.
	let Limit = num_cpus::get();

	// Step 3: Create a stream of futures, one for each target path.
	// We use `map` to define what work to do, but it doesn't run anything yet.
	let results_stream = stream::iter(Target).map(|Path| {
		// Clone the Arcs for this specific task.
		let LocalCommand = Arc::clone(&CommandArc);
		let LocalSign = Arc::clone(&SignArc);

		// The async block is the future that will be executed for each Path.
		async move {
			let task_futures = LocalCommand.iter().enumerate().map(|(i, command_str)| {
				let requires_gpg_lock = LocalSign[i];
				let command_parts:Vec<String> = command_str.split(' ').map(String::from).collect();
				let entry_path = Path.clone();

				async move {
					// Execute the command, acquiring the GPG lock if necessary.
					let output = if requires_gpg_lock {
						let _guard = GPG_MUTEX.lock().await;
						Process::Fn(&command_parts, &entry_path).await
					} else {
						Process::Fn(&command_parts, &entry_path).await
					};
					// Trim whitespace and control characters from the output.
					output.trim().to_string()
				}
			});

			// Wait for all commands for this *single* path to complete.
			let outputs:Vec<String> = futures::future::join_all(task_futures)
				.await
				.into_iter()
				.filter(|s| !s.is_empty())
				.collect();

			// Format the collected outputs for this path into a single string.
			format!("--- Results for: {} ---\n{}", Path, outputs.join("\n"))
		}
	});

	// Step 4: Run the futures concurrently and collect all results.
	// `buffer_unordered` runs up to `Limit` futures at a time.
	// `collect` waits for all of them to finish and gathers the results into a Vec.
	let all_results:Vec<String> = results_stream.buffer_unordered(Limit).collect().await;

	// Step 5: Print the results sequentially.
	// This loop is not concurrent, guaranteeing clean, non-interleaved output.
	for result_block in all_results {
		// Add an extra newline for better separation between blocks.
		println!("{}\n", result_block);
	}
}

use std::sync::Arc;

use futures::stream::{self, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

// Import the Option struct that defines the program's configuration.
use crate::Struct::Binary::Command::Entry::Struct as Option;

// A global, lazily-initialized mutex to ensure that GPG-related commands,
// which may require user interaction via pinentry, are run one at a time.
static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

// Declare the GPG and Process sub-modules.
// These modules are expected to be in files named `src/Parallel/GPG.rs`
// and `src/Parallel/Process.rs` respectively.
pub mod GPG;
pub mod Process;
