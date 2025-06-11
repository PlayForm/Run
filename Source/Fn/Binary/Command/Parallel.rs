// src/Parallel.rs

use std::sync::Arc;

use futures::stream::{self, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::Struct::Binary::Command::Entry::Struct as Option;

// A global mutex for GPG-related commands.
static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

// Sub-modules
pub mod GPG;
pub mod Process;

/// Executes a series of commands on a list of entries concurrently.
///
/// This function processes each target path in parallel. Within each path,
/// it executes the specified commands sequentially to prevent I/O race
/// conditions.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }:Option) {
	let CommandSign:Vec<bool> = Command
		.iter()
		.map(|cmd_str| {
			let parts:Vec<String> = cmd_str.split(' ').map(String::from).collect();
			GPG::Fn(&parts)
		})
		.collect();

	let CommandArc = Arc::new(Command);
	let SignArc = Arc::new(CommandSign);

	let Target:Vec<String> = Entry
		.into_iter()
		.filter_map(|Part| {
			Part.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Part[0..Part.len() - 1].join(&Separator.to_string()))
		})
		.collect();

	let Limit = num_cpus::get();

	// Create a stream of futures, one for each target path.
	let results_stream = stream::iter(Target).map(|Path| {
		let LocalCommand = Arc::clone(&CommandArc);
		let LocalSign = Arc::clone(&SignArc);

		async move {
			// This vector will hold the outputs for commands run on this single Path.
			let mut outputs:Vec<String> = Vec::new();

			// Execute commands sequentially for this path to prevent race conditions.
			for (i, command_str) in LocalCommand.iter().enumerate() {
				let requires_gpg_lock = LocalSign[i];
				let command_parts:Vec<String> = command_str.split(' ').map(String::from).collect();

				// Now calls our new, robust Process::Fn.
				let output = if requires_gpg_lock {
					let _guard = GPG_MUTEX.lock().await;
					Process::Fn(&command_parts, &Path).await
				} else {
					Process::Fn(&command_parts, &Path).await
				};

				if !output.is_empty() {
					outputs.push(output);
				}
			}

			// Format the collected outputs for this path into a single, clean block.
			format!("--- Results for: {} ---\n{}", Path, outputs.join("\n"))
		}
	});

	// Run the futures for each PATH concurrently.
	let all_results:Vec<String> = results_stream.buffer_unordered(Limit).collect().await;

	// Print the collected results sequentially.
	for result_block in all_results {
		println!("{}\n", result_block);
	}
}
