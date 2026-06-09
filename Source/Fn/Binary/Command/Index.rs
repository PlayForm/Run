pub mod Lock;

/// Determines if a given command is a Git operation that modifies the index.
///
/// Index-modifying operations require checking for an existing
/// `.git/index.lock` file before execution, as concurrent or chained git
/// commands on the same repository fail when the index is already locked.
///
/// # Arguments
///
/// * `CommandParts`: A slice of strings representing the parsed command.
///
/// # Returns
///
/// `true` if the command is a git command that writes to the index,
/// `false` otherwise.
pub fn Fn(CommandParts:&[String]) -> bool {
	if CommandParts.first().map(String::as_str) != Some("git") {
		return false;
	}

	matches!(
		CommandParts.get(1).map(String::as_str),
		Some(
			"add"
				| "apply"
				| "checkout"
				| "cherry-pick"
				| "commit"
				| "merge"
				| "mv"
				| "rebase"
				| "reset"
				| "restore"
				| "rm"
				| "stash"
		)
	)
}

#[cfg(test)]
mod Tests {
	use super::Fn;

	fn Parts(Cmd:&[&str]) -> Vec<String> { Cmd.iter().map(|S| S.to_string()).collect() }

	#[test]
	fn Add_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "add", "."]))); }

	#[test]
	fn Commit_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "commit", "-m", "msg"]))); }

	#[test]
	fn Reset_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "reset", "--hard"]))); }

	#[test]
	fn Checkout_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "checkout", "main"]))); }

	#[test]
	fn Merge_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "merge", "feature"]))); }

	#[test]
	fn Rebase_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "rebase", "main"]))); }

	#[test]
	fn Stash_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "stash"]))); }

	#[test]
	fn Rm_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "rm", "file.txt"]))); }

	#[test]
	fn Mv_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "mv", "a", "b"]))); }

	#[test]
	fn Apply_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "apply", "patch.diff"]))); }

	#[test]
	fn Restore_Is_Index_Modifying() { assert!(Fn(&Parts(&["git", "restore", "file.txt"]))); }

	#[test]
	fn Cherry_Pick_Is_Index_Modifying() {
		assert!(Fn(&Parts(&["git", "cherry-pick", "abc123"])));
	}

	#[test]
	fn Status_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "status"]))); }

	#[test]
	fn Log_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "log"]))); }

	#[test]
	fn Push_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "push"]))); }

	#[test]
	fn Fetch_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "fetch"]))); }

	#[test]
	fn Pull_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "pull"]))); }

	#[test]
	fn Diff_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&["git", "diff"]))); }

	#[test]
	fn Non_Git_Command_Is_Not_Index_Modifying() {
		assert!(!Fn(&Parts(&["cargo", "build"])));
	}

	#[test]
	fn Empty_Command_Is_Not_Index_Modifying() { assert!(!Fn(&Parts(&[]))); }

	#[test]
	fn Bare_Git_With_No_Subcommand_Is_Not_Index_Modifying() {
		assert!(!Fn(&Parts(&["git"])));
	}
}
