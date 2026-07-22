use std::io;

use tokio::io::AsyncBufReadExt;
use tokio::process::Command as TokioCommand;

/// Executes a command asynchronously in a specified directory.
///
/// This function spawns a process using Tokio's non-blocking `Command`,
/// streams stdout into a local buffer line by line as it's produced,
/// captures stderr, and checks the exit status on completion.
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

	// Use spawn + piped streams so the process runs concurrently and we can
	// collect stdout as it arrives, rather than buffering it all at the end.
	let mut Child = TokioCommand::new(&CommandParts[0])
		.args(&CommandParts[1..])
		.current_dir(EntryDirectory)
		.stdout(std::process::Stdio::piped())
		.stderr(std::process::Stdio::piped())
		.spawn()?;

	// Collect stdout line by line as it's produced.
	let mut StdoutBuf = String::new();
	{
		let StdoutReader = Child.stdout.take().unwrap();
		let mut Lines = tokio::io::BufReader::new(StdoutReader).lines();
		while let Ok(Some(Line)) = Lines.next_line().await {
			StdoutBuf.push_str(&Line);
			StdoutBuf.push('\n');
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

	let Status = Child.wait().await?;

	if !Status.success() {
		Err(io::Error::other(format!(
			"Command failed with status {}. Stderr: {}",
			Status, StderrBuf.trim()
		)))
	} else {
		Ok(StdoutBuf)
	}
}
