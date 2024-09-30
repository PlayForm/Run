pub mod GPG;
pub mod Process;

/// Executes a series of commands on a list of entries concurrently.
///
/// # Arguments
///
/// * `Option` - An optional struct containing the following fields:
///     * `Entry`: A vector of strings representing the entries to process.
///     * `Separator`: A string used to join the entry parts after filtering.
///     * `Pattern`: A string used to filter the entries.
///     * `Command`: A vector of strings representing the commands to execute on each entry.
///
/// # Example
///
/// ```rust
/// use your_crate::Fn;
///
/// let options = Some(Option {
///     Entry: vec!["entry1/part1".to_string(), "entry2/part1".to_string()],
///     Separator: "/".to_string(),
///     Pattern: "part1".to_string(),
///     Command: vec!["echo {}".to_string(), "ls -l {}".to_string()],
/// });
///
/// tokio_test::block_on(Fn(options));
/// ```
///
/// This example defines a vector of entries, a separator, a pattern and a vector of commands.
/// The `Fn` function is then called with the options.
///
/// # Details
///
/// The function first filters the entries based on the provided pattern.
/// Then, it creates a queue of filtered entries and spawns multiple worker tasks.
/// Each worker task picks an entry from the queue and executes the provided commands on it.
/// The output of each command is collected and printed to the console.
///
/// The function utilizes parallel processing using `rayon` and asynchronous programming using `tokio` to improve performance.
///
/// The `GPG_MUTEX` is used to ensure that only one thread can access the GPG functions at a time.
///
/// # Note
///
/// The function assumes that the provided commands are valid shell commands.
///
/// The function also assumes that the `GPG::Fn` and `Process::Fn` functions are defined elsewhere and have the following signatures:
///
/// ```rust
/// fn GPG::Fn(command: &[String]) -> bool;
/// async fn Process::Fn(command: &[String], entry: &str) -> String;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let (Allow, mut Receive) = mpsc::unbounded_channel();
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

	let (AllowWork, ReceiveWork) = mpsc::channel::<String>(32);
	let ReceiveWork = Arc::new(Mutex::new(ReceiveWork));

	let Output = tokio::spawn(async move {
		while let Some(Output) = Receive.recv().await {
			for Output in Output {
				println!("{}", Output);
			}
		}
	});

	for _ in 0..Force {
		let ReceiveWork = Arc::clone(&ReceiveWork);

		let Allow = Allow.clone();

		let Command = Command.clone();

		tokio::spawn(async move {
			loop {
				let Entry = { ReceiveWork.lock().await.recv().await };

				match Entry {
					Some(Entry) => {
						let mut Output = Vec::new();

						for Command in &Command {
							let Command: Vec<String> =
								Command.split(' ').map(String::from).collect();

							if GPG::Fn(&Command) {
								let _Lock = GPG_MUTEX.lock().await;
							}

							Output.push(Process::Fn(&Command, &Entry).await);
						}

						if let Err(_) = Allow.send(Output) {
							eprintln!("Cannot send.");
						}
					}
					None => break,
				}
			}
		});
	}

	(0..Force).into_par_iter().for_each(|_| {
		let AllowWork = AllowWork.clone();

		let Queue = Arc::clone(&Queue);

		tokio::runtime::Runtime::new().expect("Cannot Runtime.").block_on(async {
			while let Some(Entry) = Queue.pop() {
				AllowWork.send(Entry).await.expect("Cannot send.");
			}
		});
	});

	drop(Allow);
	drop(AllowWork);

	Output.await.expect("Output task failed");
}

use crate::Struct::Binary::Command::Entry::Struct as Option;

use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

static GPG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
