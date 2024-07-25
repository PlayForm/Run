#![allow(non_snake_case)]

mod Fn;
mod Struct;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
	(Struct::Binary::Command::Struct::Fn().Fn)().await
}
