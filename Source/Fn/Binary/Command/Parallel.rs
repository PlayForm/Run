use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tokio::sync::{Mutex, mpsc};

use crate::Struct::Binary::Command::Entry::Struct as ExecutionOption;

pub mod GPG;
pub mod Process;

/// A global, asynchronous mutex to ensure that only one GPG-related git command
/// runs at any given time, preventing conflicts with the GPG agent.
static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// Represents a command that has been pre-processed for efficient execution.
///
/// This struct holds the parsed command parts and a boolean indicating if it
/// requires a GPG lock, preventing redundant processing inside the main
/// execution loop.
struct ProcessedCommand {
	Parts:Vec<String>,
	RequiresGpgLock:bool,
}

/// Executes commands in parallel across multiple directories.
///
/// This function orchestrates a complex workflow:
/// 1. Pre-parses all user-provided commands.
/// 2. Filters the list of candidate paths to find target execution directories.
/// 3. Sets up a multi-producer, single-consumer channel for work distribution.
/// 4. Spawns a pool of Tokio worker tasks.
/// 5. Each worker pulls a directory from the queue and executes all commands
///    within it.
/// 6. A dedicated output task prints results to stdout as they become
///    available.
pub async fn Fn(Option:ExecutionOption) {
	// 1. Pre-process commands: Parse strings and check for GPG requirements once.
	let ProcessedCommands:Arc<Vec<ProcessedCommand>> = Arc::new(
		Option
			.Command
			.par_iter()
			.map(|CommandString| {
				let Parts:Vec<String> = CommandString.split_whitespace().map(String::from).collect();
				let RequiresGpgLock = GPG::Fn(&Parts);
				ProcessedCommand { Parts, RequiresGpgLock }
			})
			.collect(),
	);

	// 2. Identify target directories based on the pattern.
	// This efficiently finds the parent directory of each path that matches the
	// pattern.
	let TargetDirs:Vec<PathBuf> = Option
		.Entry
		.into_par_iter()
		.filter_map(|CandidatePath| {
			if CandidatePath.file_name().map_or(false, |Name| Name == Option.Pattern.as_str()) {
				CandidatePath.parent().map(Path::to_path_buf)
			} else {
				None
			}
		})
		.collect();

	if TargetDirs.is_empty() {
		return;
	}

	// 3. Set up the work queue and the results channel.
	let (Tx, mut Rx) = mpsc::unbounded_channel::<String>();
	let WorkQueue = Arc::new(ArrayQueue::new(TargetDirs.len()));
	for Dir in TargetDirs {
		WorkQueue
			.push(Dir)
			.expect("Queue should have enough capacity for all target directories.");
	}

	// 4. Spawn the output task to print results from the channel.
	let OutputTask = tokio::spawn(async move {
		while let Some(Output) = Rx.recv().await {
			if !Output.trim().is_empty() {
				println!("{}", Output);
			}
		}
	});

	// 5. Spawn worker tasks, one for each available CPU core.
	let WorkerCount = rayon::current_num_threads();
	let mut WorkerHandles = Vec::with_capacity(WorkerCount);
	for _ in 0..WorkerCount {
		let Queue = Arc::clone(&WorkQueue);
		let Commands = Arc::clone(&ProcessedCommands);
		let Producer = Tx.clone();

		let WorkerHandle = tokio::spawn(async move {
			while let Some(Directory) = Queue.pop() {
				let DirectoryString = Directory.to_string_lossy();
				let CommandFutures = Commands.iter().map(|Cmd| {
					async {
						if Cmd.RequiresGpgLock {
							let _GpgLock = GPG_MUTEX.lock().await;
						}
						Process::Fn(&Cmd.Parts, &DirectoryString).await
					}
				});

				for Result in futures::future::join_all(CommandFutures).await {
					match Result {
						Ok(Output) => {
							if Producer.send(Output).is_err() {
								// Receiver has been dropped, so we can stop processing.
								break;
							}
						},
						Err(Error) => {
							eprintln!("Error executing command in '{}': {}", DirectoryString, Error)
						},
					}
				}
			}
		});
		WorkerHandles.push(WorkerHandle);
	}

	// Wait for all workers to complete their tasks.
	for Handle in WorkerHandles {
		Handle.await.expect("Worker task panicked.");
	}

	// Drop the original producer, which signals to the receiver that no more
	// messages will be sent, allowing the output task to terminate gracefully.
	drop(Tx);
	OutputTask.await.expect("Output task panicked.");
}
