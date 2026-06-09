use std::{path::Path, time::Duration};

use tokio::time::sleep;

/// Maximum time in milliseconds to wait for a git index lock to be released.
const TIMEOUT_MS:u64 = 30_000;

/// Initial polling interval in milliseconds when the lock is first detected.
const POLL_INITIAL_MS:u64 = 50;

/// Maximum polling interval cap in milliseconds (exponential backoff ceiling).
const POLL_MAX_MS:u64 = 500;

/// Lock file age in seconds above which it is treated as stale and removed.
const STALE_AGE_SECS:u64 = 30;

/// Waits for the `.git/index.lock` file in `Directory` to be released.
///
/// Polls with exponential backoff, starting at `POLL_INITIAL_MS` ms and
/// doubling each attempt up to `POLL_MAX_MS` ms, for a total of `TIMEOUT_MS`
/// ms. If the lock file's modification time is older than `STALE_AGE_SECS`
/// seconds, it is removed immediately as a leftover from a killed process.
///
/// # Arguments
///
/// * `Directory`: Path to the git repository root (the parent of `.git/`).
///
/// # Returns
///
/// `true` if the index is free before the timeout elapses, `false` on timeout.
pub async fn Fn(Directory:&str) -> bool {
	Inner(Directory, TIMEOUT_MS, POLL_INITIAL_MS, STALE_AGE_SECS).await
}

async fn Inner(
	Directory:&str,
	TimeoutMs:u64,
	PollInitialMs:u64,
	StaleAgeSecs:u64,
) -> bool {
	let LockPath = Path::new(Directory).join(".git").join("index.lock");

	if !LockPath.exists() {
		return true;
	}

	// A lock file older than StaleAgeSecs was left by a killed or crashed process.
	if let Ok(Metadata) = std::fs::metadata(&LockPath)
		&& let Ok(Modified) = Metadata.modified()
		&& let Ok(Age) = Modified.elapsed()
		&& Age.as_secs() >= StaleAgeSecs
	{
		eprintln!(
			"Removing stale git index lock in '{}' (age: {}s).",
			Directory,
			Age.as_secs()
		);
		if let Err(Error) = std::fs::remove_file(&LockPath) {
			eprintln!("Failed to remove stale index lock in '{}': {}", Directory, Error);
		}
		return !LockPath.exists();
	}

	let mut Elapsed:u64 = 0;
	let mut Interval:u64 = PollInitialMs;

	loop {
		if Elapsed >= TimeoutMs {
			eprintln!(
				"Timed out waiting {}ms for git index lock in '{}'.",
				TimeoutMs, Directory
			);
			return false;
		}

		sleep(Duration::from_millis(Interval)).await;
		Elapsed += Interval;
		Interval = (Interval * 2).min(POLL_MAX_MS);

		if !LockPath.exists() {
			return true;
		}
	}
}

#[cfg(test)]
mod Tests {
	use std::{fs, path::PathBuf};

	use super::Inner;

	/// Creates a temporary directory with a `.git/` subdirectory and returns the root path.
	fn TempGitDir(Label:&str) -> PathBuf {
		let Dir = std::env::temp_dir()
			.join(format!("prun_index_lock_test_{}_{}", Label, std::process::id()));
		fs::create_dir_all(Dir.join(".git")).expect("create temp .git dir");
		Dir
	}

	/// Cleans up a temporary directory created by `TempGitDir`.
	fn Cleanup(Dir:&PathBuf) { let _ = fs::remove_dir_all(Dir); }

	#[tokio::test]
	async fn Returns_True_When_No_Lock_File_Exists() {
		let Dir = TempGitDir("no_lock");
		// No index.lock created - should return immediately.
		let Result = Inner(&Dir.to_string_lossy(), 500, 10, 0).await;
		Cleanup(&Dir);
		assert!(Result);
	}

	#[tokio::test]
	async fn Returns_True_When_Lock_Released_Before_Timeout() {
		let Dir = TempGitDir("released");
		let LockFile = Dir.join(".git").join("index.lock");
		fs::write(&LockFile, b"").expect("write lock file");

		let LockFileClone = LockFile.clone();
		tokio::spawn(async move {
			tokio::time::sleep(std::time::Duration::from_millis(80)).await;
			let _ = fs::remove_file(LockFileClone);
		});

		let Result = Inner(&Dir.to_string_lossy(), 2_000, 10, 30).await;
		Cleanup(&Dir);
		assert!(Result);
	}

	#[tokio::test]
	async fn Returns_False_When_Lock_Never_Released() {
		let Dir = TempGitDir("timeout");
		let LockFile = Dir.join(".git").join("index.lock");
		fs::write(&LockFile, b"").expect("write lock file");

		// Use a very short timeout so the test completes quickly.
		let Result = Inner(&Dir.to_string_lossy(), 150, 10, 30).await;
		Cleanup(&Dir);
		assert!(!Result);
	}

	#[tokio::test]
	async fn Removes_Stale_Lock_And_Returns_True() {
		let Dir = TempGitDir("stale");
		let LockFile = Dir.join(".git").join("index.lock");
		fs::write(&LockFile, b"").expect("write lock file");

		// StaleAgeSecs = 0 so any existing lock is immediately treated as stale.
		let Result = Inner(&Dir.to_string_lossy(), 500, 10, 0).await;
		// Assert before Cleanup so the check is not trivially true from directory removal.
		assert!(Result);
		assert!(!LockFile.exists(), "stale lock should have been removed by Inner");
		Cleanup(&Dir);
	}

	#[tokio::test]
	async fn Returns_True_For_Missing_Git_Dir() {
		let Dir = std::env::temp_dir()
			.join(format!("prun_index_lock_test_no_git_{}", std::process::id()));
		// Deliberately do NOT create the .git subdirectory.
		fs::create_dir_all(&Dir).expect("create dir");

		let Result = Inner(&Dir.to_string_lossy(), 500, 10, 0).await;
		let _ = fs::remove_dir_all(&Dir);
		assert!(Result);
	}
}
