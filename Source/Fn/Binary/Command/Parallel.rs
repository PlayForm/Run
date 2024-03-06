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
