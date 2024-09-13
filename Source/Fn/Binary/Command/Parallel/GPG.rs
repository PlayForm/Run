pub mod Git;

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
