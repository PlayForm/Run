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

	let Entry = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect::<Vec<String>>();

	let Pool = Arc::new(rayon::ThreadPoolBuilder::new().build().expect("Cannot build."));

	Entry.into_par_iter().for_each(|Entry| {
		let Command = Command.clone();
		let Approval = Approval.clone();

		Pool.spawn(move || {
			let Output = tokio::runtime::Runtime::new().unwrap().block_on(async move {
				let mut Output = Vec::new();

				for Command in &Command {
					let Command: Vec<String> = Command.split(' ').map(String::from).collect();

					if GPG::Fn(&Command) {
						let Lock = GPG_MUTEX.lock().expect("Cannot lock.");
						drop(Lock);
					}

					Output.push(Process::Fn(&Command, &Entry).await);
				}

				Output
			});

			if let Err(_) = Approval.send(Output) {
				eprintln!("Cannot send.");
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
use std::sync::{Arc, Mutex};

static GPG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
