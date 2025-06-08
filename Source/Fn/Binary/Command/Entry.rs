/// This Rust function walks through a directory, filters out certain files
/// based on exclusion criteria, and returns a collection of paths.
///
/// Arguments:
///
/// some file system operations based on the provided configuration. Here's a
/// breakdown of the parameters:
///
/// Returns:
///
/// a `Vec<String>` containing paths that meet the specified criteria after
/// processing the entries from the directory specified by the `Root` parameter.
pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }:&Option) -> Return {
	// The `Exclude` vector is now captured by reference, not cloned repeatedly.
	WalkDir::new(Root)
		.follow_links(false)
		.into_iter()
		.filter_map(|entry_result| {
			let entry = entry_result.expect("Cannot read directory entry.");
			let path = entry.path();
			let path_str = path.display().to_string();

			let is_excluded = Exclude.iter().any(|exclude_pattern| {
				if !Pattern.contains(exclude_pattern) {
					// Simple optimization
					let is_match = path_str.contains(exclude_pattern);
					if *File {
						// Check if it's a directory that matches the exclude pattern
						path.is_dir() && is_match
					} else {
						is_match
					}
				} else {
					false
				}
			});

			if !is_excluded {
				Some(path_str.split(*Separator).map(|s| s.to_string()).collect())
			} else {
				None
			}
		})
		.collect()
}

use walkdir::WalkDir;

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};
