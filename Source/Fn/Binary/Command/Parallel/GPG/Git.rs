pub fn Fn() -> bool {
	if !COMMIT_SIGNING_CHECKED.load(Ordering::Relaxed) {
		COMMIT_SIGNING_ENABLED.store(
			String::from_utf8_lossy(
				&Command::new("git")
					.args(&["config", "--get", "commit.gpgsign"])
					.output()
					.expect("Failed to execute git command")
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
