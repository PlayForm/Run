use std::path::{Path, PathBuf};

// CRITICAL FIX: Use the non-blocking version of Command for async functions.
use tokio::process::Command;

use crate::Struct::Binary::Command::Entry::Struct as Option;

pub async fn Fn(Option { Command, Entry, Pattern, .. }:Option) {
	// OPTIMIZATION: Pre-parse command strings once.
	let processed_commands:Vec<Vec<String>> = Command
		.iter()
		.map(|cmd_str| cmd_str.split_whitespace().map(String::from).collect())
		.collect();

	// OPTIMIZATION: Use efficient PathBuf methods to find target directories.
	let target_dirs:Vec<PathBuf> = Entry
		.into_iter()
		.filter_map(|path| {
			if path.file_name().map_or(false, |name| name == Pattern.as_str()) {
				path.parent().map(Path::to_path_buf)
			} else {
				None
			}
		})
		.collect();

	for dir in target_dirs {
		let dir_str = dir.to_string_lossy();
		for cmd_parts in &processed_commands {
			if cmd_parts.is_empty() {
				continue;
			}

			// Use tokio's Command and .await to keep the runtime unblocked.
			let output_result = Command::new(&cmd_parts[0])
				.args(&cmd_parts[1..])
				.current_dir(dir_str.as_ref())
				.output()
				.await;

			match output_result {
				Ok(output) => {
					let stdout = String::from_utf8_lossy(&output.stdout);
					if !stdout.trim().is_empty() {
						println!("{}", stdout);
					}
					if !output.status.success() {
						let stderr = String::from_utf8_lossy(&output.stderr);
						eprintln!(
							"Command failed in '{}' with status {}. Stderr: {}",
							dir_str,
							output.status,
							stderr.trim()
						);
					}
				},
				Err(e) => eprintln!("Failed to spawn command in '{}': {}", dir_str, e),
			}
		}
	}
}
