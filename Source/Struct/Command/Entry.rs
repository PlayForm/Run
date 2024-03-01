pub(crate) struct Struct {
	pub Command: super::Option::Command,
	pub Entry: todo!(),
	pub Pattern: super::Option::Pattern,
	pub Separator: super::Option::Separator,
	pub Parallel: super::Option::Parallel,
}

impl Struct {
	pub fn Fn(Option: Option) -> Self {
		Self {
			Command: Option.Command,
			Entry: super::Entry::Fn(Option),
			Pattern: Option.Pattern,
			Separator: Option.Separator,
			Parallel: Option.Parallel,
		}
	}
}
