pub mod Lock;

/// Determines if a given command string is a Git operation that modifies the
/// index.
///
/// The command string is split on whitespace for classification only;
/// execution always goes through `sh -c` so the original string is preserved.
///
/// # Arguments
///
/// * `Command`: The full command string as provided by the user.
///
/// # Returns
///
/// `true` if the command is a git command that writes to the index,
/// `false` otherwise.
pub fn Fn(Command:&str) -> bool {
	let Parts:Vec<&str> = Command.split_whitespace().collect();

	if Parts.first().copied() != Some("git") {
		return false;
	}

	matches!(
		Parts.get(1).copied(),
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

	#[test]
	fn Add_Is_Index_Modifying() { assert!(Fn("git add .")); }

	#[test]
	fn Commit_Is_Index_Modifying() { assert!(Fn("git commit -m msg")); }

	#[test]
	fn Reset_Is_Index_Modifying() { assert!(Fn("git reset --hard")); }

	#[test]
	fn Checkout_Is_Index_Modifying() { assert!(Fn("git checkout main")); }

	#[test]
	fn Merge_Is_Index_Modifying() { assert!(Fn("git merge feature")); }

	#[test]
	fn Rebase_Is_Index_Modifying() { assert!(Fn("git rebase main")); }

	#[test]
	fn Stash_Is_Index_Modifying() { assert!(Fn("git stash")); }

	#[test]
	fn Rm_Is_Index_Modifying() { assert!(Fn("git rm file.txt")); }

	#[test]
	fn Mv_Is_Index_Modifying() { assert!(Fn("git mv a b")); }

	#[test]
	fn Apply_Is_Index_Modifying() { assert!(Fn("git apply patch.diff")); }

	#[test]
	fn Restore_Is_Index_Modifying() { assert!(Fn("git restore file.txt")); }

	#[test]
	fn Cherry_Pick_Is_Index_Modifying() {
		assert!(Fn("git cherry-pick abc123"));
	}

	#[test]
	fn Status_Is_Not_Index_Modifying() { assert!(!Fn("git status")); }

	#[test]
	fn Log_Is_Not_Index_Modifying() { assert!(!Fn("git log")); }

	#[test]
	fn Push_Is_Not_Index_Modifying() { assert!(!Fn("git push")); }

	#[test]
	fn Fetch_Is_Not_Index_Modifying() { assert!(!Fn("git fetch")); }

	#[test]
	fn Pull_Is_Not_Index_Modifying() { assert!(!Fn("git pull")); }

	#[test]
	fn Diff_Is_Not_Index_Modifying() { assert!(!Fn("git diff")); }

	#[test]
	fn Non_Git_Command_Is_Not_Index_Modifying() {
		assert!(!Fn("cargo build"));
	}

	#[test]
	fn Empty_Command_Is_Not_Index_Modifying() { assert!(!Fn("")); }

	#[test]
	fn Bare_Git_With_No_Subcommand_Is_Not_Index_Modifying() {
		assert!(!Fn("git"));
	}
}
