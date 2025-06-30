use std::path::PathBuf;

use crate::Struct::Binary::Command::Option::{Command, Parallel, Pattern, Separator, Struct as CommandOption};

/// A type alias for a list of resolved filesystem paths.
pub type Type = Vec<PathBuf>;

/// A struct that holds all necessary information for an execution strategy.
///
/// This struct is created after the initial file system walk and contains the
/// finalized list of entries and commands to be processed.
pub struct Struct {
	/// The list of user-provided commands to execute.
	pub Command:Command,
	/// The list of candidate file paths gathered from the filesystem.
	pub Entry:Type,
	/// A flag indicating whether execution should be parallel.
	pub Parallel:Parallel,
	/// The pattern to match against candidate paths.
	pub Pattern:Pattern,
	/// The OS-specific path separator character.
	#[allow(dead_code)]
	pub Separator:Separator,
}

impl Struct {
	/// Creates a new `Struct` instance by processing the command-line options.
	pub fn Fn(Option:&CommandOption) -> Self {
		Self {
			Command:Option.Command.clone(),
			Entry:crate::Fn::Binary::Command::Entry::Fn(Option),
			Parallel:Option.Parallel,
			Pattern:Option.Pattern.clone(),
			Separator:Option.Separator,
		}
	}
}
