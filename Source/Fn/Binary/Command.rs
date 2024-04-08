pub mod Entry;
pub mod Parallel;
pub mod Sequential;

pub fn Fn() -> ArgMatches {
	Command::new("Run")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Playform <hello@playform.cloud>")
		.about("🍺 Run.")
		.arg(
			Arg::new("File")
				.short('F')
				.long("File")
				.action(SetTrue)
				.display_order(2)
				.value_name("FILE")
				.required(false)
				.help("File."),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
				.long("Parallel")
				.action(SetTrue)
				.display_order(3)
				.value_name("PARALLEL")
				.required(false)
				.help("Parallel."),
		)
		.arg(
			Arg::new("Root")
				.short('R')
				.long("Root")
				.display_order(4)
				.value_name("ROOT")
				.required(false)
				.help("Root.")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(5)
				.value_name("EXCLUDE")
				.required(false)
				.help("Exclude.")
				.default_value("node_modules .git target dist vendor"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(6)
				.value_name("PATTERN")
				.required(true)
				.help("Pattern.")
				.default_value("."),
		)
		.arg(
			Arg::new("Command")
				.num_args(0..=10)
				.display_order(7)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.allow_negative_numbers(true)
				.help("Command."),
		)
		.get_matches()
}

use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};
