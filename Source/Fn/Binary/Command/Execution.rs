use super::Parameter;

mod Parallel;
mod Sequential;

pub fn Fn() {
	let Parameter::Struct { File, Parallel, Exclude, Pattern, Command, .. } =
		Parameter::Struct::Fn();

	let Separator: char = std::path::MAIN_SEPARATOR;

	if Option {
		Parallel::Fn(Separator);
	} else {
		Sequential::Fn(Separator);
	}
}
