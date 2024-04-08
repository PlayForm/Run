pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
		.into_iter()
		.filter_map(|Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			// TODO: Separate this into Entry/Exclude.rs
			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
				let Match = Path.contains(&Exclude);

				match File {
					true => match std::fs::metadata(std::path::PathBuf::from(&Path)) {
						Ok(Metadata) => Metadata.is_dir() && Match,
						Err(_Error) => false,
					},
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
