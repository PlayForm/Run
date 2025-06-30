use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tokio::sync::{Mutex, mpsc};

use crate::Struct::Binary::Command::Entry::Struct as Option;

pub mod GPG;
pub mod Process;

static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// A command that has been pre-parsed and analyzed for GPG requirements.
struct ProcessedCommand {
	parts:Vec<String>,
	requires_gpg_lock:bool,
}

pub async fn Fn(Option { Entry, Pattern, Command, .. }:Option) {
	// OPTIMIZATION: Parse and analyze commands ONCE, outside the hot loops.
	// This prevents millions of redundant string splits and GPG checks.
	let processed_commands:Arc<Vec<ProcessedCommand>> = Arc::new(
		Command
			.par_iter()
			.map(|cmd_str| {
				let parts:Vec<String> = cmd_str.split_whitespace().map(String::from).collect();
				let requires_gpg_lock = GPG::Fn(&parts);
				ProcessedCommand { parts, requires_gpg_lock }
			})
			.collect(),
	);

	// OPTIMIZATION: Efficiently filter paths and find their parent directories.
	// This uses proper path manipulation instead of string splitting/joining.
	let target_dirs:Vec<PathBuf> = Entry
		.into_par_iter()
		.filter_map(|path| {
			if path.file_name().map_or(false, |name| name == Pattern.as_str()) {
				path.parent().map(Path::to_path_buf)
			} else {
				None
			}
		})
		.collect();

	if target_dirs.is_empty() {
		return;
	}

	let (tx, mut rx) = mpsc::unbounded_channel::<String>();
	let queue = Arc::new(ArrayQueue::new(target_dirs.len()));
	for dir in target_dirs {
		queue.push(dir).expect("Queue should have enough capacity");
	}

	// This task now handles printing results as they arrive.
	let output_task = tokio::spawn(async move {
		while let Some(output) = rx.recv().await {
			if !output.trim().is_empty() {
				println!("{}", output);
			}
		}
	});

	let worker_count = rayon::current_num_threads();
	let mut workers = Vec::with_capacity(worker_count);
	for _ in 0..worker_count {
		let queue = Arc::clone(&queue);
		let commands = Arc::clone(&processed_commands);
		let tx = tx.clone();

		workers.push(tokio::spawn(async move {
			while let Some(dir) = queue.pop() {
				let dir_str = dir.to_string_lossy();
				let command_futures = commands.iter().map(|cmd| {
					async {
						if cmd.requires_gpg_lock {
							let _lock = GPG_MUTEX.lock().await;
						}
						// Use pre-parsed command parts.
						Process::Fn(&cmd.parts, &dir_str).await
					}
				});

				for result in futures::future::join_all(command_futures).await {
					match result {
						Ok(output) => {
							if tx.send(output).is_err() {
								// Receiver dropped, stop trying to send.
								break;
							}
						},
						Err(e) => eprintln!("Error executing command in '{}': {}", dir_str, e),
					}
				}
			}
		}));
	}

	for worker in workers {
		worker.await.expect("Worker task panicked");
	}

	// Drop the original sender to signal the output_task that no more messages will
	// come.
	drop(tx);
	output_task.await.expect("Output task panicked");
}
