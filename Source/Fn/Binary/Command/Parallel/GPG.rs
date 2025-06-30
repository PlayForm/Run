pub mod Git;

/// Determines if a given command requires Git commit signing.
pub fn Fn(command:&[String]) -> bool {
	if command.first().map(String::as_str) == Some("git") {
		match command.get(1).map(String::as_str) {
			Some("commit" | "tag" | "merge" | "rebase" | "cherry-pick") => Git::Fn(),
			_ => false,
		}
	} else {
		false
	}
}
