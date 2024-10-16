pub struct Struct {
	pub Separator:Option::Separator,
	pub Fn:Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	pub fn Fn() -> Self {
		Self {
			Separator:std::path::MAIN_SEPARATOR,
			Fn:Box::new(|| {
				Box::pin(async move {
					let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));

					match Option.Parallel {
						true => {
							Parallel::Fn(Option).await;
						},
						false => {
							Sequential::Fn(Option).await;
						},
					};
				})
			}),
		}
	}
}

pub mod Entry;
pub mod Option;

use std::pin::Pin;

use futures::Future;

use crate::Fn::Binary::Command::{Parallel, Sequential};
