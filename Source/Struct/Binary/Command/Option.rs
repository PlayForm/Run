use clap::ArgMatches;
use once_cell::sync::Lazy;

use crate::{Fn::Binary::Command::Fn as ParseClap, Struct::Binary::Command::Struct as CommandStruct};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;

static ARGS:Lazy<ArgMatches> = Lazy::new(ParseClap);

/// Raw parsed options from the command line.
///
/// `Tui` is `true` when `-T` / `--Tui` was passed.
pub struct Struct {
	pub Command:Command,
	pub Exclude:Vec<String>,
	pub File:bool,
	pub Parallel:Parallel,
	pub Pattern:Pattern,
	pub Root:String,
	pub Separator:Separator,
	pub Tui:bool,
}

impl Struct {
	pub fn Fn(_Option:CommandStruct) -> Self {
		Self {
			File:ARGS.get_flag("File"),
			Parallel:ARGS.get_flag("Parallel"),
			Tui:ARGS.get_flag("Tui"),
			Root:ARGS.get_one::<String>("Root").expect("Root argument is required.").to_owned(),
			Exclude:ARGS
				.get_many::<String>("Exclude")
				.unwrap_or_default()
				.flat_map(|Value| Value.split_whitespace())
				.map(String::from)
				.collect::<Vec<_>>(),
			Pattern:ARGS
				.get_one::<String>("Pattern")
				.expect("Pattern argument is required.")
				.to_owned(),
			Command:ARGS
				.get_many::<String>("Command")
				.expect("Command argument is required.")
				.cloned()
				.collect(),
			Separator:_Option.Separator,
		}
	}
}
