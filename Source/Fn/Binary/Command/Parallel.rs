use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tokio::sync::{Mutex, mpsc};

use crate::{
	Fn::Binary::Command::Index,
	Struct::Binary::Command::Entry::Struct as ExecutionOption,
};

pub mod GPG;
pub mod Process;

/// A global, asynchronous mutex to ensure that only one GPG-related git command
/// runs at any given time, preventing conflicts with the GPG agent.
static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

/// Represents a command that has been pre-processed for efficient execution.
///
/// This struct holds the command string and booleans indicating whether the
/// command requires a GPG lock or an index-lock wait, preventing redundant
/// classification work inside the main execution loop.
struct ProcessedCommand {
	Command:String,
	RequiresGpgLock:bool,
	RequiresIndexLock:bool,
}

/// Executes commands in parallel across multiple directories.
///
/// This function orchestrates a complex workflow:
/// 1. Pre-classifies all user-provided commands for lock requirements.
/// 2. Filters the candidate paths to identify target execution directories.
/// 3. Sets up a multi-producer, single-consumer channel for work distribution.
/// 4. Spawns a pool of Tokio worker tasks.
/// 5. Each worker pulls a directory from the queue and executes all commands
///    **sequentially** within it via `sh -c`, preserving order and preventing
///    index-lock conflicts between chained git commands.
/// 6. Before any index-modifying git command the worker waits for
///    `.git/index.lock` to be released, handling both active locks from other
///    processes and stale locks left by previously killed processes.
/// 7. A dedicated output task prints results to stdout as they arrive.
pub async fn Fn(Option:ExecutionOption) {
	// 1. Pre-process commands: classify lock requirements once.
	let ProcessedCommands:Arc<Vec<ProcessedCommand>> = Arc::new(
		Option
			.Command
			.par_iter()
			.map(|CommandString| {
				let RequiresGpgLock = GPG::Fn(CommandString);
				let RequiresIndexLock = Index::Fn(CommandString);
				ProcessedCommand { Command:CommandString.clone(), RequiresGpgLock, RequiresIndexLock }
			})
			.collect(),
	);

	// 2. Identify target directories based on the pattern.
	let TargetDirs:Vec<PathBuf> = Option
		.Entry
		.into_par_iter()
		.filter_map(|CandidatePath| {
			if CandidatePath.file_name().is_some_and(|Name| Name == Option.Pattern.as_str()) {
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

				// Commands are executed sequentially within each directory.
				// This preserves the user-supplied order (e.g. `git add` before
				// `git commit`) and ensures only one command at a time holds
				// the git index lock for this repository.
				//
				// All output for this directory is collected into a single
				// buffer and sent atomically through the channel so that
				// results from different directories do not interleave.
				let mut DirectoryOutput = String::new();

				'commands: for Cmd in Commands.iter() {
					// Wait for any in-flight index lock before writing to the index.
					if Cmd.RequiresIndexLock
						&& !Index::Lock::Fn(&DirectoryString).await
					{
						eprintln!(
							"Skipping remaining commands in '{}': git index lock timed out.",
							DirectoryString
						);
						break 'commands;
					}

					// Hold the GPG mutex for the entire duration of the process
					// so the GPG agent is not shared concurrently.
					let Result = if Cmd.RequiresGpgLock {
						let _GpgLock = GPG_MUTEX.lock().await;
						Process::Fn(&Cmd.Command, &DirectoryString).await
					} else {
						Process::Fn(&Cmd.Command, &DirectoryString).await
					};

					match Result {
						Ok(Output) => DirectoryOutput.push_str(&Output),
						Err(Error) => {
							eprintln!(
								"Error executing command in '{}': {}",
								DirectoryString, Error
							)
						},
					}
				}

				if !DirectoryOutput.trim().is_empty()
					&& Producer.send(DirectoryOutput).is_err()
				{
					// Receiver dropped - stop processing entirely.
					break;
				}
			}
		});
		WorkerHandles.push(WorkerHandle);
	}

	// Wait for all workers to complete their tasks.
	for Handle in WorkerHandles {
		Handle.await.expect("Worker task panicked.");
	}

	// Drop the original producer to signal the output task to terminate.
	drop(Tx);
	OutputTask.await.expect("Output task panicked.");
}
