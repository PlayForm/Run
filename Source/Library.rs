#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]

mod Fn;
mod Struct;

/// The main entry point of the application.
///
/// This function initializes the command structure and invokes the main
/// execution future, which handles argument parsing, path finding, and command
/// execution.
#[tokio::main]
#[allow(dead_code)]
async fn main() {
	// The `Fn` field on the returned struct holds the main async closure.
	(Struct::Binary::Command::Struct::Fn().Fn)().await
}
