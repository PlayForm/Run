mod Entry;
mod Parallel;
mod Sequential;

#[derive(Debug)]
pub(crate) struct Struct {
	pub Separator: Option::Separator,
	pub Fn: fn(),
}

impl Struct {
	pub fn Fn() -> Self {
		Self {
			Separator: std::path::MAIN_SEPARATOR,
			Fn: || {
				let Option = Entry::Struct::Fn(Option::Struct::Fn(Struct::Fn()));

				match Option.Parallel {
					true => {
						Parallel::Fn(Option);
					}
					false => {
						Sequential::Fn(Option);
					}
				};
			},
		}
	}
}
