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
			let Output = if cfg!(target_os = "windows") {
				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
			} else {
				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
			};

			Queue.push(async move {
				println!(
					"{}",
					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
				);
			});
		});

	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.expect("Cannot Runtime.")
		.block_on(async {
			for Queue in Queue {
				Queue.await;
			}
		})
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use tokio::process::Command;
