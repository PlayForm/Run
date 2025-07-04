use std::path::PathBuf;

use globset::{Glob, GlobSetBuilder};
use walkdir::WalkDir;

use crate::Struct::Binary::Command::Option::Struct as CommandOption;

/// A type alias for the return type of the `Fn` function.
pub type Return = Vec<PathBuf>;

/// Walks a directory tree to find candidate paths, filtering based on exclusion
/// criteria.
///
/// This function serves as the first stage of path gathering. It efficiently
/// traverses the filesystem starting from a root directory, using a `GlobSet`
/// to quickly discard any paths matching a set of exclusion patterns. The
/// responsibility of matching against the final `Pattern` is deferred to the
/// consumer (e.g., the Parallel or Sequential execution modules).
///
/// # Arguments
///
/// * `Option`: A reference to the command options struct which contains `Root`,
///   `Exclude`, and the `File` flag.
///
/// # Returns
///
/// A `Vec<PathBuf>` containing all paths that did not match the exclusion
/// criteria.
pub fn Fn(Option:&CommandOption) -> Return {
	let mut GlobBuilder = GlobSetBuilder::new();

	// Compile the exclusion patterns provided by the user into a high-performance
	// GlobSet.
	for ExcludePattern in &Option.Exclude {
		// This pattern matches the directory/file itself.
		let DirectGlob = Glob::new(ExcludePattern).expect("Failed to parse glob pattern.");

		let InteriorGlobPattern = format!("{}/**", ExcludePattern.trim_end_matches('/'));
		let InteriorGlob = Glob::new(&InteriorGlobPattern).expect("Failed to parse interior glob pattern.");

		GlobBuilder.add(DirectGlob);
		GlobBuilder.add(InteriorGlob);
	}

	let ExcludeSet = GlobBuilder.build().expect("Failed to build glob set.");

	WalkDir::new(&Option.Root)
		.follow_links(false)
		.into_iter()
		.filter_entry(|e| !ExcludeSet.is_match(e.path()))
		.filter_map(Result::ok)
		.filter(|DirEntry| if Option.File { DirEntry.file_type().is_file() } else { true })
		.map(|DirEntry| DirEntry.into_path())
		.collect()
}
