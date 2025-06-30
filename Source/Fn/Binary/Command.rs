use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};

pub mod Entry;
pub mod Parallel;
pub mod Sequential;

/// Defines and configures the command-line interface for the "Run" utility.
///
/// This function uses the `clap` crate to specify all possible arguments,
/// flags, and options, including their help messages, default values, and
/// relationships.
///
/// # Returns
///
/// An `ArgMatches` object containing the parsed values from the command line.
pub fn Fn() -> ArgMatches {
	ClapCommand::new("Run")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Source ✍🏻 Open 👐🏻 <Source/Open@PlayForm.Cloud>")
		.about("A utility to run commands in directories matching a pattern.")
		.arg(
			Arg::new("File")
				.short('F')
				.long("File")
				.action(ArgAction::SetTrue)
				.display_order(1)
				.help("Target files directly instead of directories containing a pattern."),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
				.long("Parallel")
				.action(ArgAction::SetTrue)
				.display_order(2)
				.help("Execute commands in parallel across all found directories."),
		)
		.arg(
			Arg::new("Root")
				.short('R')
				.long("Root")
				.display_order(3)
				.value_name("DIRECTORY")
				.help("The root directory to start the search from.")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("PATTERNS")
				.help("A space-separated list of glob patterns to exclude from the search.")
				.default_value("node_modules .git target dist vendor"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(true)
				.help("The file or directory name that identifies a target directory."),
		)
		.arg(
			Arg::new("Command")
				.short('C')
				.long("Command")
				.action(ArgAction::Append)
				.display_order(6)
				.value_name("COMMAND")
				.required(true)
				.allow_hyphen_values(true)
				.help("The command to execute. Can be specified multiple times."),
		)
		.get_matches()
}
