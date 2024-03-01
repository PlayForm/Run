pub fn Fn(Option { File, Root, Exclude, Pattern, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		// TODO: BENCH THIS
		.max_open(60)
		.into_iter()
		.filter_entry(move |Entry| {
			let Path = Entry.path().display().to_string();

			!Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(
				|Exclude| {
					match File {
						true => {
							std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
						}
						false => Path.contains(&Exclude),
					}
				},
			)
		})
		.map(|Entry| {
			Entry.unwrap().path().display().to_string().split(*Separator).collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

use crate::Struct::Binary::Command::{Entry::Entry as Return, Option::Struct as Option};

use walkdir::WalkDir;
