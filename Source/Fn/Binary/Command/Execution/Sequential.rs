use std::{
	io::Read,
	process::{Command, Stdio},
};

use crate::Command::Execution::Struct;

pub fn Fn(Option: Struct) {
	println!("Executing code in sequential.");

	let Struct { Entry, Separator, Pattern, Command, .. } = Option;

	// Execution: Sequential
	for Entry in Entry {
		let Path = Entry.unwrap().path().display().to_string();
		let Path: Vec<&str> = Path.split(Separator).collect();

		if let Some(Last) = Path.last() {
			if *Last == Pattern {
				let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());

				let mut Out = match cfg!(target_os = "windows") {
					true => Command::new("cmd")
						.args(["/C", &Command])
						.current_dir(Directory)
						.stdout(Stdio::piped())
						.spawn()
						.expect("Failed to execute process."),
					false => Command::new("sh")
						.arg("-c")
						.current_dir(Directory)
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
			}
		}
	}
}
