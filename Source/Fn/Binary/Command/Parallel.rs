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
	let Sign:Vec<bool> = Command
		.iter()
		.map(|cmd_str| {
			let parts:Vec<String> = cmd_str.split(' ').map(String::from).collect();

			GPG::Fn(&parts)
		})
		.collect();

	let Command = Arc::new(Command);

	let SignArc = Arc::new(Sign);

	let Target:Vec<String> = Entry
		.into_iter()
		.filter_map(|Part| {
			Part.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Part[0..Part.len() - 1].join(&Separator.to_string()))
		})
		.collect();

	let Limit = num_cpus::get();

	stream::iter(Target)
		.for_each_concurrent(Limit, |Path| {
			let Local = Arc::clone(&Command);

			let Sign = Arc::clone(&SignArc);

			async move {
				let Task = Local.iter().enumerate().map(|(Current, Command)| {
					let SignLocal = Sign[Current];

					let Part:Vec<String> = Command.split(' ').map(String::from).collect();

					let Entry = Path.clone();

					async move {
						if SignLocal {
							let _lock = GPG_MUTEX.lock().await;

							Process::Fn(&Part, &Entry).await
						} else {
							Process::Fn(&Part, &Entry).await
						}
					}
				});

				let outputs = futures::future::join_all(Task).await;

				for output in outputs.into_iter().filter(|s| !s.is_empty()) {
					println!("{}", output);
				}
			}
		})
		.await;
}

use std::sync::Arc;

use futures::stream::{self, StreamExt};
// The GPG mutex is still necessary to serialize signing operations.
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::Struct::Binary::Command::Entry::Struct as Option;

static GPG_MUTEX:Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub mod GPG;

pub mod Process;
