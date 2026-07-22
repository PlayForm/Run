pub mod Entry;
pub mod Option;

use tokio::sync::mpsc;

/// The main command configuration struct.
///
/// The `Fn` closure now creates an `mpsc` channel and routes its sender into
/// the execution engine. In TUI mode the receiver is handed to `Tui::Fn`; in
/// CLI mode a lightweight printer task drains it with `println!`.
pub struct Struct {
	pub Separator:Option::Separator,
	pub Fn:Box<dyn Fn() -> std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	pub fn Fn() -> Self {
		Self {
			Separator:std::path::MAIN_SEPARATOR,
			Fn:Box::new(|| {
				Box::pin(async move {
					let OptionsConfig = Self::Fn();
					let CommandLineOptions = Option::Struct::Fn(OptionsConfig);
					let ExecutionOptions = Entry::Struct::Fn(&CommandLineOptions);
					let IsTui = CommandLineOptions.Tui;
					let IsParallel = ExecutionOptions.Parallel;

					// Unbounded channel — execution engines are never back-pressured
					// by a slow consumer.
					let (Tx, Rx) = mpsc::unbounded_channel::<crate::Struct::Event::Struct>();

					if IsTui {
						// Spawn execution in the background; TUI runs on this task.
						if IsParallel {
							tokio::spawn(
								crate::Fn::Binary::Command::Parallel::Fn(ExecutionOptions, Tx),
							);
						} else {
							tokio::spawn(
								crate::Fn::Binary::Command::Sequential::Fn(ExecutionOptions, Tx),
							);
						}
						crate::Fn::Tui::Fn(Rx).await;
					} else {
						// CLI mode: plain printer task drains the channel.
						use crate::Struct::Event::Struct as Event;
						let PrintTask = tokio::spawn(async move {
							let mut Rx = Rx;
							while let Some(Ev) = Rx.recv().await {
								match Ev {
									Event::Line { Text, IsStderr, .. } => {
										if IsStderr {
											eprintln!("{}", Text);
										} else {
											println!("{}", Text);
										}
									}
									Event::IndexLockTimeout { Directory } => {
										eprintln!("Skipping '{}': git index lock timed out.", Directory);
									}
									Event::JobFinished { Directory, Success } if !Success => {
										eprintln!("✗ Failed: {}", Directory);
									}
									_ => {}
								}
							}
						});

						if IsParallel {
							crate::Fn::Binary::Command::Parallel::Fn(ExecutionOptions, Tx).await;
						} else {
							crate::Fn::Binary::Command::Sequential::Fn(ExecutionOptions, Tx).await;
						}
						PrintTask.await.ok();
					}
				})
			}),
		}
	}
}
