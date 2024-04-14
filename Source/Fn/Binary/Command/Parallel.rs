/// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
/// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
/// as the current directory, and prints the output of each Command execution.
///
/// Arguments:
///
/// * ``: It looks like you have a Rust function named `Fn` that takes an `Option` enum as a parameter.
/// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
/// fields.
pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let mut Queue = Vec::new();

	Entry
		.into_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.for_each(|Entry| {
			let Output = Command::new(Command.get(0).expect("Cannot Command."))
				.args(&Command[1..])
				.current_dir(Entry)
				.output();

			Queue.push(async move { Output.await.expect("Cannot await.").stdout });
		});

	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.expect("Cannot Runtime.")
		.block_on(async {
			for Queue in Queue {
				println!("{}", String::from_utf8_lossy(&Queue.await));
			}
		})
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use tokio::process::Command;
