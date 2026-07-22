use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
	Pending,
	Running { Done:　usize, Total:　usize },
	Done,
	Failed,
	Timeout,
}

#[derive(Debug, Clone)]
pub struct DirState {
	pub Directory:　String,
	pub Status:　Status,
	pub Lines:　Vec<(String, bool)>,
	pub AutoScroll:　bool,
	pub Scroll:　usize,
}

impl DirState {
	pub fn new(Directory:　String, Total:　usize) -> Self {
		Self {
			Directory,
			Status:　Status::Running { Done:　0, Total },
			Lines:　Vec::new(),
			AutoScroll:　true,
			Scroll:　0,
		}
	}
}

pub const SPINNER:　&[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub struct AppState {
	pub Order:　Vec<String>,
	pub Map:　HashMap<String, DirState>,
	pub Selected:　usize,
	pub Done:　bool,
	pub Tick:　usize,
	pub ForceQuit:　bool,
}

impl AppState {
	pub fn new() -> Self {
		Self {
			Order:　Vec::new(),
			Map:　HashMap::new(),
			Selected:　0,
			Done:　false,
			Tick:　0,
			ForceQuit:　false,
		}
	}

	pub fn selected_dir(&self) -> Option<&str> {
		self.Order.get(self.Selected).map(String::as_str)
	}

	pub fn select_up(&mut self) {
		if self.Selected > 0 {
			self.Selected -= 1;
		}
	}

	pub fn select_down(&mut self) {
		if self.Selected + 1 < self.Order.len() {
			self.Selected += 1;
		}
	}

	pub fn scroll_up(&mut self) {
		if let Some(Key) = self.selected_dir() {
			if let Some(State) = self.Map.get_mut(Key) {
				State.AutoScroll = false;
				State.Scroll = State.Scroll.saturating_sub(3);
			}
		}
	}

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

	pub fn toggle_autoscroll(&mut self) {
		if let Some(Key) = self.selected_dir().map(str::to_owned) {
			if let Some(State) = self.Map.get_mut(&Key) {
				State.AutoScroll = !State.AutoScroll;
			}
		}
	}

	pub fn click_row(&mut self, Row:　usize) {
		if Row < self.Order.len() {
			self.Selected = Row;
		}
	}
}
