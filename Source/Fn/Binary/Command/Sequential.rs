use std::path::{Path, PathBuf};

use tokio::io::AsyncBufReadExt;
use tokio::process::Command as TokioCommand;
use tokio::sync::mpsc::Sender;

use crate::{
	Fn::Binary::Command::Index,
	Struct::{
		Binary::Command::Entry::Struct as ExecutionOption,
		Event::Struct as Event,
	},
};

/// Executes commands sequentially, one directory at a time.
///
/// All output is emitted through `Tx` as typed `Event` variants so the caller
/// (CLI printer or TUI) can render it however it likes. No I/O happens here.
pub async fn Fn(Option:　ExecutionOption, Tx:　Sender<Event>) {
	let TotalCommands = Option.Command.len();

	let ProcessedCommands:　Vec<(String, bool)> = Option
		.Command
		.iter()
		.map(|CommandString| {
			let RequiresIndexLock = Index::Fn(CommandString);
			(CommandString.clone(), RequiresIndexLock)
		})
		.collect();

	let TargetDirs:　Vec<PathBuf> = Option
		.Entry
		.into_iter()
		.filter_map(|CandidatePath| {
			if CandidatePath.file_name().is_some_and(|Name| Name == Option.Pattern.as_str()) {
				CandidatePath.parent().map(Path::to_path_buf)
			} else {
				None
			}
		})
		.collect();

	'directories: for Directory in TargetDirs {
		let DirectoryString = Directory.to_string_lossy().to_string();

		let _ = Tx.send(Event::JobStarted {
			Directory:　DirectoryString.clone(),
			Total:　TotalCommands,
		}).await;

		let mut AllSuccess = true;

		for (CmdIdx, (CommandString, RequiresIndexLock)) in ProcessedCommands.iter().enumerate() {
			if CommandString.trim().is_empty() {
				continue;
			}

			if *RequiresIndexLock && !Index::Lock::Fn(&DirectoryString).await {
				let _ = Tx.send(Event::IndexLockTimeout {
					Directory:　DirectoryString.clone(),
				}).await;
				continue 'directories;
			}

			let mut Child = match TokioCommand::new("sh")
				.args(["-c", CommandString])
				.current_dir(&DirectoryString)
				.stdout(std::process::Stdio::piped())
				.stderr(std::process::Stdio::piped())
				.spawn()
			{
				Ok(Child) => Child,
				Err(Error) => {
					let _ = Tx.send(Event::Line {
						Directory:　DirectoryString.clone(),
						Text:　format!("Failed to spawn: {}", Error),
						IsStderr:　true,
					}).await;
					AllSuccess = false;
					continue;
				}
			};

			let StdoutReader = Child.stdout.take().unwrap();
			{
				let mut Lines = tokio::io::BufReader::new(StdoutReader).lines();
				while let Ok(Some(Line)) = Lines.next_line().await {
					if !Line.trim().is_empty() {
						let _ = Tx.send(Event::Line {
							Directory:　DirectoryString.clone(),
							Text:　Line,
							IsStderr:　false,
						}).await;
					}
				}
			}

			let mut StderrBuf = String::new();
			{
				let StderrReader = Child.stderr.take().unwrap();
				tokio::io::AsyncReadExt::read_to_string(
					&mut tokio::io::BufReader::new(StderrReader),
					&mut StderrBuf,
				)
				.await
				.unwrap_or(0);
			}

			let ExitStatus = Child.wait().await;
			let Success = matches!(ExitStatus, Ok(S) if S.success());

			if !StderrBuf.trim().is_empty() {
				for Line in StderrBuf.lines() {
					if !Line.trim().is_empty() {
						let _ = Tx.send(Event::Line {
							Directory:　DirectoryString.clone(),
							Text:　Line.to_owned(),
							IsStderr:　true,
						}).await;
					}
				}
			}

			if !Success {
				AllSuccess = false;
			}

			let _ = Tx.send(Event::JobProgress {
				Directory:　DirectoryString.clone(),
				Done:　CmdIdx + 1,
				Total:　TotalCommands,
				Success:　Success,
			}).await;
		}

		let _ = Tx.send(Event::JobFinished {
			Directory:　DirectoryString.clone(),
			Success:　AllSuccess,
		}).await;
	}

	let _ = Tx.send(Event::AllDone).await;
}
