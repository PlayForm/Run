pub mod Entry;
pub mod Option;

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
					let options_config = Struct::Fn();
					let options = Entry::Struct::Fn(&Option::Struct::Fn(options_config));

					if options.Parallel {
						crate::Fn::Binary::Command::Parallel::Fn(options).await;
					} else {
						crate::Fn::Binary::Command::Sequential::Fn(options).await;
					};
				})
			}),
		}
	}
}
