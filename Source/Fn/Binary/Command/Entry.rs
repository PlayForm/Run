/// This Rust function walks through a directory, efficiently filters out paths
/// based on glob-based exclusion criteria, and returns a collection of paths.
///
/// It no longer handles pattern matching, which is deferred to the consumer for
/// clarity and correctness.
///
/// Returns:
///
/// a `Vec<PathBuf>` containing candidate paths.
pub fn Fn(Option { Exclude, File, Root, .. }:&Option) -> Return {
	let mut Glob = GlobSetBuilder::new();

	for Exclude in Exclude {
		let Pattern = format!("{}/**", Exclude.trim_end_matches('/'));

		Glob.add(Glob::new(Exclude).expect("Failed to parse glob pattern."));

		Glob.add(Glob::new(&Pattern).expect("Failed to parse glob pattern."));
	}

	let Exclude = Glob.build().expect("Failed to build glob set.");

	WalkDir::new(Root)
		.follow_links(false)
		.into_iter()
		.filter_map(Result::ok)
		.filter(|Entry| {
			let Path = Entry.path();

			if Exclude.is_match(Path) {
				return false;
			}

			if *File { Path.is_file() } else { true }
		})
		.map(|entry| entry.into_path())
		.collect()
}

use std::path::PathBuf;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

use crate::Struct::Binary::Command::Option::Struct as Option;

pub type Return = Vec<PathBuf>;
