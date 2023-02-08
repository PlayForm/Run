use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
	#[arg(short, long)]
	directories: Vec<String>,

	/// Number of times to greet
	#[arg(short, long, default_value_t = 1)]
	count: u8,
}

fn main() {}
