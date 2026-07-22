use std::path::{Path, PathBuf};

use tokio::io::AsyncBufReadExt;
use tokio::process::Command as TokioCommand;

use crate::{
	Fn::Binary::Command::Index,
	Struct::Binary::Command::Entry::Struct as ExecutionOption,
};

/// Executes commands sequentially, one directory at a time.
///
/// This function provides a non-parallel execution strategy. It iterates
/// through each target directory and runs all specified commands within it
/// before moving to the next. Before any index-modifying git command it waits
/// for `.git/index.lock` to be released, handling both active locks held by
/// other processes and stale locks left by previously killed processes.
///
/// # Arguments
///
/// * `Option`: An `ExecutionOption` struct containing the commands, paths, and
///   pattern.
pub async fn Fn(Option:ExecutionOption) {
	// Pre-parse command strings into their component parts once.
	let ProcessedCommands:Vec<(Vec<String>, bool)> = Option
		.Command
		.iter()
		.map(|CommandString| {
			let Parts:Vec<String> =
				CommandString.split_whitespace().map(String::from).collect();
			let RequiresIndexLock = Index::Fn(&Parts);
			(Parts, RequiresIndexLock)
		})
		.collect();

	// Identify target directories where commands will be executed.
	let TargetDirs:Vec<PathBuf> = Option
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
		let DirectoryString = Directory.to_string_lossy();

		for (CommandParts, RequiresIndexLock) in &ProcessedCommands {
			if CommandParts.is_empty() {
				continue;
			}

			// Wait for any in-flight index lock before writing to the index.
			if *RequiresIndexLock && !Index::Lock::Fn(&DirectoryString).await {
				eprintln!(
					"Skipping remaining commands in '{}': git index lock timed out.",
					DirectoryString
				);
				continue 'directories;
			}

			let mut Child = match TokioCommand::new(&CommandParts[0])
				.args(&CommandParts[1..])
				.current_dir(DirectoryString.as_ref())
				.stdout(std::process::Stdio::piped())
				.stderr(std::process::Stdio::piped())
				.spawn()
			{
				Ok(Child) => Child,
				Err(Error) => {
					eprintln!("Failed to spawn command in '{}': {}", DirectoryString, Error);
					continue;
				}
			};

			// Stream stdout to the terminal line by line as it's produced.
			let StdoutReader = Child.stdout.take().unwrap();
			{
				let mut Lines = tokio::io::BufReader::new(StdoutReader).lines();
				while let Ok(Some(Line)) = Lines.next_line().await {
					if !Line.trim().is_empty() {
						println!("{}", Line);
					}
				}
			}

			// Capture stderr for error reporting.
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

			let Status = Child.wait().await;

			match Status {
				Ok(ExitStatus) if !ExitStatus.success() => {
					eprintln!(
						"Command failed in '{}' with status {}. Stderr: {}",
						DirectoryString,
						ExitStatus,
						StderrBuf.trim()
					);
				}
				Err(Error) => {
					eprintln!(
						"Command in '{}' was terminated: {}",
						DirectoryString,
						Error
					);
				}
				_ => {}
			}
		}
	}
}
