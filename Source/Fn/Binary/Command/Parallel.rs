pub mod GPG;
pub mod Process;

/// Executes a command pipeline based on input entries and a pattern.
///
/// This function processes a set of string entries, filters them based on a pattern,
/// and then executes a command pipeline for each matching entry. The results of each
/// pipeline execution are then sent through a channel for further processing.
///
/// # Arguments
///
/// * `options`: An optional struct containing the following fields:
///     * `Entry`: A vector of string slices representing the input entries.
///     * `Separator`: A string slice used to join the filtered parts of an entry.
///     * `Pattern`: A string slice representing the pattern to filter entries.
///     * `Command`: A vector of string slices representing the command pipeline to execute.
///
/// # Example
///
/// ```
/// use your_crate::Fn;
///
/// let options = Some(YourOptionStruct {
///     Entry: vec!["entry1/part1", "entry1/part2", "entry2/part1"],
///     Separator: "/",
///     Pattern: "part2",
///     Command: vec!["echo hello", "grep world"],
/// });
///
/// tokio::runtime::Runtime::new().unwrap().block_on(Fn(options));
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let (Approval, mut Receive) = tokio::sync::mpsc::unbounded_channel();
	let Force = rayon::current_num_threads();

	let Entry = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect::<Vec<String>>();

	let Queue = Arc::new(crossbeam_queue::ArrayQueue::new(Entry.len()));

	for Entry in Entry {
		Queue.push(Entry).expect("Cannot push.");
	}

	(0..Force).into_par_iter().for_each(|_| {
		let Runtime = tokio::runtime::Runtime::new().expect("Cannot Runtime.");

		Runtime.block_on(async {
			let Queue = Arc::clone(&Queue);
			let Approval = Approval.clone();
			let Command = Command.clone();

			loop {
				if let Some(Entry) = Queue.pop() {
					let mut Output = Vec::new();

					for Command in &Command {
						let Command: Vec<String> = Command.split(' ').map(String::from).collect();

						if GPG::Fn(&Command) {
							let _Lock = GPG_MUTEX.lock().await;
						}

						Output.push(Process::Fn(&Command, &Entry).await);
					}

					if let Err(_) = Approval.send(Output) {
						eprintln!("Cannot send.");
					}
				} else {
					break;
				}
			}
		});
	});

	drop(Approval);

	while let Some(Output) = Receive.recv().await {
		for Output in Output {
			println!("{}", Output);
		}
	}
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;
use tokio::sync::Mutex;

static GPG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
