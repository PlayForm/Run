pub mod Input;
pub mod Render;

use std::io::stdout;
use std::time::Duration;

use crossterm::{
	event::{self, DisableMouseCapture, EnableMouseCapture},
	execute,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc::Receiver;

use crate::Struct::{Event::Struct as Event, Tui::AppState};

struct TerminalGuard;

impl Drop for TerminalGuard {
	fn drop(&mut self) {
		let _ = disable_raw_mode();
		let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
	}
}

pub async fn Fn(mut Rx:　Receiver<Event>) {
	enable_raw_mode().expect("Failed to enable raw mode");
	execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)
		.expect("Failed to enter alternate screen");

	let _Guard = TerminalGuard;

	let Backend = CrosstermBackend::new(stdout());
	let mut Term = Terminal::new(Backend).expect("Failed to create terminal");

	let mut State = AppState::new();
	let TickRate = Duration::from_millis(100);

	loop {
		loop {
			match Rx.try_recv() {
				Ok(Event::JobStarted { Directory, Total }) => {
					if !State.Map.contains_key(&Directory) {
						State.Order.push(Directory.clone());
						State
							.Map
							.insert(Directory.clone(), crate::Struct::Tui::DirState::new(Directory, Total));
					}
				}
				Ok(Event::Line { Directory, Text, IsStderr }) => {
					if let Some(DS) = State.Map.get_mut(&Directory) {
						DS.Lines.push((Text, IsStderr));
						if DS.AutoScroll {
							DS.Scroll = DS.Lines.len().saturating_sub(1);
						}
					}
				}
				Ok(Event::JobProgress { Directory, Done, Total, .. }) => {
					if let Some(DS) = State.Map.get_mut(&Directory) {
						DS.Status = crate::Struct::Tui::Status::Running { Done, Total };
					}
				}
				Ok(Event::JobFinished { Directory, Success }) => {
					if let Some(DS) = State.Map.get_mut(&Directory) {
						DS.Status = if Success {
							crate::Struct::Tui::Status::Done
						} else {
							crate::Struct::Tui::Status::Failed
						};
					}
				}
				Ok(Event::IndexLockTimeout { Directory }) => {
					if let Some(DS) = State.Map.get_mut(&Directory) {
						DS.Status = crate::Struct::Tui::Status::Timeout;
						DS.Lines.push(("⚠  git index lock timed out".to_owned(), true));
					}
				}
				Ok(Event::AllDone) => {
					State.Done = true;
				}
				Err(_) => break,
			}
		}

		Term.draw(|Frame| Render::Fn(Frame, &State))
			.expect("Failed to draw frame");

		if event::poll(TickRate).unwrap_or(false) {
			if let Ok(Ev) = event::read() {
				if Input::Fn(&mut State, Ev) {
					break;
				}
			}
		}

		State.Tick = State.Tick.wrapping_add(1);

		if State.ForceQuit {
			break;
		}
	}
}
