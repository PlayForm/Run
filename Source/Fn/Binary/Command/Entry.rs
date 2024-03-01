pub fn Fn(Option { File, Root, Exclude, Pattern, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		// TODO: BENCH THIS
		.max_open(60)
		.into_iter()
		.filter_map( |Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(
				|Exclude| {
					match File {
						true => {
							std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
						}
						false => Path.contains(&Exclude),
					}
				},
			) {
				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};

use walkdir::WalkDir;
