extern crate clap;
extern crate crossbeam;
extern crate rayon;
extern crate walkdir;

use clap::{Arg, ArgAction, Command as ClapCommand};
use crossbeam::scope;
use rayon::prelude::*;
use std::{fs, process::Command};
use walkdir::WalkDir;

fn main() {
	let matches = ClapCommand::new("Innkeeper")
		.version("0.0.4")
		.about("Runs a command in all directories having a certain folder.")
		.arg(
			Arg::new("parallel")
				.short('p')
				.long("parallel")
				.action(ArgAction::SetTrue)
				.display_order(1)
				.value_name("PARALLEL")
				.required(false)
				.help("Execute code in parallel."),
		)
		.arg(
			Arg::new("root")
				.short('r')
				.long("root")
				.display_order(2)
				.value_name("ROOT")
				.required(false)
				.help("Current working directory.")
				.default_value("."),
		)
		.arg(
			Arg::new("folder")
				.display_order(3)
				.value_name("FOLDER")
				.required(true)
				.help("Search folder.")
				.default_value("."),
		)
		.arg(
			Arg::new("command")
				.num_args(0..=10)
				.display_order(4)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.allow_negative_numbers(true)
				.help("Command to run."),
		)
		.get_matches();

	let parallel = matches.get_flag("parallel");
	let root = matches.get_one::<String>("root").unwrap();
	let folder = matches.get_one::<String>("folder").unwrap();
	let command = &matches
		.get_many::<String>("command")
		.unwrap_or_default()
		.map(|v| v.as_str())
		.collect::<Vec<_>>()
		.join(" ");

	let ds = std::path::MAIN_SEPARATOR;

	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
		fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir()
			&& (!e.path().display().to_string().contains("node_modules")
				|| !folder.contains("node_modules"))
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
						if last == folder {
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
				if last == folder {
					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());

					println!("Executing {} for every {} in {}", command, last, root);

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
}
