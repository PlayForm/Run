/// Executes a series of commands on a list of entries concurrently.
///
/// # Arguments
/// * `Option` - Struct with fields: `Entry` (Vec<String>), `Separator`
///   (String), `Pattern` (String), `Command` (Vec<String>).
///
/// # Details
/// Filters entries by pattern, processes them with commands in parallel using
/// workers, and prints outputs. Uses Rayon for filtering and Tokio for async
/// command execution, with a lock-free queue for work distribution.
pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }:Option) {
	let (Allow, mut Receive) = mpsc::unbounded_channel::<Vec<String>>();

	let Command = Arc::new(Command);

	let Entry:Vec<String> = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect();

	let Queue = Arc::new(ArrayQueue::new(Entry.len()));

	Entry.into_par_iter().for_each(|Entry| {
		Queue.push(Entry).expect("Queue capacity should suffice");
	});

	let Force = rayon::current_num_threads();

	let Output = tokio::spawn(async move {
		while let Some(Output) = Receive.recv().await {
			Output.into_par_iter().for_each(|Output| {
				println!("{}", Output);
			});
		}
	});

	let Worker = (0..Force)
		.map(|_| {
			let Allow = Allow.clone();

			let Command = Arc::clone(&Command);

			let Queue = Arc::clone(&Queue);

			tokio::spawn(async move {
				while let Some(Entry) = Queue.pop() {
					let mut Output = Vec::new();

					Output.extend(
						futures::future::join_all(
							Command
								.par_iter()
								.map(|Command| {
									async {
										let Part = Command.split(' ').map(String::from).collect::<Vec<String>>();

										match GPG::Fn(&Part) {
											true => {
												let _Lock = GPG_MUTEX.lock().await;
											},
											false => (),
										}

										Process::Fn(&Part, &Entry).await
									}
								})
								.collect::<Vec<_>>(),
						)
						.await,
					);

					match Allow.send(Output) {
						Err(e) => {
							eprintln!("Failed to send output: {}", e);
						},
						_ => (),
					}
				}
			})
		})
		.collect::<Vec<_>>();

	futures::future::join_all(
		Worker
			.into_par_iter()
			.map(|Worker| async { Worker.await.expect("Worker task failed") })
			.collect::<Vec<_>>(),
	)
	.await;

	drop(Allow);

	Output.await.expect("Output task failed");
}

use std::sync::Arc;

use crossbeam_queue::ArrayQueue;
use once_cell::sync::Lazy;
use rayon::{
	iter::{IntoParallelIterator, ParallelIterator},
	prelude::IntoParallelRefIterator,
};
use tokio::sync::{Mutex, mpsc};

use crate::Struct::Binary::Command::Entry::Struct as Option;

static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub mod GPG;

pub mod Process;
