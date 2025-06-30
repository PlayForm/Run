pub mod Git;

/// Determines if a given command is a Git operation that may require GPG
/// signing.
///
/// This function is used to decide whether the global GPG mutex needs to be
/// acquired before executing a command in parallel, preventing race conditions
/// with the GPG agent.
///
/// # Arguments
///
/// * `CommandParts`: A slice of strings representing the parsed command.
///
/// # Returns
///
/// `true` if the command is a signing-related Git command, `false` otherwise.
pub fn Fn(CommandParts:&[String]) -> bool {
	// Check if the first part of the command is "git".
	if CommandParts.first().map(String::as_str) == Some("git") {
		// Check the subcommand (e.g., "commit", "tag").
		match CommandParts.get(1).map(String::as_str) {
			Some("commit" | "tag" | "merge" | "rebase" | "cherry-pick") => Git::Fn(),
			_ => false,
		}
	} else {
		false
	}
}
