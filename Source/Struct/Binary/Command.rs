pub mod Entry;
pub mod Option;

/// The main command configuration struct that holds the execution logic.
///
/// This struct's primary role is to create and hold a boxed closure (`Fn`) that
/// encapsulates the entire application workflow.
pub struct Struct {
	pub Separator:Option::Separator,
	pub Fn:Box<dyn Fn() -> std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Constructs the command struct and its main execution closure.
	///
	/// The returned `Fn` closure, when called, will handle everything from
	/// parsing CLI arguments to selecting the parallel or sequential execution
	/// strategy.
	pub fn Fn() -> Self {
		Self {
			Separator:std::path::MAIN_SEPARATOR,
			Fn:Box::new(|| {
				Box::pin(async move {
					// This initialization pattern allows `Option::Fn` to access the `Separator`
					// from the `options_config` while still using the static `ARGS` for CLI
					// parsing.
					let OptionsConfig = Self::Fn();
					let CommandLineOptions = Option::Struct::Fn(OptionsConfig);
					let ExecutionOptions = Entry::Struct::Fn(&CommandLineOptions);

					if ExecutionOptions.Parallel {
						crate::Fn::Binary::Command::Parallel::Fn(ExecutionOptions).await;
					} else {
						crate::Fn::Binary::Command::Sequential::Fn(ExecutionOptions).await;
					};
				})
			}),
		}
	}
}
