pub mod Git;

/// Determines if a given command requires Git commit signing.
///
/// This function checks if the provided command is a Git command that typically
/// requires commit signing. It supports various Git operations such as commit,
/// tag, merge, rebase, cherry-pick, notes, push, and pull.
///
/// # Arguments
///
/// * `Command` - A slice of strings representing the command and its arguments.
///
/// # Returns
///
/// * `bool` - `true` if the command requires commit signing, `false` otherwise.
///
/// # Examples
///
/// ```
/// let command = vec!["git".to_string(), "commit".to_string()];
/// let requires_signing = Fn(&command);
/// assert!(requires_signing);
///
/// let command = vec!["git".to_string(), "status".to_string()];
/// let requires_signing = Fn(&command);
/// assert!(!requires_signing);
/// ```
pub fn Fn(Command: &[String]) -> bool {
	if Command.get(0) == Some(&"git".to_string()) {
		match Command.get(1).map(String::as_str) {
			Some("commit") | Some("ecommit") | Some("tag") | Some("merge") => Git::Fn(),
			Some("rebase") | Some("cherry-pick") => true,
			Some("notes") => Command.get(2) == Some(&"add".to_string()),
			Some("push") | Some("pull") => true,
			_ => false,
		}
	} else {
		false
	}
}
