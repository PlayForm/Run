use std::io;

use tokio::process::Command as TokioCommand;

/// Executes a command asynchronously in a specified directory.
///
/// This function spawns a process using Tokio's non-blocking `Command`,
/// captures its output, and handles success or failure cases gracefully.
///
/// # Arguments
///
/// * `CommandParts`: A slice of strings representing the command and its
///   arguments.
/// * `EntryDirectory`: The working directory in which to execute the command.
///
/// # Returns
///
/// A `Result` containing the command's stdout as a `String` on success, or an
/// `io::Error` on failure.
pub async fn Fn(CommandParts:&[String], EntryDirectory:&str) -> io::Result<String> {
	if CommandParts.is_empty() {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command provided"));
	}

	let Output = TokioCommand::new(&CommandParts[0])
		.args(&CommandParts[1..])
		.current_dir(EntryDirectory)
		.output()
		.await?; // Propagate I/O errors from spawning the process.

	if !Output.status.success() {
		let Stderr = String::from_utf8_lossy(&Output.stderr);
		Err(io::Error::new(
			io::ErrorKind::Other,
			format!("Command failed with status {}. Stderr: {}", Output.status, Stderr),
		))
	} else {
		Ok(String::from_utf8_lossy(&Output.stdout).to_string())
	}
}
