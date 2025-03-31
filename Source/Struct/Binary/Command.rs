pub struct Struct {
	pub Separator: Option::Separator,
	pub Fn: Box<dyn Fn() -> std::pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	pub fn Fn() -> Self {
		Self {
			Separator: std::path::MAIN_SEPARATOR,
			Fn: Box::new(|| {
				Box::pin(async move {
					let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));

					match Option.Parallel {
						true => {
							crate::Fn::Binary::Command::Parallel::Fn(Option).await;
						},
						false => {
							crate::Fn::Binary::Command::Sequential::Fn(Option).await;
						},
					};
				})
			}),
		}
	}
}

pub mod Entry;

pub mod Option;
