mod Entry;
mod Execution;
mod Option;

#[derive(Debug)]
pub struct Struct {
	pub Separator: Option::Separator,
}

pub fn Fn() {
	Execution::Fn(Option::Struct::Fn());
}
