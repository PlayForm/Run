use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Rect},
	style::{Color, Modifier, Style},
	text::{Line as TLine, Span},
	widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::Struct::Tui::{AppState, SPINNER, Status};

/// Top-level render function — called once per tick.
///
/// Layout (horizontal split):
///   ┌────────────────┬──────────────────────────────────────┐
///   │  Directories   │  Log                                  │
///   │  (30 % width)  │  (70 % width)                        │
///   └────────────────┴──────────────────────────────────────┘
/// Status bar spans the full width at the bottom (1 line).
pub fn Fn(Frame:&mut Frame, State:&AppState) {
	let Size = Frame.area();

	// Reserve 1 row at the bottom for the status bar.
	let Outer = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Min(3), Constraint::Length(1)])
		.split(Size);

	let Panels = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
		.split(Outer[0]);

	render_dir_list(Frame, State, Panels[0]);
	render_log(Frame, State, Panels[1]);
	render_status_bar(Frame, State, Outer[1]);
}

// ─── Left panel: directory list ──────────────────────────────────────────────

fn render_dir_list(Frame:&mut Frame, State:&AppState, Area:Rect) {
	let Spinner = SPINNER[State.Tick % SPINNER.len()];

	let Items:Vec<ListItem> = State
		.Order
		.iter()
		.enumerate()
		.map(|(I, Key)| {
			let DS = &State.Map[Key];

			let (Icon, IconStyle) = match &DS.Status {
				Status::Pending => ("○ ".to_owned(), Style::default().fg(Color::DarkGray)),
				Status::Running { Done, Total } => (
					format!("{} {}/{} ", Spinner, Done, Total),
					Style::default().fg(Color::Yellow),
				),
				Status::Done => ("✓ ".to_owned(), Style::default().fg(Color::Green)),
				Status::Failed => ("✗ ".to_owned(), Style::default().fg(Color::Red)),
				Status::Timeout => ("⚠ ".to_owned(), Style::default().fg(Color::Magenta)),
			};

			// Shorten long paths: keep the last two path components only.
			let Label = shorten(Key);

			let Row = TLine::from(vec![
				Span::styled(Icon, IconStyle),
				Span::raw(Label),
			]);

			let Style = if I == State.Selected {
				Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
			} else {
				Style::default()
			};

			ListItem::new(Row).style(Style)
		})
		.collect();

	let Block = Block::default()
		.title(" ◈ Directories ")
		.borders(Borders::ALL)
		.border_style(Style::default().fg(Color::Cyan));

	let List = List::new(Items).block(Block);
	let mut ListSt = ListState::default();
	ListSt.select(Some(State.Selected));
	Frame.render_stateful_widget(List, Area, &mut ListSt);
}

// ─── Right panel: log viewer ─────────────────────────────────────────────────

fn render_log(Frame:&mut Frame, State:&AppState, Area:Rect) {
	let (Title, Lines, Scroll) = match State.selected_dir() {
		None => (" ◈ Log ".to_owned(), vec![], 0usize),
		Some(Key) => {
			let DS = &State.Map[Key];
			let Title = format!(" ◈ {} ", shorten(Key));
			let Lines:Vec<TLine> = DS
				.Lines
				.iter()
				.map(|(Text, IsStderr)| {
					let Style = if *IsStderr {
						Style::default().fg(Color::Red)
					} else {
						Style::default()
					};
					TLine::from(Span::styled(Text.clone(), Style))
				})
				.collect();
			let Scroll = if DS.AutoScroll {
				Lines.len().saturating_sub(Area.height as usize - 2)
			} else {
				DS.Scroll
			};
			(Title, Lines, Scroll)
		}
	};

	let Block = Block::default()
		.title(Title)
		.borders(Borders::ALL)
		.border_style(Style::default().fg(Color::Cyan));

	let Para = Paragraph::new(Lines)
		.block(Block)
		.wrap(Wrap { trim:false })
		.scroll((Scroll as u16, 0));

	Frame.render_widget(Para, Area);
}

// ─── Bottom status bar ────────────────────────────────────────────────────────

fn render_status_bar(Frame:&mut Frame, State:&AppState, Area:Rect) {
	let Done = State.Order.iter().filter(|K| {
		matches!(State.Map[*K].Status, Status::Done | Status::Failed | Status::Timeout)
	}).count();
	let Total = State.Order.len();

	let Status = if State.Done {
		format!(" ✓ All done ({}/{}) — q quit", Done, Total)
	} else {
		format!(" {} Running {}/{} — ↑↓ select  PgUp/PgDn scroll  s auto-scroll  q quit",
			SPINNER[State.Tick % SPINNER.len()], Done, Total)
	};

	let Style = if State.Done {
		Style::default().fg(Color::Green)
	} else {
		Style::default().fg(Color::DarkGray)
	};

	Frame.render_widget(Paragraph::new(Status).style(Style), Area);
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

/// Keep at most the last two path components to fit narrow panels.
fn shorten(Path:&str) -> String {
	let Parts:Vec<&str> = Path.trim_end_matches('/').rsplit('/').take(2).collect();
	if Parts.is_empty() {
		return Path.to_owned();
	}
	parts_join(Parts)
}

fn parts_join(mut Parts:Vec<&str>) -> String {
	Parts.reverse();
	Parts.join("/")
}
