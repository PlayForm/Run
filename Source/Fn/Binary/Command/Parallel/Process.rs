use std::io;

use tokio::io::AsyncBufReadExt;
use tokio::process::Command as TokioCommand;

/// Executes a command asynchronously in a specified directory via `sh -c`.
///
/// This function spawns a shell so that `~`, `$HOME`, pipes, redirects, and
/// other shell features work.  stdout is collected line-by-line into a local
/// buffer; stderr is captured for error reporting.
///
/// # Arguments
///
/// * `CommandString`: The full command string as provided by the user.
/// * `EntryDirectory`: The working directory in which to execute the command.
///
/// # Returns
///
/// A `Result` containing the command's stdout as a `String` on success, or an
/// `io::Error` on failure.
pub async fn Fn(CommandString:&str, EntryDirectory:&str) -> io::Result<String> {
	let Trimmed = CommandString.trim();
	if Trimmed.is_empty() {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command provided"));
	}

	// Use `sh -c` so shell expansion works.
	let mut Child = TokioCommand::new("sh")
		.args(["-c", Trimmed])
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
