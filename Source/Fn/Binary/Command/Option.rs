mod Match;

pub type Pattern = String;
pub type Command = String;
pub type Separator = char;

#[derive(Debug)]
pub struct Struct {
	pub File: bool,
	pub Parallel: bool,
	pub Root: String,
	pub Exclude: Vec<String>,
	pub Pattern: Pattern,
	pub Command: String,
	pub Separator: Separator,
}

impl Struct {
	pub fn Fn() -> Self {
		Self {
			File: Match::Fn().get_flag("File"),
			Parallel: Match::Fn().get_flag("Parallel"),
			Root: Match::Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Exclude: Match::Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Pattern: Match::Fn().get_one::<String>("Pattern").expect("Cannot Pattern").to_owned(),
			Command: Match::Fn()
				.get_many::<String>("Command")
				.expect("Cannot Command")
				.map(|Command| Command.as_str())
				.collect::<Vec<_>>()
				.join(" "),
		}
	}
}
