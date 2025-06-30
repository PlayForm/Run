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

		// This pattern matches everything *inside* a directory. e.g., "target" becomes
		// "target/**".
		let InteriorGlobPattern = format!("{}/**", ExcludePattern.trim_end_matches('/'));
		let InteriorGlob = Glob::new(&InteriorGlobPattern).expect("Failed to parse interior glob pattern.");

		GlobBuilder.add(DirectGlob);
		GlobBuilder.add(InteriorGlob);
	}

	let ExcludeSet = GlobBuilder.build().expect("Failed to build glob set.");

	WalkDir::new(&Option.Root)
		.follow_links(false)
		.into_iter()
		// Skip any entries that result in an error (e.g., permission denied).
		.filter_map(Result::ok)
		.filter(|DirEntry| {
			let Path = DirEntry.path();

			// If the path matches any exclusion glob, filter it out.
			if ExcludeSet.is_match(Path) {
				return false;
			}

			// If the --File flag is active, we only want to consider files.
			// Otherwise, we consider everything that wasn't excluded (directories and files).
			if Option.File {
				Path.is_file()
			} else {
				true
			}
		})
		.map(|DirEntry| DirEntry.into_path())
		.collect()
}
