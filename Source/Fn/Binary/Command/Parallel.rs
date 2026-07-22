use std::{
	path::{Path, PathBuf},
	sync::Arc,
};

use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use tokio::sync::{Mutex, mpsc::Sender};

use crate::{
	Fn::Binary::Command::Index,
	Struct::{
		Binary::Command::Entry::Struct as ExecutionOption,
		Event::Struct as Event,
	},
};

pub mod GPG;
pub mod Process;

static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

struct ProcessedCommand {
	Command:String,
	RequiresGpgLock:bool,
	RequiresIndexLock:bool,
}

/// Executes commands in parallel across multiple directories.
///
/// Identical logic to the previous implementation; the only change is that all
/// output is routed through `Tx` (typed `Event`) instead of `println!`.
pub async fn Fn(Option:ExecutionOption, Tx:Sender<Event>) {
	let TotalCommands = Option.Command.len();

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
		let _ = Tx.send(Event::AllDone).await;
		return;
	}

	let WorkQueue = Arc::new(ArrayQueue::new(TargetDirs.len()));
	for Dir in TargetDirs {
		WorkQueue.push(Dir).expect("Queue capacity pre-allocated");
	}

	let WorkerCount = rayon::current_num_threads();
	let mut WorkerHandles = Vec::with_capacity(WorkerCount);

	for _ in 0..WorkerCount {
		let Queue = Arc::clone(&WorkQueue);
		let Commands = Arc::clone(&ProcessedCommands);
		let Producer = Tx.clone();

		let WorkerHandle = tokio::spawn(async move {
			while let Some(Directory) = Queue.pop() {
				let DirectoryString = Directory.to_string_lossy().to_string();

				let _ = Producer.send(Event::JobStarted {
					Directory:DirectoryString.clone(),
					Total:TotalCommands,
				}).await;

				let mut AllSuccess = true;

				'commands: for (CmdIdx, Cmd) in Commands.iter().enumerate() {
					if Cmd.RequiresIndexLock
						&& !Index::Lock::Fn(&DirectoryString).await
					{
						let _ = Producer.send(Event::IndexLockTimeout {
							Directory:DirectoryString.clone(),
						}).await;
						break 'commands;
					}

					let Result = if Cmd.RequiresGpgLock {
						let _GpgLock = GPG_MUTEX.lock().await;
						Process::Fn(&Cmd.Command, &DirectoryString).await
					} else {
						Process::Fn(&Cmd.Command, &DirectoryString).await
					};

					match Result {
						Ok(Output) => {
							for Line in Output.lines() {
								if !Line.trim().is_empty() {
									let _ = Producer.send(Event::Line {
										Directory:DirectoryString.clone(),
										Text:Line.to_owned(),
										IsStderr:false,
									}).await;
								}
							}
						}
						Err(Error) => {
							let _ = Producer.send(Event::Line {
								Directory:DirectoryString.clone(),
								Text:format!("Error: {}", Error),
								IsStderr:true,
							}).await;
							AllSuccess = false;
						}
					}

					let _ = Producer.send(Event::JobProgress {
						Directory:DirectoryString.clone(),
						Done:CmdIdx + 1,
						Total:TotalCommands,
						Success:AllSuccess,
					}).await;
				}

				let _ = Producer.send(Event::JobFinished {
					Directory:DirectoryString.clone(),
					Success:AllSuccess,
				}).await;
			}
		});
		WorkerHandles.push(WorkerHandle);
	}

	for Handle in WorkerHandles {
		Handle.await.expect("Worker task panicked.");
	}

	let _ = Tx.send(Event::AllDone).await;
}
