pub mod GPG;
pub mod Process;

/// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
/// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
/// as the current directory, and prints the output of each Command execution.
///
/// Arguments:
///
/// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
/// fields.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
	let (Approval, mut Receive) = tokio::sync::mpsc::unbounded_channel();

	let Entry = tokio::task::spawn_blocking(move || {
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>()
	})
	.await
	.expect("Cannot blocking.");

	futures::stream::iter(Entry.into_iter())
		.map(|Entry| {
			let Command = Command.clone();
			let Approval = Approval.clone();

			async move {
				let mut Output = Vec::new();

				for Command in &Command {
					let Command: Vec<String> = Command.split(' ').map(String::from).collect();

					if GPG::Fn(&Command) {
						let Lock = GPG_MUTEX.lock().await;
						drop(Lock);
					}

					Output.push(Process::Fn(&Command, &Entry).await);
				}

				if let Err(_) = Approval.send(Output) {
					eprintln!("Cannot send.");
				}
			}
		})
		.buffer_unordered(num_cpus::get())
		.collect::<Vec<()>>()
		.await;

	drop(Approval);

	while let Some(Output) = Receive.recv().await {
		for Output in Output {
			println!("{}", Output);
		}
	}
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::StreamExt;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::Mutex;

static GPG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
