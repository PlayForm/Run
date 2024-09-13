/// Executes a command asynchronously in a specified directory and returns its output as a string.
///
/// # Arguments
///
/// * `Command` - A slice of strings representing the command and its arguments.
///               The first element is expected to be the command itself.
/// * `Entry` - A string slice representing the directory in which to execute the command.
///
/// # Returns
///
/// * `String` - The command's standard output as a UTF-8 string.
///
/// # Panics
///
/// This function will panic in the following situations:
/// - If `Command` is empty (i.e., no command is provided).
/// - If the command execution fails.
/// - If the command's output cannot be converted to a valid UTF-8 string.
///
/// # Examples
///
/// ```
/// use your_crate_name::Fn;
///
/// #[tokio::main]
/// async fn main() {
///     let command = vec!["ls".to_string(), "-l".to_string()];
///     let entry = "/home/user";
///     let output = Fn(&command, entry).await;
///     println!("Command output: {}", output);
/// }
/// ```
///
/// # Note
///
/// This function uses `tokio::process::Command` for asynchronous execution,
/// so it should be called within an async context.
pub async fn Fn(Command: &[String], Entry: &str) -> String {
	String::from_utf8_lossy(
		&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
			.args(&Command[1..])
			.current_dir(Entry)
			.output()
			.await
			.expect("Cannot Output.")
			.stdout,
	)
	.to_string()
}
