// src/Parallel/Process.rs

use std::process::Stdio;

use tokio::process::Command;

/// Executes a command asynchronously, capturing both stdout and stderr.
///
/// This function prevents race conditions by redirecting both output streams
/// from the child process and capturing them, rather than letting them print
//  directly to the console.
/// # Arguments
/// * `Command` - A slice of strings representing the command and its arguments.
/// * `Entry` - The directory in which to execute the command.
///
/// # Returns
/// A single `String` containing the combined output from both stdout and
/// stderr.
pub async fn Fn(CommandParts:&[String], Entry:&str) -> String {
	// Configure the command to run in the specified directory.
	let mut command = Command::new(CommandParts.get(0).expect("Cannot Command: command is empty."));
	command.args(&CommandParts[1..]);
	command.current_dir(Entry);

	// CRITICAL: Redirect stdout and stderr to a pipe so we can capture them.
	// This prevents the child process from writing directly to our terminal.
	command.stdout(Stdio::piped());
	command.stderr(Stdio::piped());

	// Execute the command and wait for all output.
	let output = command.output().await.expect("Failed to execute command.");

	// Convert both stdout and stderr from bytes to strings.
	let stdout_str = String::from_utf8_lossy(&output.stdout);
	let stderr_str = String::from_utf8_lossy(&output.stderr);

	// Combine the outputs for a complete picture.
	// We trim to remove unnecessary leading/trailing whitespace.
	let mut combined_output = String::new();
	if !stdout_str.is_empty() {
		combined_output.push_str(stdout_str.trim());
	}
	if !stderr_str.is_empty() {
		// Add a newline if both streams have content.
		if !combined_output.is_empty() {
			combined_output.push('\n');
		}
		combined_output.push_str(stderr_str.trim());
	}

	// You could also add error handling here if you want.
	// For example, if !output.status.success() { ... }

	combined_output
}
