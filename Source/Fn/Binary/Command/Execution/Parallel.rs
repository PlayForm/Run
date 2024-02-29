use tokio::process::Command as CommandTokio;

pub fn Fn(Separator) {
	println!("Executing code in parallel.");

	// Execution: Parallel
	let mut Queue = Vec::new();

	Entry
		.map(|Entry| {
			let Path = Entry.unwrap().path().display().to_string();
			let Path: Vec<&str> = Path.split(Separator).collect();

			match Path.last() {
				Some(Last) => {
					if *Last == Pattern {
						Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
					} else {
						None
					}
				}
				None => None,
			}
		})
		.filter_map(|Entry| Entry)
		.for_each(|Directory| {
			let Output;

			if cfg!(target_os = "windows") {
				Output = CommandTokio::new("cmd")
					.args(["/C", Command.as_str()])
					.current_dir(Directory)
					.output();
			} else {
				Output =
					CommandTokio::new("sh").arg("-c").current_dir(Directory).arg(&Command).output();
			}

			Queue.push(async move {
				println!(
					"{}",
					String::from_utf8_lossy(
						&Output.await.expect("Failed to execute process.").stdout
					)
				);
			});
		});

	tokio::runtime::Runtime::new().unwrap().block_on(async {
		for Queue in Queue {
			Queue.await;
		}
	});
}
