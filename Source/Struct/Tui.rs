use std::collections::HashMap;

/// Status of one directory entry shown in the left panel.
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
	Pending,
	Running { Done:usize, Total:usize },
	Done,
	Failed,
	Timeout,
}

/// Per-directory state kept by the TUI app.
#[derive(Debug, Clone)]
pub struct DirState {
	pub Directory:String,
	pub Status:Status,
	/// Accumulated output lines (stdout + stderr) for the log panel.
	pub Lines:Vec<(String, bool)>, // (text, is_stderr)
	/// Whether the log panel should auto-scroll to the bottom.
	pub AutoScroll:bool,
	/// Current scroll offset in the log panel for this directory.
	pub Scroll:usize,
}

impl DirState {
	pub fn new(Directory:String, Total:usize) -> Self {
		Self {
			Directory,
			Status:Status::Running { Done:0, Total },
			Lines:Vec::new(),
			AutoScroll:true,
			Scroll:0,
		}
	}
}

/// Spinner frames cycled on each tick for running jobs.
pub const SPINNER:&[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// The complete application state owned by the TUI event loop.
pub struct AppState {
	/// Ordered directory keys (insertion order = discovery order).
	pub Order:Vec<String>,
	/// Per-directory state, keyed by directory path string.
	pub Map:HashMap<String, DirState>,
	/// Currently selected row in the left panel (0-based).
	pub Selected:usize,
	/// Whether execution is fully complete.
	pub Done:bool,
	/// Monotonic tick counter — used to animate the spinner.
	pub Tick:usize,
	/// If `true`, `q` / Ctrl-C quits immediately even while running.
	pub ForceQuit:bool,
}

impl AppState {
	pub fn new() -> Self {
		Self {
			Order:Vec::new(),
			Map:HashMap::new(),
			Selected:0,
			Done:false,
			Tick:0,
			ForceQuit:false,
		}
	}

	/// Returns the currently selected directory key, if any.
	pub fn selected_dir(&self) -> Option<&str> {
		self.Order.get(self.Selected).map(String::as_str)
	}

	/// Move selection up by 1, clamping at 0.
	pub fn select_up(&mut self) {
		if self.Selected > 0 {
			self.Selected -= 1;
		}
	}

	/// Move selection down by 1, clamping at last entry.
	pub fn select_down(&mut self) {
		if self.Selected + 1 < self.Order.len() {
			self.Selected += 1;
		}
	}

	/// Scroll the log panel of the selected directory up.
	pub fn scroll_up(&mut self) {
		if let Some(Key) = self.selected_dir() {
			if let Some(State) = self.Map.get_mut(Key) {
				State.AutoScroll = false;
				State.Scroll = State.Scroll.saturating_sub(3);
			}
		}
	}

	/// Scroll the log panel of the selected directory down.
	pub fn scroll_down(&mut self) {
		if let Some(Key) = self.selected_dir().map(str::to_owned) {
			if let Some(State) = self.Map.get_mut(&Key) {
				let Max = State.Lines.len().saturating_sub(1);
				State.Scroll = (State.Scroll + 3).min(Max);
				if State.Scroll >= Max {
					State.AutoScroll = true;
				}
			}
		}
	}

	/// Toggle auto-scroll for the selected directory.
	pub fn toggle_autoscroll(&mut self) {
		if let Some(Key) = self.selected_dir().map(str::to_owned) {
			if let Some(State) = self.Map.get_mut(&Key) {
				State.AutoScroll = !State.AutoScroll;
			}
		}
	}

	/// Click on a row in the left panel (0-based row index inside the list).
	pub fn click_row(&mut self, Row:usize) {
		if Row < self.Order.len() {
			self.Selected = Row;
		}
	}
}
