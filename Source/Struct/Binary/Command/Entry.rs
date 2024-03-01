pub type Type = Vec<Vec<String>>;

pub struct Struct {
	pub Command: Command,
	pub Entry: Type,
	pub Parallel: Parallel,
	pub Pattern: Pattern,
	pub Separator: Separator,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Command: Option.Command.clone(),
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
		}
	}
}

use crate::Struct::Binary::Command::Option::{
	Command, Parallel, Pattern, Separator, Struct as Option,
};
