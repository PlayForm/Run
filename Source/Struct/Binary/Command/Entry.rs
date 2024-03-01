pub struct Struct {
	pub Command: Command,
	pub Entry: todo!("Implement this to have the same type as the function return type Entry"),
	pub Pattern: Pattern,
	pub Separator: Separator,
	pub Parallel: Parallel,
}

impl Struct {
	pub fn Fn(Option: Option) -> Self {
		Self {
			Command: Option.Command,
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Pattern: Option.Pattern,
			Separator: Option.Separator,
			Parallel: Option.Parallel,
		}
	}
}

use crate::Struct::Binary::Command::Option::{
	Command, Parallel, Pattern, Separator, Struct as Option,
};
