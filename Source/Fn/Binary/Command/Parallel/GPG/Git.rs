use std::process::Command;

use once_cell::sync::OnceCell;

/// A static, lazily-initialized cell to cache the result of the Git GPG check.
///
/// Using `OnceCell` is the idiomatic, thread-safe way to ensure a value is
/// computed only once and then read cheaply by all subsequent calls across all
/// threads.
static IS_SIGNING_ENABLED:OnceCell<bool> = OnceCell::new();

/// Checks if Git commit signing (`commit.gpgsign`) is enabled in the Git
/// configuration.
///
/// The result is cached globally after the first invocation to avoid repeated,
/// slow shell command executions.
///
/// # Returns
///
/// `true` if `git config --get commit.gpgsign` returns "true", otherwise
/// `false`.
pub fn Fn() -> bool {
	*IS_SIGNING_ENABLED.get_or_init(|| {
		// This closure is only executed the very first time `Fn` is called.
		if let Ok(Output) = Command::new("git").args(["config", "--get", "commit.gpgsign"]).output() {
			// Check if the command output, when trimmed, is exactly "true".
			String::from_utf8_lossy(&Output.stdout).trim() == "true"
		} else {
			// If the git command fails (e.g., git not installed), assume signing is off.
			false
		}
	})
}
