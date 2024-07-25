/// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
/// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
/// as the current directory, and prints the output of each Command execution.
///
/// Arguments:
///
/// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
/// fields.
pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let Queue: Vec<_> = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.map(|Entry| {
			Command::new(Command.get(0).expect("Cannot Command."))
				.args(&Command[1..])
				.current_dir(Entry)
				.output()
				.expect("Cannot Output.")
				.stdout
		})
		.collect();

	for Queue in Queue {
		println!("{}", String::from_utf8_lossy(&Queue));
	}
}

use std::process::Command;

use crate::Struct::Binary::Command::Entry::Struct as Option;
use rayon::prelude::*;
