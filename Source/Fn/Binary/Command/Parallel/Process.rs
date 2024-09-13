pub async fn Fn(Command: &[String], Entry: &str) -> String {
	String::from_utf8_lossy(
		&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
			.args(&Command[1..])
			.current_dir(Entry)
			.output()
			.await
			.expect("Cannot Output.")
			.stdout,
	)
	.to_string()
}
