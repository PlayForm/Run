pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.into_iter()
		.filter_map(|Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
				let Match = Path.contains(&Exclude);

				match File {
					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
					false => Match,
				}
			}) {
				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};

use walkdir::WalkDir;
