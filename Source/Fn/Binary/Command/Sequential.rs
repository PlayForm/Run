pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.for_each(|Entry| {
			let mut Out = match cfg!(target_os = "windows") {
				true => Command::new("cmd")
					.args(["/C", &Command])
					.current_dir(Entry)
					.stdout(Stdio::piped())
					.spawn()
					.expect("Failed to execute process."),
				false => Command::new("sh")
					.arg("-c")
					.current_dir(Entry)
					.arg(Command.clone())
					.stdout(Stdio::piped())
					.spawn()
					.expect("Failed to execute process."),
			}
			.stdout
			.expect("Failed to get stdout handle");

			let mut Output = String::new();

			loop {
				let mut Buffer = [0; 512];
				let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");

				if Byte == 0 {
					break;
				}

				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
			}

			println!("{}", Output);
		})
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use std::{
	io::Read,
	process::{Command, Stdio},
};
