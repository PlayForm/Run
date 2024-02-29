mod Parallel;
mod Sequential;

pub struct Struct {
	pub Command: super::Option::Command,
	pub Entry: Vec<_>,
	pub Pattern: super::Option::Pattern,
	pub Separator: super::Option::Separator,
}

impl Struct {
	pub fn Fn(Option: super::Option::Struct) -> Self {
		Self {
			Command: Option.Command,
			// Entry: super::Entry::Fn(Option),
			Pattern: Option.Pattern,
		}
	}
}

pub fn Fn(Option: super::Option::Struct) {
	let super::Option::Struct { Parallel, .. } = Option;

	let Option = Struct::Fn(Option);

	match Parallel {
		true => {
			Parallel::Fn(Option);
		}
		false => {
			Sequential::Fn(Option);
		}
	}
}
