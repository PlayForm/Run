use crossterm::event::{
	Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::Struct::Tui::AppState;

/// Process one crossterm event. Returns `true` if the app should quit.
pub fn Fn(State:&mut AppState, Ev:Event) -> bool {
	match Ev {
		Event::Key(KeyEvent { code, modifiers, .. }) => handle_key(State, code, modifiers),
		Event::Mouse(MouseEvent { kind, column, row, .. }) => {
			handle_mouse(State, kind, column, row);
			false
		}
		_ => false,
	}
}

fn handle_key(State:&mut AppState, Code:KeyCode, Mods:KeyModifiers) -> bool {
	match Code {
		// Quit
		KeyCode::Char('q') | KeyCode::Char('Q') => {
			State.ForceQuit = true;
			true
		}
		KeyCode::Char('c') if Mods.contains(KeyModifiers::CONTROL) => {
			State.ForceQuit = true;
			true
		}

		// Directory navigation
		KeyCode::Up | KeyCode::Char('k') => {
			State.select_up();
			false
		}
		KeyCode::Down | KeyCode::Char('j') => {
			State.select_down();
			false
		}

		// Log scroll
		KeyCode::PageUp => {
			State.scroll_up();
			false
		}
		KeyCode::PageDown => {
			State.scroll_down();
			false
		}

		// Toggle auto-scroll for the selected directory
		KeyCode::Char('s') | KeyCode::Char('S') => {
			State.toggle_autoscroll();
			false
		}

		_ => false,
	}
}

/// Handle mouse clicks: clicking a row in the left panel selects that directory.
///
/// Left panel is the first 30 % of the terminal width; we subtract the 1-char
/// border and the header row to compute the list row index.
fn handle_mouse(State:&mut AppState, Kind:MouseEventKind, Column:u16, Row:u16) {
	if !matches!(Kind, MouseEventKind::Down(MouseButton::Left)) {
		return;
	}

	// Approximate: terminal width is not available here without passing it in,
	// so we use the column heuristic: left panel spans columns 0 .. (width*0.3).
	// We instead use a fixed threshold of 40 columns which covers most terminals.
	// This is replaced by exact bounds in a future batch once we thread area
	// coordinates through Input.
	if Column < 40 && Row >= 2 {
		// Row 0 = border, Row 1 = title, Row 2+ = list items.
		State.click_row((Row - 2) as usize);
	}
}
