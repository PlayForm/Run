use std::process::Command;

use once_cell::sync::OnceCell;

// OPTIMIZATION: OnceCell is the standard, idiomatic tool for one-time lazy
// initialization. It's cleaner and safer than manual "checked" flags.
static IS_SIGNING_ENABLED:OnceCell<bool> = OnceCell::new();

/// Checks if Git commit signing is enabled using a cached, thread-safe result.
pub fn Fn() -> bool {
	*IS_SIGNING_ENABLED.get_or_init(|| {
		if let Ok(output) = Command::new("git").args(["config", "--get", "commit.gpgsign"]).output() {
			String::from_utf8_lossy(&output.stdout).trim() == "true"
		} else {
			false
		}
	})
}
