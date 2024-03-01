pub fn Fn(Option: Option) {
	println!("Executing code in parallel.");

	let Option { Entry, Separator, Pattern, Command, .. } = Option;

	// Execution: Parallel
	let mut Queue = Vec::new();

	for Entry in Entry
		.map(|Entry| match Entry.last() {
			Some(Last) => {
				if *Last == Pattern {
					Some(Entry[0..Entry.len() - 1].join(&Separator.to_string()))
				} else {
					None
				}
			}
			None => None,
		})
		.filter_map(|Entry| Entry)
	{
		let Output;

		if cfg!(target_os = "windows") {
			Output =
				CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output();
		} else {
			Output = CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output();
		}

		Queue.push(async move {
			println!(
				"{}",
				String::from_utf8_lossy(&Output.await.expect("Failed to execute process.").stdout)
			);
		});
	}

	tokio::runtime::Runtime::new().unwrap().block_on(async {
		for Queue in Queue {
			Queue.await;
		}
	});
}

use crate::Command::Entry::Struct as Option;

use tokio::process::Command as CommandTokio;
