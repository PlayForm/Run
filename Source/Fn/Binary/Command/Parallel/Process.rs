use std::io;

/// Executes a command asynchronously and returns its output, handling errors
/// gracefully.
pub async fn Fn(command:&[String], entry:&str) -> io::Result<String> {
	if command.is_empty() {
		return Err(io::Error::new(io::ErrorKind::InvalidInput, "Empty command provided"));
	}

	let output = tokio::process::Command::new(&command[0])
		.args(&command[1..])
		.current_dir(entry)
		.output()
		.await?;

	if !output.status.success() {
		let stderr = String::from_utf8_lossy(&output.stderr);
		Err(io::Error::new(
			io::ErrorKind::Other,
			format!("Command failed with status {}. Stderr: {}", output.status, stderr),
		))
	} else {
		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}
}
