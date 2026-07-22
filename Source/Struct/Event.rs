/// A single typed event emitted by an execution engine (Sequential or
/// Parallel) and consumed by either the plain CLI printer or the TUI app.
///
/// Both engines emit these instead of calling `println!`/`eprintln!` directly,
/// keeping execution logic completely I/O-free and making the TUI a zero-cost
/// opt-in.
#[derive(Debug, Clone)]
pub enum Struct {
	/// A directory has been picked up and its first command is about to run.
	JobStarted {
		Directory:String,
		/// Total number of commands queued for this directory.
		Total:usize,
	},
	/// One line of stdout or stderr arrived from a running command.
	Line {
		Directory:String,
		Text:String,
		/// `true` → stderr (rendered in red in the TUI).
		IsStderr:bool,
	},
	/// A single command inside a directory finished.
	JobProgress {
		Directory:String,
		/// Index of the command that just finished (0-based).
		Done:usize,
		Total:usize,
		Success:bool,
	},
	/// All commands inside a directory have finished.
	JobFinished { Directory:String, Success:bool },
	/// The index-lock timed out; remaining commands for this directory skipped.
	IndexLockTimeout { Directory:String },
	/// Every directory has been processed — engines send this last.
	AllDone,
}
