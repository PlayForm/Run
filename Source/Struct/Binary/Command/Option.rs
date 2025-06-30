use clap::ArgMatches;
use once_cell::sync::Lazy;

use crate::{
	Fn::Binary::Command::Fn as ParseClap,
	Struct::Binary::Command::{Struct as CommandStruct, Struct as Option},
};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;

// OPTIMIZATION: Parse command-line arguments only once and store them in a
// static.
static ARGS:Lazy<ArgMatches> = Lazy::new(ParseClap);

pub struct Struct {
	pub Command:Command,
	pub Exclude:Vec<String>,
	pub File:bool,
	pub Parallel:Parallel,
	pub Pattern:Pattern,
	pub Root:String,
	pub Separator:Separator,
}

impl Struct {
	pub fn Fn(Option { Separator, .. }:CommandStruct) -> Self {
		Self {
			File:ARGS.get_flag("File"),
			Parallel:ARGS.get_flag("Parallel"),
			Root:ARGS.get_one::<String>("Root").expect("Root is required.").to_owned(),
			Exclude:ARGS
				.get_one::<String>("Exclude")
				.unwrap_or(&"".to_string())
				.split_whitespace()
				.map(String::from)
				.collect::<Vec<_>>(),
			Pattern:ARGS.get_one::<String>("Pattern").expect("Pattern is required.").to_owned(),
			Command:ARGS
				.get_many::<String>("Command")
				.expect("Command is required.")
				.cloned()
				.collect(),
			Separator,
		}
	}
}
