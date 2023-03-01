extern crate clap;
extern crate walkdir;

use std::{fs, process::Command};

use clap::{Arg, Command as ClapCommand};
use walkdir::WalkDir;

fn main() {
	let matches = ClapCommand::new("Innkeeper")
		.version("0.0.4")
		.about("Runs a command in all directories having a certain folder.")
		.arg(
			Arg::new("root")
				.short('r')
				.long("root")
				.display_order(1)
				.value_name("ROOT")
				.required(false)
				.help("Current working directory.")
				.default_value("."),
		)
		.arg(
			Arg::new("folder")
				.display_order(2)
				.value_name("FOLDER")
				.required(true)
				.help("Search folder.")
				.default_value("."),
		)
		.arg(
			Arg::new("command")
				.num_args(0..=10)
				.display_order(3)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.allow_negative_numbers(true)
				.help("Command to run."),
		)
		.get_matches();

	let root = matches.get_one::<String>("root").unwrap();
	let folder = matches.get_one::<String>("folder").unwrap();
	let command = &matches
		.get_many::<String>("command")
		.unwrap_or_default()
		.map(|v| v.as_str())
		.collect::<Vec<_>>()
		.join(" ");

	let ds = std::path::MAIN_SEPARATOR;

	for entry in WalkDir::new(root).into_iter().filter_entry(|e| {
		fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir()
			&& (!e.path().display().to_string().contains("node_modules")
				|| !folder.contains("node_modules"))
	}) {
		let entry_dir = entry.unwrap().path().display().to_string();
		let paths: Vec<&str> = entry_dir.split(ds).collect();

		if let Some(last) = paths.last() {
			if last == folder {
				let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());

				println!("Executing {} for every {} in {}.", command, last, root);

				let output = match cfg!(target_os = "windows") {
					true => Command::new("cmd")
						.args(["/C", command])
						.current_dir(working_directory)
						.output()
						.expect("Failed to execute process."),
					false => Command::new("sh")
						.arg("-c")
						.current_dir(working_directory)
						.arg(command)
						.output()
						.expect("Failed to execute process."),
				};

				println!("{}", String::from_utf8_lossy(&output.stdout));
			}
		}
	}
}
