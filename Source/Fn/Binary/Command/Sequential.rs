use std::path::{Path, PathBuf};

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
