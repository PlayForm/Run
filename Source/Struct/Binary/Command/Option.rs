pub type Command = Vec<String>;

pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;

pub struct Struct {
	pub Command: Command,
	pub Exclude: Vec<String>,
	pub File: bool,
	pub Parallel: Parallel,
	pub Pattern: Pattern,
	pub Root: String,
	pub Separator: Separator,
}

impl Struct {
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			File: Fn().get_flag("File"),
			Parallel: Fn().get_flag("Parallel"),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Exclude: Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Command: Fn()
				.get_many::<String>("Command")
				.expect("Cannot Command.")
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Separator,
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
