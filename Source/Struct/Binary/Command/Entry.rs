use std::path::PathBuf;

use crate::Struct::Binary::Command::Option::{Command, Parallel, Pattern, Separator, Struct as Option};

// OPTIMIZATION: The type alias now correctly reflects that we are working
// with a list of file paths, not strings.
pub type Type = Vec<PathBuf>;

pub struct Struct {
	pub Command:Command,

	pub Entry:Type,

	pub Parallel:Parallel,

	pub Pattern:Pattern,

	#[allow(dead_code)]
	pub Separator:Separator,
}

impl Struct {
	pub fn Fn(Option:&Option) -> Self {
		Self {
			Command:Option.Command.clone(),

			Entry:crate::Fn::Binary::Command::Entry::Fn(Option),

			Parallel:Option.Parallel,

			Pattern:Option.Pattern.clone(),

			Separator:Option.Separator,
		}
	}
}
