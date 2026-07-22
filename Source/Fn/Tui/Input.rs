use crossterm::event::{
	Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};

use crate::Struct::Tui::AppState;

pub fn Fn(State:　&mut AppState, Ev:　Event) -> bool {
	match Ev {
		Event::Key(KeyEvent { code, modifiers, .. }) => handle_key(State, code, modifiers),
		Event::Mouse(MouseEvent { kind, column, row, .. }) => {
			handle_mouse(State, kind, column, row);
			false
		}
		_ => false,
	}
}

fn handle_key(State:　&mut AppState, Code:　KeyCode, Mods:　KeyModifiers) -> bool {
	match Code {
		KeyCode::Char('q') | KeyCode::Char('Q') => {
			State.ForceQuit = true;
			true
		}
		KeyCode::Char('c') if Mods.contains(KeyModifiers::CONTROL) => {
			State.ForceQuit = true;
			true
		}
		KeyCode::Up | KeyCode::Char('k') => {
			State.select_up();
			false
		}
		KeyCode::Down | KeyCode::Char('j') => {
			State.select_down();
			false
		}
		KeyCode::PageUp => {
			State.scroll_up();
			false
		}
		KeyCode::PageDown => {
			State.scroll_down();
			false
		}
		KeyCode::Char('s') | KeyCode::Char('S') => {
			State.toggle_autoscroll();
			false
		}
		_ => false,
	}
}

fn handle_mouse(State:　&mut AppState, Kind:　MouseEventKind, Column:　u16, Row:　u16) {
	if !matches!(Kind, MouseEventKind::Down(MouseButton::Left)) {
		return;
	}
	if Column < 40 && Row >= 2 {
		State.click_row((Row - 2) as usize);
	}
}
