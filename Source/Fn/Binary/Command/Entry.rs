use walkdir::WalkDir;

use crate::Command::Option::Struct;

pub fn Fn(Option: Struct) {
	let Struct { Root, Exclude, File, Pattern, Separator, .. } = Option;

	WalkDir::new(Root)
		.into_iter()
		.filter_entry(move |Entry| {
			let Path = Entry.path().display().to_string();

			!Exclude.clone().into_iter().filter(|Exclude| Pattern != *Exclude).any(|Exclude| {
				if File {
					std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
				} else {
					Path.contains(&Exclude)
				}
			})
		})
		.map(|Entry| Entry.unwrap().path().display().to_string().split(Separator).collect())
}
