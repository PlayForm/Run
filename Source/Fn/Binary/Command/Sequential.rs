/// Executes a command with arguments in a specific directory for each entry in
/// the given list.
///
/// # Arguments
///
/// * `Option` - A struct containing `Command`, `Entry`, `Pattern`, `Separator`,
///   and other optional fields.
///
/// # Example
///
/// ```
/// use std::process::{Command, Stdio};
///
/// let options = Option {
/// 	Command:vec!["ls".to_string()],
/// 	Entry:vec!["/path/to/dir".to_string()],
/// 	Pattern:"pattern",
/// 	Separator:'/'.to_string(),
/// };
/// Fn(options);
/// ```
pub async fn Fn(Option { Command, Entry, Pattern, Separator, .. }:Option) {
	for Entry in Entry.into_iter().filter_map(|Entry| {
		Entry
			.last()
			.filter(|Last| *Last == &Pattern)
			.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
	}) {
		for Command in &Command {
			let Command:Vec<String> = Command.split(' ').map(String::from).collect();
			let Entry = Entry.clone();

			let mut Command = Command::new(Command.get(0).expect("Cannot Command."))
				.args(&Command[1..])
				.current_dir(Entry)
				.stdout(Stdio::piped())
				.spawn()
				.expect("Cannot spawn.")
				.stdout
				.expect("Cannot stdout.");

			let mut Output = String::new();

			loop {
				let mut Buffer = [0; 512];
				let Byte = Command.read(&mut Buffer).expect("Cannot read.");

				if Byte == 0 {
					break;
				}

				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
			}

			println!("{}", Output);
		}
	}
}

use std::{
	io::Read,
	process::{Command, Stdio},
};

use crate::Struct::Binary::Command::Entry::Struct as Option;
