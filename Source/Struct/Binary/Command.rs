pub mod Entry;
pub mod Option;

use tokio::sync::mpsc;

/// The main command configuration struct that holds the execution logic.
///
/// This struct's primary role is to create and hold a boxed closure (`Fn`) that
/// encapsulates the entire application workflow.
pub struct Struct {
	pub Separator:　Option::Separator,
	pub Fn:　Box<dyn Fn() -> std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Constructs the command struct and its main execution closure.
	///
	/// The returned `Fn` closure, when called, will handle everything from
	/// parsing CLI arguments to selecting the parallel or sequential execution
	/// strategy.
	pub fn Fn() -> Self {
		Self {
			Separator:　std::path::MAIN_SEPARATOR,
			Fn:　Box::new(|| {
				Box::pin(async move {
					// This initialization pattern allows `Option::Fn` to access the `Separator`
					// from the `options_config` while still using the static `ARGS` for CLI
					// parsing.
					let OptionsConfig = Self::Fn();
					let CommandLineOptions = Option::Struct::Fn(OptionsConfig);
					let ExecutionOptions = Entry::Struct::Fn(&CommandLineOptions);
					let IsTui = CommandLineOptions.Tui;
					let IsParallel = ExecutionOptions.Parallel;

					let (Tx, Rx) = mpsc::unbounded_channel::<crate::Struct::Event::Struct>();

					if IsTui {
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
