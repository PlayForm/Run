use clap::ArgMatches;
use once_cell::sync::Lazy;

use crate::{Fn::Binary::Command::Fn as ParseClap, Struct::Binary::Command::Struct as CommandStruct};

/// A type alias for a list of command strings.
pub type Command = Vec<String>;
/// A type alias for the boolean `Parallel` flag.
pub type Parallel = bool;
/// A type alias for the `Pattern` string.
pub type Pattern = String;
/// A type alias for the path separator character.
pub type Separator = char;

/// Caches the parsed command-line arguments in a thread-safe, static variable.
///
/// This ensures that `clap` argument parsing logic is executed only once,
/// no matter how many times the configuration is accessed.
static ARGS:　Lazy<ArgMatches> = Lazy::new(ParseClap);

/// A struct that holds the raw, parsed options from the command line.
pub struct Struct {
	pub Command:　Command,
	pub Exclude:　Vec<String>,
	pub File:　bool,
	pub Parallel:　Parallel,
	pub Pattern:　Pattern,
	pub Root:　String,
	pub Separator:　Separator,
	pub Tui:　bool,
}

impl Struct {
	/// Creates a new `Struct` instance from the statically parsed `clap`
	/// arguments.
	pub fn Fn(_Option:　CommandStruct) -> Self {
		Self {
			File:　ARGS.get_flag("File"),
			Parallel:　ARGS.get_flag("Parallel"),
			Tui:　ARGS.get_flag("Tui"),
			Root:　ARGS.get_one::<String>("Root").expect("Root argument is required.").to_owned(),
			Exclude:　ARGS
				.get_many::<String>("Exclude")
				.unwrap_or_default()
				.flat_map(|Value| Value.split_whitespace())
				.map(String::from)
				.collect::<Vec<_>>(),
			Pattern:　ARGS
				.get_one::<String>("Pattern")
				.expect("Pattern argument is required.")
				.to_owned(),
			Command:　ARGS
				.get_many::<String>("Command")
				.expect("Command argument is required.")
				.cloned()
				.collect(),
			// The separator is passed through from the initial config.
			Separator:　_Option.Separator,
		}
	}
}
