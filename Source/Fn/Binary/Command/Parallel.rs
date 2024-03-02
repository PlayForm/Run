pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let mut Queue = Vec::new();
	let Deque = std::sync::Arc::new(std::sync::Mutex::new(deque::Worker::new_fifo()));
	let mut Stealer = Vec::new();

	for _ in 0..6 {
		Stealer.push(Deque.lock().unwrap().stealer());
	}

	for Entry in Entry
		.into_iter()
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
		let Deque = std::sync::Arc::clone(&Deque);

		if cfg!(target_os = "windows") {
			Output =
				CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output();
		} else {
			Output = CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output();
		}

		Queue.push(async move {
			Deque.lock().unwrap().push(Output.await.expect("Failed to execute process.").stdout);
		});
	}

	tokio::runtime::Runtime::new().expect("Cannot Runtime.").block_on(async {
		for Queue in Queue {
			Queue.await;
		}
	});

	for Stealer in Stealer {
		loop {
			match Stealer.steal() {
				deque::Steal::Success(Success) => {
					println!("{}", String::from_utf8_lossy(&Success));
				}
				deque::Steal::Empty => break,
				deque::Steal::Retry => continue,
			}
		}
	}
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use crossbeam::deque;
use tokio::process::Command as CommandTokio;
