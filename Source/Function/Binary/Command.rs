extern crate clap;
extern crate crossbeam;
extern crate rayon;
extern crate walkdir;

use self::{
	clap::{Arg, ArgAction, Command as ClapCommand},
	crossbeam::scope,
	rayon::prelude::*,
	walkdir::WalkDir,
};

use std::{
	fs,
	io::Read,
	process::{Command, Stdio},
};

#[allow(dead_code)]
pub fn run() {
	let Match = ClapCommand::new("Innkeeper")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
		.about("Run a command in all directories having a certain pattern.")
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

	let File = Match.get_flag("file");
	let Parallel = Match.get_flag("parallel");
	let Root = Match.get_one::<String>("root").unwrap();
	let Pattern = Match.get_one::<String>("pattern").unwrap();
	let Command = &Match
		.get_many::<String>("command")
		.unwrap_or_default()
		.map(|v| v.as_str())
		.collect::<Vec<_>>()
		.join(" ");

	let Separator = std::path::MAIN_SEPARATOR;

	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
		if !Pattern.contains("node_modules") {
			return !Entry.path().display().to_string().contains("node_modules");
		}

		if !File {
			println!("{:?}", Entry.path().display().to_string().contains("node_modules"));

			fs::metadata(Entry.path().display().to_string().clone()).unwrap().is_dir()
		} else {
			true
		}
	});

	if Parallel {
		println!("Executing code in parallel.");

		// Execution: Parallel
		scope(|s| {
			Entry
				.map(|Entry| {
					let entry_dir = Entry.unwrap().path().display().to_string();
					let paths: Vec<&str> = entry_dir.split(Separator).collect();

					match paths.last() {
						Some(last) => {
							if last == Pattern {
								let working_directory =
									&paths[0..paths.len() - 1].join(&Separator.to_string());
								Some(working_directory.to_owned())
							} else {
								None
							}
						}
						None => None,
					}
				})
				.filter_map(|x| x)
				.collect::<Vec<String>>()
				.into_par_iter()
				.for_each_with(s, |scope, dir| {
					scope.spawn(move |_| {
						println!("Executing {} for every {} in {}", Command, dir, Root);

						println!(
							"{}",
							String::from_utf8_lossy(
								&match cfg!(target_os = "windows") {
									true => Command::new("cmd")
										.args(["/C", Command.as_str()])
										.current_dir(dir)
										.output()
										.expect("Failed to execute process."),
									false => Command::new("sh")
										.arg("-c")
										.current_dir(dir)
										.arg(Command)
										.output()
										.expect("Failed to execute process."),
								}
								.stdout
							)
						);
					});
				});
		})
		.unwrap();
	} else {
		println!("Executing code in sequential.");

		// Execution: Sequential
		for Entry in Entry {
			let entry_dir = Entry.unwrap().path().display().to_string();
			let paths: Vec<&str> = entry_dir.split(Separator).collect();

			if let Some(last) = paths.last() {
				if last == Pattern {
					let working_directory = &paths[0..paths.len() - 1].join(&Separator.to_string());

					println!("Executing {} for every {} in {}", Command, last, Root);

					let child = match cfg!(target_os = "windows") {
						true => Command::new("cmd")
							.args(["/C", Command])
							.current_dir(working_directory)
							.stdout(Stdio::piped())
							.spawn()
							.expect("Failed to execute process."),
						false => Command::new("sh")
							.arg("-c")
							.current_dir(working_directory)
							.arg(Command)
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
