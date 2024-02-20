extern crate clap;
extern crate crossbeam;
extern crate rayon;
extern crate walkdir;

use clap::{Arg, ArgAction, Command as ClapCommand};
use crossbeam::scope;
use rayon::prelude::*;
use std::{
	fs,
	io::Read,
	process::{Command, Stdio},
};
use walkdir::WalkDir;

pub fn run() {
	let matches = ClapCommand::new("Innkeeper")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
		.about("Runs a command in all directories having a certain pattern.")
		.arg(
			Arg::new("file")
				.short('f')
				.long("file")
				.action(ArgAction::SetTrue)
				.display_order(1)
				.value_name("FILE")
				.required(false)
				.help("Search file."),
		)
		.arg(
			Arg::new("parallel")
				.short('p')
				.long("parallel")
				.action(ArgAction::SetTrue)
				.display_order(2)
				.value_name("PARALLEL")
				.required(false)
				.help("Execute code in parallel."),
		)
		.arg(
			Arg::new("root")
				.short('r')
				.long("root")
				.display_order(3)
				.value_name("ROOT")
				.required(false)
				.help("Current working directory.")
				.default_value("."),
		)
		.arg(
			Arg::new("pattern")
				.display_order(4)
				.value_name("PATTERN")
				.required(true)
				.help("Search pattern.")
				.default_value("."),
		)
		.arg(
			Arg::new("command")
				.num_args(0..=10)
				.display_order(5)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.allow_negative_numbers(true)
				.help("Command to run."),
		)
		.get_matches();

	let file = matches.get_flag("file");
	let parallel = matches.get_flag("parallel");
	let root = matches.get_one::<String>("root").unwrap();
	let pattern = matches.get_one::<String>("pattern").unwrap();
	let command = &matches
		.get_many::<String>("command")
		.unwrap_or_default()
		.map(|v| v.as_str())
		.collect::<Vec<_>>()
		.join(" ");

	let ds = std::path::MAIN_SEPARATOR;

	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
		if !pattern.contains("node_modules") {
			return e.path().display().to_string().contains("node_modules");
		}

		if !file {
			println!("{:?}", e.path().display().to_string().contains("node_modules"));
			return fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir();
		} else {
			return true;
		}
	});

	if parallel {
		println!("Executing code in parallel.");

		// Parallel
		let dirs = entries
			.map(|entry| {
				let entry_dir = entry.unwrap().path().display().to_string();
				let paths: Vec<&str> = entry_dir.split(ds).collect();

				match paths.last() {
					Some(last) => {
						if last == pattern {
							let working_directory =
								&paths[0..paths.len() - 1].join(&ds.to_string());
							Some(working_directory.to_owned())
						} else {
							None
						}
					}
					None => None,
				}
			})
			.filter_map(|x| x)
			.collect::<Vec<String>>();

		scope(|s| {
			dirs.into_par_iter().for_each_with(s, |scope, dir| {
				scope.spawn(move |_| {
					println!("Executing {} for every {} in {}", command, dir, root);

					let output = match cfg!(target_os = "windows") {
						true => Command::new("cmd")
							.args(["/C", command.as_str()])
							.current_dir(dir)
							.output()
							.expect("Failed to execute process."),
						false => Command::new("sh")
							.arg("-c")
							.current_dir(dir)
							.arg(command)
							.output()
							.expect("Failed to execute process."),
					};

					println!("{}", String::from_utf8_lossy(&output.stdout));
				});
			});
		})
		.unwrap();
	} else {
		println!("Executing code in sequential.");

		// Sequential
		for entry in entries {
			let entry_dir = entry.unwrap().path().display().to_string();
			let paths: Vec<&str> = entry_dir.split(ds).collect();

			if let Some(last) = paths.last() {
				if last == pattern {
					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());

					println!("Executing {} for every {} in {}", command, last, root);

					let child = match cfg!(target_os = "windows") {
						true => Command::new("cmd")
							.args(["/C", command])
							.current_dir(working_directory)
							.stdout(Stdio::piped())
							.spawn()
							.expect("Failed to execute process."),
						false => Command::new("sh")
							.arg("-c")
							.current_dir(working_directory)
							.arg(command)
							.stdout(Stdio::piped())
							.spawn()
							.expect("Failed to execute process."),
					};

					let mut stdout = child.stdout.expect("Failed to get stdout handle");

					let mut output = String::new();

					loop {
						let mut buffer = [0; 512];
						let bytes_read =
							stdout.read(&mut buffer).expect("Failed to read from pipe");

						if bytes_read == 0 {
							break;
						}

						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
					}

					println!("{}", output);
				}
			}
		}
	}
}
