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
	let (Approval, mut Receipt) = tokio::sync::mpsc::unbounded_channel();
	let Queue = futures::stream::FuturesUnordered::new();

	for Entry in Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect::<Vec<String>>()
	{
		let Command = Command.clone();
		let Approval = Approval.clone();

		Queue.push(tokio::spawn(async move {
			if GPG::Fn(&Command) {
				let Lock = GPG_MUTEX.lock().await;
				drop(Lock);
			}

			match Approval.send(Process::Fn(&Command, &Entry).await) {
				Ok(_) => (),
				Err(_Error) => eprintln!("Cannot send Approval: {}", _Error),
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(Approval);
	});

	while let Some(Output) = Receipt.recv().await {
		println!("{}", Output);
	}
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::StreamExt;
use once_cell::sync::Lazy;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokio::sync::Mutex;

static GPG_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
