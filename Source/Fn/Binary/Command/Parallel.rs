/// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
/// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
/// as the current directory, and prints the output of each Command execution.
///
/// Arguments:
///
/// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
/// fields.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let Queue: Vec<String> = futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| {
		let Command = Command.clone();

		async move {
			String::from_utf8_lossy(
				&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
					.args(&Command[1..])
					.current_dir(Entry)
					.output()
					.await
					.expect("Cannot Output.")
					.stdout,
			)
			.to_string()
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.await;

	Queue.par_iter().for_each(|Output| println!("{}", Output));
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::StreamExt;
use rayon::prelude::*;
