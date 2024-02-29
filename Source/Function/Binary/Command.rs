use clap::{Arg, ArgAction::SetTrue, Command as CommandClap};
use std::process::{Command, Stdio};
use tokio::process::Command as CommandTokio;
use walkdir::WalkDir;

#[allow(dead_code)]
pub fn run() {
	let Match = CommandClap::new("Innkeeper")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
		.about("Run a command in all directories having a certain pattern.")
		.arg(
			Arg::new("File")
				.short('F')
				.long("File")
				.action(SetTrue)
				.display_order(1)
				.value_name("FILE")
				.required(false)
				.help("Search file."),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
				.long("Parallel")
				.action(SetTrue)
				.display_order(2)
				.value_name("PARALLEL")
				.required(false)
				.help("Execute code in parallel."),
		)
		.arg(
			Arg::new("Root")
				.short('R')
				.long("Root")
				.display_order(3)
				.value_name("ROOT")
				.required(false)
				.help("Current working directory.")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("Exclude pattern.")
				.default_value("node_modules .git target dist vendor"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(true)
				.help("Search pattern.")
				.default_value("."),
		)
		.arg(
			Arg::new("Command")
				.num_args(0..=10)
				.display_order(6)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.allow_negative_numbers(true)
				.help("Command to run."),
		)
		.get_matches();

	let File = Match.get_flag("File");
	let Parallel = Match.get_flag("Parallel");
	let Root = Match.get_one::<String>("Root").unwrap();
	let Exclude = Match.get_one::<String>("Exclude").unwrap().split(" ");
	let Pattern = Match.get_one::<String>("Pattern").unwrap();
	let Command = &Match
		.get_many::<String>("Command")
		.unwrap_or_default()
		.map(|Command| Command.as_str())
		.collect::<Vec<_>>()
		.join(" ");

	let Separator = std::path::MAIN_SEPARATOR;

	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
		let Path = Entry.path().display().to_string();

		!Exclude.clone().into_iter().filter(|Exclude| Pattern != Exclude).any(|Exclude| {
			if File {
				std::fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
			} else {
				Path.contains(Exclude)
			}
		})
	});

	if Parallel {
		println!("Executing code in parallel.");

		// Execution: Parallel
		let mut Queue = Vec::new();

		Entry
			.map(|Entry| {
				let Path = Entry.unwrap().path().display().to_string();
				let Path: Vec<&str> = Path.split(Separator).collect();

				match Path.last() {
					Some(Last) => {
						if Last == Pattern {
							Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
						} else {
							None
						}
					}
					None => None,
				}
			})
			.filter_map(|Entry| Entry)
			.for_each(|Directory| {
				let Output;

				if cfg!(target_os = "windows") {
					Output = CommandTokio::new("cmd")
						.args(["/C", Command.as_str()])
						.current_dir(Directory.clone())
						.output();
				} else {
					Output = CommandTokio::new("sh")
						.arg("-c")
						.current_dir(Directory.clone())
						.arg(Command)
						.output();
				}

				Queue.push(async move {
					println!("Executing {} for every {} in {}", Command, Directory, Root);

					println!(
						"{}",
						String::from_utf8_lossy(
							&Output.await.expect("Failed to execute process.").stdout
						)
					);
				});
			});

		tokio::runtime::Runtime::new().unwrap().block_on(async {
			for Queue in Queue {
				Queue.await;
			}
		});
	} else {
		println!("Executing code in sequential.");

		// Execution: Sequential
		for Entry in Entry {
			let Path = Entry.unwrap().path().display().to_string();
			let Path: Vec<&str> = Path.split(Separator).collect();

			if let Some(Last) = Path.last() {
				if Last == Pattern {
					let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());

					println!("Executing {} for every {} in {}", Command, Last, Root);

					let mut Out = match cfg!(target_os = "windows") {
						true => Command::new("cmd")
							.args(["/C", Command])
							.current_dir(Directory)
							.stdout(Stdio::piped())
							.spawn()
							.expect("Failed to execute process."),
						false => Command::new("sh")
							.arg("-c")
							.current_dir(Directory)
							.arg(Command)
							.stdout(Stdio::piped())
							.spawn()
							.expect("Failed to execute process."),
					}
					.stdout
					.expect("Failed to get stdout handle");

					let mut Output = String::new();

					loop {
						let mut Buffer = [0; 512];
						let Byte = std::io::Read::read(&mut Out, &mut Buffer).expect("Failed to read from pipe");

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
}
