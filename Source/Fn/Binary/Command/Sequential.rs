use std::path::{Path, PathBuf};

use tokio::process::Command as TokioCommand;

use crate::Struct::Binary::Command::Entry::Struct as ExecutionOption;

/// Executes commands sequentially, one directory at a time.
///
/// This function provides a non-parallel execution strategy. It iterates
/// through each target directory and runs all specified commands within it
/// before moving to the next. It correctly uses Tokio's non-blocking `Command`
/// to avoid stalling the async runtime.
///
/// # Arguments
///
/// * `Option`: An `ExecutionOption` struct containing the commands, paths, and
///   pattern.
pub async fn Fn(Option:ExecutionOption) {
	// Pre-parse command strings into their component parts once.
	let ProcessedCommands:Vec<Vec<String>> = Option
		.Command
		.iter()
		.map(|CommandString| CommandString.split_whitespace().map(String::from).collect())
		.collect();

	// Identify target directories where commands will be executed.
	let TargetDirs:Vec<PathBuf> = Option
		.Entry
		.into_iter()
		.filter_map(|CandidatePath| {
			if CandidatePath.file_name().map_or(false, |Name| Name == Option.Pattern.as_str()) {
				CandidatePath.parent().map(Path::to_path_buf)
			} else {
				None
			}
		})
		.collect();

	let mut tmp = TargetDirs.into_iter();

	while let Some(Directory) = tmp.next() {
		let DirectoryString = Directory.to_string_lossy();
		let mut tmp = ProcessedCommands.iter();
		while let Some(CommandParts) = tmp.next() {
			match CommandParts.is_empty() {
				true => continue,
				false => (),
			}

			// Execute the command using Tokio's async Command.
			let OutputResult = TokioCommand::new(&CommandParts[0])
				.args(&CommandParts[1..])
				.current_dir(DirectoryString.as_ref())
				.output()
				.await;

			match OutputResult {
				Ok(Output) => {
					let Stdout = String::from_utf8_lossy(&Output.stdout);
					if !Stdout.trim().is_empty() {
						println!("{}", Stdout);
					}
					if !Output.status.success() {
						let Stderr = String::from_utf8_lossy(&Output.stderr);
						eprintln!(
							"Command failed in '{}' with status {}. Stderr: {}",
							DirectoryString,
							Output.status,
							Stderr.trim()
						);
					}
				},
				Err(Error) => {
					eprintln!("Failed to spawn command in '{}': {}", DirectoryString, Error)
				},
			}
		}
	}
}
