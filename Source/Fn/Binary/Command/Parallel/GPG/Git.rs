/// Checks if Git commit signing is enabled.
///
/// This function determines whether Git commit signing is enabled by checking
/// the Git configuration. It uses a caching mechanism to avoid repeated Git
/// command executions.
///
/// # Returns
///
/// * `bool` - `true` if commit signing is enabled, `false` otherwise.
///
/// # Panics
///
/// This function will panic if it fails to execute the Git command to check
/// the commit signing configuration.
///
/// # Thread Safety
///
/// This function uses atomic operations for thread-safe access to shared state.
///
/// # Examples
///
/// ```
/// let is_signing_enabled = Fn();
/// println!("Commit signing is enabled: {}", is_signing_enabled);
/// ```
pub fn Fn() -> bool {
	if !COMMIT_SIGNING_CHECKED.load(Ordering::Relaxed) {
		COMMIT_SIGNING_ENABLED.store(
			String::from_utf8_lossy(
				&Command::new("git")
					.args(&["config", "--get", "commit.gpgsign"])
					.output()
					.expect("Cannot output.")
					.stdout,
			)
			.trim() == "true",
			Ordering::Relaxed,
		);

		COMMIT_SIGNING_CHECKED.store(true, Ordering::Relaxed);
	}

	COMMIT_SIGNING_ENABLED.load(Ordering::Relaxed)
}

use once_cell::sync::Lazy;
use std::{
	process::Command,
	sync::atomic::{AtomicBool, Ordering},
};

static COMMIT_SIGNING_ENABLED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
static COMMIT_SIGNING_CHECKED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));
