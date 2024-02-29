use walkdir::WalkDir;

use super::Parameter;

pub fn Fn() -> walkdir::FilterEntry<walkdir::IntoIter, impl FnMut(&walkdir::DirEntry) -> bool> {
	let Parameter::Struct { File, Root, Exclude, Pattern, .. } = Parameter::Struct::Fn();

	WalkDir::new(Root).into_iter().filter_entry(move |Entry| {
		let Path = Entry.path().display().to_string();

		!Exclude.clone().into_iter().filter(|Exclude| Pattern != *Exclude).any(|Exclude| {
			if File {
				std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
			} else {
				Path.contains(&Exclude)
			}
		})
	})
}
