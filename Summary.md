🗣️ Summary from Inn/v0.1.9 to Run/v0.0.1 in .
diff --git a/.gitignore b/.gitignore
index cbb2f71..33f48c4 100644
--- a/.gitignore
+++ b/.gitignore
- !/target/release/Inn
- !/target/release/Innkeeper
+ !/target/release/PRun
+ !/target/release/Run
diff --git a/Cargo.toml b/Cargo.toml
index b8e9a72..08052b2 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- name = "Inn"
+ name = "PRun"
- name = "Innkeeper"
+ name = "Run"
- default-run = "Inn"
+ default-run = "Run"
- name = "innkeeper"
- repository = "https://github.com/NikolaRHristov/Inn.git"
- version = "0.1.9"
+ name = "prun"
+ repository = "https://github.com/Playform/Run.git"
+ version = "0.0.1"
diff --git a/README.md b/README.md
index f9edcac..81e5bf5 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Inn]
+ # 🍺 [Run]
- Inn is a command-line tool designed to execute a specified command in all
+ `Run` is a command-line tool designed to execute a specified command in all
- [Inn]: https://crates.io/crates/innkeeper
+ [Run]: https://crates.io/crates/prun
- 			<pre>Inn -P .git ls</pre>
+ 			<pre>Run -P .git ls</pre>
- 			<pre>Inn -P .git git status</pre>
+ 			<pre>Run -P .git git status</pre>
- 			<pre>Inn -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
+ 			<pre>Run -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
- cargo install innkeeper
+ cargo install prun
- Inn .git git fetch upstream
+ Run .git git fetch upstream
- You can hide the command output by specifying an `-H` or `--Hide` parameter:
- 
- ```sh
- Inn -H -F package.json ncu -u
- ```
- 
- Inn -F astro.config.ts npx astro add @playform/compress
+ Run -F astro.config.ts npx astro add @playform/compress
- Inn -R D:\Developer .git git fetch upstream
+ Run -R D:\Developer .git git fetch upstream
- Inn -P -R D:\Developer .git git fetch upstream
+ Run -P -R D:\Developer .git git fetch upstream
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index f301f4f..4236297 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 	Command::new("Inn")
+ 	Command::new("Run")
- 		.author("Nikola R. Hristov <nikola@playform.cloud>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("Hide")
- 				.short('H')
- 				.long("Hide")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("HIDE")
- 				.required(false)
- 				.help("Hide output."),
- 		)
+ 		.author("Playform <hello@playform.cloud>")
+ 		.about("🍺 Run.")
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 847b4ea..7b413bb 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
+ 		.follow_links(false)
- 					true => {
- 						std::fs::metadata(std::path::PathBuf::from(&Path))
- 							.expect("Cannot Metadata.")
- 							.is_dir() && Match
- 					}
+ 					true => match std::fs::metadata(std::path::PathBuf::from(&Path)) {
+ 						Ok(Metadata) => Metadata.is_dir() && Match,
+ 						Err(_Error) => false,
+ 					},
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c3e16f8..5b62589 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
- 	pub Hide: Hide,
- 			Hide: Option.Hide.clone(),
- 	Command, Hide, Parallel, Pattern, Separator, Struct as Option,
+ 	Command, Parallel, Pattern, Separator, Struct as Option,
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 82d1dc4..5a6eae6 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- pub type Hide = bool;
+ 
- 	pub Hide: Hide,
+ 
- 			Hide: Fn().get_flag("Hide"),

🗣️ Summary from Run/v0.0.1 to Run/v0.0.2 in .
diff --git a/Cargo.toml b/Cargo.toml
index 08052b2..054df0b 100644
--- a/Cargo.toml
+++ b/Cargo.toml
+ [[bin]]
+ name = "Inn"
+ path = "Source/Library.rs"
+ 
+ [[bin]]
+ name = "Innkeeper"
+ path = "Source/Library.rs"
+ 
- version = "0.0.1"
+ version = "0.0.2"

🗣️ Summary from Run/v0.0.2 to Run/v0.0.3 in .
diff --git a/Cargo.toml b/Cargo.toml
index 054df0b..eef51f8 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.2"
+ version = "0.0.3"

🗣️ Summary from Run/v0.0.3 to Run/v0.0.4 in .
diff --git a/Cargo.toml b/Cargo.toml
index eef51f8..d7ac511 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.3"
+ version = "0.0.4"
diff --git a/README.md b/README.md
index 81e5bf5..4cdb5e1 100644
--- a/README.md
+++ b/README.md
- To specify a `--File` argument or `-F`, if you would like to search for a file
- instead of a directory, use:
+ If you want to limit execution to files matching a certain pattern only specify
+ a `--File` argument or `-F`:
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 4236297..d807fb1 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 				.help("Search file."),
+ 				.help("File."),
- 				.help("Execute code in parallel."),
+ 				.help("Parallel."),
- 				.help("Current working directory.")
+ 				.help("Root.")
- 				.help("Exclude pattern.")
+ 				.help("Exclude.")
- 				.help("Search pattern.")
+ 				.help("Pattern.")
- 				.help("Command to run."),
+ 				.help("Command."),

🗣️ Summary from Run/v0.0.4 to Run/v0.0.5 in .
diff --git a/Cargo.toml b/Cargo.toml
index d7ac511..91794ac 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- tokio = { features = ["full"], version = "1.36.0" }
+ tokio = { features = ["full"], version = "1.37.0" }
- description = "🍺 Run."
+ description = "🍺 Run"
- version = "0.0.4"
+ version = "0.0.5"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index 13ffed8..8c55e25 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
+ [homepage]: HTTPS://www.contributor-covenant.org
+ [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
+ [FAQ]: HTTPS://www.contributor-covenant.org/faq
+ [translations]: HTTPS://www.contributor-covenant.org/translations
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 4700843..c749fbe 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
+ [homepage]: HTTPS://www.contributor-covenant.org
+ [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
+ [FAQ]: HTTPS://www.contributor-covenant.org/faq
+ [translations]: HTTPS://www.contributor-covenant.org/translations
diff --git a/README.md b/README.md
index 4cdb5e1..bea9282 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Run]
+ # 🍺 [Run.]
- [Run]: https://crates.io/crates/prun
+ [Run]: HTTPS://crates.io/crates/prun
+ [Run.]: HTTPS://crates.io/crates/prun
+ 
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 7b413bb..3b4e3ca 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
+ /// This Rust function walks through a directory, filters out certain files based on exclusion criteria,
+ /// and returns a collection of paths.
+ /// 
+ /// Arguments:
+ /// 
+ /// * ``: It looks like you have a function that takes an `Option` struct as a parameter and performs
+ /// some file system operations based on the provided configuration. Here's a breakdown of the
+ /// parameters:
+ /// 
+ /// Returns:
+ /// 
+ /// a `Vec<String>` containing paths that meet the specified criteria after processing the entries from
+ /// the directory specified by the `Root` parameter.
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 11cc07e..5b5b704 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
+ /// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
+ /// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
+ /// as the current directory, and prints the output of each Command execution.
+ ///
+ /// Arguments:
+ ///
+ /// * ``: It looks like you have a Rust function named `Fn` that takes an `Option` enum as a parameter.
+ /// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
+ /// fields.

🗣️ Summary from Run/v0.0.5 to Run/v0.0.6 in .
diff --git a/.github/FUNDING.yml b/.github/FUNDING.yml
new file mode 100644
index 0000000..3ba6945
--- /dev/null
+++ b/.github/FUNDING.yml
+ open_collective: playform-cloud-collective
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index cfa5b96..387fece 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-             - uses: dependabot/fetch-metadata@v2.0.0
+             - uses: dependabot/fetch-metadata@v2.2.0
-             - uses: dependabot/fetch-metadata@v2.0.0
+             - uses: dependabot/fetch-metadata@v2.2.0
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index 0be30ba..7b1e399 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
+             TERRAFORM_TELEMETRY: 0
-             - uses: pozil/auto-assign-issue@v1.14.0
+             - uses: pozil/auto-assign-issue@v2.0.0
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index cfdee9a..9edf181 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
+             TERRAFORM_TELEMETRY: 0
-             - uses: actions/checkout@v4.1.2
+             - uses: actions/checkout@v4.1.7
diff --git a/.gitignore b/.gitignore
index 33f48c4..370ce1d 100644
--- a/.gitignore
+++ b/.gitignore
- Cargo.lock
- 
diff --git a/build.rs b/build.rs
index 550762c..73ccc94 100644
--- a/build.rs
+++ b/build.rs
+ use serde::Deserialize;
+ #[derive(Deserialize)]
+ struct Toml {
+ 	package: Package,
+ }
+ 
+ #[derive(Deserialize)]
+ struct Package {
+ 	version: String,
+ }
+ 
+ 
- 		fs::read_to_string("Cargo.toml")
- 			.expect("Cannot Cargo.toml.")
- 			.lines()
- 			.find(|Line| Line.starts_with("version"))
- 			.expect("Cannot Version.")
- 			.split('=')
- 			.nth(1)
- 			.expect("Cannot nth.")
- 			.trim()
- 			.trim_matches('"')
+ 		(toml::from_str::<Toml>(&fs::read_to_string("Cargo.toml").expect("Cannot Cargo.toml."))
+ 			.expect("Cannot toml."))
+ 		.package
+ 		.version
diff --git a/Cargo.toml b/Cargo.toml
index 91794ac..86d4fd3 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- clap = { features = ["derive"], version = "4.5.4" }
- tokio = { features = ["full"], version = "1.37.0" }
+ clap = { features = ["derive"], version = "4.5.11" }
+ futures = "0.3.30"
+ rayon = "1.10.0"
+ tokio = { version = "1.39.1", features = ["full"] }
+ num_cpus = "1.16.0"
+ 
+ [build-dependencies]
+ serde = { version = "1.0.204", features = ["derive"] }
+ toml = "0.8.16"
- description = "🍺 Run"
+ description = "🍺 Run"
- repository = "https://github.com/Playform/Run.git"
- version = "0.0.5"
+ repository = "https://github.com/PlayForm/Run.git"
+ version = "0.0.6"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index 8c55e25..01e92b5 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- community@playform.cloud. All complaints will be reviewed and investigated
+ Community@PlayForm.Cloud. All complaints will be reviewed and investigated
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index c749fbe..c390eae 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- community@playform.cloud. All complaints will be reviewed and investigated
+ Community@PlayForm.Cloud. All complaints will be reviewed and investigated
diff --git a/LICENSE b/LICENSE
index c47b9fa..f236d76 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2023-2024 Nikola R. Hristov
+ Copyright (c) 2023-2024 PlayForm
diff --git a/README.md b/README.md
index bea9282..0fad80f 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Run.]
+ # 🍺 [Run] —
- 			<pre>Run -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
+ 			<pre>Run -P .git 'git add . && git ecommit && git sync'</pre>
- bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
+ bash -c 'git add . && git ecommit && git sync' \;</pre>
- -   `tokio` - Enables parallel execution of tasks.
+ -   `rayon` - Enables parallel execution of tasks.
- [Run.]: HTTPS://crates.io/crates/prun
+ [Run]: HTTPS://crates.io/crates/prun
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index d807fb1..5515b71 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 		.author("Playform <hello@playform.cloud>")
- 		.about("🍺 Run.")
+ 		.author("PlayForm <hello@playform.cloud>")
+ 		.about("🍺 Run —")
- 				.help("File."),
+ 				.help("📝 File —"),
- 				.help("Parallel."),
+ 				.help("⏩ Parallel —"),
- 				.help("Root.")
+ 				.help("📂 Root —")
- 				.help("Exclude.")
+ 				.help("🚫 Exclude —")
- 				.help("Pattern.")
+ 				.help("🔍 Pattern —")
- 				.help("Command."),
+ 				.help("🖥️ Command —"),
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 3b4e3ca..b771f11 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
- /// * ``: It looks like you have a function that takes an `Option` struct as a parameter and performs
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 5b5b704..ff9548b 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- /// * ``: It looks like you have a Rust function named `Fn` that takes an `Option` enum as a parameter.
- pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	let mut Queue = Vec::new();
- 
+ pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	let Queue: Vec<String> = stream::iter(
- 		.into_iter()
+ 			.into_par_iter()
- 		.for_each(|Entry| {
- 			let Output = Command::new(Command.get(0).expect("Cannot Command."))
+ 			.collect::<Vec<String>>(),
+ 	)
+ 	.map(|Entry| {
+ 		let Command = Command.clone();
+ 
+ 		async move {
+ 			String::from_utf8_lossy(
+ 				&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
- 				.output();
- 
- 			Queue.push(async move { Output.await.expect("Cannot await.").stdout });
- 		});
- 
- 	tokio::runtime::Builder::new_multi_thread()
- 		.enable_all()
- 		.build()
- 		.expect("Cannot Runtime.")
- 		.block_on(async {
- 			for Queue in Queue {
- 				println!("{}", String::from_utf8_lossy(&Queue.await));
+ 					.output()
+ 					.await
+ 					.expect("Cannot Output.")
+ 					.stdout,
+ 			)
+ 			.to_string()
+ 	.buffer_unordered(num_cpus::get())
+ 	.collect()
+ 	.await;
+ 
+ 	Queue.par_iter().for_each(|Output| println!("{}", Output));
- use tokio::process::Command;
+ use futures::stream::{self, StreamExt};
+ use rayon::prelude::*;
diff --git a/Source/Library.rs b/Source/Library.rs
index 982f1a3..62cfaff 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
- fn main() {
- 	(Struct::Binary::Command::Struct::Fn().Fn)()
+ #[tokio::main]
+ async fn main() {
+ 	(Struct::Binary::Command::Struct::Fn().Fn)().await
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index faff4af..2a79ad1 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
- pub mod Entry;
- pub mod Option;
- 
- use crate::Fn::Binary::Command::{Parallel, Sequential};
- 
- #[derive(Debug)]
- 	pub Fn: fn(),
+ 	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
- 			Fn: || {
+ 			Fn: Box::new(|| {
+ 				Box::pin(async move {
- 						Parallel::Fn(Option);
+ 							Parallel::Fn(Option).await;
- 			},
+ 				})
+ 			}),
+ 
+ pub mod Entry;
+ pub mod Option;
+ 
+ use crate::Fn::Binary::Command::{Parallel, Sequential};
+ 
+ use futures::Future;
+ use std::pin::Pin;

🗣️ Summary from Run/v0.0.6 to Run/v0.0.7 in .
diff --git a/.cargo/Config.toml b/.cargo/Config.toml
new file mode 100644
index 0000000..5507528
--- /dev/null
+++ b/.cargo/Config.toml
+ [build]
+ target-dir = "Target"
+ 
+ [cargo-new]
+ vcs = "git"
+ 
+ [profile.release]
+ opt-level = 3
+ codegen-units = 1
+ debug = false
+ lto = true
+ panic = "abort"
diff --git a/.gitignore b/.gitignore
index 370ce1d..34f0334 100644
--- a/.gitignore
+++ b/.gitignore
- /target/*
- !/target/release
- /target/release/*
- !/target/release/*.deb
- !/target/release/*.exe
- !/target/release/PRun
- !/target/release/Run
+ /Target/*
+ 
+ !/Target/release
+ /Target/release/*
+ 
+ !/Target/release/*.deb
+ !/Target/release/*.exe
+ !/Target/release/PRun
+ !/Target/release/Run
diff --git a/Cargo.toml b/Cargo.toml
index 86d4fd3..8fc9504 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- description = "🍺 Run"
+ description = "🍺 Run —"
- version = "0.0.6"
+ version = "0.0.7"
diff --git a/README.md b/README.md
index 0fad80f..77f3dee 100644
--- a/README.md
+++ b/README.md
- `Run` is a command-line tool designed to execute a specified command in all
- directories that match a certain pattern within a given root directory. It
- provides flexibility and efficiency in running commands across multiple
- directories with customizable patterns.
+ `Run` is a command-line tool that executes commands in multiple directories
+ simultaneously. It leverages parallel processing and concurrent I/O to
+ efficiently run tasks across directories.
- This command will fetch from `upstream` for all the `.git` repositories inside
- the current directory. Basically, it replaces the following command:
+ This command will fetch from upstream for all .git repositories inside the
+ current directory. It essentially replaces the following command:
- If you want to limit execution to files matching a certain pattern only specify
- a `--File` argument or `-F`:
+ ## Options
+ 
+ #### --File or -F:
+ 
+ Limit execution to files matching a certain pattern:
- Additionally, you can provide a `--Root` argument or `-R` to set the current
- working directory to a different folder. The default is `.`.
+ #### --Root or -R:
+ 
+ Set the current working directory to a different folder (default is .):
- Specify a `--Parallel` argument or `-P` if you would like to run commands in
- parallel. The default is sequential.
+ #### --Parallel or -P:
+ 
+ Run commands in parallel (default is sequential):
+ #### --Exclude:
+ 
+ Exclude certain files or directories
+ 
+ #### --Pattern:
+ 
+ Specify a custom pattern for matching
+ 
+ #### --Separator:
+ 
+ Define a custom separator
+ 
- The code imports several crates:
+ `Run` relies on several Rust crates to provide its functionality:
- -   `clap` - For parsing command-line arguments.
- -   `rayon` - Enables parallel execution of tasks.
- -   `walkdir` - Facilitates filesystem traversal.
+ -   `clap` - Parses command-line arguments
+ -   `rayon` - Enables parallel processing
+ -   `tokio` - Provides an asynchronous runtime
+ -   `walkdir` - Facilitates efficient filesystem traversal
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 5515b71..f9a3fbf 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
+ /// This function defines and configures command line arguments for the "Run" command.
+ /// It sets up various arguments such as File, Parallel, Root, Exclude, Pattern, and Command.
+ /// Each argument has specific properties like short and long flags, display order, value names, required status, help messages, and default values.
+ /// The function returns the parsed command line arguments using ArgMatches.
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 9a9d6fa..2c80c56 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
+ /// Executes a command with arguments in a specific directory for each entry in the given list.
+ ///
+ /// # Arguments
+ ///
+ /// * `Option` - A struct containing `Command`, `Entry`, `Pattern`, `Separator`, and other optional fields.
+ ///
+ /// # Example
+ ///
+ /// ```
+ /// use std::process::{Command, Stdio};
+ ///
+ /// let options = Option { Command: vec!["ls".to_string()], Entry: vec!["/path/to/dir".to_string()], Pattern: "pattern", Separator: '/'.to_string() };
+ /// Fn(options);
+ /// ```

🗣️ Summary from Run/v0.0.7 to Run/v0.1.0 in .
diff --git a/Cargo.toml b/Cargo.toml
index 8fc9504..7185e08 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- tokio = { version = "1.39.1", features = ["full"] }
+ tokio = { version = "1.39.2", features = ["full"] }
- toml = "0.8.16"
+ toml = "0.8.17"
- version = "0.0.7"
+ version = "0.1.0"
+ include = [
+ 	"Source/**/*",
+ 	"LICENSE",
+ 	"README.md",
+ 	"CHANGELOG.md",
+ 	"build.rs",
+ 	"Cargo.toml",
+ ]
diff --git a/README.md b/README.md
index 77f3dee..706161c 100644
--- a/README.md
+++ b/README.md
- simultaneously. It leverages parallel processing and concurrent I/O to
+ simultaneously. It leverages parallel processing and concurrent `I/O` to
- ## Benchmark
+ ## Bench
- 			<pre>Run -P .git ls</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m9.441s
- user    0m0.030s
- sys     0m0.046s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 			<pre>find -iname .git -execdir ls \;</pre>
- 			<pre>real    0m14.293s +5s
- user    0m4.645s +4s
- sys     0m8.937s +8s</pre>
+ 			<pre>real    0m14.476s
+ user    0m5.260s
+ sys     0m7.526s</pre>
- 			<pre>Run -P .git git status</pre>
+ 			<pre>Run -P .git ls</pre>
- 			<pre>real    0m24.146s
+ 			<pre>real    0m7.194s
- sys     0m0.062s</pre>
+ sys     0m0.045s</pre>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 			<pre>find -iname .git -type d -execdir git status \;</pre>
- 			<pre>real    0m28.584s +4s
- user    0m4.695s +4s
- sys     0m8.354s +8s</pre>
+ 			<pre>real    1m1.242s
+ user    0m4.080s
+ sys     0m6.354s</pre>
- 			<pre>Run -P .git 'git add . && git ecommit && git sync'</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m33.813s
- user    0m0.015s
- sys     0m0.060s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir \
- bash -c 'git add . && git ecommit && git sync' \;</pre>
+ 			<pre>Run -P .git git status</pre>
- 			<pre>real    0m53.122s +20s
- user    0m9.449s +9s
- sys     0m14.442s +14s</pre>
+ 			<pre>real    0m21.947s
+ user    0m0.045s
+ sys     0m0.031s</pre>
- This command will fetch from upstream for all .git repositories inside the
+ This command will fetch from upstream for all `.git` repositories inside the
- Set the current working directory to a different folder (default is .):
+ Set the current working directory to a different folder (default is `.`):
- Run commands in parallel (default is sequential):
+ Run commands in `parallel` (default is `sequential`):
- Exclude certain files or directories
+ Exclude certain files or directories (defailt is
+ `node_modules .git target dist vendor`)
- #### --Separator:
- 
- Define a custom separator
- 
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index f9a3fbf..5eca35e 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 		.author("PlayForm <hello@playform.cloud>")
+ 		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index ff9548b..5e71ca9 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 	let Queue: Vec<String> = stream::iter(
+ 	let Queue: Vec<String> = futures::stream::iter(
- use futures::stream::{self, StreamExt};
+ use futures::stream::StreamExt;

🗣️ Summary from Run/v0.1.0 to v0.0.1 in .
diff --git a/.cargo/Config.toml b/.cargo/Config.toml
deleted file mode 100644
index 5507528..0000000
--- a/.cargo/Config.toml
+++ /dev/null
- [build]
- target-dir = "Target"
- 
- [cargo-new]
- vcs = "git"
- 
- [profile.release]
- opt-level = 3
- codegen-units = 1
- debug = false
- lto = true
- panic = "abort"
diff --git a/.github/FUNDING.yml b/.github/FUNDING.yml
deleted file mode 100644
index 3ba6945..0000000
--- a/.github/FUNDING.yml
+++ /dev/null
- open_collective: playform-cloud-collective
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index 387fece..819f8a1 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-             - uses: dependabot/fetch-metadata@v2.2.0
+             - uses: dependabot/fetch-metadata@v1.6.0
-             - uses: dependabot/fetch-metadata@v2.2.0
+             - uses: dependabot/fetch-metadata@v1.6.0
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index 7b1e399..ffde8df 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
+             TELEMETRY_DISABLED: 1
+             DO_NOT_TRACK: 1
-             DO_NOT_TRACK: 1
-             GATSBY_TELEMETRY_OPTOUT: 1
-             GRIT_TELEMETRY_DISABLED: 1
+             GATSBY_TELEMETRY_OPTOUT: 1
-             TELEMETRY_DISABLED: 1
-             TERRAFORM_TELEMETRY: 0
-             - uses: pozil/auto-assign-issue@v2.0.0
+             - uses: pozil/auto-assign-issue@v1.13.0
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 9edf181..161d7eb 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-         branches: [Current]
+         branches: [main]
-         branches: [Current]
+         branches: [main]
+             TELEMETRY_DISABLED: 1
+             DO_NOT_TRACK: 1
-             DO_NOT_TRACK: 1
-             GATSBY_TELEMETRY_OPTOUT: 1
-             GRIT_TELEMETRY_DISABLED: 1
+             GATSBY_TELEMETRY_OPTOUT: 1
-             TELEMETRY_DISABLED: 1
-             TERRAFORM_TELEMETRY: 0
-             - uses: actions/checkout@v4.1.7
+             - uses: actions/checkout@v4.1.1
-             - uses: actions/cache@v4.0.2
+             - uses: actions/cache@v4.0.1
diff --git a/.gitignore b/.gitignore
index 34f0334..cbb2f71 100644
--- a/.gitignore
+++ b/.gitignore
- /Target/*
+ Cargo.lock
- !/Target/release
- /Target/release/*
- 
- !/Target/release/*.deb
- !/Target/release/*.exe
- !/Target/release/PRun
- !/Target/release/Run
+ /target/*
+ !/target/release
+ /target/release/*
+ !/target/release/*.deb
+ !/target/release/*.exe
+ !/target/release/Inn
+ !/target/release/Innkeeper
diff --git a/build.rs b/build.rs
index 73ccc94..550762c 100644
--- a/build.rs
+++ b/build.rs
- use serde::Deserialize;
- #[derive(Deserialize)]
- struct Toml {
- 	package: Package,
- }
- 
- #[derive(Deserialize)]
- struct Package {
- 	version: String,
- }
- 
- 
- 		(toml::from_str::<Toml>(&fs::read_to_string("Cargo.toml").expect("Cannot Cargo.toml."))
- 			.expect("Cannot toml."))
- 		.package
- 		.version
+ 		fs::read_to_string("Cargo.toml")
+ 			.expect("Cannot Cargo.toml.")
+ 			.lines()
+ 			.find(|Line| Line.starts_with("version"))
+ 			.expect("Cannot Version.")
+ 			.split('=')
+ 			.nth(1)
+ 			.expect("Cannot nth.")
+ 			.trim()
+ 			.trim_matches('"')
diff --git a/Cargo.toml b/Cargo.toml
index 7185e08..37256c8 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- name = "PRun"
- path = "Source/Library.rs"
- 
- [[bin]]
- name = "Run"
- path = "Source/Library.rs"
- 
- [[bin]]
- name = "Inn"
- path = "Source/Library.rs"
- 
- [[bin]]
- name = "Innkeeper"
+ name = "INN2"
- clap = { features = ["derive"], version = "4.5.11" }
+ clap = { features = ["derive"], version = "4.5.1" }
+ tokio = { features = ["full"], version = "1.36.0" }
- futures = "0.3.30"
- rayon = "1.10.0"
- tokio = { version = "1.39.2", features = ["full"] }
- num_cpus = "1.16.0"
- 
- [build-dependencies]
- serde = { version = "1.0.204", features = ["derive"] }
- toml = "0.8.17"
- default-run = "Run"
- description = "🍺 Run —"
+ default-run = "INN2"
+ description = "🍺 INN2 lets you execute parallel commands in multiple directories."
- name = "prun"
- repository = "https://github.com/PlayForm/Run.git"
- version = "0.1.0"
+ name = "inn2"
+ repository = "https://github.com/NikolaRHristov/INN2.git"
+ version = "0.0.1"
- include = [
- 	"Source/**/*",
- 	"LICENSE",
- 	"README.md",
- 	"CHANGELOG.md",
- 	"build.rs",
- 	"Cargo.toml",
- ]
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index 01e92b5..b4f1f9b 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- Community@PlayForm.Cloud. All complaints will be reviewed and investigated
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- [homepage]: HTTPS://www.contributor-covenant.org
- [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
- [FAQ]: HTTPS://www.contributor-covenant.org/faq
- [translations]: HTTPS://www.contributor-covenant.org/translations
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index c390eae..c740185 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- Community@PlayForm.Cloud. All complaints will be reviewed and investigated
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- [homepage]: HTTPS://www.contributor-covenant.org
- [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
- [FAQ]: HTTPS://www.contributor-covenant.org/faq
- [translations]: HTTPS://www.contributor-covenant.org/translations
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
diff --git a/LICENSE b/LICENSE
index f236d76..c47b9fa 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2023-2024 PlayForm
+ Copyright (c) 2023-2024 Nikola R. Hristov
diff --git a/README.md b/README.md
index 706161c..26d3127 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Run] —
+ # 🍺 [INN2]
- `Run` is a command-line tool that executes commands in multiple directories
- simultaneously. It leverages parallel processing and concurrent `I/O` to
- efficiently run tasks across directories.
+ INN2 is a command-line tool designed to execute a specified command in all
+ directories that match a certain pattern within a given root directory. It
+ provides flexibility and efficiency in running commands across multiple
+ directories with customizable patterns.
- [Run]: HTTPS://crates.io/crates/prun
+ [INN2]: https://crates.io/crates/inn2
- ## Bench
+ ## Benchmark
- 			<pre>find -iname .git -execdir ls \;</pre>
+ 			<pre>INN2 -P .git ls</pre>
- 			<pre>real    0m14.476s
- user    0m5.260s
- sys     0m7.526s</pre>
+ 			<pre>real    0m9.441s
+ user    0m0.030s
+ sys     0m0.046s</pre>
- 			<pre>Run -P .git ls</pre>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m14.293s +5s
+ user    0m4.645s +4s
+ sys     0m8.937s +8s</pre>
+ 	</tr>
+ 	<tr>
- 			<pre>real    0m7.194s
+ 			<pre>INN2 -P .git git status</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m24.146s
- sys     0m0.045s</pre>
+ sys     0m0.062s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m28.584s +4s
+ user    0m4.695s +4s
+ sys     0m8.354s +8s</pre>
- 			<pre>find -iname .git -type d -execdir git status \;</pre>
+ 			<pre>INN2 -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
- 			<pre>real    1m1.242s
- user    0m4.080s
- sys     0m6.354s</pre>
+ 			<pre>real    0m33.813s
+ user    0m0.015s
+ sys     0m0.060s</pre>
- 			<pre>Run -P .git git status</pre>
+ 			<pre>find -iname .git -type d -execdir \
+ bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
- 			<pre>real    0m21.947s
- user    0m0.045s
- sys     0m0.031s</pre>
+ 			<pre>real    0m53.122s +20s
+ user    0m9.449s +9s
+ sys     0m14.442s +14s</pre>
- cargo install prun
+ cargo install inn2
- Run .git git fetch upstream
+ INN2 .git git fetch upstream
- This command will fetch from upstream for all `.git` repositories inside the
- current directory. It essentially replaces the following command:
+ This command will fetch from upstream for all the .git repositories inside the
+ current directory. Essentially, it replaces the following shell command:
- ## Options
- 
- #### --File or -F:
- 
- Limit execution to files matching a certain pattern:
+ To specify a `--File` argument or `-F`, if you would like to search for a file
+ instead of a directory, use:
- Run -F astro.config.ts npx astro add @playform/compress
+ INN2 -F astro.config.ts npx astro add astro-compress
- #### --Root or -R:
- 
- Set the current working directory to a different folder (default is `.`):
+ Additionally, you can provide a `--Root` argument or `-R` to set the current
+ working directory to a different folder. The default is `.`.
- Run -R D:\Developer .git git fetch upstream
+ INN2 -R D:\Developer .git git fetch upstream
- #### --Parallel or -P:
- 
- Run commands in `parallel` (default is `sequential`):
+ Specify a `--Parallel` argument or `-P` if you would like to run commands in
+ parallel. The default is sequential.
- Run -P -R D:\Developer .git git fetch upstream
+ INN2 -P -R D:\Developer .git git fetch upstream
- #### --Exclude:
- 
- Exclude certain files or directories (defailt is
- `node_modules .git target dist vendor`)
- 
- #### --Pattern:
- 
- Specify a custom pattern for matching
- 
- `Run` relies on several Rust crates to provide its functionality:
- 
- -   `clap` - Parses command-line arguments
- -   `rayon` - Enables parallel processing
- -   `tokio` - Provides an asynchronous runtime
- -   `walkdir` - Facilitates efficient filesystem traversal
+ The code imports several crates:
- [Run]: HTTPS://crates.io/crates/prun
+ -   `clap` - For parsing command-line arguments.
+ -   `tokio` - Enables parallel execution of tasks.
+ -   `walkdir` - Facilitates filesystem traversal.
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 5eca35e..d9d47b8 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- /// This function defines and configures command line arguments for the "Run" command.
- /// It sets up various arguments such as File, Parallel, Root, Exclude, Pattern, and Command.
- /// Each argument has specific properties like short and long flags, display order, value names, required status, help messages, and default values.
- /// The function returns the parsed command line arguments using ArgMatches.
- 	Command::new("Run")
+ 	Command::new("Innkeeper")
- 		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
- 		.about("🍺 Run —")
+ 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.about("Run a command in all directories having a certain pattern.")
- 				.display_order(2)
+ 				.display_order(1)
- 				.help("📝 File —"),
+ 				.help("Search file."),
- 				.display_order(3)
+ 				.display_order(2)
- 				.help("⏩ Parallel —"),
+ 				.help("Execute code in parallel."),
- 				.display_order(4)
+ 				.display_order(3)
- 				.help("📂 Root —")
+ 				.help("Current working directory.")
- 				.display_order(5)
+ 				.display_order(4)
- 				.help("🚫 Exclude —")
+ 				.help("Exclude pattern.")
- 				.display_order(6)
+ 				.display_order(5)
- 				.help("🔍 Pattern —")
+ 				.help("Search pattern.")
- 				.display_order(7)
+ 				.display_order(6)
- 				.help("🖥️ Command —"),
+ 				.help("Command to run."),
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index b771f11..bf4ce76 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
- /// This Rust function walks through a directory, filters out certain files based on exclusion criteria,
- /// and returns a collection of paths.
- ///
- /// Arguments:
- ///
- /// some file system operations based on the provided configuration. Here's a breakdown of the
- /// parameters:
- ///
- /// Returns:
- ///
- /// a `Vec<String>` containing paths that meet the specified criteria after processing the entries from
- /// the directory specified by the `Root` parameter.
- 		.follow_links(false)
- 			// TODO: Separate this into Entry/Exclude.rs
- 					true => match std::fs::metadata(std::path::PathBuf::from(&Path)) {
- 						Ok(Metadata) => Metadata.is_dir() && Match,
- 						Err(_Error) => false,
- 					},
+ 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 5e71ca9..efd9808 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- /// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
- /// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
- /// as the current directory, and prints the output of each Command execution.
- ///
- /// Arguments:
- ///
- /// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
- /// fields.
- pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	let Queue: Vec<String> = futures::stream::iter(
+ pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	let mut Queue = Vec::new();
+ 
- 			.into_par_iter()
+ 		.into_iter()
- 			.collect::<Vec<String>>(),
- 	)
- 	.map(|Entry| {
- 		let Command = Command.clone();
+ 		.for_each(|Entry| {
+ 			let Output = if cfg!(target_os = "windows") {
+ 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
+ 			} else {
+ 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
+ 			};
+ 
+ 			Queue.push(async move {
+ 				println!(
+ 					"{}",
+ 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
+ 				);
+ 			});
+ 		});
- 		async move {
- 			String::from_utf8_lossy(
- 				&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
- 					.args(&Command[1..])
- 					.current_dir(Entry)
- 					.output()
- 					.await
- 					.expect("Cannot Output.")
- 					.stdout,
- 			)
- 			.to_string()
+ 	tokio::runtime::Builder::new_multi_thread()
+ 		.enable_all()
+ 		.build()
+ 		.expect("Cannot Runtime.")
+ 		.block_on(async {
+ 			for Queue in Queue {
+ 				Queue.await;
- 	.buffer_unordered(num_cpus::get())
- 	.collect()
- 	.await;
- 
- 	Queue.par_iter().for_each(|Output| println!("{}", Output));
- use futures::stream::StreamExt;
- use rayon::prelude::*;
+ use tokio::process::Command;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 2c80c56..8eef18b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
- /// Executes a command with arguments in a specific directory for each entry in the given list.
- ///
- /// # Arguments
- ///
- /// * `Option` - A struct containing `Command`, `Entry`, `Pattern`, `Separator`, and other optional fields.
- ///
- /// # Example
- ///
- /// ```
- /// use std::process::{Command, Stdio};
- ///
- /// let options = Option { Command: vec!["ls".to_string()], Entry: vec!["/path/to/dir".to_string()], Pattern: "pattern", Separator: '/'.to_string() };
- /// Fn(options);
- /// ```
- 			let mut Command = Command::new(Command.get(0).expect("Cannot Command."))
- 				.args(&Command[1..])
+ 			let mut Out = match cfg!(target_os = "windows") {
+ 				true => Command::new("cmd")
+ 					.args(["/C", &Command])
- 				.expect("Cannot spawn.")
+ 					.expect("Cannot spawn."),
+ 				false => Command::new("sh")
+ 					.arg("-c")
+ 					.current_dir(Entry)
+ 					.arg(Command.clone())
+ 					.stdout(Stdio::piped())
+ 					.spawn()
+ 					.expect("Cannot spawn."),
+ 			}
- 				let Byte = Command.read(&mut Buffer).expect("Cannot read.");
+ 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..982f1a3 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
- #[tokio::main]
- async fn main() {
- 	(Struct::Binary::Command::Struct::Fn().Fn)().await
+ fn main() {
+ 	(Struct::Binary::Command::Struct::Fn().Fn)()
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 2a79ad1..fdaa4e3 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
+ pub mod Entry;
+ pub mod Option;
+ 
+ use crate::Fn::Binary::Command::{Parallel, Sequential};
+ 
+ #[derive(Debug)]
- 	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
+ 	pub Fn: fn(),
- 			Fn: Box::new(|| {
- 				Box::pin(async move {
+ 			Fn: || {
- 							Parallel::Fn(Option).await;
+ 						Parallel::Fn(Option);
- 				})
- 			}),
+ 			},
- 
- pub mod Entry;
- pub mod Option;
- 
- use crate::Fn::Binary::Command::{Parallel, Sequential};
- 
- use futures::Future;
- use std::pin::Pin;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index e1294e3..4c946ac 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- pub type Command = Vec<String>;
- 
+ pub type Command = String;
- 	pub Command: Command,
+ 	pub Command: String,
- 	pub Parallel: Parallel,
+ 	pub Parallel: bool,
- 				.map(|Command| Command.to_string())
- 				.collect::<Vec<_>>(),
+ 				.map(|Command| Command.as_str())
+ 				.collect::<Vec<_>>()
+ 				.join(" "),

🗣️ Summary from first commit to vInn/v0.1.9 in .
diff --git a/.github/FUNDING.yml b/.github/FUNDING.yml
deleted file mode 100644
index 3ba6945..0000000
--- a/.github/FUNDING.yml
+++ /dev/null
- open_collective: playform-cloud-collective
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index 387fece..cfa5b96 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-             - uses: dependabot/fetch-metadata@v2.2.0
+             - uses: dependabot/fetch-metadata@v2.0.0
-             - uses: dependabot/fetch-metadata@v2.2.0
+             - uses: dependabot/fetch-metadata@v2.0.0
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index 7b1e399..0be30ba 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
-             TERRAFORM_TELEMETRY: 0
-             - uses: pozil/auto-assign-issue@v2.0.0
+             - uses: pozil/auto-assign-issue@v1.14.0
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 9edf181..cfdee9a 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-             TERRAFORM_TELEMETRY: 0
-             - uses: actions/checkout@v4.1.7
+             - uses: actions/checkout@v4.1.2
diff --git a/.gitignore b/.gitignore
index 370ce1d..cbb2f71 100644
--- a/.gitignore
+++ b/.gitignore
+ Cargo.lock
+ 
- !/target/release/PRun
- !/target/release/Run
+ !/target/release/Inn
+ !/target/release/Innkeeper
diff --git a/build.rs b/build.rs
index 73ccc94..550762c 100644
--- a/build.rs
+++ b/build.rs
- use serde::Deserialize;
- #[derive(Deserialize)]
- struct Toml {
- 	package: Package,
- }
- 
- #[derive(Deserialize)]
- struct Package {
- 	version: String,
- }
- 
- 
- 		(toml::from_str::<Toml>(&fs::read_to_string("Cargo.toml").expect("Cannot Cargo.toml."))
- 			.expect("Cannot toml."))
- 		.package
- 		.version
+ 		fs::read_to_string("Cargo.toml")
+ 			.expect("Cannot Cargo.toml.")
+ 			.lines()
+ 			.find(|Line| Line.starts_with("version"))
+ 			.expect("Cannot Version.")
+ 			.split('=')
+ 			.nth(1)
+ 			.expect("Cannot nth.")
+ 			.trim()
+ 			.trim_matches('"')
diff --git a/Cargo.toml b/Cargo.toml
index b5d4b17..b8e9a72 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- [[bin]]
- name = "PRun"
- path = "Source/Library.rs"
- 
- [[bin]]
- name = "Run"
- path = "Source/Library.rs"
- 
- clap = { features = ["derive"], version = "4.5.9" }
- tokio = { features = ["full"], version = "1.38.1" }
+ clap = { features = ["derive"], version = "4.5.4" }
+ tokio = { features = ["full"], version = "1.36.0" }
- [build-dependencies]
- serde = { version = "1.0.204", features = ["derive"] }
- toml = "0.8.15"
- 
- default-run = "Run"
- description = "🍺 Run"
+ default-run = "Inn"
+ description = "🍺 Run."
- name = "prun"
- repository = "https://github.com/PlayForm/Run.git"
- version = "0.0.5"
+ name = "innkeeper"
+ repository = "https://github.com/NikolaRHristov/Inn.git"
+ version = "0.1.9"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index d35c668..13ffed8 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- Community@PlayForm.Cloud. All complaints will be reviewed and investigated
+ community@playform.cloud. All complaints will be reviewed and investigated
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index c749fbe..4700843 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- [homepage]: HTTPS://www.contributor-covenant.org
- [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
- [FAQ]: HTTPS://www.contributor-covenant.org/faq
- [translations]: HTTPS://www.contributor-covenant.org/translations
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
diff --git a/LICENSE b/LICENSE
index f236d76..c47b9fa 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2023-2024 PlayForm
+ Copyright (c) 2023-2024 Nikola R. Hristov
diff --git a/README.md b/README.md
index 4732518..f9edcac 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Run] —
+ # 🍺 [Inn]
- `Run` is a command-line tool designed to execute a specified command in all
+ Inn is a command-line tool designed to execute a specified command in all
- [Run]: HTTPS://crates.io/crates/prun
+ [Inn]: https://crates.io/crates/innkeeper
- 			<pre>Run -P .git ls</pre>
+ 			<pre>Inn -P .git ls</pre>
- 			<pre>Run -P .git git status</pre>
+ 			<pre>Inn -P .git git status</pre>
- 			<pre>Run -P .git 'git add . && git ecommit && git sync'</pre>
+ 			<pre>Inn -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
- bash -c 'git add . && git ecommit && git sync' \;</pre>
+ bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
- cargo install prun
+ cargo install innkeeper
- Run .git git fetch upstream
+ Inn .git git fetch upstream
- If you want to limit execution to files matching a certain pattern only specify
- a `--File` argument or `-F`:
+ You can hide the command output by specifying an `-H` or `--Hide` parameter:
- Run -F astro.config.ts npx astro add @playform/compress
+ Inn -H -F package.json ncu -u
+ ```
+ 
+ To specify a `--File` argument or `-F`, if you would like to search for a file
+ instead of a directory, use:
+ 
+ ```sh
+ Inn -F astro.config.ts npx astro add @playform/compress
- Run -R D:\Developer .git git fetch upstream
+ Inn -R D:\Developer .git git fetch upstream
- Run -P -R D:\Developer .git git fetch upstream
+ Inn -P -R D:\Developer .git git fetch upstream
- [Run]: HTTPS://crates.io/crates/prun
- 
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 5515b71..f301f4f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 	Command::new("Run")
+ 	Command::new("Inn")
- 		.author("PlayForm <hello@playform.cloud>")
- 		.about("🍺 Run —")
+ 		.author("Nikola R. Hristov <nikola@playform.cloud>")
+ 		.about("Run a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("Hide")
+ 				.short('H')
+ 				.long("Hide")
+ 				.action(SetTrue)
+ 				.display_order(1)
+ 				.value_name("HIDE")
+ 				.required(false)
+ 				.help("Hide output."),
+ 		)
- 				.help("📝 File —"),
+ 				.help("Search file."),
- 				.help("⏩ Parallel —"),
+ 				.help("Execute code in parallel."),
- 				.help("📂 Root —")
+ 				.help("Current working directory.")
- 				.help("🚫 Exclude —")
+ 				.help("Exclude pattern.")
- 				.help("🔍 Pattern —")
+ 				.help("Search pattern.")
- 				.help("🖥️ Command —"),
+ 				.help("Command to run."),
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 7f8a2ca..847b4ea 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
- /// This Rust function walks through a directory, filters out certain files based on exclusion criteria,
- /// and returns a collection of paths.
- ///
- /// Arguments:
- ///
- /// * ``: It looks like you have a function that takes an `Option` struct as a parameter and performs
- /// some file system operations based on the provided configuration. Here's a breakdown of the
- /// parameters:
- ///
- /// Returns:
- ///
- /// a `Vec<String>` containing paths that meet the specified criteria after processing the entries from
- /// the directory specified by the `Root` parameter.
- 		.follow_links(false)
- 					true => match std::fs::metadata(std::path::PathBuf::from(&Path)) {
- 						Ok(Metadata) => Metadata.is_dir() && Match,
- 						Err(_Error) => false,
- 					},
+ 					true => {
+ 						std::fs::metadata(std::path::PathBuf::from(&Path))
+ 							.expect("Cannot Metadata.")
+ 							.is_dir() && Match
+ 					}
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index a0b67b8..11cc07e 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- /// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
- /// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
- /// as the current directory, and prints the output of each Command execution.
- ///
- /// Arguments:
- ///
- /// * ``: It looks like you have a Rust function named `Fn` that takes an `Option` enum as a parameter.
- /// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
- /// fields.
- 			Queue.push(async move { Output.await.expect("Cannot Output.").stdout });
+ 			Queue.push(async move { Output.await.expect("Cannot await.").stdout });
- 			println!("{}", String::from_utf8_lossy(&Queue.remove(0).await));
- 
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index 5b62589..c3e16f8 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
+ 	pub Hide: Hide,
+ 			Hide: Option.Hide.clone(),
- 	Command, Parallel, Pattern, Separator, Struct as Option,
+ 	Command, Hide, Parallel, Pattern, Separator, Struct as Option,
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index e1294e3..82d1dc4 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- 
+ pub type Hide = bool;
+ 	pub Hide: Hide,
+ 			Hide: Fn().get_flag("Hide"),

🗣️ Summary from v0.0.1 to v0.0.10 in .
diff --git a/.github/dependabot.yml b/.github/dependabot.yml
index 44227ad..2b59ab4 100644
--- a/.github/dependabot.yml
+++ b/.github/dependabot.yml
- enable-beta-ecosystems: true
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
deleted file mode 100644
index ffde8df..0000000
--- a/.github/workflows/GitHub.yml
+++ /dev/null
- name: GitHub
- 
- concurrency:
-     group: GitHub-${{ github.workflow }}-${{ github.ref }}
-     cancel-in-progress: true
- 
- permissions:
-     issues: write
-     pull-requests: write
- 
- on:
-     issues:
-         types: [opened]
-     pull_request:
-         types: [opened]
- 
- jobs:
-     Assign:
-         runs-on: ubuntu-latest
- 
-         env:
-             ADBLOCK: true
-             TELEMETRY_DISABLED: 1
-             ASTRO_TELEMETRY_DISABLED: 1
-             AUTOMATEDLAB_TELEMETRY_OPTOUT: 1
-             AZURE_CORE_COLLECT_TELEMETRY: 0
-             CHOOSENIM_NO_ANALYTICS: 1
-             DIEZ_DO_NOT_TRACK: 1
-             DO_NOT_TRACK: 1
-             DOTNET_CLI_TELEMETRY_OPTOUT: 1
-             DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT: 1
-             ET_NO_TELEMETRY: 1
-             GATSBY_TELEMETRY_DISABLED: 1
-             GATSBY_TELEMETRY_OPT_OUT: 1
-             GATSBY_TELEMETRY_OPTOUT: 1
-             HASURA_GRAPHQL_ENABLE_TELEMETRY: false
-             HINT_TELEMETRY: off
-             HOMEBREW_NO_ANALYTICS: 1
-             INFLUXD_REPORTING_DISABLED: true
-             ITERATIVE_DO_NOT_TRACK: 1
-             NEXT_TELEMETRY_DEBUG: 1
-             NEXT_TELEMETRY_DISABLED: 1
-             NG_CLI_ANALYTICS: false
-             NUXT_TELEMETRY_DISABLED: 1
-             PIN_DO_NOT_TRACK: 1
-             POWERSHELL_TELEMETRY_OPTOUT: 1
-             SAM_CLI_TELEMETRY: 0
-             STNOUPGRADE: 1
-             STRIPE_CLI_TELEMETRY_OPTOUT: 1
- 
-         steps:
-             - uses: pozil/auto-assign-issue@v1.13.0
-               with:
-                   repo-token: ${{ secrets.GITHUB_TOKEN }}
-                   assignees: NikolaRHristov
-                   numOfAssignee: 1
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 161d7eb..45c06fb 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-             - uses: actions/cache@v4.0.1
+             - uses: actions/cache@v3.3.2
diff --git a/.gitignore b/.gitignore
index cbb2f71..a60405a 100644
--- a/.gitignore
+++ b/.gitignore
- /target/*
- !/target/release
- /target/release/*
- !/target/release/*.deb
- !/target/release/*.exe
- !/target/release/Inn
- !/target/release/Innkeeper
+ /Target/*
+ !/Target/release
+ /Target/release/*
+ !/Target/release/*.deb
+ !/Target/release/*.exe
+ !/Target/release/inn
+ !/Target/release/innkeeper
diff --git a/build.rs b/build.rs
deleted file mode 100644
index 550762c..0000000
--- a/build.rs
+++ /dev/null
- #![allow(non_snake_case)]
- 
- use std::fs;
- 
- fn main() {
- 	println!("cargo:rerun-if-changed=Cargo.toml");
- 	println!(
- 		"cargo:rustc-env=CARGO_PKG_VERSION={}",
- 		fs::read_to_string("Cargo.toml")
- 			.expect("Cannot Cargo.toml.")
- 			.lines()
- 			.find(|Line| Line.starts_with("version"))
- 			.expect("Cannot Version.")
- 			.split('=')
- 			.nth(1)
- 			.expect("Cannot nth.")
- 			.trim()
- 			.trim_matches('"')
- 	);
- }
diff --git a/Cargo.toml b/Cargo.toml
index 37256c8..6956652 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- [[bin]]
- name = "INN2"
- path = "Source/Library.rs"
+ [package]
+ name = "innkeeper"
+ version = "0.0.10"
+ description = "🍺 Inn is a tiny Rust utility that lets execute commands in different directories."
+ license = "MIT"
+ default-run = "inn"
+ repository = "https://github.com/NikolaRHristov/Inn.git"
- clap = { features = ["derive"], version = "4.5.1" }
- tokio = { features = ["full"], version = "1.36.0" }
- walkdir = "2.5.0"
+ clap = { version = "4.4.11", features = ["derive"] }
+ walkdir = { version = "2.4.0" }
+ rayon = { version = "1.8.0" }
+ crossbeam = { version = "0.8.2" }
+ toml = "0.8.8"
- crate-type = ["staticlib", "cdylib", "rlib"]
- name = "Library"
- path = "Source/Library.rs"
+ name = "inn"
+ path = "Source/Library/main.rs"
- [package]
- autobenches = false
- autobins = false
- autoexamples = false
- autotests = false
- default-run = "INN2"
- description = "🍺 INN2 lets you execute parallel commands in multiple directories."
- license = "MIT"
- name = "inn2"
- repository = "https://github.com/NikolaRHristov/INN2.git"
- version = "0.0.1"
- edition = "2021"
+ [[bin]]
+ name = "inn"
+ path = "Source/Binary/inn.rs"
+ 
+ [[bin]]
+ name = "innkeeper"
+ path = "Source/Binary/innkeeper.rs"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
deleted file mode 100644
index b4f1f9b..0000000
--- a/CODE_OF_CONDUCT.md
+++ /dev/null
- # Code of Conduct
- 
- ## Our Pledge
- 
- Welcome to our community! We are committed to creating a welcoming and inclusive
- environment for all contributors. As members, contributors, and leaders, we
- pledge to make participation in our community a harassment-free experience for
- everyone, regardless of:
- 
- -   Age
- -   Body size
- -   Visible or invisible disability
- -   Ethnicity
- -   Sex characteristics
- -   Gender identity and expression
- -   Level of experience
- -   Education
- -   Socio-economic status
- -   Nationality
- -   Personal appearance
- -   Race
- -   Caste
- -   Color
- -   Religion
- -   Sexual identity and orientation
- 
- We promise to act and interact in ways that contribute to an open, welcoming,
- diverse, inclusive, and healthy community.
- 
- ## Our Standards
- 
- Examples of behavior that contributes to a positive environment for our
- community include:
- 
- -   Demonstrating empathy and kindness toward other people
- -   Being respectful of differing opinions, viewpoints, and experiences
- -   Giving and gracefully accepting constructive feedback
- -   Accepting responsibility and apologizing to those affected by our mistakes,
-     and learning from the experience
- -   Focusing on what is best not just for us as individuals but for the overall
-     community
- 
- Examples of unacceptable behavior include:
- 
- -   The use of sexualized language or imagery, and sexual attention or advances
-     of any kind
- -   Trolling, insulting, or derogatory comments, and personal or political
-     attacks
- -   Public or private harassment
- -   Publishing others' private information, such as a physical or email address,
-     without their explicit permission
- -   Other conduct which could reasonably be considered inappropriate in a
-     professional setting
- 
- ## Enforcement Responsibilities
- 
- Community leaders are responsible for clarifying and enforcing our standards of
- acceptable behavior. They will take appropriate and fair corrective action in
- response to any behavior they deem inappropriate, threatening, offensive, or
- harmful. This may include removing, editing, or rejecting comments, commits,
- code, wiki edits, issues, and other contributions that do not align with this
- Code of Conduct. Community leaders will communicate reasons for moderation
- decisions when appropriate.
- 
- ## Scope
- 
- This Code of Conduct applies within all community spaces, and also applies when
- an individual is officially representing the community in public spaces.
- Examples of representing our community include using an official e-mail address,
- posting via an official social media account, or acting as an appointed
- representative at an online or offline event.
- 
- ## Enforcement
- 
- Instances of abusive, harassing, or otherwise unacceptable behavior may be
- reported to the community leaders responsible for enforcement at
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- promptly and fairly. All community leaders are obligated to respect the privacy
- and security of the reporter of any incident.
- 
- ## Enforcement Guidelines
- 
- Community leaders will follow these Community Impact Guidelines in determining
- the consequences for any action they deem in violation of this Code of Conduct:
- 
- ### 1. Correction
- 
- **Community Impact**: Use of inappropriate language or other behavior deemed
- unprofessional or unwelcome in the community.
- 
- **Consequence**: A private, written warning from community leaders, providing
- clarity around the nature of the violation and an explanation of why the
- behavior was inappropriate. A public apology may be requested.
- 
- ### 2. Warning
- 
- **Community Impact**: A violation through a single incident or series of
- actions.
- 
- **Consequence**: A warning with consequences for continued behavior. No
- interaction with the people involved, including unsolicited interaction with
- those enforcing the Code of Conduct, for a specified period of time. This
- includes avoiding interactions in community spaces as well as external channels
- like social media. Violating these terms may lead to a temporary or permanent
- ban.
- 
- ### 3. Temporary Ban
- 
- **Community Impact**: A serious violation of community standards, including
- sustained inappropriate behavior.
- 
- **Consequence**: A temporary ban from any sort of interaction or public
- communication with the community for a specified period of time. No public or
- private interaction with the people involved, including unsolicited interaction
- with those enforcing the Code of Conduct, is allowed during this period.
- Violating these terms may lead to a permanent ban.
- 
- ### 4. Permanent Ban
- 
- **Community Impact**: Demonstrating a pattern of violation of community
- standards, including sustained inappropriate behavior, harassment of an
- individual, or aggression toward or disparagement of classes of individuals.
- 
- **Consequence**: A permanent ban from any sort of public interaction within the
- community.
- 
- ## Attribution
- 
- This Code of Conduct is adapted from the [Contributor Covenant][homepage],
- version 2.1, available at
- [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
- Community Impact Guidelines were inspired by [Mozilla's code of conduct
- enforcement ladder][Mozilla CoC].
- 
- For answers to common questions about this code of conduct, see the FAQ at
- [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
- [https://www.contributor-covenant.org/translations][translations].
- 
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
- 
- Thank you for being part of our community and helping us create a safe and
- respectful environment for everyone!
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
deleted file mode 100644
index c740185..0000000
--- a/CONTRIBUTING.md
+++ /dev/null
- # Contributing Guidelines
- 
- Welcome to our community! We are committed to creating a welcoming and inclusive
- environment for all contributors. Before you get started, please read and adhere
- to the following code of conduct. By participating in our community, you agree
- to abide by these guidelines.
- 
- ## Our Pledge
- 
- We, as members, contributors, and leaders, pledge to make participation in our
- community a harassment-free experience for everyone, regardless of age, body
- size, visible or invisible disability, ethnicity, sex characteristics, gender
- identity and expression, level of experience, education, socio-economic status,
- nationality, personal appearance, race, caste, color, religion, or sexual
- identity and orientation. We pledge to act and interact in ways that contribute
- to an open, welcoming, diverse, inclusive, and healthy community.
- 
- ## Our Standards
- 
- Examples of behavior that contributes to a positive environment for our
- community include:
- 
- -   Demonstrating empathy and kindness toward other people
- -   Being respectful of differing opinions, viewpoints, and experiences
- -   Giving and gracefully accepting constructive feedback
- -   Accepting responsibility and apologizing to those affected by our mistakes,
-     and learning from the experience
- -   Focusing on what is best not just for us as individuals, but for the overall
-     community
- 
- Examples of unacceptable behavior include:
- 
- -   The use of sexualized language or imagery, and sexual attention or advances
-     of any kind
- -   Trolling, insulting, or derogatory comments, and personal or political
-     attacks
- -   Public or private harassment
- -   Publishing others' private information, such as a physical or email address,
-     without their explicit permission
- -   Other conduct which could reasonably be considered inappropriate in a
-     professional setting
- 
- ## Enforcement Responsibilities
- 
- Community leaders are responsible for clarifying and enforcing our standards of
- acceptable behavior and will take appropriate and fair corrective action in
- response to any behavior that they deem inappropriate, threatening, offensive,
- or harmful. Community leaders have the right and responsibility to remove, edit,
- or reject comments, commits, code, wiki edits, issues, and other contributions
- that are not aligned with this Code of Conduct, and will communicate reasons for
- moderation decisions when appropriate.
- 
- ## Scope
- 
- This Code of Conduct applies within all community spaces, and also applies when
- an individual is officially representing the community in public spaces.
- Examples of representing our community include using an official e-mail address,
- posting via an official social media account, or acting as an appointed
- representative at an online or offline event.
- 
- ## Enforcement
- 
- Instances of abusive, harassing, or otherwise unacceptable behavior may be
- reported to the community leaders responsible for enforcement at
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- promptly and fairly. All community leaders are obligated to respect the privacy
- and security of the reporter of any incident.
- 
- ## Enforcement Guidelines
- 
- Community leaders will follow these Community Impact Guidelines in determining
- the consequences for any action they deem in violation of this Code of Conduct:
- 
- ### 1. Correction
- 
- **Community Impact**: Use of inappropriate language or other behavior deemed
- unprofessional or unwelcome in the community.
- 
- **Consequence**: A private, written warning from community leaders, providing
- clarity around the nature of the violation and an explanation of why the
- behavior was inappropriate. A public apology may be requested.
- 
- ### 2. Warning
- 
- **Community Impact**: A violation through a single incident or series of
- actions.
- 
- **Consequence**: A warning with consequences for continued behavior. No
- interaction with the people involved, including unsolicited interaction with
- those enforcing the Code of Conduct, for a specified period of time. This
- includes avoiding interactions in community spaces as well as external channels
- like social media. Violating these terms may lead to a temporary or permanent
- ban.
- 
- ### 3. Temporary Ban
- 
- **Community Impact**: A serious violation of community standards, including
- sustained inappropriate behavior.
- 
- **Consequence**: A temporary ban from any sort of interaction or public
- communication with the community for a specified period of time. No public or
- private interaction with the people involved, including unsolicited interaction
- with those enforcing the Code of Conduct, is allowed during this period.
- Violating these terms may lead to a permanent ban.
- 
- ### 4. Permanent Ban
- 
- **Community Impact**: Demonstrating a pattern of violation of community
- standards, including sustained inappropriate behavior, harassment of an
- individual, or aggression toward or disparagement of classes of individuals.
- 
- **Consequence**: A permanent ban from any sort of public interaction within the
- community.
- 
- ## Attribution
- 
- This Code of Conduct is adapted from the [Contributor Covenant][homepage],
- version 2.1, available at
- [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
- Community Impact Guidelines were inspired by [Mozilla's code of conduct
- enforcement ladder][Mozilla CoC].
- 
- For answers to common questions about this code of conduct, see the FAQ at
- [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
- [https://www.contributor-covenant.org/translations][translations].
- 
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
- 
- Thank you for being part of our community and helping us create a safe and
- respectful environment for everyone!
diff --git a/LICENSE b/LICENSE
deleted file mode 100644
index c47b9fa..0000000
--- a/LICENSE
+++ /dev/null
- MIT License
- 
- Copyright (c) 2023-2024 Nikola R. Hristov
- 
- Permission is hereby granted, free of charge, to any person obtaining a copy of
- this software and associated documentation files (the "Software"), to deal in
- the Software without restriction, including without limitation the rights to
- use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
- the Software, and to permit persons to whom the Software is furnished to do so,
- subject to the following conditions:
- 
- The above copyright notice and this permission notice shall be included in all
- copies or substantial portions of the Software.
- 
- THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
- IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
- FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
- COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
- IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
- CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
diff --git a/README.md b/README.md
index 26d3127..8cb0322 100644
--- a/README.md
+++ b/README.md
- # 🍺 [INN2]
- 
- INN2 is a command-line tool designed to execute a specified command in all
- directories that match a certain pattern within a given root directory. It
- provides flexibility and efficiency in running commands across multiple
- directories with customizable patterns.
- 
- [INN2]: https://crates.io/crates/inn2
- 
- ## Benchmark
- 
- <table>
- 	<tr>
- 		<th>Command:</th>
- 		<th>Time:</th>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git ls</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m9.441s
- user    0m0.030s
- sys     0m0.046s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m14.293s +5s
- user    0m4.645s +4s
- sys     0m8.937s +8s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git git status</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m24.146s
- user    0m0.030s
- sys     0m0.062s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m28.584s +4s
- user    0m4.695s +4s
- sys     0m8.354s +8s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m33.813s
- user    0m0.015s
- sys     0m0.060s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir \
- bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m53.122s +20s
- user    0m9.449s +9s
- sys     0m14.442s +14s</pre>
- 		</td>
- 	</tr>
- </table>
+ # [Inn] 🍺
+ 
+ Inn is a tiny Rust utility that lets execute commmands in different directories
+ concurrently.
+ 
+ [Inn]: https://crates.io/crates/innkeeper
- cargo install inn2
+ cargo install innkeeper
- INN2 .git git fetch upstream
+ inn .git git fetch upstream
- This command will fetch from upstream for all the .git repositories inside the
- current directory. Essentially, it replaces the following shell command:
+ This will fetch from upstream for all the `.git` repositories inside the current
+ directory. Basically it replaces:
- To specify a `--File` argument or `-F`, if you would like to search for a file
- instead of a directory, use:
+ Specify a `--file` argument or `-f` if you would like to search for file instead
+ of a directory. Default is `false` or no flag at all.
- INN2 -F astro.config.ts npx astro add astro-compress
+ inn -f astro.config.ts npx astro add astro-compress
- Additionally, you can provide a `--Root` argument or `-R` to set the current
- working directory to a different folder. The default is `.`.
+ You can also provide a `--root` argument or `-r` which sets the current working
+ directory to a different folder. Default is `.`.
- INN2 -R D:\Developer .git git fetch upstream
+ inn -r D:\Developer .git git fetch upstream
- Specify a `--Parallel` argument or `-P` if you would like to run commands in
- parallel. The default is sequential.
+ Specify a `--parallel` argument or `-p` if you would like to run commands in
+ parallel. Default is sequential.
- INN2 -P -R D:\Developer .git git fetch upstream
+ inn -p -r D:\Developer .git git fetch upstream
- ## Dependencies
- 
- The code imports several crates:
- 
- -   `clap` - For parsing command-line arguments.
- -   `tokio` - Enables parallel execution of tasks.
- -   `walkdir` - Facilitates filesystem traversal.
- 
diff --git a/Source/Binary/inn.rs b/Source/Binary/inn.rs
new file mode 100644
index 0000000..e2fe709
--- /dev/null
+++ b/Source/Binary/inn.rs
+ fn main() {
+ 	inn::run()
+ }
diff --git a/Source/Binary/innkeeper.rs b/Source/Binary/innkeeper.rs
new file mode 100644
index 0000000..e2fe709
--- /dev/null
+++ b/Source/Binary/innkeeper.rs
+ fn main() {
+ 	inn::run()
+ }
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
deleted file mode 100644
index d9d47b8..0000000
--- a/Source/Fn/Binary/Command.rs
+++ /dev/null
- pub mod Entry;
- pub mod Parallel;
- pub mod Sequential;
- 
- pub fn Fn() -> ArgMatches {
- 	Command::new("Innkeeper")
- 		.version(env!("CARGO_PKG_VERSION"))
- 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("File")
- 				.short('F')
- 				.long("File")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("Parallel")
- 				.short('P')
- 				.long("Parallel")
- 				.action(SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("Root")
- 				.short('R')
- 				.long("Root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Exclude")
- 				.short('E')
- 				.long("Exclude")
- 				.display_order(4)
- 				.value_name("EXCLUDE")
- 				.required(false)
- 				.help("Exclude pattern.")
- 				.default_value("node_modules .git target dist vendor"),
- 		)
- 		.arg(
- 			Arg::new("Pattern")
- 				.display_order(5)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Command")
- 				.num_args(0..=10)
- 				.display_order(6)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches()
- }
- 
- use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
deleted file mode 100644
index bf4ce76..0000000
--- a/Source/Fn/Binary/Command/Entry.rs
+++ /dev/null
- pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
- 	WalkDir::new(Root)
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			let Path = Entry.expect("Cannot Entry.").path().display().to_string();
- 
- 			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
- 				let Match = Path.contains(&Exclude);
- 
- 				match File {
- 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
- 					false => Match,
- 				}
- 			}) {
- 				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
- 			} else {
- 				None
- 			}
- 		})
- 		.collect::<Vec<_>>()
- }
- 
- use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};
- 
- use walkdir::WalkDir;
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
deleted file mode 100644
index efd9808..0000000
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ /dev/null
- pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	let mut Queue = Vec::new();
- 
- 	Entry
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			Entry
- 				.last()
- 				.filter(|Last| *Last == &Pattern)
- 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
- 		})
- 		.for_each(|Entry| {
- 			let Output = if cfg!(target_os = "windows") {
- 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
- 			} else {
- 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
- 			};
- 
- 			Queue.push(async move {
- 				println!(
- 					"{}",
- 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
- 				);
- 			});
- 		});
- 
- 	tokio::runtime::Builder::new_multi_thread()
- 		.enable_all()
- 		.build()
- 		.expect("Cannot Runtime.")
- 		.block_on(async {
- 			for Queue in Queue {
- 				Queue.await;
- 			}
- 		})
- }
- 
- use crate::Struct::Binary::Command::Entry::Struct as Option;
- use tokio::process::Command;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
deleted file mode 100644
index 8eef18b..0000000
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ /dev/null
- pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
- 	Entry
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			Entry
- 				.last()
- 				.filter(|Last| *Last == &Pattern)
- 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
- 		})
- 		.for_each(|Entry| {
- 			let mut Out = match cfg!(target_os = "windows") {
- 				true => Command::new("cmd")
- 					.args(["/C", &Command])
- 					.current_dir(Entry)
- 					.stdout(Stdio::piped())
- 					.spawn()
- 					.expect("Cannot spawn."),
- 				false => Command::new("sh")
- 					.arg("-c")
- 					.current_dir(Entry)
- 					.arg(Command.clone())
- 					.stdout(Stdio::piped())
- 					.spawn()
- 					.expect("Cannot spawn."),
- 			}
- 			.stdout
- 			.expect("Cannot stdout.");
- 
- 			let mut Output = String::new();
- 
- 			loop {
- 				let mut Buffer = [0; 512];
- 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");
- 
- 				if Byte == 0 {
- 					break;
- 				}
- 
- 				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
- 			}
- 
- 			println!("{}", Output);
- 		})
- }
- 
- use crate::Struct::Binary::Command::Entry::Struct as Option;
- 
- use std::{
- 	io::Read,
- 	process::{Command, Stdio},
- };
diff --git a/Source/Fn/Binary/mod.rs b/Source/Fn/Binary/mod.rs
deleted file mode 100644
index 9da7843..0000000
--- a/Source/Fn/Binary/mod.rs
+++ /dev/null
- pub mod Command;
diff --git a/Source/Fn/mod.rs b/Source/Fn/mod.rs
deleted file mode 100644
index a56e8ce..0000000
--- a/Source/Fn/mod.rs
+++ /dev/null
- pub mod Binary;
diff --git a/Source/Library.rs b/Source/Library.rs
deleted file mode 100644
index 982f1a3..0000000
--- a/Source/Library.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Fn;
- mod Struct;
- 
- #[allow(dead_code)]
- fn main() {
- 	(Struct::Binary::Command::Struct::Fn().Fn)()
- }
diff --git a/Source/Library/main.rs b/Source/Library/main.rs
new file mode 100644
index 0000000..a179931
--- /dev/null
+++ b/Source/Library/main.rs
+ extern crate clap;
+ extern crate crossbeam;
+ extern crate rayon;
+ extern crate walkdir;
+ 
+ use clap::{Arg, ArgAction, Command as ClapCommand};
+ use crossbeam::scope;
+ use rayon::prelude::*;
+ use std::{
+ 	fs,
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
+ use walkdir::WalkDir;
+ 
+ pub fn run() {
+ 	let matches = ClapCommand::new("Innkeeper")
+ 		.version("0.0.8")
+ 		.about("Runs a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("file")
+ 				.short('f')
+ 				.long("file")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("parallel")
+ 				.short('p')
+ 				.long("parallel")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("root")
+ 				.short('r')
+ 				.long("root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("pattern")
+ 				.display_order(4)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("command")
+ 				.num_args(0..=10)
+ 				.display_order(5)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches();
+ 
+ 	let file = matches.get_flag("file");
+ 	let parallel = matches.get_flag("parallel");
+ 	let root = matches.get_one::<String>("root").unwrap();
+ 	let pattern = matches.get_one::<String>("pattern").unwrap();
+ 	let command = &matches
+ 		.get_many::<String>("command")
+ 		.unwrap_or_default()
+ 		.map(|v| v.as_str())
+ 		.collect::<Vec<_>>()
+ 		.join(" ");
+ 
+ 	let ds = std::path::MAIN_SEPARATOR;
+ 
+ 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
+ 		if !pattern.contains("node_modules") {
+ 			return e.path().display().to_string().contains("node_modules");
+ 		}
+ 
+ 		if !file {
+ 			println!("{:?}", e.path().display().to_string().contains("node_modules"));
+ 			return fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir();
+ 		} else {
+ 			return true;
+ 		}
+ 	});
+ 
+ 	if parallel {
+ 		println!("Executing code in parallel.");
+ 
+ 		// Parallel
+ 		let dirs = entries
+ 			.map(|entry| {
+ 				let entry_dir = entry.unwrap().path().display().to_string();
+ 				let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 				match paths.last() {
+ 					Some(last) => {
+ 						if last == pattern {
+ 							let working_directory =
+ 								&paths[0..paths.len() - 1].join(&ds.to_string());
+ 							Some(working_directory.to_owned())
+ 						} else {
+ 							None
+ 						}
+ 					}
+ 					None => None,
+ 				}
+ 			})
+ 			.filter_map(|x| x)
+ 			.collect::<Vec<String>>();
+ 
+ 		scope(|s| {
+ 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
+ 				scope.spawn(move |_| {
+ 					println!("Executing {} for every {} in {}", command, dir, root);
+ 
+ 					let output = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", command.as_str()])
+ 							.current_dir(dir)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(dir)
+ 							.arg(command)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					println!("{}", String::from_utf8_lossy(&output.stdout));
+ 				});
+ 			});
+ 		})
+ 		.unwrap();
+ 	} else {
+ 		println!("Executing code in sequential.");
+ 
+ 		// Sequential
+ 		for entry in entries {
+ 			let entry_dir = entry.unwrap().path().display().to_string();
+ 			let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 			if let Some(last) = paths.last() {
+ 				if last == pattern {
+ 					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
+ 
+ 					println!("Executing {} for every {} in {}", command, last, root);
+ 
+ 					let child = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", command])
+ 							.current_dir(working_directory)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(working_directory)
+ 							.arg(command)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
+ 
+ 					let mut output = String::new();
+ 
+ 					loop {
+ 						let mut buffer = [0; 512];
+ 						let bytes_read =
+ 							stdout.read(&mut buffer).expect("Failed to read from pipe");
+ 
+ 						if bytes_read == 0 {
+ 							break;
+ 						}
+ 
+ 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
+ 					}
+ 
+ 					println!("{}", output);
+ 				}
+ 			}
+ 		}
+ 	}
+ }
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
deleted file mode 100644
index fdaa4e3..0000000
--- a/Source/Struct/Binary/Command.rs
+++ /dev/null
- pub mod Entry;
- pub mod Option;
- 
- use crate::Fn::Binary::Command::{Parallel, Sequential};
- 
- #[derive(Debug)]
- pub struct Struct {
- 	pub Separator: Option::Separator,
- 	pub Fn: fn(),
- }
- 
- impl Struct {
- 	pub fn Fn() -> Self {
- 		Self {
- 			Separator: std::path::MAIN_SEPARATOR,
- 			Fn: || {
- 				let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));
- 				
- 				match Option.Parallel {
- 					true => {
- 						Parallel::Fn(Option);
- 					}
- 					false => {
- 						Sequential::Fn(Option);
- 					}
- 				};
- 			},
- 		}
- 	}
- }
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
deleted file mode 100644
index 5b62589..0000000
--- a/Source/Struct/Binary/Command/Entry.rs
+++ /dev/null
- pub type Type = Vec<Vec<String>>;
- 
- pub struct Struct {
- 	pub Command: Command,
- 	pub Entry: Type,
- 	pub Parallel: Parallel,
- 	pub Pattern: Pattern,
- 	pub Separator: Separator,
- }
- 
- impl Struct {
- 	pub fn Fn(Option: &Option) -> Self {
- 		Self {
- 			Command: Option.Command.clone(),
- 			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
- 			Parallel: Option.Parallel,
- 			Pattern: Option.Pattern.clone(),
- 			Separator: Option.Separator,
- 		}
- 	}
- }
- 
- use crate::Struct::Binary::Command::Option::{
- 	Command, Parallel, Pattern, Separator, Struct as Option,
- };
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
deleted file mode 100644
index 4c946ac..0000000
--- a/Source/Struct/Binary/Command/Option.rs
+++ /dev/null
- pub type Command = String;
- pub type Parallel = bool;
- pub type Pattern = String;
- pub type Separator = char;
- 
- pub struct Struct {
- 	pub Command: String,
- 	pub Exclude: Vec<String>,
- 	pub File: bool,
- 	pub Parallel: bool,
- 	pub Pattern: Pattern,
- 	pub Root: String,
- 	pub Separator: Separator,
- }
- 
- impl Struct {
- 	pub fn Fn(Option { Separator, .. }: Option) -> Self {
- 		Self {
- 			File: Fn().get_flag("File"),
- 			Parallel: Fn().get_flag("Parallel"),
- 			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
- 			Exclude: Fn()
- 				.get_one::<String>("Exclude")
- 				.expect("Cannot Exclude.")
- 				.split(" ")
- 				.map(|Command| Command.to_string())
- 				.collect::<Vec<_>>(),
- 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
- 			Command: Fn()
- 				.get_many::<String>("Command")
- 				.expect("Cannot Command.")
- 				.map(|Command| Command.as_str())
- 				.collect::<Vec<_>>()
- 				.join(" "),
- 			Separator,
- 		}
- 	}
- }
- 
- use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
diff --git a/Source/Struct/Binary/mod.rs b/Source/Struct/Binary/mod.rs
deleted file mode 100644
index 9da7843..0000000
--- a/Source/Struct/Binary/mod.rs
+++ /dev/null
- pub mod Command;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
deleted file mode 100644
index a56e8ce..0000000
--- a/Source/Struct/mod.rs
+++ /dev/null
- pub mod Binary;

🗣️ Summary from v0.0.10 to v0.0.11 in .
diff --git a/.github/dependabot.yml b/.github/dependabot.yml
index 2b59ab4..44227ad 100644
--- a/.github/dependabot.yml
+++ b/.github/dependabot.yml
+ enable-beta-ecosystems: true
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
new file mode 100644
index 0000000..ffde8df
--- /dev/null
+++ b/.github/workflows/GitHub.yml
+ name: GitHub
+ 
+ concurrency:
+     group: GitHub-${{ github.workflow }}-${{ github.ref }}
+     cancel-in-progress: true
+ 
+ permissions:
+     issues: write
+     pull-requests: write
+ 
+ on:
+     issues:
+         types: [opened]
+     pull_request:
+         types: [opened]
+ 
+ jobs:
+     Assign:
+         runs-on: ubuntu-latest
+ 
+         env:
+             ADBLOCK: true
+             TELEMETRY_DISABLED: 1
+             ASTRO_TELEMETRY_DISABLED: 1
+             AUTOMATEDLAB_TELEMETRY_OPTOUT: 1
+             AZURE_CORE_COLLECT_TELEMETRY: 0
+             CHOOSENIM_NO_ANALYTICS: 1
+             DIEZ_DO_NOT_TRACK: 1
+             DO_NOT_TRACK: 1
+             DOTNET_CLI_TELEMETRY_OPTOUT: 1
+             DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT: 1
+             ET_NO_TELEMETRY: 1
+             GATSBY_TELEMETRY_DISABLED: 1
+             GATSBY_TELEMETRY_OPT_OUT: 1
+             GATSBY_TELEMETRY_OPTOUT: 1
+             HASURA_GRAPHQL_ENABLE_TELEMETRY: false
+             HINT_TELEMETRY: off
+             HOMEBREW_NO_ANALYTICS: 1
+             INFLUXD_REPORTING_DISABLED: true
+             ITERATIVE_DO_NOT_TRACK: 1
+             NEXT_TELEMETRY_DEBUG: 1
+             NEXT_TELEMETRY_DISABLED: 1
+             NG_CLI_ANALYTICS: false
+             NUXT_TELEMETRY_DISABLED: 1
+             PIN_DO_NOT_TRACK: 1
+             POWERSHELL_TELEMETRY_OPTOUT: 1
+             SAM_CLI_TELEMETRY: 0
+             STNOUPGRADE: 1
+             STRIPE_CLI_TELEMETRY_OPTOUT: 1
+ 
+         steps:
+             - uses: pozil/auto-assign-issue@v1.13.0
+               with:
+                   repo-token: ${{ secrets.GITHUB_TOKEN }}
+                   assignees: NikolaRHristov
+                   numOfAssignee: 1
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 45c06fb..bca13f1 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-             - uses: actions/cache@v3.3.2
+             - uses: actions/cache@v4.0.0
diff --git a/build.rs b/build.rs
new file mode 100644
index 0000000..b533aa3
--- /dev/null
+++ b/build.rs
+ use std::fs;
+ 
+ fn main() {
+ 	println!("cargo:rerun-if-changed=Cargo.toml");
+ 	println!(
+ 		"cargo:rustc-env=CARGO_PKG_VERSION={}",
+ 		fs::read_to_string("Cargo.toml")
+ 			.expect("Failed to read Cargo.toml.")
+ 			.lines()
+ 			.find(|line| line.starts_with("version"))
+ 			.unwrap()
+ 			.split('=')
+ 			.nth(1)
+ 			.expect("Invalid version line format.")
+ 			.trim()
+ 			.trim_matches('"')
+ 	);
+ }
diff --git a/Cargo.toml b/Cargo.toml
index 6956652..9f6e224 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.10"
+ version = "0.0.11"
- default-run = "inn"
+ default-run = "Inn"
+ autobins = false
+ autobenches = false
+ autotests = false
+ autoexamples = false
- clap = { version = "4.4.11", features = ["derive"] }
+ clap = { version = "4.5.1", features = ["derive"] }
- rayon = { version = "1.8.0" }
- crossbeam = { version = "0.8.2" }
- toml = "0.8.8"
- 
- [lib]
- name = "inn"
- path = "Source/Library/main.rs"
+ rayon = { version = "1.8.1" }
+ crossbeam = { version = "0.8.4" }
+ toml = "0.8.10"
- name = "inn"
- path = "Source/Binary/inn.rs"
+ name = "Inn"
+ path = "Source/Function/Binary/Inn.rs"
- name = "innkeeper"
- path = "Source/Binary/innkeeper.rs"
+ name = "Innkeeper"
+ path = "Source/Function/Binary/Innkeeper.rs"
diff --git a/Source/Binary/inn.rs b/Source/Binary/inn.rs
deleted file mode 100644
index e2fe709..0000000
--- a/Source/Binary/inn.rs
+++ /dev/null
- fn main() {
- 	inn::run()
- }
diff --git a/Source/Binary/innkeeper.rs b/Source/Binary/innkeeper.rs
deleted file mode 100644
index e2fe709..0000000
--- a/Source/Binary/innkeeper.rs
+++ /dev/null
- fn main() {
- 	inn::run()
- }
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
new file mode 100644
index 0000000..07bb03b
--- /dev/null
+++ b/Source/Function/Binary/Command.rs
+ extern crate clap;
+ extern crate crossbeam;
+ extern crate rayon;
+ extern crate walkdir;
+ 
+ use self::{
+ 	clap::{Arg, ArgAction, Command as ClapCommand},
+ 	crossbeam::scope,
+ 	rayon::prelude::*,
+ 	walkdir::WalkDir,
+ };
+ 
+ use std::{
+ 	fs,
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
+ 
+ #[allow(dead_code)]
+ pub fn run() {
+ 	let Match = ClapCommand::new("Innkeeper")
+ 		.version(env!("CARGO_PKG_VERSION"))
+ 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.about("Run a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("file")
+ 				.short('f')
+ 				.long("file")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("parallel")
+ 				.short('p')
+ 				.long("parallel")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("root")
+ 				.short('r')
+ 				.long("root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("pattern")
+ 				.display_order(4)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("command")
+ 				.num_args(0..=10)
+ 				.display_order(5)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches();
+ 
+ 	let File = Match.get_flag("file");
+ 	let Parallel = Match.get_flag("parallel");
+ 	let Root = Match.get_one::<String>("root").unwrap();
+ 	let Pattern = Match.get_one::<String>("pattern").unwrap();
+ 	let Command = &Match
+ 		.get_many::<String>("command")
+ 		.unwrap_or_default()
+ 		.map(|v| v.as_str())
+ 		.collect::<Vec<_>>()
+ 		.join(" ");
+ 
+ 	let Separator = std::path::MAIN_SEPARATOR;
+ 
+ 	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
+ 		if !Pattern.contains("node_modules") {
+ 			return !Entry.path().display().to_string().contains("node_modules");
+ 		}
+ 
+ 		if !File {
+ 			println!("{:?}", Entry.path().display().to_string().contains("node_modules"));
+ 
+ 			fs::metadata(Entry.path().display().to_string().clone()).unwrap().is_dir()
+ 		} else {
+ 			true
+ 		}
+ 	});
+ 
+ 	if Parallel {
+ 		println!("Executing code in parallel.");
+ 
+ 		// Execution: Parallel
+ 		scope(|s| {
+ 			Entry
+ 				.map(|Entry| {
+ 					let entry_dir = Entry.unwrap().path().display().to_string();
+ 					let paths: Vec<&str> = entry_dir.split(Separator).collect();
+ 
+ 					match paths.last() {
+ 						Some(last) => {
+ 							if last == Pattern {
+ 								let working_directory =
+ 									&paths[0..paths.len() - 1].join(&Separator.to_string());
+ 								Some(working_directory.to_owned())
+ 							} else {
+ 								None
+ 							}
+ 						}
+ 						None => None,
+ 					}
+ 				})
+ 				.filter_map(|x| x)
+ 				.collect::<Vec<String>>()
+ 				.into_par_iter()
+ 				.for_each_with(s, |scope, dir| {
+ 					scope.spawn(move |_| {
+ 						println!("Executing {} for every {} in {}", Command, dir, Root);
+ 
+ 						println!(
+ 							"{}",
+ 							String::from_utf8_lossy(
+ 								&match cfg!(target_os = "windows") {
+ 									true => Command::new("cmd")
+ 										.args(["/C", Command.as_str()])
+ 										.current_dir(dir)
+ 										.output()
+ 										.expect("Failed to execute process."),
+ 									false => Command::new("sh")
+ 										.arg("-c")
+ 										.current_dir(dir)
+ 										.arg(Command)
+ 										.output()
+ 										.expect("Failed to execute process."),
+ 								}
+ 								.stdout
+ 							)
+ 						);
+ 					});
+ 				});
+ 		})
+ 		.unwrap();
+ 	} else {
+ 		println!("Executing code in sequential.");
+ 
+ 		// Execution: Sequential
+ 		for Entry in Entry {
+ 			let entry_dir = Entry.unwrap().path().display().to_string();
+ 			let paths: Vec<&str> = entry_dir.split(Separator).collect();
+ 
+ 			if let Some(last) = paths.last() {
+ 				if last == Pattern {
+ 					let working_directory = &paths[0..paths.len() - 1].join(&Separator.to_string());
+ 
+ 					println!("Executing {} for every {} in {}", Command, last, Root);
+ 
+ 					let child = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", Command])
+ 							.current_dir(working_directory)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(working_directory)
+ 							.arg(Command)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
+ 
+ 					let mut output = String::new();
+ 
+ 					loop {
+ 						let mut buffer = [0; 512];
+ 						let bytes_read =
+ 							stdout.read(&mut buffer).expect("Failed to read from pipe");
+ 
+ 						if bytes_read == 0 {
+ 							break;
+ 						}
+ 
+ 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
+ 					}
+ 
+ 					println!("{}", output);
+ 				}
+ 			}
+ 		}
+ 	}
+ }
diff --git a/Source/Function/Binary/Inn.rs b/Source/Function/Binary/Inn.rs
new file mode 100644
index 0000000..38e3fe5
--- /dev/null
+++ b/Source/Function/Binary/Inn.rs
+ #![allow(non_snake_case)]
+ mod Command;
+ 
+ fn main() {
+ 	Command::run()
+ }
diff --git a/Source/Function/Binary/Innkeeper.rs b/Source/Function/Binary/Innkeeper.rs
new file mode 100644
index 0000000..38e3fe5
--- /dev/null
+++ b/Source/Function/Binary/Innkeeper.rs
+ #![allow(non_snake_case)]
+ mod Command;
+ 
+ fn main() {
+ 	Command::run()
+ }
diff --git a/Source/Library/main.rs b/Source/Library/main.rs
deleted file mode 100644
index a179931..0000000
--- a/Source/Library/main.rs
+++ /dev/null
- extern crate clap;
- extern crate crossbeam;
- extern crate rayon;
- extern crate walkdir;
- 
- use clap::{Arg, ArgAction, Command as ClapCommand};
- use crossbeam::scope;
- use rayon::prelude::*;
- use std::{
- 	fs,
- 	io::Read,
- 	process::{Command, Stdio},
- };
- use walkdir::WalkDir;
- 
- pub fn run() {
- 	let matches = ClapCommand::new("Innkeeper")
- 		.version("0.0.8")
- 		.about("Runs a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("file")
- 				.short('f')
- 				.long("file")
- 				.action(ArgAction::SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("parallel")
- 				.short('p')
- 				.long("parallel")
- 				.action(ArgAction::SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("root")
- 				.short('r')
- 				.long("root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("pattern")
- 				.display_order(4)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("command")
- 				.num_args(0..=10)
- 				.display_order(5)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches();
- 
- 	let file = matches.get_flag("file");
- 	let parallel = matches.get_flag("parallel");
- 	let root = matches.get_one::<String>("root").unwrap();
- 	let pattern = matches.get_one::<String>("pattern").unwrap();
- 	let command = &matches
- 		.get_many::<String>("command")
- 		.unwrap_or_default()
- 		.map(|v| v.as_str())
- 		.collect::<Vec<_>>()
- 		.join(" ");
- 
- 	let ds = std::path::MAIN_SEPARATOR;
- 
- 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
- 		if !pattern.contains("node_modules") {
- 			return e.path().display().to_string().contains("node_modules");
- 		}
- 
- 		if !file {
- 			println!("{:?}", e.path().display().to_string().contains("node_modules"));
- 			return fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir();
- 		} else {
- 			return true;
- 		}
- 	});
- 
- 	if parallel {
- 		println!("Executing code in parallel.");
- 
- 		// Parallel
- 		let dirs = entries
- 			.map(|entry| {
- 				let entry_dir = entry.unwrap().path().display().to_string();
- 				let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 				match paths.last() {
- 					Some(last) => {
- 						if last == pattern {
- 							let working_directory =
- 								&paths[0..paths.len() - 1].join(&ds.to_string());
- 							Some(working_directory.to_owned())
- 						} else {
- 							None
- 						}
- 					}
- 					None => None,
- 				}
- 			})
- 			.filter_map(|x| x)
- 			.collect::<Vec<String>>();
- 
- 		scope(|s| {
- 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
- 				scope.spawn(move |_| {
- 					println!("Executing {} for every {} in {}", command, dir, root);
- 
- 					let output = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command.as_str()])
- 							.current_dir(dir)
- 							.output()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(dir)
- 							.arg(command)
- 							.output()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					println!("{}", String::from_utf8_lossy(&output.stdout));
- 				});
- 			});
- 		})
- 		.unwrap();
- 	} else {
- 		println!("Executing code in sequential.");
- 
- 		// Sequential
- 		for entry in entries {
- 			let entry_dir = entry.unwrap().path().display().to_string();
- 			let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 			if let Some(last) = paths.last() {
- 				if last == pattern {
- 					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
- 
- 					println!("Executing {} for every {} in {}", command, last, root);
- 
- 					let child = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command])
- 							.current_dir(working_directory)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(working_directory)
- 							.arg(command)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
- 
- 					let mut output = String::new();
- 
- 					loop {
- 						let mut buffer = [0; 512];
- 						let bytes_read =
- 							stdout.read(&mut buffer).expect("Failed to read from pipe");
- 
- 						if bytes_read == 0 {
- 							break;
- 						}
- 
- 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
- 					}
- 
- 					println!("{}", output);
- 				}
- 			}
- 		}
- 	}
- }

🗣️ Summary from v0.0.11 to v0.0.12 in .
diff --git a/build.rs b/build.rs
index b533aa3..8f162d7 100644
--- a/build.rs
+++ b/build.rs
+ #![allow(non_snake_case)]
+ 
- 			.find(|line| line.starts_with("version"))
+ 			.find(|Line| Line.starts_with("version"))
diff --git a/Cargo.toml b/Cargo.toml
index 9f6e224..1fe4261 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.11"
+ version = "0.0.12"
- rayon = { version = "1.8.1" }
+ rayon = { version = "1.9.0" }
- toml = "0.8.10"
diff --git a/README.md b/README.md
index 8cb0322..16f2af5 100644
--- a/README.md
+++ b/README.md
- # [Inn] 🍺
+ # 🍺 [Inn]
- inn .git git fetch upstream
+ Inn .git git fetch upstream
- Specify a `--file` argument or `-f` if you would like to search for file instead
+ Specify a `--File` argument or `-F` if you would like to search for file instead
- inn -f astro.config.ts npx astro add astro-compress
+ Inn -F astro.config.ts npx astro add astro-compress
- You can also provide a `--root` argument or `-r` which sets the current working
+ You can also provide a `--Root` argument or `-R` which sets the current working
- inn -r D:\Developer .git git fetch upstream
+ Inn -R D:\Developer .git git fetch upstream
- Specify a `--parallel` argument or `-p` if you would like to run commands in
+ Specify a `--Parallel` argument or `-P` if you would like to run commands in
- inn -p -r D:\Developer .git git fetch upstream
+ Inn -P -R D:\Developer .git git fetch upstream
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
index 07bb03b..3df03b9 100644
--- a/Source/Function/Binary/Command.rs
+++ b/Source/Function/Binary/Command.rs
- 			Arg::new("file")
- 				.short('f')
- 				.long("file")
+ 			Arg::new("File")
+ 				.short('F')
+ 				.long("File")
- 			Arg::new("parallel")
- 				.short('p')
- 				.long("parallel")
+ 			Arg::new("Parallel")
+ 				.short('P')
+ 				.long("Parallel")
- 			Arg::new("root")
- 				.short('r')
- 				.long("root")
+ 			Arg::new("Root")
+ 				.short('R')
+ 				.long("Root")
- 			Arg::new("pattern")
+ 			Arg::new("Pattern")
- 			Arg::new("command")
+ 			Arg::new("Command")
- 	let File = Match.get_flag("file");
- 	let Parallel = Match.get_flag("parallel");
- 	let Root = Match.get_one::<String>("root").unwrap();
- 	let Pattern = Match.get_one::<String>("pattern").unwrap();
+ 	let File = Match.get_flag("File");
+ 	let Parallel = Match.get_flag("Parallel");
+ 	let Root = Match.get_one::<String>("Root").unwrap();
+ 	let Pattern = Match.get_one::<String>("Pattern").unwrap();
- 		.get_many::<String>("command")
+ 		.get_many::<String>("Command")
- 		.map(|v| v.as_str())
+ 		.map(|Command| Command.as_str())
- 		scope(|s| {
+ 		scope(|Scope| {
- 					let entry_dir = Entry.unwrap().path().display().to_string();
- 					let paths: Vec<&str> = entry_dir.split(Separator).collect();
- 
- 					match paths.last() {
- 						Some(last) => {
- 							if last == Pattern {
- 								let working_directory =
- 									&paths[0..paths.len() - 1].join(&Separator.to_string());
- 								Some(working_directory.to_owned())
+ 					let Directory = Entry.unwrap().path().display().to_string();
+ 					let Path: Vec<&str> = Directory.split(Separator).collect();
+ 
+ 					match Path.last() {
+ 						Some(Last) => {
+ 							if Last == Pattern {
+ 								Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
- 				.for_each_with(s, |scope, dir| {
- 					scope.spawn(move |_| {
- 						println!("Executing {} for every {} in {}", Command, dir, Root);
+ 				.for_each_with(Scope, |Scope, Directory| {
+ 					Scope.spawn(move |_Scope| {
+ 						println!("Executing {} for every {} in {}", Command, Directory, Root);
- 										.current_dir(dir)
+ 										.current_dir(Directory)
- 										.current_dir(dir)
+ 										.current_dir(Directory)
- 			let entry_dir = Entry.unwrap().path().display().to_string();
- 			let paths: Vec<&str> = entry_dir.split(Separator).collect();
+ 			let Directory = Entry.unwrap().path().display().to_string();
+ 			let Path: Vec<&str> = Directory.split(Separator).collect();
- 			if let Some(last) = paths.last() {
- 				if last == Pattern {
- 					let working_directory = &paths[0..paths.len() - 1].join(&Separator.to_string());
+ 			if let Some(Last) = Path.last() {
+ 				if Last == Pattern {
+ 					let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());
- 					println!("Executing {} for every {} in {}", Command, last, Root);
+ 					println!("Executing {} for every {} in {}", Command, Last, Root);
- 					let child = match cfg!(target_os = "windows") {
+ 					let Command = match cfg!(target_os = "windows") {
- 							.current_dir(working_directory)
+ 							.current_dir(Directory)
- 							.current_dir(working_directory)
+ 							.current_dir(Directory)
- 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
+ 					let mut Out = Command.stdout.expect("Failed to get stdout handle");
- 					let mut output = String::new();
+ 					let mut Output = String::new();
- 						let mut buffer = [0; 512];
- 						let bytes_read =
- 							stdout.read(&mut buffer).expect("Failed to read from pipe");
+ 						let mut Buffer = [0; 512];
+ 						let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
- 						if bytes_read == 0 {
+ 						if Byte == 0 {
- 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
+ 						Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
- 					println!("{}", output);
+ 					println!("{}", Output);

🗣️ Summary from v0.0.12 to v0.0.13 in .
diff --git a/Cargo.toml b/Cargo.toml
index 1fe4261..b3dd05b 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.12"
+ version = "0.0.13"
- [dependencies]
- clap = { version = "4.5.1", features = ["derive"] }
- walkdir = { version = "2.4.0" }
- rayon = { version = "1.9.0" }
- crossbeam = { version = "0.8.4" }
- 
+ 
+ [dependencies]
+ clap = { version = "4.5.1", features = ["derive"] }
+ crossbeam = { version = "0.8.4" }
+ rayon = { version = "1.9.0" }
+ walkdir = { version = "2.4.0" }
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
new file mode 100644
index 0000000..b4f1f9b
--- /dev/null
+++ b/CODE_OF_CONDUCT.md
+ # Code of Conduct
+ 
+ ## Our Pledge
+ 
+ Welcome to our community! We are committed to creating a welcoming and inclusive
+ environment for all contributors. As members, contributors, and leaders, we
+ pledge to make participation in our community a harassment-free experience for
+ everyone, regardless of:
+ 
+ -   Age
+ -   Body size
+ -   Visible or invisible disability
+ -   Ethnicity
+ -   Sex characteristics
+ -   Gender identity and expression
+ -   Level of experience
+ -   Education
+ -   Socio-economic status
+ -   Nationality
+ -   Personal appearance
+ -   Race
+ -   Caste
+ -   Color
+ -   Religion
+ -   Sexual identity and orientation
+ 
+ We promise to act and interact in ways that contribute to an open, welcoming,
+ diverse, inclusive, and healthy community.
+ 
+ ## Our Standards
+ 
+ Examples of behavior that contributes to a positive environment for our
+ community include:
+ 
+ -   Demonstrating empathy and kindness toward other people
+ -   Being respectful of differing opinions, viewpoints, and experiences
+ -   Giving and gracefully accepting constructive feedback
+ -   Accepting responsibility and apologizing to those affected by our mistakes,
+     and learning from the experience
+ -   Focusing on what is best not just for us as individuals but for the overall
+     community
+ 
+ Examples of unacceptable behavior include:
+ 
+ -   The use of sexualized language or imagery, and sexual attention or advances
+     of any kind
+ -   Trolling, insulting, or derogatory comments, and personal or political
+     attacks
+ -   Public or private harassment
+ -   Publishing others' private information, such as a physical or email address,
+     without their explicit permission
+ -   Other conduct which could reasonably be considered inappropriate in a
+     professional setting
+ 
+ ## Enforcement Responsibilities
+ 
+ Community leaders are responsible for clarifying and enforcing our standards of
+ acceptable behavior. They will take appropriate and fair corrective action in
+ response to any behavior they deem inappropriate, threatening, offensive, or
+ harmful. This may include removing, editing, or rejecting comments, commits,
+ code, wiki edits, issues, and other contributions that do not align with this
+ Code of Conduct. Community leaders will communicate reasons for moderation
+ decisions when appropriate.
+ 
+ ## Scope
+ 
+ This Code of Conduct applies within all community spaces, and also applies when
+ an individual is officially representing the community in public spaces.
+ Examples of representing our community include using an official e-mail address,
+ posting via an official social media account, or acting as an appointed
+ representative at an online or offline event.
+ 
+ ## Enforcement
+ 
+ Instances of abusive, harassing, or otherwise unacceptable behavior may be
+ reported to the community leaders responsible for enforcement at
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ promptly and fairly. All community leaders are obligated to respect the privacy
+ and security of the reporter of any incident.
+ 
+ ## Enforcement Guidelines
+ 
+ Community leaders will follow these Community Impact Guidelines in determining
+ the consequences for any action they deem in violation of this Code of Conduct:
+ 
+ ### 1. Correction
+ 
+ **Community Impact**: Use of inappropriate language or other behavior deemed
+ unprofessional or unwelcome in the community.
+ 
+ **Consequence**: A private, written warning from community leaders, providing
+ clarity around the nature of the violation and an explanation of why the
+ behavior was inappropriate. A public apology may be requested.
+ 
+ ### 2. Warning
+ 
+ **Community Impact**: A violation through a single incident or series of
+ actions.
+ 
+ **Consequence**: A warning with consequences for continued behavior. No
+ interaction with the people involved, including unsolicited interaction with
+ those enforcing the Code of Conduct, for a specified period of time. This
+ includes avoiding interactions in community spaces as well as external channels
+ like social media. Violating these terms may lead to a temporary or permanent
+ ban.
+ 
+ ### 3. Temporary Ban
+ 
+ **Community Impact**: A serious violation of community standards, including
+ sustained inappropriate behavior.
+ 
+ **Consequence**: A temporary ban from any sort of interaction or public
+ communication with the community for a specified period of time. No public or
+ private interaction with the people involved, including unsolicited interaction
+ with those enforcing the Code of Conduct, is allowed during this period.
+ Violating these terms may lead to a permanent ban.
+ 
+ ### 4. Permanent Ban
+ 
+ **Community Impact**: Demonstrating a pattern of violation of community
+ standards, including sustained inappropriate behavior, harassment of an
+ individual, or aggression toward or disparagement of classes of individuals.
+ 
+ **Consequence**: A permanent ban from any sort of public interaction within the
+ community.
+ 
+ ## Attribution
+ 
+ This Code of Conduct is adapted from the [Contributor Covenant][homepage],
+ version 2.1, available at
+ [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
+ Community Impact Guidelines were inspired by [Mozilla's code of conduct
+ enforcement ladder][Mozilla CoC].
+ 
+ For answers to common questions about this code of conduct, see the FAQ at
+ [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
+ [https://www.contributor-covenant.org/translations][translations].
+ 
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
+ 
+ Thank you for being part of our community and helping us create a safe and
+ respectful environment for everyone!
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
new file mode 100644
index 0000000..c740185
--- /dev/null
+++ b/CONTRIBUTING.md
+ # Contributing Guidelines
+ 
+ Welcome to our community! We are committed to creating a welcoming and inclusive
+ environment for all contributors. Before you get started, please read and adhere
+ to the following code of conduct. By participating in our community, you agree
+ to abide by these guidelines.
+ 
+ ## Our Pledge
+ 
+ We, as members, contributors, and leaders, pledge to make participation in our
+ community a harassment-free experience for everyone, regardless of age, body
+ size, visible or invisible disability, ethnicity, sex characteristics, gender
+ identity and expression, level of experience, education, socio-economic status,
+ nationality, personal appearance, race, caste, color, religion, or sexual
+ identity and orientation. We pledge to act and interact in ways that contribute
+ to an open, welcoming, diverse, inclusive, and healthy community.
+ 
+ ## Our Standards
+ 
+ Examples of behavior that contributes to a positive environment for our
+ community include:
+ 
+ -   Demonstrating empathy and kindness toward other people
+ -   Being respectful of differing opinions, viewpoints, and experiences
+ -   Giving and gracefully accepting constructive feedback
+ -   Accepting responsibility and apologizing to those affected by our mistakes,
+     and learning from the experience
+ -   Focusing on what is best not just for us as individuals, but for the overall
+     community
+ 
+ Examples of unacceptable behavior include:
+ 
+ -   The use of sexualized language or imagery, and sexual attention or advances
+     of any kind
+ -   Trolling, insulting, or derogatory comments, and personal or political
+     attacks
+ -   Public or private harassment
+ -   Publishing others' private information, such as a physical or email address,
+     without their explicit permission
+ -   Other conduct which could reasonably be considered inappropriate in a
+     professional setting
+ 
+ ## Enforcement Responsibilities
+ 
+ Community leaders are responsible for clarifying and enforcing our standards of
+ acceptable behavior and will take appropriate and fair corrective action in
+ response to any behavior that they deem inappropriate, threatening, offensive,
+ or harmful. Community leaders have the right and responsibility to remove, edit,
+ or reject comments, commits, code, wiki edits, issues, and other contributions
+ that are not aligned with this Code of Conduct, and will communicate reasons for
+ moderation decisions when appropriate.
+ 
+ ## Scope
+ 
+ This Code of Conduct applies within all community spaces, and also applies when
+ an individual is officially representing the community in public spaces.
+ Examples of representing our community include using an official e-mail address,
+ posting via an official social media account, or acting as an appointed
+ representative at an online or offline event.
+ 
+ ## Enforcement
+ 
+ Instances of abusive, harassing, or otherwise unacceptable behavior may be
+ reported to the community leaders responsible for enforcement at
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ promptly and fairly. All community leaders are obligated to respect the privacy
+ and security of the reporter of any incident.
+ 
+ ## Enforcement Guidelines
+ 
+ Community leaders will follow these Community Impact Guidelines in determining
+ the consequences for any action they deem in violation of this Code of Conduct:
+ 
+ ### 1. Correction
+ 
+ **Community Impact**: Use of inappropriate language or other behavior deemed
+ unprofessional or unwelcome in the community.
+ 
+ **Consequence**: A private, written warning from community leaders, providing
+ clarity around the nature of the violation and an explanation of why the
+ behavior was inappropriate. A public apology may be requested.
+ 
+ ### 2. Warning
+ 
+ **Community Impact**: A violation through a single incident or series of
+ actions.
+ 
+ **Consequence**: A warning with consequences for continued behavior. No
+ interaction with the people involved, including unsolicited interaction with
+ those enforcing the Code of Conduct, for a specified period of time. This
+ includes avoiding interactions in community spaces as well as external channels
+ like social media. Violating these terms may lead to a temporary or permanent
+ ban.
+ 
+ ### 3. Temporary Ban
+ 
+ **Community Impact**: A serious violation of community standards, including
+ sustained inappropriate behavior.
+ 
+ **Consequence**: A temporary ban from any sort of interaction or public
+ communication with the community for a specified period of time. No public or
+ private interaction with the people involved, including unsolicited interaction
+ with those enforcing the Code of Conduct, is allowed during this period.
+ Violating these terms may lead to a permanent ban.
+ 
+ ### 4. Permanent Ban
+ 
+ **Community Impact**: Demonstrating a pattern of violation of community
+ standards, including sustained inappropriate behavior, harassment of an
+ individual, or aggression toward or disparagement of classes of individuals.
+ 
+ **Consequence**: A permanent ban from any sort of public interaction within the
+ community.
+ 
+ ## Attribution
+ 
+ This Code of Conduct is adapted from the [Contributor Covenant][homepage],
+ version 2.1, available at
+ [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
+ Community Impact Guidelines were inspired by [Mozilla's code of conduct
+ enforcement ladder][Mozilla CoC].
+ 
+ For answers to common questions about this code of conduct, see the FAQ at
+ [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
+ [https://www.contributor-covenant.org/translations][translations].
+ 
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
+ 
+ Thank you for being part of our community and helping us create a safe and
+ respectful environment for everyone!
diff --git a/README.md b/README.md
index 16f2af5..a7c4e95 100644
--- a/README.md
+++ b/README.md
- Inn is a tiny Rust utility that lets execute commmands in different directories
- concurrently.
+ Innkeeper is a command-line tool designed to execute a specified command in all
+ directories that match a certain pattern within a given root directory. It
+ provides flexibility and efficiency in running commands across multiple
+ directories with customizable patterns.
+ ## Dependencies:
+ 
+ The code imports several crates:
+ 
+ -   `clap` - For parsing command-line arguments.
+ -   `crossbeam` - Used for creating scoped threads.
+ -   `rayon` - Enables parallel execution of tasks.
+ -   `walkdir` - Facilitates filesystem traversal.
+ 

🗣️ Summary from v0.0.13 to v0.0.14 in .
diff --git a/Cargo.toml b/Cargo.toml
index b3dd05b..f3f38ac 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.13"
+ version = "0.0.14"
diff --git a/README.md b/README.md
index a7c4e95..2c5a5f5 100644
--- a/README.md
+++ b/README.md
- ## Dependencies:
+ ## Dependencies
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
index 3df03b9..d456e06 100644
--- a/Source/Function/Binary/Command.rs
+++ b/Source/Function/Binary/Command.rs
- 	clap::{Arg, ArgAction, Command as ClapCommand},
+ 	clap::{Arg, ArgAction::SetTrue, Command as ClapCommand},
- 				.action(ArgAction::SetTrue)
+ 				.action(SetTrue)
- 				.action(ArgAction::SetTrue)
+ 				.action(SetTrue)

🗣️ Summary from v0.0.14 to v0.0.15 in .
diff --git a/Cargo.toml b/Cargo.toml
index f3f38ac..ac44f76 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- [package]
- name = "innkeeper"
- version = "0.0.14"
- description = "🍺 Inn is a tiny Rust utility that lets execute commands in different directories."
- license = "MIT"
- default-run = "Inn"
- repository = "https://github.com/NikolaRHristov/Inn.git"
- autobins = false
- autobenches = false
- autotests = false
- autoexamples = false
- 
- clap = { version = "4.5.1", features = ["derive"] }
+ clap = { features = ["derive"], version = "4.5.1" }
- walkdir = { version = "2.4.0" }
+ walkdir = "2.4.0"
+ 
+ [package]
+ autobenches = false
+ autobins = false
+ autoexamples = false
+ autotests = false
+ default-run = "Inn"
+ description = "🍺 Innkeeper is a command-line tool designed to execute a specified command in all directories that match a certain pattern within a given root directory. It provides flexibility and efficiency in running commands across multiple directories with customizable patterns."
+ license = "MIT"
+ name = "innkeeper"
+ repository = "https://github.com/NikolaRHristov/Inn.git"
+ version = "0.0.15"
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
index d456e06..af6967c 100644
--- a/Source/Function/Binary/Command.rs
+++ b/Source/Function/Binary/Command.rs
- 			Arg::new("Pattern")
+ 			Arg::new("Exclude")
+ 				.short('E')
+ 				.long("Exclude")
+ 				.value_name("EXCLUDE")
+ 				.required(false)
+ 				.help("Exclude pattern.")
+ 				.default_value("node_modules .git target dist vendor"),
+ 		)
+ 		.arg(
+ 			Arg::new("Pattern")
+ 				.display_order(5)
- 				.display_order(5)
+ 				.display_order(6)
+ 	let Exclude = Match.get_one::<String>("Exclude").unwrap().split(" ");
- 		if !Pattern.contains("node_modules") {
- 			return !Entry.path().display().to_string().contains("node_modules");
- 		}
+ 		let Path = Entry.path().display().to_string();
- 		if !File {
- 			println!("{:?}", Entry.path().display().to_string().contains("node_modules"));
- 
- 			fs::metadata(Entry.path().display().to_string().clone()).unwrap().is_dir()
+ 		!Exclude.clone().into_iter().any(|Pattern| {
+ 			if File {
+ 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Pattern)
- 			true
+ 				Path.contains(Pattern)
+ 		})
- 					let Directory = Entry.unwrap().path().display().to_string();
- 					let Path: Vec<&str> = Directory.split(Separator).collect();
+ 					let Path = Entry.unwrap().path().display().to_string();
+ 					let Path: Vec<&str> = Path.split(Separator).collect();
- 			let Directory = Entry.unwrap().path().display().to_string();
- 			let Path: Vec<&str> = Directory.split(Separator).collect();
+ 			let Path = Entry.unwrap().path().display().to_string();
+ 			let Path: Vec<&str> = Path.split(Separator).collect();

🗣️ Summary from v0.0.15 to v0.0.16 in .
diff --git a/Cargo.toml b/Cargo.toml
index ac44f76..97d8add 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- crossbeam = { version = "0.8.4" }
- rayon = { version = "1.9.0" }
+ tokio = { features = ["full"], version = "1.36.0" }
- version = "0.0.15"
+ version = "0.0.16"
+ edition = "2021"
diff --git a/README.md b/README.md
index 2c5a5f5..8460887 100644
--- a/README.md
+++ b/README.md
- -   `crossbeam` - Used for creating scoped threads.
- -   `rayon` - Enables parallel execution of tasks.
+ -   `tokio` - Enables parallel execution of tasks.
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
index af6967c..2485de6 100644
--- a/Source/Function/Binary/Command.rs
+++ b/Source/Function/Binary/Command.rs
- extern crate clap;
- extern crate crossbeam;
- extern crate rayon;
- extern crate walkdir;
- 
- use self::{
- 	clap::{Arg, ArgAction::SetTrue, Command as ClapCommand},
- 	crossbeam::scope,
- 	rayon::prelude::*,
- 	walkdir::WalkDir,
- };
+ use clap::{Arg, ArgAction::SetTrue, Command as CommandClap};
+ use tokio::process::Command as CommandTokio;
+ use walkdir::WalkDir;
- 	let Match = ClapCommand::new("Innkeeper")
+ 	let Match = CommandClap::new("Innkeeper")
- 		!Exclude.clone().into_iter().any(|Pattern| {
+ 		!Exclude.clone().into_iter().filter(|Exclude| Pattern != Exclude).any(|Exclude| {
- 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Pattern)
+ 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
- 				Path.contains(Pattern)
+ 				Path.contains(Exclude)
- 		scope(|Scope| {
+ 		let mut Task = Vec::new();
+ 
- 				.filter_map(|x| x)
- 				.collect::<Vec<String>>()
- 				.into_par_iter()
- 				.for_each_with(Scope, |Scope, Directory| {
- 					Scope.spawn(move |_Scope| {
- 						println!("Executing {} for every {} in {}", Command, Directory, Root);
+ 			.filter_map(|Entry| Entry)
+ 			.for_each(|Directory| {
+ 				let command;
- 						println!(
- 							"{}",
- 							String::from_utf8_lossy(
- 								&match cfg!(target_os = "windows") {
- 									true => Command::new("cmd")
+ 				if cfg!(target_os = "windows") {
+ 					command = CommandTokio::new("cmd")
- 										.current_dir(Directory)
- 										.output()
- 										.expect("Failed to execute process."),
- 									false => Command::new("sh")
+ 						.current_dir(Directory.clone())
+ 						.output();
+ 				} else {
+ 					command = CommandTokio::new("sh")
- 										.current_dir(Directory)
+ 						.current_dir(Directory.clone())
- 										.output()
- 										.expect("Failed to execute process."),
+ 						.output();
- 								.stdout
+ 
+ 				Task.push(async move {
+ 					println!("Executing {} for every {} in {}", Command, Directory, Root);
+ 
+ 					println!(
+ 						"{}",
+ 						String::from_utf8_lossy(
+ 							&command.await.expect("Failed to execute process.").stdout
- 		})
- 		.unwrap();
+ 
+ 		tokio::runtime::Runtime::new().unwrap().block_on(async {
+ 			for Task in Task {
+ 				Task.await;
+ 			}
+ 		});
- 					let Command = match cfg!(target_os = "windows") {
+ 					let mut Out = match cfg!(target_os = "windows") {
- 					};
- 
- 					let mut Out = Command.stdout.expect("Failed to get stdout handle");
+ 					}
+ 					.stdout
+ 					.expect("Failed to get stdout handle");

🗣️ Summary from v0.0.16 to v0.0.17 in .
diff --git a/Cargo.toml b/Cargo.toml
index 97d8add..e0ee1fb 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.16"
+ version = "0.0.17"

🗣️ Summary from v0.0.17 to v0.0.2 in .
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index bca13f1..161d7eb 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-             - uses: actions/cache@v4.0.0
+             - uses: actions/cache@v4.0.1
diff --git a/.gitignore b/.gitignore
index a60405a..cbb2f71 100644
--- a/.gitignore
+++ b/.gitignore
- /Target/*
- !/Target/release
- /Target/release/*
- !/Target/release/*.deb
- !/Target/release/*.exe
- !/Target/release/inn
- !/Target/release/innkeeper
+ /target/*
+ !/target/release
+ /target/release/*
+ !/target/release/*.deb
+ !/target/release/*.exe
+ !/target/release/Inn
+ !/target/release/Innkeeper
diff --git a/build.rs b/build.rs
index 8f162d7..550762c 100644
--- a/build.rs
+++ b/build.rs
- 			.expect("Failed to read Cargo.toml.")
+ 			.expect("Cannot Cargo.toml.")
- 			.unwrap()
+ 			.expect("Cannot Version.")
- 			.expect("Invalid version line format.")
+ 			.expect("Cannot nth.")
diff --git a/Cargo.toml b/Cargo.toml
index e0ee1fb..bd5db8e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- name = "Inn"
- path = "Source/Function/Binary/Inn.rs"
- 
- [[bin]]
- name = "Innkeeper"
- path = "Source/Function/Binary/Innkeeper.rs"
+ name = "INN2"
+ path = "Source/Library.rs"
- walkdir = "2.4.0"
+ walkdir = "2.5.0"
+ 
+ [lib]
+ crate-type = ["staticlib", "cdylib", "rlib"]
+ name = "Library"
+ path = "Source/Library.rs"
- default-run = "Inn"
- description = "🍺 Innkeeper is a command-line tool designed to execute a specified command in all directories that match a certain pattern within a given root directory. It provides flexibility and efficiency in running commands across multiple directories with customizable patterns."
+ default-run = "INN2"
+ description = "🍺 INN2 lets you execute parallel commands in multiple directories."
- name = "innkeeper"
- repository = "https://github.com/NikolaRHristov/Inn.git"
- version = "0.0.17"
+ name = "inn2"
+ repository = "https://github.com/NikolaRHristov/INN2.git"
+ version = "0.0.2"
diff --git a/LICENSE b/LICENSE
new file mode 100644
index 0000000..c47b9fa
--- /dev/null
+++ b/LICENSE
+ MIT License
+ 
+ Copyright (c) 2023-2024 Nikola R. Hristov
+ 
+ Permission is hereby granted, free of charge, to any person obtaining a copy of
+ this software and associated documentation files (the "Software"), to deal in
+ the Software without restriction, including without limitation the rights to
+ use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
+ the Software, and to permit persons to whom the Software is furnished to do so,
+ subject to the following conditions:
+ 
+ The above copyright notice and this permission notice shall be included in all
+ copies or substantial portions of the Software.
+ 
+ THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
+ IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
+ FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
+ COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
+ IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
+ CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
diff --git a/README.md b/README.md
index 8460887..26d3127 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Inn]
+ # 🍺 [INN2]
- Innkeeper is a command-line tool designed to execute a specified command in all
+ INN2 is a command-line tool designed to execute a specified command in all
- [Inn]: https://crates.io/crates/innkeeper
+ [INN2]: https://crates.io/crates/inn2
+ 
+ ## Benchmark
+ 
+ <table>
+ 	<tr>
+ 		<th>Command:</th>
+ 		<th>Time:</th>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>INN2 -P .git ls</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m9.441s
+ user    0m0.030s
+ sys     0m0.046s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m14.293s +5s
+ user    0m4.645s +4s
+ sys     0m8.937s +8s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>INN2 -P .git git status</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m24.146s
+ user    0m0.030s
+ sys     0m0.062s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m28.584s +4s
+ user    0m4.695s +4s
+ sys     0m8.354s +8s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>INN2 -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m33.813s
+ user    0m0.015s
+ sys     0m0.060s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>find -iname .git -type d -execdir \
+ bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m53.122s +20s
+ user    0m9.449s +9s
+ sys     0m14.442s +14s</pre>
+ 		</td>
+ 	</tr>
+ </table>
- cargo install innkeeper
+ cargo install inn2
- Inn .git git fetch upstream
+ INN2 .git git fetch upstream
- This will fetch from upstream for all the `.git` repositories inside the current
- directory. Basically it replaces:
+ This command will fetch from upstream for all the .git repositories inside the
+ current directory. Essentially, it replaces the following shell command:
- Specify a `--File` argument or `-F` if you would like to search for file instead
- of a directory. Default is `false` or no flag at all.
+ To specify a `--File` argument or `-F`, if you would like to search for a file
+ instead of a directory, use:
- Inn -F astro.config.ts npx astro add astro-compress
+ INN2 -F astro.config.ts npx astro add astro-compress
- You can also provide a `--Root` argument or `-R` which sets the current working
- directory to a different folder. Default is `.`.
+ Additionally, you can provide a `--Root` argument or `-R` to set the current
+ working directory to a different folder. The default is `.`.
- Inn -R D:\Developer .git git fetch upstream
+ INN2 -R D:\Developer .git git fetch upstream
- parallel. Default is sequential.
+ parallel. The default is sequential.
- Inn -P -R D:\Developer .git git fetch upstream
+ INN2 -P -R D:\Developer .git git fetch upstream
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
new file mode 100644
index 0000000..d9d47b8
--- /dev/null
+++ b/Source/Fn/Binary/Command.rs
+ pub mod Entry;
+ pub mod Parallel;
+ pub mod Sequential;
+ 
+ pub fn Fn() -> ArgMatches {
+ 	Command::new("Innkeeper")
+ 		.version(env!("CARGO_PKG_VERSION"))
+ 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.about("Run a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("File")
+ 				.short('F')
+ 				.long("File")
+ 				.action(SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("Parallel")
+ 				.short('P')
+ 				.long("Parallel")
+ 				.action(SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("Root")
+ 				.short('R')
+ 				.long("Root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Exclude")
+ 				.short('E')
+ 				.long("Exclude")
+ 				.display_order(4)
+ 				.value_name("EXCLUDE")
+ 				.required(false)
+ 				.help("Exclude pattern.")
+ 				.default_value("node_modules .git target dist vendor"),
+ 		)
+ 		.arg(
+ 			Arg::new("Pattern")
+ 				.display_order(5)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Command")
+ 				.num_args(0..=10)
+ 				.display_order(6)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches()
+ }
+ 
+ use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
new file mode 100644
index 0000000..bf4ce76
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
+ pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
+ 	WalkDir::new(Root)
+ 		.into_iter()
+ 		.filter_map(|Entry| {
+ 			let Path = Entry.expect("Cannot Entry.").path().display().to_string();
+ 
+ 			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
+ 				let Match = Path.contains(&Exclude);
+ 
+ 				match File {
+ 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
+ 					false => Match,
+ 				}
+ 			}) {
+ 				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
+ 			} else {
+ 				None
+ 			}
+ 		})
+ 		.collect::<Vec<_>>()
+ }
+ 
+ use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};
+ 
+ use walkdir::WalkDir;
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
new file mode 100644
index 0000000..efd9808
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
+ pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	let mut Queue = Vec::new();
+ 
+ 	Entry
+ 		.into_iter()
+ 		.filter_map(|Entry| {
+ 			Entry
+ 				.last()
+ 				.filter(|Last| *Last == &Pattern)
+ 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
+ 		})
+ 		.for_each(|Entry| {
+ 			let Output = if cfg!(target_os = "windows") {
+ 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
+ 			} else {
+ 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
+ 			};
+ 
+ 			Queue.push(async move {
+ 				println!(
+ 					"{}",
+ 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
+ 				);
+ 			});
+ 		});
+ 
+ 	tokio::runtime::Builder::new_multi_thread()
+ 		.enable_all()
+ 		.build()
+ 		.expect("Cannot Runtime.")
+ 		.block_on(async {
+ 			for Queue in Queue {
+ 				Queue.await;
+ 			}
+ 		})
+ }
+ 
+ use crate::Struct::Binary::Command::Entry::Struct as Option;
+ use tokio::process::Command;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..8eef18b
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
+ pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
+ 	Entry
+ 		.into_iter()
+ 		.filter_map(|Entry| {
+ 			Entry
+ 				.last()
+ 				.filter(|Last| *Last == &Pattern)
+ 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
+ 		})
+ 		.for_each(|Entry| {
+ 			let mut Out = match cfg!(target_os = "windows") {
+ 				true => Command::new("cmd")
+ 					.args(["/C", &Command])
+ 					.current_dir(Entry)
+ 					.stdout(Stdio::piped())
+ 					.spawn()
+ 					.expect("Cannot spawn."),
+ 				false => Command::new("sh")
+ 					.arg("-c")
+ 					.current_dir(Entry)
+ 					.arg(Command.clone())
+ 					.stdout(Stdio::piped())
+ 					.spawn()
+ 					.expect("Cannot spawn."),
+ 			}
+ 			.stdout
+ 			.expect("Cannot stdout.");
+ 
+ 			let mut Output = String::new();
+ 
+ 			loop {
+ 				let mut Buffer = [0; 512];
+ 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");
+ 
+ 				if Byte == 0 {
+ 					break;
+ 				}
+ 
+ 				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
+ 			}
+ 
+ 			println!("{}", Output);
+ 		})
+ }
+ 
+ use crate::Struct::Binary::Command::Entry::Struct as Option;
+ 
+ use std::{
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
diff --git a/Source/Fn/Binary/mod.rs b/Source/Fn/Binary/mod.rs
new file mode 100644
index 0000000..9da7843
--- /dev/null
+++ b/Source/Fn/Binary/mod.rs
+ pub mod Command;
diff --git a/Source/Fn/mod.rs b/Source/Fn/mod.rs
new file mode 100644
index 0000000..a56e8ce
--- /dev/null
+++ b/Source/Fn/mod.rs
+ pub mod Binary;
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
deleted file mode 100644
index 2485de6..0000000
--- a/Source/Function/Binary/Command.rs
+++ /dev/null
- use clap::{Arg, ArgAction::SetTrue, Command as CommandClap};
- use tokio::process::Command as CommandTokio;
- use walkdir::WalkDir;
- 
- use std::{
- 	fs,
- 	io::Read,
- 	process::{Command, Stdio},
- };
- 
- #[allow(dead_code)]
- pub fn run() {
- 	let Match = CommandClap::new("Innkeeper")
- 		.version(env!("CARGO_PKG_VERSION"))
- 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("File")
- 				.short('F')
- 				.long("File")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("Parallel")
- 				.short('P')
- 				.long("Parallel")
- 				.action(SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("Root")
- 				.short('R')
- 				.long("Root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Exclude")
- 				.short('E')
- 				.long("Exclude")
- 				.display_order(4)
- 				.value_name("EXCLUDE")
- 				.required(false)
- 				.help("Exclude pattern.")
- 				.default_value("node_modules .git target dist vendor"),
- 		)
- 		.arg(
- 			Arg::new("Pattern")
- 				.display_order(5)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Command")
- 				.num_args(0..=10)
- 				.display_order(6)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches();
- 
- 	let File = Match.get_flag("File");
- 	let Parallel = Match.get_flag("Parallel");
- 	let Root = Match.get_one::<String>("Root").unwrap();
- 	let Exclude = Match.get_one::<String>("Exclude").unwrap().split(" ");
- 	let Pattern = Match.get_one::<String>("Pattern").unwrap();
- 	let Command = &Match
- 		.get_many::<String>("Command")
- 		.unwrap_or_default()
- 		.map(|Command| Command.as_str())
- 		.collect::<Vec<_>>()
- 		.join(" ");
- 
- 	let Separator = std::path::MAIN_SEPARATOR;
- 
- 	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
- 		let Path = Entry.path().display().to_string();
- 
- 		!Exclude.clone().into_iter().filter(|Exclude| Pattern != Exclude).any(|Exclude| {
- 			if File {
- 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
- 			} else {
- 				Path.contains(Exclude)
- 			}
- 		})
- 	});
- 
- 	if Parallel {
- 		println!("Executing code in parallel.");
- 
- 		// Execution: Parallel
- 		let mut Task = Vec::new();
- 
- 		Entry
- 			.map(|Entry| {
- 				let Path = Entry.unwrap().path().display().to_string();
- 				let Path: Vec<&str> = Path.split(Separator).collect();
- 
- 				match Path.last() {
- 					Some(Last) => {
- 						if Last == Pattern {
- 							Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
- 						} else {
- 							None
- 						}
- 					}
- 					None => None,
- 				}
- 			})
- 			.filter_map(|Entry| Entry)
- 			.for_each(|Directory| {
- 				let command;
- 
- 				if cfg!(target_os = "windows") {
- 					command = CommandTokio::new("cmd")
- 						.args(["/C", Command.as_str()])
- 						.current_dir(Directory.clone())
- 						.output();
- 				} else {
- 					command = CommandTokio::new("sh")
- 						.arg("-c")
- 						.current_dir(Directory.clone())
- 						.arg(Command)
- 						.output();
- 				}
- 
- 				Task.push(async move {
- 					println!("Executing {} for every {} in {}", Command, Directory, Root);
- 
- 					println!(
- 						"{}",
- 						String::from_utf8_lossy(
- 							&command.await.expect("Failed to execute process.").stdout
- 						)
- 					);
- 				});
- 			});
- 
- 		tokio::runtime::Runtime::new().unwrap().block_on(async {
- 			for Task in Task {
- 				Task.await;
- 			}
- 		});
- 	} else {
- 		println!("Executing code in sequential.");
- 
- 		// Execution: Sequential
- 		for Entry in Entry {
- 			let Path = Entry.unwrap().path().display().to_string();
- 			let Path: Vec<&str> = Path.split(Separator).collect();
- 
- 			if let Some(Last) = Path.last() {
- 				if Last == Pattern {
- 					let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());
- 
- 					println!("Executing {} for every {} in {}", Command, Last, Root);
- 
- 					let mut Out = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", Command])
- 							.current_dir(Directory)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(Directory)
- 							.arg(Command)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 					}
- 					.stdout
- 					.expect("Failed to get stdout handle");
- 
- 					let mut Output = String::new();
- 
- 					loop {
- 						let mut Buffer = [0; 512];
- 						let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
- 
- 						if Byte == 0 {
- 							break;
- 						}
- 
- 						Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
- 					}
- 
- 					println!("{}", Output);
- 				}
- 			}
- 		}
- 	}
- }
diff --git a/Source/Function/Binary/Inn.rs b/Source/Function/Binary/Inn.rs
deleted file mode 100644
index 38e3fe5..0000000
--- a/Source/Function/Binary/Inn.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Command;
- 
- fn main() {
- 	Command::run()
- }
diff --git a/Source/Function/Binary/Innkeeper.rs b/Source/Function/Binary/Innkeeper.rs
deleted file mode 100644
index 38e3fe5..0000000
--- a/Source/Function/Binary/Innkeeper.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Command;
- 
- fn main() {
- 	Command::run()
- }
diff --git a/Source/Library.rs b/Source/Library.rs
new file mode 100644
index 0000000..982f1a3
--- /dev/null
+++ b/Source/Library.rs
+ #![allow(non_snake_case)]
+ mod Fn;
+ mod Struct;
+ 
+ #[allow(dead_code)]
+ fn main() {
+ 	(Struct::Binary::Command::Struct::Fn().Fn)()
+ }
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
new file mode 100644
index 0000000..fdaa4e3
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
+ pub mod Entry;
+ pub mod Option;
+ 
+ use crate::Fn::Binary::Command::{Parallel, Sequential};
+ 
+ #[derive(Debug)]
+ pub struct Struct {
+ 	pub Separator: Option::Separator,
+ 	pub Fn: fn(),
+ }
+ 
+ impl Struct {
+ 	pub fn Fn() -> Self {
+ 		Self {
+ 			Separator: std::path::MAIN_SEPARATOR,
+ 			Fn: || {
+ 				let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));
+ 				
+ 				match Option.Parallel {
+ 					true => {
+ 						Parallel::Fn(Option);
+ 					}
+ 					false => {
+ 						Sequential::Fn(Option);
+ 					}
+ 				};
+ 			},
+ 		}
+ 	}
+ }
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..5b62589
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
+ pub type Type = Vec<Vec<String>>;
+ 
+ pub struct Struct {
+ 	pub Command: Command,
+ 	pub Entry: Type,
+ 	pub Parallel: Parallel,
+ 	pub Pattern: Pattern,
+ 	pub Separator: Separator,
+ }
+ 
+ impl Struct {
+ 	pub fn Fn(Option: &Option) -> Self {
+ 		Self {
+ 			Command: Option.Command.clone(),
+ 			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
+ 			Parallel: Option.Parallel,
+ 			Pattern: Option.Pattern.clone(),
+ 			Separator: Option.Separator,
+ 		}
+ 	}
+ }
+ 
+ use crate::Struct::Binary::Command::Option::{
+ 	Command, Parallel, Pattern, Separator, Struct as Option,
+ };
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..4c946ac
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
+ pub type Command = String;
+ pub type Parallel = bool;
+ pub type Pattern = String;
+ pub type Separator = char;
+ 
+ pub struct Struct {
+ 	pub Command: String,
+ 	pub Exclude: Vec<String>,
+ 	pub File: bool,
+ 	pub Parallel: bool,
+ 	pub Pattern: Pattern,
+ 	pub Root: String,
+ 	pub Separator: Separator,
+ }
+ 
+ impl Struct {
+ 	pub fn Fn(Option { Separator, .. }: Option) -> Self {
+ 		Self {
+ 			File: Fn().get_flag("File"),
+ 			Parallel: Fn().get_flag("Parallel"),
+ 			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
+ 			Exclude: Fn()
+ 				.get_one::<String>("Exclude")
+ 				.expect("Cannot Exclude.")
+ 				.split(" ")
+ 				.map(|Command| Command.to_string())
+ 				.collect::<Vec<_>>(),
+ 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
+ 			Command: Fn()
+ 				.get_many::<String>("Command")
+ 				.expect("Cannot Command.")
+ 				.map(|Command| Command.as_str())
+ 				.collect::<Vec<_>>()
+ 				.join(" "),
+ 			Separator,
+ 		}
+ 	}
+ }
+ 
+ use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
diff --git a/Source/Struct/Binary/mod.rs b/Source/Struct/Binary/mod.rs
new file mode 100644
index 0000000..9da7843
--- /dev/null
+++ b/Source/Struct/Binary/mod.rs
+ pub mod Command;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
new file mode 100644
index 0000000..a56e8ce
--- /dev/null
+++ b/Source/Struct/mod.rs
+ pub mod Binary;

🗣️ Summary from v0.0.2 to v0.0.3 in .
diff --git a/.github/dependabot.yml b/.github/dependabot.yml
index 44227ad..d4ca004 100644
--- a/.github/dependabot.yml
+++ b/.github/dependabot.yml
- enable-beta-ecosystems: true
- 
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index 819f8a1..254bb2f 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-     group: Dependabot-${{ github.workflow }}-${{ github.ref }}
+     group: dependabot-${{ github.workflow }}-${{ github.ref }}
- permissions:
-     security-events: write
-     contents: write
-     pull-requests: write
- 
+ permissions:
+     contents: write
+     pull-requests: write
+ 
-     Approve:
+     approve-and-merge:
- 
- 
-             - uses: dependabot/fetch-metadata@v1.6.0
+             - uses: dependabot/fetch-metadata@v1.3.6
- 
- 
-     Merge:
-         runs-on: ubuntu-latest
- 
-         if: ${{ github.actor == 'dependabot[bot]' }}
- 
-         steps:
-             - uses: dependabot/fetch-metadata@v1.6.0
-               with:
-                   github-token: "${{ secrets.GITHUB_TOKEN }}"
- 
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
deleted file mode 100644
index ffde8df..0000000
--- a/.github/workflows/GitHub.yml
+++ /dev/null
- name: GitHub
- 
- concurrency:
-     group: GitHub-${{ github.workflow }}-${{ github.ref }}
-     cancel-in-progress: true
- 
- permissions:
-     issues: write
-     pull-requests: write
- 
- on:
-     issues:
-         types: [opened]
-     pull_request:
-         types: [opened]
- 
- jobs:
-     Assign:
-         runs-on: ubuntu-latest
- 
-         env:
-             ADBLOCK: true
-             TELEMETRY_DISABLED: 1
-             ASTRO_TELEMETRY_DISABLED: 1
-             AUTOMATEDLAB_TELEMETRY_OPTOUT: 1
-             AZURE_CORE_COLLECT_TELEMETRY: 0
-             CHOOSENIM_NO_ANALYTICS: 1
-             DIEZ_DO_NOT_TRACK: 1
-             DO_NOT_TRACK: 1
-             DOTNET_CLI_TELEMETRY_OPTOUT: 1
-             DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT: 1
-             ET_NO_TELEMETRY: 1
-             GATSBY_TELEMETRY_DISABLED: 1
-             GATSBY_TELEMETRY_OPT_OUT: 1
-             GATSBY_TELEMETRY_OPTOUT: 1
-             HASURA_GRAPHQL_ENABLE_TELEMETRY: false
-             HINT_TELEMETRY: off
-             HOMEBREW_NO_ANALYTICS: 1
-             INFLUXD_REPORTING_DISABLED: true
-             ITERATIVE_DO_NOT_TRACK: 1
-             NEXT_TELEMETRY_DEBUG: 1
-             NEXT_TELEMETRY_DISABLED: 1
-             NG_CLI_ANALYTICS: false
-             NUXT_TELEMETRY_DISABLED: 1
-             PIN_DO_NOT_TRACK: 1
-             POWERSHELL_TELEMETRY_OPTOUT: 1
-             SAM_CLI_TELEMETRY: 0
-             STNOUPGRADE: 1
-             STRIPE_CLI_TELEMETRY_OPTOUT: 1
- 
-         steps:
-             - uses: pozil/auto-assign-issue@v1.13.0
-               with:
-                   repo-token: ${{ secrets.GITHUB_TOKEN }}
-                   assignees: NikolaRHristov
-                   numOfAssignee: 1
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 161d7eb..eeb1562 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-     group: Rust-${{ github.workflow }}-${{ github.ref }}
+     group: rust-${{ github.workflow }}-${{ github.ref }}
-     Build:
+     rust:
-             TELEMETRY_DISABLED: 1
-             - uses: actions/checkout@v4.1.1
- 
+             - uses: actions/checkout@v3.3.0
-             - uses: actions/cache@v4.0.1
+             - uses: actions/cache@v3.2.6
-                       Target/
-                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
+                   key: ${{ runner.os }}-cargo-${{ hash('./Cargo.toml') }}
diff --git a/.gitignore b/.gitignore
index cbb2f71..96ef6c0 100644
--- a/.gitignore
+++ b/.gitignore
+ /target
- 
- /target/*
- !/target/release
- /target/release/*
- !/target/release/*.deb
- !/target/release/*.exe
- !/target/release/Inn
- !/target/release/Innkeeper
diff --git a/build.rs b/build.rs
deleted file mode 100644
index 550762c..0000000
--- a/build.rs
+++ /dev/null
- #![allow(non_snake_case)]
- 
- use std::fs;
- 
- fn main() {
- 	println!("cargo:rerun-if-changed=Cargo.toml");
- 	println!(
- 		"cargo:rustc-env=CARGO_PKG_VERSION={}",
- 		fs::read_to_string("Cargo.toml")
- 			.expect("Cannot Cargo.toml.")
- 			.lines()
- 			.find(|Line| Line.starts_with("version"))
- 			.expect("Cannot Version.")
- 			.split('=')
- 			.nth(1)
- 			.expect("Cannot nth.")
- 			.trim()
- 			.trim_matches('"')
- 	);
- }
diff --git a/Cargo.toml b/Cargo.toml
index bd5db8e..ed01669 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- [[bin]]
- name = "INN2"
- path = "Source/Library.rs"
+ [package]
+ name = "innkeeper"
+ version = "0.0.3"
+ description = "Inn is a shell script that allows you to execute commmands in different directories."
+ license = "MIT"
- clap = { features = ["derive"], version = "4.5.1" }
- tokio = { features = ["full"], version = "1.36.0" }
- walkdir = "2.5.0"
- 
- [lib]
- crate-type = ["staticlib", "cdylib", "rlib"]
- name = "Library"
- path = "Source/Library.rs"
+ clap = { version = "4.1.8", features = ["derive"] }
+ walkdir = { version = "2.3.2" }
- [package]
- autobenches = false
- autobins = false
- autoexamples = false
- autotests = false
- default-run = "INN2"
- description = "🍺 INN2 lets you execute parallel commands in multiple directories."
- license = "MIT"
- name = "inn2"
- repository = "https://github.com/NikolaRHristov/INN2.git"
- version = "0.0.2"
- edition = "2021"
+ [[bin]]
+ name = "inn"
+ path = "src/main.rs"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
deleted file mode 100644
index b4f1f9b..0000000
--- a/CODE_OF_CONDUCT.md
+++ /dev/null
- # Code of Conduct
- 
- ## Our Pledge
- 
- Welcome to our community! We are committed to creating a welcoming and inclusive
- environment for all contributors. As members, contributors, and leaders, we
- pledge to make participation in our community a harassment-free experience for
- everyone, regardless of:
- 
- -   Age
- -   Body size
- -   Visible or invisible disability
- -   Ethnicity
- -   Sex characteristics
- -   Gender identity and expression
- -   Level of experience
- -   Education
- -   Socio-economic status
- -   Nationality
- -   Personal appearance
- -   Race
- -   Caste
- -   Color
- -   Religion
- -   Sexual identity and orientation
- 
- We promise to act and interact in ways that contribute to an open, welcoming,
- diverse, inclusive, and healthy community.
- 
- ## Our Standards
- 
- Examples of behavior that contributes to a positive environment for our
- community include:
- 
- -   Demonstrating empathy and kindness toward other people
- -   Being respectful of differing opinions, viewpoints, and experiences
- -   Giving and gracefully accepting constructive feedback
- -   Accepting responsibility and apologizing to those affected by our mistakes,
-     and learning from the experience
- -   Focusing on what is best not just for us as individuals but for the overall
-     community
- 
- Examples of unacceptable behavior include:
- 
- -   The use of sexualized language or imagery, and sexual attention or advances
-     of any kind
- -   Trolling, insulting, or derogatory comments, and personal or political
-     attacks
- -   Public or private harassment
- -   Publishing others' private information, such as a physical or email address,
-     without their explicit permission
- -   Other conduct which could reasonably be considered inappropriate in a
-     professional setting
- 
- ## Enforcement Responsibilities
- 
- Community leaders are responsible for clarifying and enforcing our standards of
- acceptable behavior. They will take appropriate and fair corrective action in
- response to any behavior they deem inappropriate, threatening, offensive, or
- harmful. This may include removing, editing, or rejecting comments, commits,
- code, wiki edits, issues, and other contributions that do not align with this
- Code of Conduct. Community leaders will communicate reasons for moderation
- decisions when appropriate.
- 
- ## Scope
- 
- This Code of Conduct applies within all community spaces, and also applies when
- an individual is officially representing the community in public spaces.
- Examples of representing our community include using an official e-mail address,
- posting via an official social media account, or acting as an appointed
- representative at an online or offline event.
- 
- ## Enforcement
- 
- Instances of abusive, harassing, or otherwise unacceptable behavior may be
- reported to the community leaders responsible for enforcement at
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- promptly and fairly. All community leaders are obligated to respect the privacy
- and security of the reporter of any incident.
- 
- ## Enforcement Guidelines
- 
- Community leaders will follow these Community Impact Guidelines in determining
- the consequences for any action they deem in violation of this Code of Conduct:
- 
- ### 1. Correction
- 
- **Community Impact**: Use of inappropriate language or other behavior deemed
- unprofessional or unwelcome in the community.
- 
- **Consequence**: A private, written warning from community leaders, providing
- clarity around the nature of the violation and an explanation of why the
- behavior was inappropriate. A public apology may be requested.
- 
- ### 2. Warning
- 
- **Community Impact**: A violation through a single incident or series of
- actions.
- 
- **Consequence**: A warning with consequences for continued behavior. No
- interaction with the people involved, including unsolicited interaction with
- those enforcing the Code of Conduct, for a specified period of time. This
- includes avoiding interactions in community spaces as well as external channels
- like social media. Violating these terms may lead to a temporary or permanent
- ban.
- 
- ### 3. Temporary Ban
- 
- **Community Impact**: A serious violation of community standards, including
- sustained inappropriate behavior.
- 
- **Consequence**: A temporary ban from any sort of interaction or public
- communication with the community for a specified period of time. No public or
- private interaction with the people involved, including unsolicited interaction
- with those enforcing the Code of Conduct, is allowed during this period.
- Violating these terms may lead to a permanent ban.
- 
- ### 4. Permanent Ban
- 
- **Community Impact**: Demonstrating a pattern of violation of community
- standards, including sustained inappropriate behavior, harassment of an
- individual, or aggression toward or disparagement of classes of individuals.
- 
- **Consequence**: A permanent ban from any sort of public interaction within the
- community.
- 
- ## Attribution
- 
- This Code of Conduct is adapted from the [Contributor Covenant][homepage],
- version 2.1, available at
- [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
- Community Impact Guidelines were inspired by [Mozilla's code of conduct
- enforcement ladder][Mozilla CoC].
- 
- For answers to common questions about this code of conduct, see the FAQ at
- [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
- [https://www.contributor-covenant.org/translations][translations].
- 
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
- 
- Thank you for being part of our community and helping us create a safe and
- respectful environment for everyone!
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
deleted file mode 100644
index c740185..0000000
--- a/CONTRIBUTING.md
+++ /dev/null
- # Contributing Guidelines
- 
- Welcome to our community! We are committed to creating a welcoming and inclusive
- environment for all contributors. Before you get started, please read and adhere
- to the following code of conduct. By participating in our community, you agree
- to abide by these guidelines.
- 
- ## Our Pledge
- 
- We, as members, contributors, and leaders, pledge to make participation in our
- community a harassment-free experience for everyone, regardless of age, body
- size, visible or invisible disability, ethnicity, sex characteristics, gender
- identity and expression, level of experience, education, socio-economic status,
- nationality, personal appearance, race, caste, color, religion, or sexual
- identity and orientation. We pledge to act and interact in ways that contribute
- to an open, welcoming, diverse, inclusive, and healthy community.
- 
- ## Our Standards
- 
- Examples of behavior that contributes to a positive environment for our
- community include:
- 
- -   Demonstrating empathy and kindness toward other people
- -   Being respectful of differing opinions, viewpoints, and experiences
- -   Giving and gracefully accepting constructive feedback
- -   Accepting responsibility and apologizing to those affected by our mistakes,
-     and learning from the experience
- -   Focusing on what is best not just for us as individuals, but for the overall
-     community
- 
- Examples of unacceptable behavior include:
- 
- -   The use of sexualized language or imagery, and sexual attention or advances
-     of any kind
- -   Trolling, insulting, or derogatory comments, and personal or political
-     attacks
- -   Public or private harassment
- -   Publishing others' private information, such as a physical or email address,
-     without their explicit permission
- -   Other conduct which could reasonably be considered inappropriate in a
-     professional setting
- 
- ## Enforcement Responsibilities
- 
- Community leaders are responsible for clarifying and enforcing our standards of
- acceptable behavior and will take appropriate and fair corrective action in
- response to any behavior that they deem inappropriate, threatening, offensive,
- or harmful. Community leaders have the right and responsibility to remove, edit,
- or reject comments, commits, code, wiki edits, issues, and other contributions
- that are not aligned with this Code of Conduct, and will communicate reasons for
- moderation decisions when appropriate.
- 
- ## Scope
- 
- This Code of Conduct applies within all community spaces, and also applies when
- an individual is officially representing the community in public spaces.
- Examples of representing our community include using an official e-mail address,
- posting via an official social media account, or acting as an appointed
- representative at an online or offline event.
- 
- ## Enforcement
- 
- Instances of abusive, harassing, or otherwise unacceptable behavior may be
- reported to the community leaders responsible for enforcement at
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
- promptly and fairly. All community leaders are obligated to respect the privacy
- and security of the reporter of any incident.
- 
- ## Enforcement Guidelines
- 
- Community leaders will follow these Community Impact Guidelines in determining
- the consequences for any action they deem in violation of this Code of Conduct:
- 
- ### 1. Correction
- 
- **Community Impact**: Use of inappropriate language or other behavior deemed
- unprofessional or unwelcome in the community.
- 
- **Consequence**: A private, written warning from community leaders, providing
- clarity around the nature of the violation and an explanation of why the
- behavior was inappropriate. A public apology may be requested.
- 
- ### 2. Warning
- 
- **Community Impact**: A violation through a single incident or series of
- actions.
- 
- **Consequence**: A warning with consequences for continued behavior. No
- interaction with the people involved, including unsolicited interaction with
- those enforcing the Code of Conduct, for a specified period of time. This
- includes avoiding interactions in community spaces as well as external channels
- like social media. Violating these terms may lead to a temporary or permanent
- ban.
- 
- ### 3. Temporary Ban
- 
- **Community Impact**: A serious violation of community standards, including
- sustained inappropriate behavior.
- 
- **Consequence**: A temporary ban from any sort of interaction or public
- communication with the community for a specified period of time. No public or
- private interaction with the people involved, including unsolicited interaction
- with those enforcing the Code of Conduct, is allowed during this period.
- Violating these terms may lead to a permanent ban.
- 
- ### 4. Permanent Ban
- 
- **Community Impact**: Demonstrating a pattern of violation of community
- standards, including sustained inappropriate behavior, harassment of an
- individual, or aggression toward or disparagement of classes of individuals.
- 
- **Consequence**: A permanent ban from any sort of public interaction within the
- community.
- 
- ## Attribution
- 
- This Code of Conduct is adapted from the [Contributor Covenant][homepage],
- version 2.1, available at
- [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
- Community Impact Guidelines were inspired by [Mozilla's code of conduct
- enforcement ladder][Mozilla CoC].
- 
- For answers to common questions about this code of conduct, see the FAQ at
- [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
- [https://www.contributor-covenant.org/translations][translations].
- 
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
- 
- Thank you for being part of our community and helping us create a safe and
- respectful environment for everyone!
diff --git a/LICENSE b/LICENSE
index c47b9fa..66f72b3 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2023-2024 Nikola R. Hristov
+ Copyright (c) 2022 Nikola Hristov
diff --git a/README.md b/README.md
index 26d3127..f51a3d6 100644
--- a/README.md
+++ b/README.md
- # 🍺 [INN2]
+ # [inn] 🍺
- INN2 is a command-line tool designed to execute a specified command in all
- directories that match a certain pattern within a given root directory. It
- provides flexibility and efficiency in running commands across multiple
- directories with customizable patterns.
+ Inn is a shell script to execute commmands in multiple directories.
- [INN2]: https://crates.io/crates/inn2
- 
- ## Benchmark
- 
- <table>
- 	<tr>
- 		<th>Command:</th>
- 		<th>Time:</th>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git ls</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m9.441s
- user    0m0.030s
- sys     0m0.046s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m14.293s +5s
- user    0m4.645s +4s
- sys     0m8.937s +8s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git git status</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m24.146s
- user    0m0.030s
- sys     0m0.062s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m28.584s +4s
- user    0m4.695s +4s
- sys     0m8.354s +8s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>INN2 -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m33.813s
- user    0m0.015s
- sys     0m0.060s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir \
- bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m53.122s +20s
- user    0m9.449s +9s
- sys     0m14.442s +14s</pre>
- 		</td>
- 	</tr>
- </table>
+ [inn]: https://crates.io/crates/innkeeper
- cargo install inn2
+ cargo install innkeeper
- ## Usage
+ # Usage
- INN2 .git git fetch upstream
+ inn .git git fetch upstream
- This command will fetch from upstream for all the .git repositories inside the
- current directory. Essentially, it replaces the following shell command:
+ This will fetch from upstream for all the .git repositories inside the current
+ folder. Basically it replaces:
- To specify a `--File` argument or `-F`, if you would like to search for a file
- instead of a directory, use:
- 
- ```sh
- INN2 -F astro.config.ts npx astro add astro-compress
- ```
- 
- Additionally, you can provide a `--Root` argument or `-R` to set the current
- working directory to a different folder. The default is `.`.
+ You can also provide a `--root` argument or `-r` which sets the current working
+ directory to a different folder.
- INN2 -R D:\Developer .git git fetch upstream
+ inn -r D:\Developer .git git fetch upstream
- 
- Specify a `--Parallel` argument or `-P` if you would like to run commands in
- parallel. The default is sequential.
- 
- ```sh
- INN2 -P -R D:\Developer .git git fetch upstream
- ```
- 
- ## Dependencies
- 
- The code imports several crates:
- 
- -   `clap` - For parsing command-line arguments.
- -   `tokio` - Enables parallel execution of tasks.
- -   `walkdir` - Facilitates filesystem traversal.
- 
- ## Changelog
- 
- See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
deleted file mode 100644
index d9d47b8..0000000
--- a/Source/Fn/Binary/Command.rs
+++ /dev/null
- pub mod Entry;
- pub mod Parallel;
- pub mod Sequential;
- 
- pub fn Fn() -> ArgMatches {
- 	Command::new("Innkeeper")
- 		.version(env!("CARGO_PKG_VERSION"))
- 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("File")
- 				.short('F')
- 				.long("File")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("Parallel")
- 				.short('P')
- 				.long("Parallel")
- 				.action(SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("Root")
- 				.short('R')
- 				.long("Root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Exclude")
- 				.short('E')
- 				.long("Exclude")
- 				.display_order(4)
- 				.value_name("EXCLUDE")
- 				.required(false)
- 				.help("Exclude pattern.")
- 				.default_value("node_modules .git target dist vendor"),
- 		)
- 		.arg(
- 			Arg::new("Pattern")
- 				.display_order(5)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Command")
- 				.num_args(0..=10)
- 				.display_order(6)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches()
- }
- 
- use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
deleted file mode 100644
index bf4ce76..0000000
--- a/Source/Fn/Binary/Command/Entry.rs
+++ /dev/null
- pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
- 	WalkDir::new(Root)
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			let Path = Entry.expect("Cannot Entry.").path().display().to_string();
- 
- 			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
- 				let Match = Path.contains(&Exclude);
- 
- 				match File {
- 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
- 					false => Match,
- 				}
- 			}) {
- 				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
- 			} else {
- 				None
- 			}
- 		})
- 		.collect::<Vec<_>>()
- }
- 
- use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};
- 
- use walkdir::WalkDir;
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
deleted file mode 100644
index efd9808..0000000
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ /dev/null
- pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	let mut Queue = Vec::new();
- 
- 	Entry
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			Entry
- 				.last()
- 				.filter(|Last| *Last == &Pattern)
- 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
- 		})
- 		.for_each(|Entry| {
- 			let Output = if cfg!(target_os = "windows") {
- 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
- 			} else {
- 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
- 			};
- 
- 			Queue.push(async move {
- 				println!(
- 					"{}",
- 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
- 				);
- 			});
- 		});
- 
- 	tokio::runtime::Builder::new_multi_thread()
- 		.enable_all()
- 		.build()
- 		.expect("Cannot Runtime.")
- 		.block_on(async {
- 			for Queue in Queue {
- 				Queue.await;
- 			}
- 		})
- }
- 
- use crate::Struct::Binary::Command::Entry::Struct as Option;
- use tokio::process::Command;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
deleted file mode 100644
index 8eef18b..0000000
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ /dev/null
- pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
- 	Entry
- 		.into_iter()
- 		.filter_map(|Entry| {
- 			Entry
- 				.last()
- 				.filter(|Last| *Last == &Pattern)
- 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
- 		})
- 		.for_each(|Entry| {
- 			let mut Out = match cfg!(target_os = "windows") {
- 				true => Command::new("cmd")
- 					.args(["/C", &Command])
- 					.current_dir(Entry)
- 					.stdout(Stdio::piped())
- 					.spawn()
- 					.expect("Cannot spawn."),
- 				false => Command::new("sh")
- 					.arg("-c")
- 					.current_dir(Entry)
- 					.arg(Command.clone())
- 					.stdout(Stdio::piped())
- 					.spawn()
- 					.expect("Cannot spawn."),
- 			}
- 			.stdout
- 			.expect("Cannot stdout.");
- 
- 			let mut Output = String::new();
- 
- 			loop {
- 				let mut Buffer = [0; 512];
- 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");
- 
- 				if Byte == 0 {
- 					break;
- 				}
- 
- 				Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
- 			}
- 
- 			println!("{}", Output);
- 		})
- }
- 
- use crate::Struct::Binary::Command::Entry::Struct as Option;
- 
- use std::{
- 	io::Read,
- 	process::{Command, Stdio},
- };
diff --git a/Source/Fn/Binary/mod.rs b/Source/Fn/Binary/mod.rs
deleted file mode 100644
index 9da7843..0000000
--- a/Source/Fn/Binary/mod.rs
+++ /dev/null
- pub mod Command;
diff --git a/Source/Fn/mod.rs b/Source/Fn/mod.rs
deleted file mode 100644
index a56e8ce..0000000
--- a/Source/Fn/mod.rs
+++ /dev/null
- pub mod Binary;
diff --git a/Source/Library.rs b/Source/Library.rs
deleted file mode 100644
index 982f1a3..0000000
--- a/Source/Library.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Fn;
- mod Struct;
- 
- #[allow(dead_code)]
- fn main() {
- 	(Struct::Binary::Command::Struct::Fn().Fn)()
- }
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
deleted file mode 100644
index fdaa4e3..0000000
--- a/Source/Struct/Binary/Command.rs
+++ /dev/null
- pub mod Entry;
- pub mod Option;
- 
- use crate::Fn::Binary::Command::{Parallel, Sequential};
- 
- #[derive(Debug)]
- pub struct Struct {
- 	pub Separator: Option::Separator,
- 	pub Fn: fn(),
- }
- 
- impl Struct {
- 	pub fn Fn() -> Self {
- 		Self {
- 			Separator: std::path::MAIN_SEPARATOR,
- 			Fn: || {
- 				let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));
- 				
- 				match Option.Parallel {
- 					true => {
- 						Parallel::Fn(Option);
- 					}
- 					false => {
- 						Sequential::Fn(Option);
- 					}
- 				};
- 			},
- 		}
- 	}
- }
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
deleted file mode 100644
index 5b62589..0000000
--- a/Source/Struct/Binary/Command/Entry.rs
+++ /dev/null
- pub type Type = Vec<Vec<String>>;
- 
- pub struct Struct {
- 	pub Command: Command,
- 	pub Entry: Type,
- 	pub Parallel: Parallel,
- 	pub Pattern: Pattern,
- 	pub Separator: Separator,
- }
- 
- impl Struct {
- 	pub fn Fn(Option: &Option) -> Self {
- 		Self {
- 			Command: Option.Command.clone(),
- 			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
- 			Parallel: Option.Parallel,
- 			Pattern: Option.Pattern.clone(),
- 			Separator: Option.Separator,
- 		}
- 	}
- }
- 
- use crate::Struct::Binary::Command::Option::{
- 	Command, Parallel, Pattern, Separator, Struct as Option,
- };
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
deleted file mode 100644
index 4c946ac..0000000
--- a/Source/Struct/Binary/Command/Option.rs
+++ /dev/null
- pub type Command = String;
- pub type Parallel = bool;
- pub type Pattern = String;
- pub type Separator = char;
- 
- pub struct Struct {
- 	pub Command: String,
- 	pub Exclude: Vec<String>,
- 	pub File: bool,
- 	pub Parallel: bool,
- 	pub Pattern: Pattern,
- 	pub Root: String,
- 	pub Separator: Separator,
- }
- 
- impl Struct {
- 	pub fn Fn(Option { Separator, .. }: Option) -> Self {
- 		Self {
- 			File: Fn().get_flag("File"),
- 			Parallel: Fn().get_flag("Parallel"),
- 			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
- 			Exclude: Fn()
- 				.get_one::<String>("Exclude")
- 				.expect("Cannot Exclude.")
- 				.split(" ")
- 				.map(|Command| Command.to_string())
- 				.collect::<Vec<_>>(),
- 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
- 			Command: Fn()
- 				.get_many::<String>("Command")
- 				.expect("Cannot Command.")
- 				.map(|Command| Command.as_str())
- 				.collect::<Vec<_>>()
- 				.join(" "),
- 			Separator,
- 		}
- 	}
- }
- 
- use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
diff --git a/Source/Struct/Binary/mod.rs b/Source/Struct/Binary/mod.rs
deleted file mode 100644
index 9da7843..0000000
--- a/Source/Struct/Binary/mod.rs
+++ /dev/null
- pub mod Command;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
deleted file mode 100644
index a56e8ce..0000000
--- a/Source/Struct/mod.rs
+++ /dev/null
- pub mod Binary;
diff --git a/src/main.rs b/src/main.rs
new file mode 100644
index 0000000..9a36f7f
--- /dev/null
+++ b/src/main.rs
+ extern crate clap;
+ extern crate walkdir;
+ 
+ use std::{fs, process::Command};
+ 
+ use clap::{Arg, Command as ClapCommand};
+ use walkdir::WalkDir;
+ 
+ fn main() {
+ 	let matches = ClapCommand::new("Innkeeper")
+ 		.version("0.0.2")
+ 		.about("Runs a command in all directories having a certain folder.")
+ 		.arg(
+ 			Arg::new("root")
+ 				.short('r')
+ 				.display_order(1)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("folder")
+ 				.display_order(2)
+ 				.value_name("FOLDER")
+ 				.required(true)
+ 				.help("Search folder.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("command")
+ 				.num_args(0..=10)
+ 				.display_order(3)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches();
+ 
+ 	let folder = matches.get_one::<String>("folder").unwrap();
+ 	let command = &matches
+ 		.get_many::<String>("command")
+ 		.unwrap_or_default()
+ 		.map(|v| v.as_str())
+ 		.collect::<Vec<_>>()
+ 		.join(" ");
+ 	let directory = matches.get_one::<String>("root").unwrap();
+ 
+ 	let ds = std::path::MAIN_SEPARATOR;
+ 
+ 	for entry in WalkDir::new(directory).into_iter().filter_entry(|e| {
+ 		fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir()
+ 			&& (!e.path().display().to_string().contains("node_modules")
+ 				|| !folder.contains("node_modules"))
+ 	}) {
+ 		let entry_dir = entry.unwrap().path().display().to_string();
+ 		let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 		if let Some(last) = paths.last() {
+ 			if last == folder {
+ 				let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
+ 
+ 				println!("Executing {} for every {} in {}", command, last, directory);
+ 
+ 				let output = match cfg!(target_os = "windows") {
+ 					true => Command::new("cmd")
+ 						.args(["/C", command])
+ 						.current_dir(working_directory)
+ 						.output()
+ 						.expect("Failed to execute process."),
+ 					false => Command::new("sh")
+ 						.arg("-c")
+ 						.current_dir(working_directory)
+ 						.arg(command)
+ 						.output()
+ 						.expect("Failed to execute process."),
+ 				};
+ 
+ 				println!("{}", String::from_utf8_lossy(&output.stdout));
+ 			}
+ 		}
+ 	}
+ }

🗣️ Summary from v0.0.3 to v0.0.4 in .
diff --git a/Cargo.toml b/Cargo.toml
index ed01669..471b63e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.3"
+ version = "0.0.4"
+ rayon = { version = "1.6.1" }
+ crossbeam = { version = "0.8.2" }
diff --git a/README.md b/README.md
index f51a3d6..ae326c5 100644
--- a/README.md
+++ b/README.md
- # Usage
+ ## Usage
- folder. Basically it replaces:
+ directory. Basically it replaces:
+ 
+ Specify a `--parallel` argument or `-p` if you would like to run functions in
+ parallel.
+ 
+ ```sh
+ inn -p -r D:\Developer .git git fetch upstream
+ ```
+ 
+ ## Changelog
+ 
+ See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
diff --git a/src/main.rs b/src/main.rs
index 9a36f7f..13fc651 100644
--- a/src/main.rs
+++ b/src/main.rs
+ extern crate crossbeam;
+ extern crate rayon;
+ use clap::{Arg, ArgAction, Command as ClapCommand};
+ use crossbeam::scope;
+ use rayon::prelude::*;
- 
- use clap::{Arg, Command as ClapCommand};
- 		.version("0.0.2")
+ 		.version("0.0.4")
+ 		.arg(
+ 			Arg::new("parallel")
+ 				.short('p')
+ 				.long("parallel")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(1)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
- 				.display_order(1)
+ 				.long("root")
+ 				.display_order(2)
- 				.display_order(2)
+ 				.display_order(3)
- 				.display_order(3)
+ 				.display_order(4)
+ 	let parallel = matches.get_flag("parallel");
+ 	let root = matches.get_one::<String>("root").unwrap();
- 	let directory = matches.get_one::<String>("root").unwrap();
- 	for entry in WalkDir::new(directory).into_iter().filter_entry(|e| {
+ 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
- 	}) {
+ 	});
+ 
+ 	if parallel {
+ 		println!("Executing code in parallel.");
+ 
+ 		// Parallel
+ 		let dirs = entries
+ 			.map(|entry| {
+ 				let entry_dir = entry.unwrap().path().display().to_string();
+ 				let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 				match paths.last() {
+ 					Some(last) => {
+ 						if last == folder {
+ 							let working_directory =
+ 								&paths[0..paths.len() - 1].join(&ds.to_string());
+ 							Some(working_directory.to_owned())
+ 						} else {
+ 							None
+ 						}
+ 					}
+ 					None => None,
+ 				}
+ 			})
+ 			.filter_map(|x| x)
+ 			.collect::<Vec<String>>();
+ 
+ 		scope(|s| {
+ 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
+ 				scope.spawn(move |_| {
+ 					println!("Executing {} for every {} in {}", command, dir, root);
+ 
+ 					let output = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", command.as_str()])
+ 							.current_dir(dir)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(dir)
+ 							.arg(command)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					println!("{}", String::from_utf8_lossy(&output.stdout));
+ 				});
+ 			});
+ 		})
+ 		.unwrap();
+ 	} else {
+ 		println!("Executing code in sequential.");
+ 
+ 		// Sequential
+ 		for entry in entries {
- 				println!("Executing {} for every {} in {}", command, last, directory);
+ 					println!("Executing {} for every {} in {}", command, last, root);
+ }

🗣️ Summary from v0.0.4 to v0.0.5 in .
diff --git a/.github/workflows/rust.yml b/.github/workflows/rust.yml
index eeb1562..5ec1f06 100644
--- a/.github/workflows/rust.yml
+++ b/.github/workflows/rust.yml
-                   key: ${{ runner.os }}-cargo-${{ hash('./Cargo.toml') }}
+                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
diff --git a/Cargo.toml b/Cargo.toml
index 471b63e..a31d6e3 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.4"
- description = "Inn is a shell script that allows you to execute commmands in different directories."
+ version = "0.0.5"
+ description = "Inn is a CLI that allows you to execute commmands in different directories."
+ [lib]
+ name = "inn"
+ path = "src/lib/main.rs"
+ 
- path = "src/main.rs"
+ path = "src/bin/inn.rs"
+ 
+ [[bin]]
+ name = "innkeeper"
+ path = "src/bin/innkeeper.rs"
diff --git a/README.md b/README.md
index ae326c5..99e2514 100644
--- a/README.md
+++ b/README.md
- Inn is a shell script to execute commmands in multiple directories.
+ Inn is a CLI that allows you to execute commmands in different directories.
- This will fetch from upstream for all the .git repositories inside the current
+ This will fetch from upstream for all the `.git` repositories inside the current
+ Specify a `--file` argument or `-f` if you would like to search for file instead
+ of a directory. Default is `false` or no flag at all.
+ 
+ ```sh
+ inn -f astro.config.ts npx astro add astro-compress
+ ```
+ 
- directory to a different folder.
+ directory to a different folder. Default is `.`.
- parallel.
+ parallel. Default is sequential.
diff --git a/src/bin/inn.rs b/src/bin/inn.rs
new file mode 100644
index 0000000..e2fe709
--- /dev/null
+++ b/src/bin/inn.rs
+ fn main() {
+ 	inn::run()
+ }
diff --git a/src/bin/innkeeper.rs b/src/bin/innkeeper.rs
new file mode 100644
index 0000000..e2fe709
--- /dev/null
+++ b/src/bin/innkeeper.rs
+ fn main() {
+ 	inn::run()
+ }
diff --git a/src/lib/main.rs b/src/lib/main.rs
new file mode 100644
index 0000000..d5b6e76
--- /dev/null
+++ b/src/lib/main.rs
+ extern crate clap;
+ extern crate crossbeam;
+ extern crate rayon;
+ extern crate walkdir;
+ 
+ use clap::{Arg, ArgAction, Command as ClapCommand};
+ use crossbeam::scope;
+ use rayon::prelude::*;
+ use std::{fs, process::Command};
+ use walkdir::WalkDir;
+ 
+ pub fn run() {
+ 	let matches = ClapCommand::new("Innkeeper")
+ 		.version("0.0.5")
+ 		.about("Runs a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("file")
+ 				.short('f')
+ 				.long("file")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("parallel")
+ 				.short('p')
+ 				.long("parallel")
+ 				.action(ArgAction::SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("root")
+ 				.short('r')
+ 				.long("root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("pattern")
+ 				.display_order(4)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("command")
+ 				.num_args(0..=10)
+ 				.display_order(5)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches();
+ 
+ 	let file = matches.get_flag("file");
+ 	let parallel = matches.get_flag("parallel");
+ 	let root = matches.get_one::<String>("root").unwrap();
+ 	let pattern = matches.get_one::<String>("pattern").unwrap();
+ 	let command = &matches
+ 		.get_many::<String>("command")
+ 		.unwrap_or_default()
+ 		.map(|v| v.as_str())
+ 		.collect::<Vec<_>>()
+ 		.join(" ");
+ 
+ 	let ds = std::path::MAIN_SEPARATOR;
+ 
+ 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
+ 		let is_node = !e.path().display().to_string().contains("node_modules")
+ 			|| !pattern.contains("node_modules");
+ 
+ 		if !file {
+ 			fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir() && is_node
+ 		} else {
+ 			is_node
+ 		}
+ 	});
+ 
+ 	if parallel {
+ 		println!("Executing code in parallel.");
+ 
+ 		// Parallel
+ 		let dirs = entries
+ 			.map(|entry| {
+ 				let entry_dir = entry.unwrap().path().display().to_string();
+ 				let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 				match paths.last() {
+ 					Some(last) => {
+ 						if last == pattern {
+ 							let working_directory =
+ 								&paths[0..paths.len() - 1].join(&ds.to_string());
+ 							Some(working_directory.to_owned())
+ 						} else {
+ 							None
+ 						}
+ 					}
+ 					None => None,
+ 				}
+ 			})
+ 			.filter_map(|x| x)
+ 			.collect::<Vec<String>>();
+ 
+ 		scope(|s| {
+ 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
+ 				scope.spawn(move |_| {
+ 					println!("Executing {} for every {} in {}", command, dir, root);
+ 
+ 					let output = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", command.as_str()])
+ 							.current_dir(dir)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(dir)
+ 							.arg(command)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					println!("{}", String::from_utf8_lossy(&output.stdout));
+ 				});
+ 			});
+ 		})
+ 		.unwrap();
+ 	} else {
+ 		println!("Executing code in sequential.");
+ 
+ 		// Sequential
+ 		for entry in entries {
+ 			let entry_dir = entry.unwrap().path().display().to_string();
+ 			let paths: Vec<&str> = entry_dir.split(ds).collect();
+ 
+ 			if let Some(last) = paths.last() {
+ 				if last == pattern {
+ 					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
+ 
+ 					println!("Executing {} for every {} in {}", command, last, root);
+ 
+ 					let output = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", command])
+ 							.current_dir(working_directory)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(working_directory)
+ 							.arg(command)
+ 							.output()
+ 							.expect("Failed to execute process."),
+ 					};
+ 
+ 					println!("{}", String::from_utf8_lossy(&output.stdout));
+ 				}
+ 			}
+ 		}
+ 	}
+ }
diff --git a/src/main.rs b/src/main.rs
deleted file mode 100644
index 13fc651..0000000
--- a/src/main.rs
+++ /dev/null
- extern crate clap;
- extern crate crossbeam;
- extern crate rayon;
- extern crate walkdir;
- 
- use clap::{Arg, ArgAction, Command as ClapCommand};
- use crossbeam::scope;
- use rayon::prelude::*;
- use std::{fs, process::Command};
- use walkdir::WalkDir;
- 
- fn main() {
- 	let matches = ClapCommand::new("Innkeeper")
- 		.version("0.0.4")
- 		.about("Runs a command in all directories having a certain folder.")
- 		.arg(
- 			Arg::new("parallel")
- 				.short('p')
- 				.long("parallel")
- 				.action(ArgAction::SetTrue)
- 				.display_order(1)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("root")
- 				.short('r')
- 				.long("root")
- 				.display_order(2)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("folder")
- 				.display_order(3)
- 				.value_name("FOLDER")
- 				.required(true)
- 				.help("Search folder.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("command")
- 				.num_args(0..=10)
- 				.display_order(4)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches();
- 
- 	let parallel = matches.get_flag("parallel");
- 	let root = matches.get_one::<String>("root").unwrap();
- 	let folder = matches.get_one::<String>("folder").unwrap();
- 	let command = &matches
- 		.get_many::<String>("command")
- 		.unwrap_or_default()
- 		.map(|v| v.as_str())
- 		.collect::<Vec<_>>()
- 		.join(" ");
- 
- 	let ds = std::path::MAIN_SEPARATOR;
- 
- 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
- 		fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir()
- 			&& (!e.path().display().to_string().contains("node_modules")
- 				|| !folder.contains("node_modules"))
- 	});
- 
- 	if parallel {
- 		println!("Executing code in parallel.");
- 
- 		// Parallel
- 		let dirs = entries
- 			.map(|entry| {
- 				let entry_dir = entry.unwrap().path().display().to_string();
- 				let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 				match paths.last() {
- 					Some(last) => {
- 						if last == folder {
- 							let working_directory =
- 								&paths[0..paths.len() - 1].join(&ds.to_string());
- 							Some(working_directory.to_owned())
- 						} else {
- 							None
- 						}
- 					}
- 					None => None,
- 				}
- 			})
- 			.filter_map(|x| x)
- 			.collect::<Vec<String>>();
- 
- 		scope(|s| {
- 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
- 				scope.spawn(move |_| {
- 					println!("Executing {} for every {} in {}", command, dir, root);
- 
- 					let output = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command.as_str()])
- 							.current_dir(dir)
- 							.output()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(dir)
- 							.arg(command)
- 							.output()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					println!("{}", String::from_utf8_lossy(&output.stdout));
- 				});
- 			});
- 		})
- 		.unwrap();
- 	} else {
- 		println!("Executing code in sequential.");
- 
- 		// Sequential
- 		for entry in entries {
- 			let entry_dir = entry.unwrap().path().display().to_string();
- 			let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 			if let Some(last) = paths.last() {
- 				if last == folder {
- 					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
- 
- 					println!("Executing {} for every {} in {}", command, last, root);
- 
- 					let output = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command])
- 							.current_dir(working_directory)
- 							.output()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(working_directory)
- 							.arg(command)
- 							.output()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					println!("{}", String::from_utf8_lossy(&output.stdout));
- 				}
- 			}
- 		}
- 	}
- }

🗣️ Summary from v0.0.5 to v0.0.6 in .
diff --git a/Cargo.toml b/Cargo.toml
index a31d6e3..90ff5b4 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.5"
- description = "Inn is a CLI that allows you to execute commmands in different directories."
+ version = "0.0.6"
+ description = "🍺 Inn is a little Rust utility that lets execute commmands in multiple directories."
- rayon = { version = "1.6.1" }
+ rayon = { version = "1.7.0" }
diff --git a/README.md b/README.md
index 99e2514..70a5250 100644
--- a/README.md
+++ b/README.md
- Inn is a CLI that allows you to execute commmands in different directories.
+ Inn is a tiny Rust CLI that lets execute commmands in multiple
+ directories.
diff --git a/src/lib/main.rs b/src/lib/main.rs
index d5b6e76..bd14eb2 100644
--- a/src/lib/main.rs
+++ b/src/lib/main.rs
- use std::{fs, process::Command};
+ use std::{
+ 	fs,
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
- 		.version("0.0.5")
+ 		.version("0.0.6")
- 					let output = match cfg!(target_os = "windows") {
+ 					let child = match cfg!(target_os = "windows") {
- 							.output()
+ 							.stdout(Stdio::piped())
+ 							.spawn()
- 							.output()
+ 							.stdout(Stdio::piped())
+ 							.spawn()
- 					println!("{}", String::from_utf8_lossy(&output.stdout));
+ 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
+ 
+ 					let mut output = String::new();
+ 
+ 					loop {
+ 						let mut buffer = [0; 512];
+ 						let bytes_read =
+ 							stdout.read(&mut buffer).expect("Failed to read from pipe");
+ 
+ 						if bytes_read == 0 {
+ 							break;
+ 						}
+ 
+ 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
+ 					}
+ 
+ 					println!("{}", output);

🗣️ Summary from v0.0.6 to v0.0.7 in .
diff --git a/.github/workflows/dependabot.yml b/.github/workflows/dependabot.yml
index 254bb2f..fb49691 100644
--- a/.github/workflows/dependabot.yml
+++ b/.github/workflows/dependabot.yml
-             - uses: dependabot/fetch-metadata@v1.3.6
+             - uses: dependabot/fetch-metadata@v1.6.0
diff --git a/.github/workflows/inn.yml b/.github/workflows/inn.yml
new file mode 100644
index 0000000..39a3ba6
--- /dev/null
+++ b/.github/workflows/inn.yml
+ name: Inn
+ 
+ concurrency:
+     group: inn-${{ github.workflow }}-${{ github.ref }}
+     cancel-in-progress: true
+ 
+ on:
+     workflow_dispatch:
+     push:
+         branches: [main]
+     pull_request:
+         branches: [main]
+     workflow_call:
+ 
+ jobs:
+     build:
+         strategy:
+             matrix:
+                 toolchain: [stable, nightly]
+                 platform: [windows-latest, ubuntu-latest, macos-latest]
+ 
+         runs-on: ${{ matrix.platform }}
+         continue-on-error: true
+ 
+         steps:
+             - uses: actions/checkout@v3.5.3
+             - uses: actions-rs/toolchain@v1.0.7
+               with:
+                   profile: minimal
+                   toolchain: ${{ matrix.toolchain }}
+ 
+             - uses: actions/cache@v3.3.1
+               with:
+                   path: |
+                       ~/.cargo/bin/
+                       ~/.cargo/registry/index/
+                       ~/.cargo/registry/cache/
+                       ~/.cargo/git/db/
+                       target/
+                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
+             - uses: actions-rs/cargo@v1.0.3
+               with:
+                   command: build
+                   args: --release --all-features --manifest-path ./Cargo.toml
+ 
+             - uses: actions/upload-artifact@v3.1.2
+               with:
+                   name: .-inn-${{ matrix.platform }}-target
+                   path: ./target
+ 
+             - run: |
+                   git add target
+                   git config --global user.name 'Inn'
+                   git config --global user.email 'inn@lightrix.help'
+                   git commit -m "Inn Release $(git log -1 --pretty=%h) $(git log -1 --pretty=%B)"
+                   git pull && git push
diff --git a/.github/workflows/rust.yml b/.github/workflows/rust.yml
index 5ec1f06..404bdc0 100644
--- a/.github/workflows/rust.yml
+++ b/.github/workflows/rust.yml
+             TELEMETRY_DISABLED: 1
-             - uses: actions/checkout@v3.3.0
+             - uses: actions/checkout@v3.5.3
-             - uses: actions/cache@v3.2.6
+             - uses: actions/cache@v3.3.1
diff --git a/.gitignore b/.gitignore
index 96ef6c0..e3e986b 100644
--- a/.gitignore
+++ b/.gitignore
- /target
+ 
+ /target/*
+ !/target/release
+ /target/release/*
+ !/target/release/*.deb
+ !/target/release/*.exe
+ !/target/release/inn
+ !/target/release/innkeeper
diff --git a/Cargo.toml b/Cargo.toml
index 90ff5b4..5bfe36f 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.6"
- description = "🍺 Inn is a little Rust utility that lets execute commmands in multiple directories."
+ version = "0.0.7"
+ description = "🍺 Inn is a little Rust utility that lets execute commands in multiple directories."
+ default-run = "inn"
+ repository = "https://github.com/Lightrix/innkeeper.git"
- clap = { version = "4.1.8", features = ["derive"] }
- walkdir = { version = "2.3.2" }
+ clap = { version = "4.3.17", features = ["derive"] }
+ walkdir = { version = "2.3.3" }
diff --git a/README.md b/README.md
index 70a5250..781899a 100644
--- a/README.md
+++ b/README.md
- Inn is a tiny Rust CLI that lets execute commmands in multiple
- directories.
+ Inn is a tiny Rust utlity that lets execute commmands in multiple directories.
- inn -r D:\Developer .git git fetch upstream
+ inn -r F:\Developer .git git fetch upstream
- inn -p -r D:\Developer .git git fetch upstream
+ inn -p -r F:\Developer .git git fetch upstream
diff --git a/src/lib/main.rs b/src/lib/main.rs
index bd14eb2..7f6fcb9 100644
--- a/src/lib/main.rs
+++ b/src/lib/main.rs
- 		let is_node = !e.path().display().to_string().contains("node_modules")
- 			|| !pattern.contains("node_modules");
+ 		if !pattern.contains("node_modules") {
+ 			return e.path().display().to_string().contains("node_modules");
+ 		}
- 			fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir() && is_node
+ 			println!("{:?}", e.path().display().to_string().contains("node_modules"));
+ 			return fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir();
- 			is_node
+ 			return true;

🗣️ Summary from v0.0.7 to v0.0.8 in .
diff --git a/Cargo.toml b/Cargo.toml
index 5bfe36f..a45237f 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.7"
+ version = "0.0.8"
- clap = { version = "4.3.17", features = ["derive"] }
+ clap = { version = "4.3.19", features = ["derive"] }
+ toml = "0.7.6"
diff --git a/src/lib/main.rs b/src/lib/main.rs
index 7f6fcb9..a179931 100644
--- a/src/lib/main.rs
+++ b/src/lib/main.rs
- 		.version("0.0.6")
+ 		.version("0.0.8")

🗣️ Summary from v0.0.8 to v0.0.9 in .
diff --git a/.github/workflows/cargo.yml b/.github/workflows/cargo.yml
new file mode 100644
index 0000000..1e413b7
--- /dev/null
+++ b/.github/workflows/cargo.yml
+ name: Cargo
+ 
+ concurrency:
+     group: cargo-${{ github.workflow }}-${{ github.ref }}
+     cancel-in-progress: true
+ 
+ permissions:
+     security-events: write
+ 
+ on:
+     workflow_dispatch:
+     release:
+         types: [created]
+     workflow_call:
+ 
+ jobs:
+     publish:
+         runs-on: ubuntu-latest
+         permissions:
+             contents: read
+             id-token: write
+ 
+         steps:
+             - uses: actions/checkout@v3.5.3
+             - uses: actions-rs/toolchain@v1.0.7
+               with:
+                   profile: minimal
+                   toolchain: stable
+ 
+             - uses: actions/cache@v3.3.1
+               with:
+                   path: |
+                       ~/.cargo/bin/
+                       ~/.cargo/registry/index/
+                       ~/.cargo/registry/cache/
+                       ~/.cargo/git/db/
+                       target/
+                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
+ 
+             - uses: actions-rs/cargo@v1.0.3
+               with:
+                   command: build
+                   args: --release --all-features --manifest-path ./Cargo.toml
+ 
+             - name: Publish ./
+               uses: actions-rs/cargo@v1.0.3
+               continue-on-error: true
+               with:
+                   command: publish
+                   args: --manifest-path ./Cargo.toml
+               env:
+                   CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
diff --git a/Cargo.toml b/Cargo.toml
index a45237f..a3c1765 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.8"
+ version = "0.0.9"
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
new file mode 100644
index 0000000..73fa540
--- /dev/null
+++ b/CONTRIBUTING.md
+ # Contributing Guidelines
+ 
+ Welcome to our community! We are committed to creating a welcoming and inclusive
+ environment for all contributors. Before you get started, please read and adhere
+ to the following code of conduct. By participating in our community, you agree
+ to abide by these guidelines.
+ 
+ ## Our Pledge
+ 
+ We, as members, contributors, and leaders, pledge to make participation in our
+ community a harassment-free experience for everyone, regardless of age, body
+ size, visible or invisible disability, ethnicity, sex characteristics, gender
+ identity and expression, level of experience, education, socio-economic status,
+ nationality, personal appearance, race, caste, color, religion, or sexual
+ identity and orientation. We pledge to act and interact in ways that contribute
+ to an open, welcoming, diverse, inclusive, and healthy community.
+ 
+ ## Our Standards
+ 
+ Examples of behavior that contributes to a positive environment for our
+ community include:
+ 
+ -   Demonstrating empathy and kindness toward other people
+ -   Being respectful of differing opinions, viewpoints, and experiences
+ -   Giving and gracefully accepting constructive feedback
+ -   Accepting responsibility and apologizing to those affected by our mistakes,
+     and learning from the experience
+ -   Focusing on what is best not just for us as individuals, but for the overall
+     community
+ 
+ Examples of unacceptable behavior include:
+ 
+ -   The use of sexualized language or imagery, and sexual attention or advances
+     of any kind
+ -   Trolling, insulting, or derogatory comments, and personal or political
+     attacks
+ -   Public or private harassment
+ -   Publishing others' private information, such as a physical or email address,
+     without their explicit permission
+ -   Other conduct which could reasonably be considered inappropriate in a
+     professional setting
+ 
+ ## Enforcement Responsibilities
+ 
+ Community leaders are responsible for clarifying and enforcing our standards of
+ acceptable behavior and will take appropriate and fair corrective action in
+ response to any behavior that they deem inappropriate, threatening, offensive,
+ or harmful. Community leaders have the right and responsibility to remove, edit,
+ or reject comments, commits, code, wiki edits, issues, and other contributions
+ that are not aligned with this Code of Conduct, and will communicate reasons for
+ moderation decisions when appropriate.
+ 
+ ## Scope
+ 
+ This Code of Conduct applies within all community spaces, and also applies when
+ an individual is officially representing the community in public spaces.
+ Examples of representing our community include using an official e-mail address,
+ posting via an official social media account, or acting as an appointed
+ representative at an online or offline event.
+ 
+ ## Enforcement
+ 
+ Instances of abusive, harassing, or otherwise unacceptable behavior may be
+ reported to the community leaders responsible for enforcement at [INSERT CONTACT
+ METHOD]. All complaints will be reviewed and investigated promptly and fairly.
+ All community leaders are obligated to respect the privacy and security of the
+ reporter of any incident.
+ 
+ ## Enforcement Guidelines
+ 
+ Community leaders will follow these Community Impact Guidelines in determining
+ the consequences for any action they deem in violation of this Code of Conduct:
+ 
+ ### 1. Correction
+ 
+ **Community Impact**: Use of inappropriate language or other behavior deemed
+ unprofessional or unwelcome in the community.
+ 
+ **Consequence**: A private, written warning from community leaders, providing
+ clarity around the nature of the violation and an explanation of why the
+ behavior was inappropriate. A public apology may be requested.
+ 
+ ### 2. Warning
+ 
+ **Community Impact**: A violation through a single incident or series of
+ actions.
+ 
+ **Consequence**: A warning with consequences for continued behavior. No
+ interaction with the people involved, including unsolicited interaction with
+ those enforcing the Code of Conduct, for a specified period of time. This
+ includes avoiding interactions in community spaces as well as external channels
+ like social media. Violating these terms may lead to a temporary or permanent
+ ban.
+ 
+ ### 3. Temporary Ban
+ 
+ **Community Impact**: A serious violation of community standards, including
+ sustained inappropriate behavior.
+ 
+ **Consequence**: A temporary ban from any sort of interaction or public
+ communication with the community for a specified period of time. No public or
+ private interaction with the people involved, including unsolicited interaction
+ with those enforcing the Code of Conduct, is allowed during this period.
+ Violating these terms may lead to a permanent ban.
+ 
+ ### 4. Permanent Ban
+ 
+ **Community Impact**: Demonstrating a pattern of violation of community
+ standards, including sustained inappropriate behavior, harassment of an
+ individual, or aggression toward or disparagement of classes of individuals.
+ 
+ **Consequence**: A permanent ban from any sort of public interaction within the
+ community.
+ 
+ ## Attribution
+ 
+ This Code of Conduct is adapted from the [Contributor Covenant][homepage],
+ version 2.1, available at
+ [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
+ Community Impact Guidelines were inspired by [Mozilla's code of conduct
+ enforcement ladder][Mozilla CoC].
+ 
+ For answers to common questions about this code of conduct, see the FAQ at
+ [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
+ [https://www.contributor-covenant.org/translations][translations].
+ 
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
+ 
+ Thank you for being part of our community and helping us create a safe and
+ respectful environment for everyone!

🗣️ Summary from v0.0.9 to v0.1.0 in .
diff --git a/.github/dependabot.yml b/.github/dependabot.yml
index d4ca004..44227ad 100644
--- a/.github/dependabot.yml
+++ b/.github/dependabot.yml
+ enable-beta-ecosystems: true
+ 
diff --git a/.github/workflows/cargo.yml b/.github/workflows/cargo.yml
deleted file mode 100644
index 1e413b7..0000000
--- a/.github/workflows/cargo.yml
+++ /dev/null
- name: Cargo
- 
- concurrency:
-     group: cargo-${{ github.workflow }}-${{ github.ref }}
-     cancel-in-progress: true
- 
- permissions:
-     security-events: write
- 
- on:
-     workflow_dispatch:
-     release:
-         types: [created]
-     workflow_call:
- 
- jobs:
-     publish:
-         runs-on: ubuntu-latest
-         permissions:
-             contents: read
-             id-token: write
- 
-         steps:
-             - uses: actions/checkout@v3.5.3
-             - uses: actions-rs/toolchain@v1.0.7
-               with:
-                   profile: minimal
-                   toolchain: stable
- 
-             - uses: actions/cache@v3.3.1
-               with:
-                   path: |
-                       ~/.cargo/bin/
-                       ~/.cargo/registry/index/
-                       ~/.cargo/registry/cache/
-                       ~/.cargo/git/db/
-                       target/
-                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
- 
-             - uses: actions-rs/cargo@v1.0.3
-               with:
-                   command: build
-                   args: --release --all-features --manifest-path ./Cargo.toml
- 
-             - name: Publish ./
-               uses: actions-rs/cargo@v1.0.3
-               continue-on-error: true
-               with:
-                   command: publish
-                   args: --manifest-path ./Cargo.toml
-               env:
-                   CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
diff --git a/.github/workflows/dependabot.yml b/.github/workflows/dependabot.yml
index fb49691..819f8a1 100644
--- a/.github/workflows/dependabot.yml
+++ b/.github/workflows/dependabot.yml
-     group: dependabot-${{ github.workflow }}-${{ github.ref }}
+     group: Dependabot-${{ github.workflow }}-${{ github.ref }}
- on:
-     workflow_dispatch:
-     pull_request:
- 
+     security-events: write
+ on:
+     workflow_dispatch:
+     pull_request:
+ 
-     approve-and-merge:
+     Approve:
+ 
+ 
+ 
+     Merge:
+         runs-on: ubuntu-latest
+ 
+         if: ${{ github.actor == 'dependabot[bot]' }}
+ 
+         steps:
+             - uses: dependabot/fetch-metadata@v1.6.0
+               with:
+                   github-token: "${{ secrets.GITHUB_TOKEN }}"
+ 
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
new file mode 100644
index 0000000..ffde8df
--- /dev/null
+++ b/.github/workflows/GitHub.yml
+ name: GitHub
+ 
+ concurrency:
+     group: GitHub-${{ github.workflow }}-${{ github.ref }}
+     cancel-in-progress: true
+ 
+ permissions:
+     issues: write
+     pull-requests: write
+ 
+ on:
+     issues:
+         types: [opened]
+     pull_request:
+         types: [opened]
+ 
+ jobs:
+     Assign:
+         runs-on: ubuntu-latest
+ 
+         env:
+             ADBLOCK: true
+             TELEMETRY_DISABLED: 1
+             ASTRO_TELEMETRY_DISABLED: 1
+             AUTOMATEDLAB_TELEMETRY_OPTOUT: 1
+             AZURE_CORE_COLLECT_TELEMETRY: 0
+             CHOOSENIM_NO_ANALYTICS: 1
+             DIEZ_DO_NOT_TRACK: 1
+             DO_NOT_TRACK: 1
+             DOTNET_CLI_TELEMETRY_OPTOUT: 1
+             DOTNET_INTERACTIVE_CLI_TELEMETRY_OPTOUT: 1
+             ET_NO_TELEMETRY: 1
+             GATSBY_TELEMETRY_DISABLED: 1
+             GATSBY_TELEMETRY_OPT_OUT: 1
+             GATSBY_TELEMETRY_OPTOUT: 1
+             HASURA_GRAPHQL_ENABLE_TELEMETRY: false
+             HINT_TELEMETRY: off
+             HOMEBREW_NO_ANALYTICS: 1
+             INFLUXD_REPORTING_DISABLED: true
+             ITERATIVE_DO_NOT_TRACK: 1
+             NEXT_TELEMETRY_DEBUG: 1
+             NEXT_TELEMETRY_DISABLED: 1
+             NG_CLI_ANALYTICS: false
+             NUXT_TELEMETRY_DISABLED: 1
+             PIN_DO_NOT_TRACK: 1
+             POWERSHELL_TELEMETRY_OPTOUT: 1
+             SAM_CLI_TELEMETRY: 0
+             STNOUPGRADE: 1
+             STRIPE_CLI_TELEMETRY_OPTOUT: 1
+ 
+         steps:
+             - uses: pozil/auto-assign-issue@v1.13.0
+               with:
+                   repo-token: ${{ secrets.GITHUB_TOKEN }}
+                   assignees: NikolaRHristov
+                   numOfAssignee: 1
diff --git a/.github/workflows/inn.yml b/.github/workflows/inn.yml
deleted file mode 100644
index 39a3ba6..0000000
--- a/.github/workflows/inn.yml
+++ /dev/null
- name: Inn
- 
- concurrency:
-     group: inn-${{ github.workflow }}-${{ github.ref }}
-     cancel-in-progress: true
- 
- on:
-     workflow_dispatch:
-     push:
-         branches: [main]
-     pull_request:
-         branches: [main]
-     workflow_call:
- 
- jobs:
-     build:
-         strategy:
-             matrix:
-                 toolchain: [stable, nightly]
-                 platform: [windows-latest, ubuntu-latest, macos-latest]
- 
-         runs-on: ${{ matrix.platform }}
-         continue-on-error: true
- 
-         steps:
-             - uses: actions/checkout@v3.5.3
-             - uses: actions-rs/toolchain@v1.0.7
-               with:
-                   profile: minimal
-                   toolchain: ${{ matrix.toolchain }}
- 
-             - uses: actions/cache@v3.3.1
-               with:
-                   path: |
-                       ~/.cargo/bin/
-                       ~/.cargo/registry/index/
-                       ~/.cargo/registry/cache/
-                       ~/.cargo/git/db/
-                       target/
-                   key: ${{ runner.os }}-cargo-${{ hashFiles('./Cargo.toml') }}
-             - uses: actions-rs/cargo@v1.0.3
-               with:
-                   command: build
-                   args: --release --all-features --manifest-path ./Cargo.toml
- 
-             - uses: actions/upload-artifact@v3.1.2
-               with:
-                   name: .-inn-${{ matrix.platform }}-target
-                   path: ./target
- 
-             - run: |
-                   git add target
-                   git config --global user.name 'Inn'
-                   git config --global user.email 'inn@lightrix.help'
-                   git commit -m "Inn Release $(git log -1 --pretty=%h) $(git log -1 --pretty=%B)"
-                   git pull && git push
diff --git a/.github/workflows/rust.yml b/.github/workflows/rust.yml
index 404bdc0..bca13f1 100644
--- a/.github/workflows/rust.yml
+++ b/.github/workflows/rust.yml
-     group: rust-${{ github.workflow }}-${{ github.ref }}
+     group: Rust-${{ github.workflow }}-${{ github.ref }}
-     rust:
+     Build:
-             - uses: actions/checkout@v3.5.3
+             - uses: actions/checkout@v4.1.1
+ 
-             - uses: actions/cache@v3.3.1
+             - uses: actions/cache@v4.0.0
+                       Target/
diff --git a/.gitignore b/.gitignore
index e3e986b..a60405a 100644
--- a/.gitignore
+++ b/.gitignore
- /target/*
- !/target/release
- /target/release/*
- !/target/release/*.deb
- !/target/release/*.exe
- !/target/release/inn
- !/target/release/innkeeper
+ /Target/*
+ !/Target/release
+ /Target/release/*
+ !/Target/release/*.deb
+ !/Target/release/*.exe
+ !/Target/release/inn
+ !/Target/release/innkeeper
diff --git a/build.rs b/build.rs
new file mode 100644
index 0000000..8f162d7
--- /dev/null
+++ b/build.rs
+ #![allow(non_snake_case)]
+ 
+ use std::fs;
+ 
+ fn main() {
+ 	println!("cargo:rerun-if-changed=Cargo.toml");
+ 	println!(
+ 		"cargo:rustc-env=CARGO_PKG_VERSION={}",
+ 		fs::read_to_string("Cargo.toml")
+ 			.expect("Failed to read Cargo.toml.")
+ 			.lines()
+ 			.find(|Line| Line.starts_with("version"))
+ 			.unwrap()
+ 			.split('=')
+ 			.nth(1)
+ 			.expect("Invalid version line format.")
+ 			.trim()
+ 			.trim_matches('"')
+ 	);
+ }
diff --git a/Cargo.toml b/Cargo.toml
index a3c1765..abda709 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- [package]
- name = "innkeeper"
- version = "0.0.9"
- description = "🍺 Inn is a little Rust utility that lets execute commands in multiple directories."
- license = "MIT"
- default-run = "inn"
- repository = "https://github.com/Lightrix/innkeeper.git"
- 
- [dependencies]
- clap = { version = "4.3.19", features = ["derive"] }
- walkdir = { version = "2.3.3" }
- rayon = { version = "1.7.0" }
- crossbeam = { version = "0.8.2" }
- toml = "0.7.6"
- 
- [lib]
- name = "inn"
- path = "src/lib/main.rs"
- 
- name = "inn"
- path = "src/bin/inn.rs"
+ name = "Inn"
+ path = "Source/Function/Binary/Inn.rs"
+ name = "Innkeeper"
+ path = "Source/Function/Binary/Innkeeper.rs"
+ 
+ [dependencies]
+ clap = { features = ["derive"], version = "4.5.1" }
+ tokio = { features = ["full"], version = "1.36.0" }
+ walkdir = "2.4.0"
+ 
+ [package]
+ autobenches = false
+ autobins = false
+ autoexamples = false
+ autotests = false
+ default-run = "Inn"
+ description = "🍺 Innkeeper is a command-line tool designed to execute a specified command in all directories that match a certain pattern within a given root directory. It provides flexibility and efficiency in running commands across multiple directories with customizable patterns."
+ license = "MIT"
- path = "src/bin/innkeeper.rs"
+ repository = "https://github.com/NikolaRHristov/Inn.git"
+ version = "0.1.0"
+ edition = "2021"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
new file mode 100644
index 0000000..b4f1f9b
--- /dev/null
+++ b/CODE_OF_CONDUCT.md
+ # Code of Conduct
+ 
+ ## Our Pledge
+ 
+ Welcome to our community! We are committed to creating a welcoming and inclusive
+ environment for all contributors. As members, contributors, and leaders, we
+ pledge to make participation in our community a harassment-free experience for
+ everyone, regardless of:
+ 
+ -   Age
+ -   Body size
+ -   Visible or invisible disability
+ -   Ethnicity
+ -   Sex characteristics
+ -   Gender identity and expression
+ -   Level of experience
+ -   Education
+ -   Socio-economic status
+ -   Nationality
+ -   Personal appearance
+ -   Race
+ -   Caste
+ -   Color
+ -   Religion
+ -   Sexual identity and orientation
+ 
+ We promise to act and interact in ways that contribute to an open, welcoming,
+ diverse, inclusive, and healthy community.
+ 
+ ## Our Standards
+ 
+ Examples of behavior that contributes to a positive environment for our
+ community include:
+ 
+ -   Demonstrating empathy and kindness toward other people
+ -   Being respectful of differing opinions, viewpoints, and experiences
+ -   Giving and gracefully accepting constructive feedback
+ -   Accepting responsibility and apologizing to those affected by our mistakes,
+     and learning from the experience
+ -   Focusing on what is best not just for us as individuals but for the overall
+     community
+ 
+ Examples of unacceptable behavior include:
+ 
+ -   The use of sexualized language or imagery, and sexual attention or advances
+     of any kind
+ -   Trolling, insulting, or derogatory comments, and personal or political
+     attacks
+ -   Public or private harassment
+ -   Publishing others' private information, such as a physical or email address,
+     without their explicit permission
+ -   Other conduct which could reasonably be considered inappropriate in a
+     professional setting
+ 
+ ## Enforcement Responsibilities
+ 
+ Community leaders are responsible for clarifying and enforcing our standards of
+ acceptable behavior. They will take appropriate and fair corrective action in
+ response to any behavior they deem inappropriate, threatening, offensive, or
+ harmful. This may include removing, editing, or rejecting comments, commits,
+ code, wiki edits, issues, and other contributions that do not align with this
+ Code of Conduct. Community leaders will communicate reasons for moderation
+ decisions when appropriate.
+ 
+ ## Scope
+ 
+ This Code of Conduct applies within all community spaces, and also applies when
+ an individual is officially representing the community in public spaces.
+ Examples of representing our community include using an official e-mail address,
+ posting via an official social media account, or acting as an appointed
+ representative at an online or offline event.
+ 
+ ## Enforcement
+ 
+ Instances of abusive, harassing, or otherwise unacceptable behavior may be
+ reported to the community leaders responsible for enforcement at
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ promptly and fairly. All community leaders are obligated to respect the privacy
+ and security of the reporter of any incident.
+ 
+ ## Enforcement Guidelines
+ 
+ Community leaders will follow these Community Impact Guidelines in determining
+ the consequences for any action they deem in violation of this Code of Conduct:
+ 
+ ### 1. Correction
+ 
+ **Community Impact**: Use of inappropriate language or other behavior deemed
+ unprofessional or unwelcome in the community.
+ 
+ **Consequence**: A private, written warning from community leaders, providing
+ clarity around the nature of the violation and an explanation of why the
+ behavior was inappropriate. A public apology may be requested.
+ 
+ ### 2. Warning
+ 
+ **Community Impact**: A violation through a single incident or series of
+ actions.
+ 
+ **Consequence**: A warning with consequences for continued behavior. No
+ interaction with the people involved, including unsolicited interaction with
+ those enforcing the Code of Conduct, for a specified period of time. This
+ includes avoiding interactions in community spaces as well as external channels
+ like social media. Violating these terms may lead to a temporary or permanent
+ ban.
+ 
+ ### 3. Temporary Ban
+ 
+ **Community Impact**: A serious violation of community standards, including
+ sustained inappropriate behavior.
+ 
+ **Consequence**: A temporary ban from any sort of interaction or public
+ communication with the community for a specified period of time. No public or
+ private interaction with the people involved, including unsolicited interaction
+ with those enforcing the Code of Conduct, is allowed during this period.
+ Violating these terms may lead to a permanent ban.
+ 
+ ### 4. Permanent Ban
+ 
+ **Community Impact**: Demonstrating a pattern of violation of community
+ standards, including sustained inappropriate behavior, harassment of an
+ individual, or aggression toward or disparagement of classes of individuals.
+ 
+ **Consequence**: A permanent ban from any sort of public interaction within the
+ community.
+ 
+ ## Attribution
+ 
+ This Code of Conduct is adapted from the [Contributor Covenant][homepage],
+ version 2.1, available at
+ [https://www.contributor-covenant.org/version/2/1/code_of_conduct.html][v2.1].
+ Community Impact Guidelines were inspired by [Mozilla's code of conduct
+ enforcement ladder][Mozilla CoC].
+ 
+ For answers to common questions about this code of conduct, see the FAQ at
+ [https://www.contributor-covenant.org/faq][FAQ]. Translations are available at
+ [https://www.contributor-covenant.org/translations][translations].
+ 
+ [homepage]: https://www.contributor-covenant.org
+ [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: https://github.com/mozilla/diversity
+ [FAQ]: https://www.contributor-covenant.org/faq
+ [translations]: https://www.contributor-covenant.org/translations
+ 
+ Thank you for being part of our community and helping us create a safe and
+ respectful environment for everyone!
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 73fa540..c740185 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- reported to the community leaders responsible for enforcement at [INSERT CONTACT
- METHOD]. All complaints will be reviewed and investigated promptly and fairly.
- All community leaders are obligated to respect the privacy and security of the
- reporter of any incident.
+ reported to the community leaders responsible for enforcement at
+ nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ promptly and fairly. All community leaders are obligated to respect the privacy
+ and security of the reporter of any incident.
diff --git a/LICENSE b/LICENSE
index 66f72b3..c47b9fa 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2022 Nikola Hristov
+ Copyright (c) 2023-2024 Nikola R. Hristov
diff --git a/README.md b/README.md
index 781899a..fed2ce7 100644
--- a/README.md
+++ b/README.md
- # [inn] 🍺
+ # 🍺 [Inn]
- Inn is a tiny Rust utlity that lets execute commmands in multiple directories.
+ Innkeeper is a command-line tool designed to execute a specified command in all
+ directories that match a certain pattern within a given root directory. It
+ provides flexibility and efficiency in running commands across multiple
+ directories with customizable patterns.
- [inn]: https://crates.io/crates/innkeeper
+ [Inn]: https://crates.io/crates/innkeeper
- inn .git git fetch upstream
+ Inn .git git fetch upstream
- This will fetch from upstream for all the `.git` repositories inside the current
- directory. Basically it replaces:
+ This command will fetch from upstream for all the .git repositories inside the
+ current directory. Essentially, it replaces the following shell command:
- Specify a `--file` argument or `-f` if you would like to search for file instead
- of a directory. Default is `false` or no flag at all.
+ To specify a `--File` argument or `-F`, if you would like to search for a file
+ instead of a directory, use:
- inn -f astro.config.ts npx astro add astro-compress
+ Inn -F astro.config.ts npx astro add astro-compress
- You can also provide a `--root` argument or `-r` which sets the current working
- directory to a different folder. Default is `.`.
+ Additionally, you can provide a `--Root` argument or `-R` to set the current
+ working directory to a different folder. The default is `.`.
- inn -r F:\Developer .git git fetch upstream
+ Inn -R D:\Developer .git git fetch upstream
- Specify a `--parallel` argument or `-p` if you would like to run functions in
- parallel. Default is sequential.
+ Specify a `--Parallel` argument or `-P` if you would like to run commands in
+ parallel. The default is sequential.
- inn -p -r F:\Developer .git git fetch upstream
+ Inn -P -R D:\Developer .git git fetch upstream
+ ## Dependencies
+ 
+ The code imports several crates:
+ 
+ -   `clap` - For parsing command-line arguments.
+ -   `tokio` - Enables parallel execution of tasks.
+ -   `walkdir` - Facilitates filesystem traversal.
+ 
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
new file mode 100644
index 0000000..2485de6
--- /dev/null
+++ b/Source/Function/Binary/Command.rs
+ use clap::{Arg, ArgAction::SetTrue, Command as CommandClap};
+ use tokio::process::Command as CommandTokio;
+ use walkdir::WalkDir;
+ 
+ use std::{
+ 	fs,
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
+ 
+ #[allow(dead_code)]
+ pub fn run() {
+ 	let Match = CommandClap::new("Innkeeper")
+ 		.version(env!("CARGO_PKG_VERSION"))
+ 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.about("Run a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("File")
+ 				.short('F')
+ 				.long("File")
+ 				.action(SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("Parallel")
+ 				.short('P')
+ 				.long("Parallel")
+ 				.action(SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("Root")
+ 				.short('R')
+ 				.long("Root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Exclude")
+ 				.short('E')
+ 				.long("Exclude")
+ 				.display_order(4)
+ 				.value_name("EXCLUDE")
+ 				.required(false)
+ 				.help("Exclude pattern.")
+ 				.default_value("node_modules .git target dist vendor"),
+ 		)
+ 		.arg(
+ 			Arg::new("Pattern")
+ 				.display_order(5)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Command")
+ 				.num_args(0..=10)
+ 				.display_order(6)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches();
+ 
+ 	let File = Match.get_flag("File");
+ 	let Parallel = Match.get_flag("Parallel");
+ 	let Root = Match.get_one::<String>("Root").unwrap();
+ 	let Exclude = Match.get_one::<String>("Exclude").unwrap().split(" ");
+ 	let Pattern = Match.get_one::<String>("Pattern").unwrap();
+ 	let Command = &Match
+ 		.get_many::<String>("Command")
+ 		.unwrap_or_default()
+ 		.map(|Command| Command.as_str())
+ 		.collect::<Vec<_>>()
+ 		.join(" ");
+ 
+ 	let Separator = std::path::MAIN_SEPARATOR;
+ 
+ 	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
+ 		let Path = Entry.path().display().to_string();
+ 
+ 		!Exclude.clone().into_iter().filter(|Exclude| Pattern != Exclude).any(|Exclude| {
+ 			if File {
+ 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
+ 			} else {
+ 				Path.contains(Exclude)
+ 			}
+ 		})
+ 	});
+ 
+ 	if Parallel {
+ 		println!("Executing code in parallel.");
+ 
+ 		// Execution: Parallel
+ 		let mut Task = Vec::new();
+ 
+ 		Entry
+ 			.map(|Entry| {
+ 				let Path = Entry.unwrap().path().display().to_string();
+ 				let Path: Vec<&str> = Path.split(Separator).collect();
+ 
+ 				match Path.last() {
+ 					Some(Last) => {
+ 						if Last == Pattern {
+ 							Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
+ 						} else {
+ 							None
+ 						}
+ 					}
+ 					None => None,
+ 				}
+ 			})
+ 			.filter_map(|Entry| Entry)
+ 			.for_each(|Directory| {
+ 				let command;
+ 
+ 				if cfg!(target_os = "windows") {
+ 					command = CommandTokio::new("cmd")
+ 						.args(["/C", Command.as_str()])
+ 						.current_dir(Directory.clone())
+ 						.output();
+ 				} else {
+ 					command = CommandTokio::new("sh")
+ 						.arg("-c")
+ 						.current_dir(Directory.clone())
+ 						.arg(Command)
+ 						.output();
+ 				}
+ 
+ 				Task.push(async move {
+ 					println!("Executing {} for every {} in {}", Command, Directory, Root);
+ 
+ 					println!(
+ 						"{}",
+ 						String::from_utf8_lossy(
+ 							&command.await.expect("Failed to execute process.").stdout
+ 						)
+ 					);
+ 				});
+ 			});
+ 
+ 		tokio::runtime::Runtime::new().unwrap().block_on(async {
+ 			for Task in Task {
+ 				Task.await;
+ 			}
+ 		});
+ 	} else {
+ 		println!("Executing code in sequential.");
+ 
+ 		// Execution: Sequential
+ 		for Entry in Entry {
+ 			let Path = Entry.unwrap().path().display().to_string();
+ 			let Path: Vec<&str> = Path.split(Separator).collect();
+ 
+ 			if let Some(Last) = Path.last() {
+ 				if Last == Pattern {
+ 					let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());
+ 
+ 					println!("Executing {} for every {} in {}", Command, Last, Root);
+ 
+ 					let mut Out = match cfg!(target_os = "windows") {
+ 						true => Command::new("cmd")
+ 							.args(["/C", Command])
+ 							.current_dir(Directory)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 						false => Command::new("sh")
+ 							.arg("-c")
+ 							.current_dir(Directory)
+ 							.arg(Command)
+ 							.stdout(Stdio::piped())
+ 							.spawn()
+ 							.expect("Failed to execute process."),
+ 					}
+ 					.stdout
+ 					.expect("Failed to get stdout handle");
+ 
+ 					let mut Output = String::new();
+ 
+ 					loop {
+ 						let mut Buffer = [0; 512];
+ 						let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
+ 
+ 						if Byte == 0 {
+ 							break;
+ 						}
+ 
+ 						Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
+ 					}
+ 
+ 					println!("{}", Output);
+ 				}
+ 			}
+ 		}
+ 	}
+ }
diff --git a/Source/Function/Binary/Inn.rs b/Source/Function/Binary/Inn.rs
new file mode 100644
index 0000000..38e3fe5
--- /dev/null
+++ b/Source/Function/Binary/Inn.rs
+ #![allow(non_snake_case)]
+ mod Command;
+ 
+ fn main() {
+ 	Command::run()
+ }
diff --git a/Source/Function/Binary/Innkeeper.rs b/Source/Function/Binary/Innkeeper.rs
new file mode 100644
index 0000000..38e3fe5
--- /dev/null
+++ b/Source/Function/Binary/Innkeeper.rs
+ #![allow(non_snake_case)]
+ mod Command;
+ 
+ fn main() {
+ 	Command::run()
+ }
diff --git a/src/bin/inn.rs b/src/bin/inn.rs
deleted file mode 100644
index e2fe709..0000000
--- a/src/bin/inn.rs
+++ /dev/null
- fn main() {
- 	inn::run()
- }
diff --git a/src/bin/innkeeper.rs b/src/bin/innkeeper.rs
deleted file mode 100644
index e2fe709..0000000
--- a/src/bin/innkeeper.rs
+++ /dev/null
- fn main() {
- 	inn::run()
- }
diff --git a/src/lib/main.rs b/src/lib/main.rs
deleted file mode 100644
index a179931..0000000
--- a/src/lib/main.rs
+++ /dev/null
- extern crate clap;
- extern crate crossbeam;
- extern crate rayon;
- extern crate walkdir;
- 
- use clap::{Arg, ArgAction, Command as ClapCommand};
- use crossbeam::scope;
- use rayon::prelude::*;
- use std::{
- 	fs,
- 	io::Read,
- 	process::{Command, Stdio},
- };
- use walkdir::WalkDir;
- 
- pub fn run() {
- 	let matches = ClapCommand::new("Innkeeper")
- 		.version("0.0.8")
- 		.about("Runs a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("file")
- 				.short('f')
- 				.long("file")
- 				.action(ArgAction::SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("parallel")
- 				.short('p')
- 				.long("parallel")
- 				.action(ArgAction::SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("root")
- 				.short('r')
- 				.long("root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("pattern")
- 				.display_order(4)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("command")
- 				.num_args(0..=10)
- 				.display_order(5)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches();
- 
- 	let file = matches.get_flag("file");
- 	let parallel = matches.get_flag("parallel");
- 	let root = matches.get_one::<String>("root").unwrap();
- 	let pattern = matches.get_one::<String>("pattern").unwrap();
- 	let command = &matches
- 		.get_many::<String>("command")
- 		.unwrap_or_default()
- 		.map(|v| v.as_str())
- 		.collect::<Vec<_>>()
- 		.join(" ");
- 
- 	let ds = std::path::MAIN_SEPARATOR;
- 
- 	let entries = WalkDir::new(root).into_iter().filter_entry(|e| {
- 		if !pattern.contains("node_modules") {
- 			return e.path().display().to_string().contains("node_modules");
- 		}
- 
- 		if !file {
- 			println!("{:?}", e.path().display().to_string().contains("node_modules"));
- 			return fs::metadata(e.path().display().to_string().clone()).unwrap().is_dir();
- 		} else {
- 			return true;
- 		}
- 	});
- 
- 	if parallel {
- 		println!("Executing code in parallel.");
- 
- 		// Parallel
- 		let dirs = entries
- 			.map(|entry| {
- 				let entry_dir = entry.unwrap().path().display().to_string();
- 				let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 				match paths.last() {
- 					Some(last) => {
- 						if last == pattern {
- 							let working_directory =
- 								&paths[0..paths.len() - 1].join(&ds.to_string());
- 							Some(working_directory.to_owned())
- 						} else {
- 							None
- 						}
- 					}
- 					None => None,
- 				}
- 			})
- 			.filter_map(|x| x)
- 			.collect::<Vec<String>>();
- 
- 		scope(|s| {
- 			dirs.into_par_iter().for_each_with(s, |scope, dir| {
- 				scope.spawn(move |_| {
- 					println!("Executing {} for every {} in {}", command, dir, root);
- 
- 					let output = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command.as_str()])
- 							.current_dir(dir)
- 							.output()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(dir)
- 							.arg(command)
- 							.output()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					println!("{}", String::from_utf8_lossy(&output.stdout));
- 				});
- 			});
- 		})
- 		.unwrap();
- 	} else {
- 		println!("Executing code in sequential.");
- 
- 		// Sequential
- 		for entry in entries {
- 			let entry_dir = entry.unwrap().path().display().to_string();
- 			let paths: Vec<&str> = entry_dir.split(ds).collect();
- 
- 			if let Some(last) = paths.last() {
- 				if last == pattern {
- 					let working_directory = &paths[0..paths.len() - 1].join(&ds.to_string());
- 
- 					println!("Executing {} for every {} in {}", command, last, root);
- 
- 					let child = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", command])
- 							.current_dir(working_directory)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(working_directory)
- 							.arg(command)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 					};
- 
- 					let mut stdout = child.stdout.expect("Failed to get stdout handle");
- 
- 					let mut output = String::new();
- 
- 					loop {
- 						let mut buffer = [0; 512];
- 						let bytes_read =
- 							stdout.read(&mut buffer).expect("Failed to read from pipe");
- 
- 						if bytes_read == 0 {
- 							break;
- 						}
- 
- 						output.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
- 					}
- 
- 					println!("{}", output);
- 				}
- 			}
- 		}
- 	}
- }

🗣️ Summary from v0.1.0 to v0.1.1 in .
diff --git a/Cargo.toml b/Cargo.toml
index abda709..61afaf1 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.1.0"
+ version = "0.1.1"
diff --git a/README.md b/README.md
index fed2ce7..5e83613 100644
--- a/README.md
+++ b/README.md
+ ## Benchmark
+ 
+ 
+ ```sh
+ time Inn -P .git ls
+ ```
+ 
+ 
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
index 2485de6..92e380e 100644
--- a/Source/Function/Binary/Command.rs
+++ b/Source/Function/Binary/Command.rs
- use tokio::process::Command as CommandTokio;
- use walkdir::WalkDir;
- 
- 	fs,
+ use tokio::process::Command as CommandTokio;
+ use walkdir::WalkDir;
- 				fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
+ 				std::fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
- 		let mut Task = Vec::new();
+ 		let mut Queue = Vec::new();
- 				let command;
+ 				let Output;
- 					command = CommandTokio::new("cmd")
+ 					Output = CommandTokio::new("cmd")
- 					command = CommandTokio::new("sh")
+ 					Output = CommandTokio::new("sh")
- 				Task.push(async move {
+ 				Queue.push(async move {
- 							&command.await.expect("Failed to execute process.").stdout
+ 							&Output.await.expect("Failed to execute process.").stdout
- 			for Task in Task {
- 				Task.await;
+ 			for Queue in Queue {
+ 				Queue.await;

🗣️ Summary from v0.1.1 to v0.1.2 in .
diff --git a/Cargo.toml b/Cargo.toml
index 61afaf1..2f992ce 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.1.1"
+ version = "0.1.2"
diff --git a/README.md b/README.md
index 5e83613..7cc63af 100644
--- a/README.md
+++ b/README.md
- 
- ```sh
- time Inn -P .git ls
- ```
- 
+ <table>
+ 	<tr>
+ 		<th>Command:</th>
+ 		<th>Time:</th>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>time Inn -P .git ls</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real 0m9.441s
+ user 0m0.030s
+ sys 0m0.046s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>time find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real 0m14.293s +5s
+ user 0m4.645s
+ sys 0m8.937s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>time Inn -P .git git status</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real 0m24.146s
+ user 0m0.030s
+ sys 0m0.062s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>time find -iname .git -type d -execdir ls \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real 0m28.584s +4s
+ user 0m4.695s
+ sys 0m8.354s</pre>
+ 		</td>
+ 	</tr>
+ </table>

🗣️ Summary from v0.1.2 to v0.1.3 in .
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index bca13f1..161d7eb 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-             - uses: actions/cache@v4.0.0
+             - uses: actions/cache@v4.0.1
diff --git a/.gitignore b/.gitignore
index a60405a..cbb2f71 100644
--- a/.gitignore
+++ b/.gitignore
- /Target/*
- !/Target/release
- /Target/release/*
- !/Target/release/*.deb
- !/Target/release/*.exe
- !/Target/release/inn
- !/Target/release/innkeeper
+ /target/*
+ !/target/release
+ /target/release/*
+ !/target/release/*.deb
+ !/target/release/*.exe
+ !/target/release/Inn
+ !/target/release/Innkeeper
diff --git a/Cargo.toml b/Cargo.toml
index 2f992ce..53af508 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- path = "Source/Function/Binary/Inn.rs"
+ path = "Source/Library.rs"
- path = "Source/Function/Binary/Innkeeper.rs"
+ path = "Source/Library.rs"
- walkdir = "2.4.0"
+ walkdir = "2.5.0"
+ 
+ [lib]
+ crate-type = ["staticlib", "cdylib", "rlib"]
+ name = "Library"
+ path = "Source/Library.rs"
- description = "🍺 Innkeeper is a command-line tool designed to execute a specified command in all directories that match a certain pattern within a given root directory. It provides flexibility and efficiency in running commands across multiple directories with customizable patterns."
+ description = "🍺 Inn lets you execute parallel commands in multiple directories."
- version = "0.1.2"
+ version = "0.1.3"
diff --git a/README.md b/README.md
index 7cc63af..99eb0cd 100644
--- a/README.md
+++ b/README.md
- 			<pre>time Inn -P .git ls</pre>
+ 			<pre>Inn -P .git ls</pre>
- 			<pre>time find -iname .git -type d -execdir ls \;</pre>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
- user 0m4.645s
- sys 0m8.937s</pre>
+ user    0m4.645s +4s
+ sys     0m8.937s +8s</pre>
- 			<pre>time Inn -P .git git status</pre>
+ 			<pre>Inn -P .git git status</pre>
- 			<pre>time find -iname .git -type d -execdir ls \;</pre>
+ 			<pre>find -iname .git -type d -execdir ls \;</pre>
- user 0m4.695s
- sys 0m8.354s</pre>
+ user    0m4.695s +4s
+ sys     0m8.354s +8s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>Inn -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m33.813s
+ user    0m0.015s
+ sys     0m0.060s</pre>
+ 		</td>
+ 	</tr>
+ 	<tr>
+ 		<td>
+ 			<pre>find -iname .git -type d -execdir \
+ bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
+ 		</td>
+ 		<td>
+ 			<pre>real    0m53.122s +20s
+ user    0m9.449s +9s
+ sys     0m14.442s +14s</pre>
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
new file mode 100644
index 0000000..d9d47b8
--- /dev/null
+++ b/Source/Fn/Binary/Command.rs
+ pub mod Entry;
+ pub mod Parallel;
+ pub mod Sequential;
+ 
+ pub fn Fn() -> ArgMatches {
+ 	Command::new("Innkeeper")
+ 		.version(env!("CARGO_PKG_VERSION"))
+ 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.about("Run a command in all directories having a certain pattern.")
+ 		.arg(
+ 			Arg::new("File")
+ 				.short('F')
+ 				.long("File")
+ 				.action(SetTrue)
+ 				.display_order(1)
+ 				.value_name("FILE")
+ 				.required(false)
+ 				.help("Search file."),
+ 		)
+ 		.arg(
+ 			Arg::new("Parallel")
+ 				.short('P')
+ 				.long("Parallel")
+ 				.action(SetTrue)
+ 				.display_order(2)
+ 				.value_name("PARALLEL")
+ 				.required(false)
+ 				.help("Execute code in parallel."),
+ 		)
+ 		.arg(
+ 			Arg::new("Root")
+ 				.short('R')
+ 				.long("Root")
+ 				.display_order(3)
+ 				.value_name("ROOT")
+ 				.required(false)
+ 				.help("Current working directory.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Exclude")
+ 				.short('E')
+ 				.long("Exclude")
+ 				.display_order(4)
+ 				.value_name("EXCLUDE")
+ 				.required(false)
+ 				.help("Exclude pattern.")
+ 				.default_value("node_modules .git target dist vendor"),
+ 		)
+ 		.arg(
+ 			Arg::new("Pattern")
+ 				.display_order(5)
+ 				.value_name("PATTERN")
+ 				.required(true)
+ 				.help("Search pattern.")
+ 				.default_value("."),
+ 		)
+ 		.arg(
+ 			Arg::new("Command")
+ 				.num_args(0..=10)
+ 				.display_order(6)
+ 				.value_name("COMMAND")
+ 				.required(true)
+ 				.allow_hyphen_values(true)
+ 				.allow_negative_numbers(true)
+ 				.help("Command to run."),
+ 		)
+ 		.get_matches()
+ }
+ 
+ use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
new file mode 100644
index 0000000..4eed41f
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
+ pub fn Fn(Option { File, Root, Exclude, Pattern, Separator, .. }: &Option) -> Return {
+ 	WalkDir::new(Root)
+ 		// TODO: BENCH THIS
+ 		.max_open(60)
+ 		.into_iter()
+ 		.filter_entry( |Entry| {
+ 			let Path = Entry.path().display().to_string();
+ 
+ 			!Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(
+ 				|Exclude| {
+ 					match File {
+ 						true => {
+ 							std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
+ 						}
+ 						false => Path.contains(&Exclude),
+ 					}
+ 				},
+ 			)
+ 
+ 		})
+ 		.filter_map(|Entry| Entry.ok())		
+ 		.map(|Entry| Entry.path().display().to_string().split(*Separator).map(|Entry| Entry.to_string()).collect())
+ 		.collect::<Vec<_>>()
+ }
+ 
+ use crate::Struct::Binary::Command::{Entry::Entry as Return, Option::Struct as Option};
+ 
+ use walkdir::WalkDir;
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
new file mode 100644
index 0000000..c4bb550
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
+ pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	println!("Executing code in parallel.");
+ 
+ 	// Execution: Parallel
+ 	let mut Queue = Vec::new();
+ 
+ 	for Entry in Entry
+ 		.into_iter()
+ 		.map(|Entry| match Entry.last() {
+ 			Some(Last) => {
+ 				if *Last == Pattern {
+ 					Some(Entry[0..Entry.len() - 1].join(&Separator.to_string()))
+ 				} else {
+ 					None
+ 				}
+ 			}
+ 			None => None,
+ 		})
+ 		.filter_map(|Entry| Entry)
+ 	{
+ 		let Output;
+ 
+ 		if cfg!(target_os = "windows") {
+ 			Output =
+ 				CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output();
+ 		} else {
+ 			Output = CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output();
+ 		}
+ 
+ 		Queue.push(async move {
+ 			println!(
+ 				"{}",
+ 				String::from_utf8_lossy(&Output.await.expect("Failed to execute process.").stdout)
+ 			);
+ 		});
+ 	}
+ 
+ 	tokio::runtime::Runtime::new().unwrap().block_on(async {
+ 		for Queue in Queue {
+ 			Queue.await;
+ 		}
+ 	});
+ }
+ 
+ use crate::Struct::Binary::Command::Entry::Struct as Option;
+ 
+ use tokio::process::Command as CommandTokio;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..3b3b13c
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
+ pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	println!("Executing code in sequential.");
+ 
+ 	// Execution: Sequential
+ 	for Entry in Entry {
+ 		if let Some(Last) = Entry.last() {
+ 			if *Last == Pattern {
+ 				let Directory = &Entry[0..Entry.len() - 1].join(&Separator.to_string());
+ 
+ 				let mut Out = match cfg!(target_os = "windows") {
+ 					true => Command::new("cmd")
+ 						.args(["/C", &Command])
+ 						.current_dir(Directory)
+ 						.stdout(Stdio::piped())
+ 						.spawn()
+ 						.expect("Failed to execute process."),
+ 					false => Command::new("sh")
+ 						.arg("-c")
+ 						.current_dir(Directory)
+ 						.arg(Command.clone())
+ 						.stdout(Stdio::piped())
+ 						.spawn()
+ 						.expect("Failed to execute process."),
+ 				}
+ 				.stdout
+ 				.expect("Failed to get stdout handle");
+ 
+ 				let mut Output = String::new();
+ 
+ 				loop {
+ 					let mut Buffer = [0; 512];
+ 					let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
+ 
+ 					if Byte == 0 {
+ 						break;
+ 					}
+ 
+ 					Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
+ 				}
+ 
+ 				println!("{}", Output);
+ 			}
+ 		}
+ 	}
+ }
+ 
+ use crate::Struct::Binary::Command::Entry::Struct as Option;
+ 
+ use std::{
+ 	io::Read,
+ 	process::{Command, Stdio},
+ };
diff --git a/Source/Fn/Binary/mod.rs b/Source/Fn/Binary/mod.rs
new file mode 100644
index 0000000..9da7843
--- /dev/null
+++ b/Source/Fn/Binary/mod.rs
+ pub mod Command;
diff --git a/Source/Fn/mod.rs b/Source/Fn/mod.rs
new file mode 100644
index 0000000..a56e8ce
--- /dev/null
+++ b/Source/Fn/mod.rs
+ pub mod Binary;
diff --git a/Source/Function/Binary/Command.rs b/Source/Function/Binary/Command.rs
deleted file mode 100644
index 92e380e..0000000
--- a/Source/Function/Binary/Command.rs
+++ /dev/null
- use clap::{Arg, ArgAction::SetTrue, Command as CommandClap};
- use std::{
- 	io::Read,
- 	process::{Command, Stdio},
- };
- use tokio::process::Command as CommandTokio;
- use walkdir::WalkDir;
- 
- #[allow(dead_code)]
- pub fn run() {
- 	let Match = CommandClap::new("Innkeeper")
- 		.version(env!("CARGO_PKG_VERSION"))
- 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("File")
- 				.short('F')
- 				.long("File")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("FILE")
- 				.required(false)
- 				.help("Search file."),
- 		)
- 		.arg(
- 			Arg::new("Parallel")
- 				.short('P')
- 				.long("Parallel")
- 				.action(SetTrue)
- 				.display_order(2)
- 				.value_name("PARALLEL")
- 				.required(false)
- 				.help("Execute code in parallel."),
- 		)
- 		.arg(
- 			Arg::new("Root")
- 				.short('R')
- 				.long("Root")
- 				.display_order(3)
- 				.value_name("ROOT")
- 				.required(false)
- 				.help("Current working directory.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Exclude")
- 				.short('E')
- 				.long("Exclude")
- 				.display_order(4)
- 				.value_name("EXCLUDE")
- 				.required(false)
- 				.help("Exclude pattern.")
- 				.default_value("node_modules .git target dist vendor"),
- 		)
- 		.arg(
- 			Arg::new("Pattern")
- 				.display_order(5)
- 				.value_name("PATTERN")
- 				.required(true)
- 				.help("Search pattern.")
- 				.default_value("."),
- 		)
- 		.arg(
- 			Arg::new("Command")
- 				.num_args(0..=10)
- 				.display_order(6)
- 				.value_name("COMMAND")
- 				.required(true)
- 				.allow_hyphen_values(true)
- 				.allow_negative_numbers(true)
- 				.help("Command to run."),
- 		)
- 		.get_matches();
- 
- 	let File = Match.get_flag("File");
- 	let Parallel = Match.get_flag("Parallel");
- 	let Root = Match.get_one::<String>("Root").unwrap();
- 	let Exclude = Match.get_one::<String>("Exclude").unwrap().split(" ");
- 	let Pattern = Match.get_one::<String>("Pattern").unwrap();
- 	let Command = &Match
- 		.get_many::<String>("Command")
- 		.unwrap_or_default()
- 		.map(|Command| Command.as_str())
- 		.collect::<Vec<_>>()
- 		.join(" ");
- 
- 	let Separator = std::path::MAIN_SEPARATOR;
- 
- 	let Entry = WalkDir::new(Root).into_iter().filter_entry(|Entry| {
- 		let Path = Entry.path().display().to_string();
- 
- 		!Exclude.clone().into_iter().filter(|Exclude| Pattern != Exclude).any(|Exclude| {
- 			if File {
- 				std::fs::metadata(Path.clone()).unwrap().is_dir() && Path.contains(Exclude)
- 			} else {
- 				Path.contains(Exclude)
- 			}
- 		})
- 	});
- 
- 	if Parallel {
- 		println!("Executing code in parallel.");
- 
- 		// Execution: Parallel
- 		let mut Queue = Vec::new();
- 
- 		Entry
- 			.map(|Entry| {
- 				let Path = Entry.unwrap().path().display().to_string();
- 				let Path: Vec<&str> = Path.split(Separator).collect();
- 
- 				match Path.last() {
- 					Some(Last) => {
- 						if Last == Pattern {
- 							Some(Path[0..Path.len() - 1].join(&Separator.to_string()))
- 						} else {
- 							None
- 						}
- 					}
- 					None => None,
- 				}
- 			})
- 			.filter_map(|Entry| Entry)
- 			.for_each(|Directory| {
- 				let Output;
- 
- 				if cfg!(target_os = "windows") {
- 					Output = CommandTokio::new("cmd")
- 						.args(["/C", Command.as_str()])
- 						.current_dir(Directory.clone())
- 						.output();
- 				} else {
- 					Output = CommandTokio::new("sh")
- 						.arg("-c")
- 						.current_dir(Directory.clone())
- 						.arg(Command)
- 						.output();
- 				}
- 
- 				Queue.push(async move {
- 					println!("Executing {} for every {} in {}", Command, Directory, Root);
- 
- 					println!(
- 						"{}",
- 						String::from_utf8_lossy(
- 							&Output.await.expect("Failed to execute process.").stdout
- 						)
- 					);
- 				});
- 			});
- 
- 		tokio::runtime::Runtime::new().unwrap().block_on(async {
- 			for Queue in Queue {
- 				Queue.await;
- 			}
- 		});
- 	} else {
- 		println!("Executing code in sequential.");
- 
- 		// Execution: Sequential
- 		for Entry in Entry {
- 			let Path = Entry.unwrap().path().display().to_string();
- 			let Path: Vec<&str> = Path.split(Separator).collect();
- 
- 			if let Some(Last) = Path.last() {
- 				if Last == Pattern {
- 					let Directory = &Path[0..Path.len() - 1].join(&Separator.to_string());
- 
- 					println!("Executing {} for every {} in {}", Command, Last, Root);
- 
- 					let mut Out = match cfg!(target_os = "windows") {
- 						true => Command::new("cmd")
- 							.args(["/C", Command])
- 							.current_dir(Directory)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 						false => Command::new("sh")
- 							.arg("-c")
- 							.current_dir(Directory)
- 							.arg(Command)
- 							.stdout(Stdio::piped())
- 							.spawn()
- 							.expect("Failed to execute process."),
- 					}
- 					.stdout
- 					.expect("Failed to get stdout handle");
- 
- 					let mut Output = String::new();
- 
- 					loop {
- 						let mut Buffer = [0; 512];
- 						let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
- 
- 						if Byte == 0 {
- 							break;
- 						}
- 
- 						Output.push_str(&String::from_utf8_lossy(&Buffer[..Byte]));
- 					}
- 
- 					println!("{}", Output);
- 				}
- 			}
- 		}
- 	}
- }
diff --git a/Source/Function/Binary/Inn.rs b/Source/Function/Binary/Inn.rs
deleted file mode 100644
index 38e3fe5..0000000
--- a/Source/Function/Binary/Inn.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Command;
- 
- fn main() {
- 	Command::run()
- }
diff --git a/Source/Function/Binary/Innkeeper.rs b/Source/Function/Binary/Innkeeper.rs
deleted file mode 100644
index 38e3fe5..0000000
--- a/Source/Function/Binary/Innkeeper.rs
+++ /dev/null
- #![allow(non_snake_case)]
- mod Command;
- 
- fn main() {
- 	Command::run()
- }
diff --git a/Source/Library.rs b/Source/Library.rs
new file mode 100644
index 0000000..2bd4781
--- /dev/null
+++ b/Source/Library.rs
+ #![allow(non_snake_case)]
+ pub mod Fn;
+ pub mod Struct;
+ 
+ #[allow(dead_code)]
+ fn main() {
+ 	(Struct::Binary::Command::Struct::Fn().Fn)()
+ }
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
new file mode 100644
index 0000000..faff4af
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
+ pub mod Entry;
+ pub mod Option;
+ 
+ use crate::Fn::Binary::Command::{Parallel, Sequential};
+ 
+ #[derive(Debug)]
+ pub struct Struct {
+ 	pub Separator: Option::Separator,
+ 	pub Fn: fn(),
+ }
+ 
+ impl Struct {
+ 	pub fn Fn() -> Self {
+ 		Self {
+ 			Separator: std::path::MAIN_SEPARATOR,
+ 			Fn: || {
+ 				let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));
+ 
+ 				match Option.Parallel {
+ 					true => {
+ 						Parallel::Fn(Option);
+ 					}
+ 					false => {
+ 						Sequential::Fn(Option);
+ 					}
+ 				};
+ 			},
+ 		}
+ 	}
+ }
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..0814262
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
+ pub type Entry = Vec<Vec<String>>;
+ 
+ pub struct Struct {
+ 	pub Command: Command,
+ 	pub Entry: Entry,
+ 	pub Parallel: Parallel,
+ 	pub Pattern: Pattern,
+ 	pub Separator: Separator,
+ }
+ 
+ impl Struct {
+ 	pub fn Fn(Option: &Option) -> Self {
+ 		Self {
+ 			Command: Option.Command.clone(),
+ 			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
+ 			Parallel: Option.Parallel,
+ 			Pattern: Option.Pattern.clone(),
+ 			Separator: Option.Separator,
+ 		}
+ 	}
+ }
+ 
+ use crate::Struct::Binary::Command::Option::{
+ 	Command, Parallel, Pattern, Separator, Struct as Option,
+ };
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..caefbae
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
+ pub type Command = String;
+ pub type Parallel = bool;
+ pub type Pattern = String;
+ pub type Separator = char;
+ 
+ pub struct Struct {
+ 	pub Command: String,
+ 	pub Exclude: Vec<String>,
+ 	pub File: bool,
+ 	pub Parallel: bool,
+ 	pub Pattern: Pattern,
+ 	pub Root: String,
+ 	pub Separator: Separator,
+ }
+ 
+ impl Struct {
+ 	pub fn Fn(Option { Separator, .. }: Option) -> Self {
+ 		Self {
+ 			File: Fn().get_flag("File"),
+ 			Parallel: Fn().get_flag("Parallel"),
+ 			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
+ 			Exclude: Fn()
+ 				.get_one::<String>("Exclude")
+ 				.expect("Cannot Exclude.")
+ 				.split(" ")
+ 				.map(|Command| Command.to_string())
+ 				.collect::<Vec<_>>(),
+ 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern").to_owned(),
+ 			Command: Fn()
+ 				.get_many::<String>("Command")
+ 				.expect("Cannot Command")
+ 				.map(|Command| Command.as_str())
+ 				.collect::<Vec<_>>()
+ 				.join(" "),
+ 			Separator,
+ 		}
+ 	}
+ }
+ 
+ use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
diff --git a/Source/Struct/Binary/mod.rs b/Source/Struct/Binary/mod.rs
new file mode 100644
index 0000000..9da7843
--- /dev/null
+++ b/Source/Struct/Binary/mod.rs
+ pub mod Command;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
new file mode 100644
index 0000000..a56e8ce
--- /dev/null
+++ b/Source/Struct/mod.rs
+ pub mod Binary;

🗣️ Summary from v0.1.3 to v0.1.4 in .
diff --git a/build.rs b/build.rs
index 8f162d7..791c8a1 100644
--- a/build.rs
+++ b/build.rs
- 			.unwrap()
+ 			.expect("Cannot version.")
diff --git a/Cargo.toml b/Cargo.toml
index 53af508..7177409 100644
--- a/Cargo.toml
+++ b/Cargo.toml
+ crossbeam = { version = "0.8.4", features = ["crossbeam-deque"] }
- version = "0.1.3"
+ version = "0.1.4"
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 4eed41f..bf4ce76 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
- pub fn Fn(Option { File, Root, Exclude, Pattern, Separator, .. }: &Option) -> Return {
+ pub fn Fn(Option { Exclude, File, Pattern, Root, Separator, .. }: &Option) -> Return {
- 		// TODO: BENCH THIS
- 		.max_open(60)
- 		.filter_entry( |Entry| {
- 			let Path = Entry.path().display().to_string();
+ 		.filter_map(|Entry| {
+ 			let Path = Entry.expect("Cannot Entry.").path().display().to_string();
+ 
+ 			if !Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(|Exclude| {
+ 				let Match = Path.contains(&Exclude);
- 			!Exclude.clone().into_iter().filter(|Exclude| *Pattern != *Exclude).any(
- 				|Exclude| {
- 						true => {
- 							std::fs::metadata(&Path).unwrap().is_dir() && Path.contains(&Exclude)
+ 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
+ 					false => Match,
- 						false => Path.contains(&Exclude),
+ 			}) {
+ 				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
+ 			} else {
+ 				None
- 				},
- 			)
- 
- 		.filter_map(|Entry| Entry.ok())		
- 		.map(|Entry| Entry.path().display().to_string().split(*Separator).map(|Entry| Entry.to_string()).collect())
- use crate::Struct::Binary::Command::{Entry::Entry as Return, Option::Struct as Option};
+ use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index c4bb550..bb07129 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 	println!("Executing code in parallel.");
- 
- 	// Execution: Parallel
+ 	// let Deque = std::sync::Arc::new(std::sync::Mutex::new(deque::Worker::new_fifo()));
+ 	// let mut Stealer = Vec::new();
+ 
+ 	// for _ in 0..12 {
+ 	// Stealer.push(Deque.lock().unwrap().stealer());
+ 	// }
+ 		// let Deque = std::sync::Arc::clone(&Deque);
+ 			// Deque.lock().unwrap().push(Output.await.expect("Failed to execute process.").stdout);
- 	tokio::runtime::Runtime::new().unwrap().block_on(async {
+ 	tokio::runtime::Runtime::new().expect("Cannot Runtime.").block_on(async {
+ 
+ 	// for Stealer in Stealer {
+ 	// 	loop {
+ 	// 		match Stealer.steal() {
+ 	// 			deque::Steal::Success(Success) => {
+ 	// 				println!("{}", String::from_utf8_lossy(&Success));
+ 	// 			}
+ 	// 			deque::Steal::Empty => break,
+ 	// 			deque::Steal::Retry => continue,
+ 	// 		}
+ 	// 	}
+ 	// }
+ // use crossbeam::deque;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 3b3b13c..bc41a8f 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
- pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	println!("Executing code in sequential.");
- 
- 	// Execution: Sequential
+ pub fn Fn(Option { Command, Entry, Pattern, Separator, .. }: Option) {
diff --git a/Source/Library.rs b/Source/Library.rs
index 2bd4781..982f1a3 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
- pub mod Fn;
- pub mod Struct;
+ mod Fn;
+ mod Struct;
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index 0814262..5b62589 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
- pub type Entry = Vec<Vec<String>>;
+ pub type Type = Vec<Vec<String>>;
- 	pub Entry: Entry,
+ 	pub Entry: Type,
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index caefbae..4c946ac 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern").to_owned(),
+ 			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
- 				.expect("Cannot Command")
+ 				.expect("Cannot Command.")

🗣️ Summary from v0.1.4 to v0.1.5 in .
diff --git a/Cargo.toml b/Cargo.toml
index 7177409..99c9371 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- crossbeam = { version = "0.8.4", features = ["crossbeam-deque"] }
- version = "0.1.4"
+ version = "0.1.5"
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index bb07129..5e3d12e 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 	// let Deque = std::sync::Arc::new(std::sync::Mutex::new(deque::Worker::new_fifo()));
- 	// let mut Stealer = Vec::new();
- 	// for _ in 0..12 {
- 	// Stealer.push(Deque.lock().unwrap().stealer());
- 	// }
- 
- 	for Entry in Entry
- 		.into_iter()
- 		.map(|Entry| match Entry.last() {
- 			Some(Last) => {
- 				if *Last == Pattern {
- 					Some(Entry[0..Entry.len() - 1].join(&Separator.to_string()))
- 				} else {
- 					None
- 				}
- 			}
- 			None => None,
- 		})
- 		.filter_map(|Entry| Entry)
- 	{
- 		let Output;
- 		// let Deque = std::sync::Arc::clone(&Deque);
- 
- 		if cfg!(target_os = "windows") {
- 			Output =
- 				CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output();
+ 	for Entry in Entry.into_iter().filter_map(|Entry| {
+ 		Entry
+ 			.last()
+ 			.filter(|Last| *Last == &Pattern)
+ 			.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
+ 	}) {
+ 		let Output = if cfg!(target_os = "windows") {
+ 			CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
- 			Output = CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output();
- 		}
+ 			CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
+ 		};
- 			// Deque.lock().unwrap().push(Output.await.expect("Failed to execute process.").stdout);
- 
- 	// for Stealer in Stealer {
- 	// 	loop {
- 	// 		match Stealer.steal() {
- 	// 			deque::Steal::Success(Success) => {
- 	// 				println!("{}", String::from_utf8_lossy(&Success));
- 	// 			}
- 	// 			deque::Steal::Empty => break,
- 	// 			deque::Steal::Retry => continue,
- 	// 		}
- 	// 	}
- 	// }
- 
- // use crossbeam::deque;

🗣️ Summary from v0.1.5 to v0.1.6 in .
diff --git a/Cargo.toml b/Cargo.toml
index 99c9371..7fd990f 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.1.5"
+ version = "0.1.6"
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 5e3d12e..1d048ea 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 	for Entry in Entry.into_iter().filter_map(|Entry| {
+ 	Entry
+ 		.into_iter()
+ 		.filter_map(|Entry| {
- 	}) {
+ 		})
+ 		.for_each(|Entry| {
- 				String::from_utf8_lossy(&Output.await.expect("Failed to execute process.").stdout)
+ 					String::from_utf8_lossy(
+ 						&Output.await.expect("Failed to execute process.").stdout
+ 					)
- 	}
+ 		});
- 	});
+ 	})
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index bc41a8f..ded668d 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
- 	for Entry in Entry {
- 		if let Some(Last) = Entry.last() {
- 			if *Last == Pattern {
- 				let Directory = &Entry[0..Entry.len() - 1].join(&Separator.to_string());
- 
+ 	Entry
+ 		.into_iter()
+ 		.filter_map(|Entry| {
+ 			Entry
+ 				.last()
+ 				.filter(|Last| *Last == &Pattern)
+ 				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
+ 		})
+ 		.for_each(|Entry| {
- 						.current_dir(Directory)
+ 					.current_dir(Entry)
- 						.current_dir(Directory)
+ 					.current_dir(Entry)
- 			}
- 		}
- 	}
+ 		})

🗣️ Summary from v0.1.6 to v0.1.7 in .
diff --git a/build.rs b/build.rs
index 791c8a1..550762c 100644
--- a/build.rs
+++ b/build.rs
- 			.expect("Failed to read Cargo.toml.")
+ 			.expect("Cannot Cargo.toml.")
- 			.expect("Cannot version.")
+ 			.expect("Cannot Version.")
- 			.expect("Invalid version line format.")
+ 			.expect("Cannot nth.")
diff --git a/Cargo.toml b/Cargo.toml
index 7fd990f..e33646c 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.1.6"
+ version = "0.1.7"
diff --git a/README.md b/README.md
index 99eb0cd..202fdaa 100644
--- a/README.md
+++ b/README.md
- Innkeeper is a command-line tool designed to execute a specified command in all
+ Inn is a command-line tool designed to execute a specified command in all
- This command will fetch from upstream for all the .git repositories inside the
- current directory. Essentially, it replaces the following shell command:
+ This command will fetch from `upstream` for all the `.git` repositories inside
+ the current directory. Basically, it replaces the following command:
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index d9d47b8..39a9066 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 	Command::new("Innkeeper")
+ 	Command::new("Inn")
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 1d048ea..efd9808 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 				CommandTokio::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
+ 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
- 				CommandTokio::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
+ 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
- 					String::from_utf8_lossy(
- 						&Output.await.expect("Failed to execute process.").stdout
- 					)
+ 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
- 	tokio::runtime::Runtime::new().expect("Cannot Runtime.").block_on(async {
+ 	tokio::runtime::Builder::new_multi_thread()
+ 		.enable_all()
+ 		.build()
+ 		.expect("Cannot Runtime.")
+ 		.block_on(async {
- use tokio::process::Command as CommandTokio;
+ use tokio::process::Command;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index ded668d..8eef18b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
- 					.expect("Failed to execute process."),
+ 					.expect("Cannot spawn."),
- 					.expect("Failed to execute process."),
+ 					.expect("Cannot spawn."),
- 			.expect("Failed to get stdout handle");
+ 			.expect("Cannot stdout.");
- 				let Byte = Out.read(&mut Buffer).expect("Failed to read from pipe");
+ 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");

🗣️ Summary from v0.1.7 to v0.1.8 in .
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index ffde8df..09e8b75 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
-             TELEMETRY_DISABLED: 1
-             DO_NOT_TRACK: 1
+             DO_NOT_TRACK: 1
-             GATSBY_TELEMETRY_OPT_OUT: 1
+             GATSBY_TELEMETRY_OPT_OUT: 1
+             GRIT_TELEMETRY_DISABLED: 1
+             TELEMETRY_DISABLED: 1
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index 161d7eb..9148b84 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
-         branches: [main]
+         branches: [Current]
-         branches: [main]
+         branches: [Current]
-             TELEMETRY_DISABLED: 1
-             DO_NOT_TRACK: 1
+             DO_NOT_TRACK: 1
-             GATSBY_TELEMETRY_OPT_OUT: 1
+             GATSBY_TELEMETRY_OPT_OUT: 1
+             GRIT_TELEMETRY_DISABLED: 1
+             TELEMETRY_DISABLED: 1
-             - uses: actions/checkout@v4.1.1
+             - uses: actions/checkout@v4.1.2
-             - uses: actions/cache@v4.0.1
+             - uses: actions/cache@v4.0.2
diff --git a/Cargo.toml b/Cargo.toml
index e33646c..2d07668 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- clap = { features = ["derive"], version = "4.5.1" }
+ clap = { features = ["derive"], version = "4.5.3" }
- description = "🍺 Inn lets you execute parallel commands in multiple directories."
+ description = "🍺 Run Command."
- version = "0.1.7"
+ version = "0.1.8"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index b4f1f9b..0e092d3 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ Community@Playform.Cloud. All complaints will be reviewed and investigated
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index c740185..b8ceeae 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- nikola@nikolahristov.tech. All complaints will be reviewed and investigated
+ Community@Playform.Cloud. All complaints will be reviewed and investigated
diff --git a/README.md b/README.md
index 202fdaa..42ed6dd 100644
--- a/README.md
+++ b/README.md
+ You can hide the command output by specifying an `-H` or `--Hide` parameter:
+ 
+ ```sh
+ Inn -H -F package.json ncu -u
+ ```
+ 
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 39a9066..773eb20 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 		.author("Nikola R. Hristov <nikola@nikolahristov.tech>")
+ 		.author("Nikola R. Hristov <Nikola@Playform.Cloud>")
+ 		.arg(
+ 			Arg::new("Hide")
+ 				.short('H')
+ 				.long("Hide")
+ 				.action(SetTrue)
+ 				.display_order(1)
+ 				.value_name("HIDE")
+ 				.required(false)
+ 				.help("Hide output."),
+ 		)
- 				.display_order(1)
+ 				.display_order(2)
- 				.display_order(2)
+ 				.display_order(3)
- 				.display_order(3)
+ 				.display_order(4)
- 				.display_order(4)
+ 				.display_order(5)
- 				.display_order(5)
+ 				.display_order(6)
- 				.display_order(6)
+ 				.display_order(7)
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index bf4ce76..847b4ea 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
+ 			// TODO: Separate this into Entry/Exclude.rs
- 					true => std::fs::metadata(&Path).expect("Cannot Metadata.").is_dir() && Match,
+ 					true => {
+ 						std::fs::metadata(std::path::PathBuf::from(&Path))
+ 							.expect("Cannot Metadata.")
+ 							.is_dir() && Match
+ 					}
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index efd9808..11cc07e 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- 			let Output = if cfg!(target_os = "windows") {
- 				Command::new("cmd").args(["/C", Command.as_str()]).current_dir(Entry).output()
- 			} else {
- 				Command::new("sh").arg("-c").current_dir(Entry).arg(&Command).output()
- 			};
+ 			let Output = Command::new(Command.get(0).expect("Cannot Command."))
+ 				.args(&Command[1..])
+ 				.current_dir(Entry)
+ 				.output();
- 			Queue.push(async move {
- 				println!(
- 					"{}",
- 					String::from_utf8_lossy(&Output.await.expect("Cannot await.").stdout)
- 				);
- 			});
+ 			Queue.push(async move { Output.await.expect("Cannot await.").stdout });
- 				Queue.await;
+ 				println!("{}", String::from_utf8_lossy(&Queue.await));
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 8eef18b..9a9d6fa 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
- 			let mut Out = match cfg!(target_os = "windows") {
- 				true => Command::new("cmd")
- 					.args(["/C", &Command])
+ 			let mut Command = Command::new(Command.get(0).expect("Cannot Command."))
+ 				.args(&Command[1..])
- 					.expect("Cannot spawn."),
- 				false => Command::new("sh")
- 					.arg("-c")
- 					.current_dir(Entry)
- 					.arg(Command.clone())
- 					.stdout(Stdio::piped())
- 					.spawn()
- 					.expect("Cannot spawn."),
- 			}
+ 				.expect("Cannot spawn.")
- 				let Byte = Out.read(&mut Buffer).expect("Cannot read.");
+ 				let Byte = Command.read(&mut Buffer).expect("Cannot read.");
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index 5b62589..c3e16f8 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
+ 	pub Hide: Hide,
+ 			Hide: Option.Hide.clone(),
- 	Command, Parallel, Pattern, Separator, Struct as Option,
+ 	Command, Hide, Parallel, Pattern, Separator, Struct as Option,
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 4c946ac..82d1dc4 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- pub type Command = String;
+ pub type Command = Vec<String>;
+ pub type Hide = bool;
- 	pub Command: String,
+ 	pub Command: Command,
+ 	pub Hide: Hide,
- 	pub Parallel: bool,
+ 	pub Parallel: Parallel,
+ 			Hide: Fn().get_flag("Hide"),
- 				.map(|Command| Command.as_str())
- 				.collect::<Vec<_>>()
- 				.join(" "),
+ 				.map(|Command| Command.to_string())
+ 				.collect::<Vec<_>>(),

🗣️ Summary from v0.1.8 to vInn/v0.1.9 in .
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index 819f8a1..cfa5b96 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-             - uses: dependabot/fetch-metadata@v1.6.0
+             - uses: dependabot/fetch-metadata@v2.0.0
-             - uses: dependabot/fetch-metadata@v1.6.0
+             - uses: dependabot/fetch-metadata@v2.0.0
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index 09e8b75..0be30ba 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
-             - uses: pozil/auto-assign-issue@v1.13.0
+             - uses: pozil/auto-assign-issue@v1.14.0
diff --git a/Cargo.toml b/Cargo.toml
index 2d07668..b8e9a72 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- clap = { features = ["derive"], version = "4.5.3" }
+ clap = { features = ["derive"], version = "4.5.4" }
- description = "🍺 Run Command."
+ description = "🍺 Run."
- version = "0.1.8"
+ version = "0.1.9"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index 0e092d3..13ffed8 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- Community@Playform.Cloud. All complaints will be reviewed and investigated
+ community@playform.cloud. All complaints will be reviewed and investigated
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index b8ceeae..4700843 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- Community@Playform.Cloud. All complaints will be reviewed and investigated
+ community@playform.cloud. All complaints will be reviewed and investigated
diff --git a/README.md b/README.md
index 42ed6dd..f9edcac 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Inn]
+ # 🍺 [Inn]
- Inn -F astro.config.ts npx astro add astro-compress
+ Inn -F astro.config.ts npx astro add @playform/compress
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 773eb20..f301f4f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
- 		.author("Nikola R. Hristov <Nikola@Playform.Cloud>")
+ 		.author("Nikola R. Hristov <nikola@playform.cloud>")

🗣️ Summary from vInn/v0.1.9 to last commit in .
diff --git a/.cargo/Config.toml b/.cargo/Config.toml
new file mode 100644
index 0000000..5507528
--- /dev/null
+++ b/.cargo/Config.toml
+ [build]
+ target-dir = "Target"
+ 
+ [cargo-new]
+ vcs = "git"
+ 
+ [profile.release]
+ opt-level = 3
+ codegen-units = 1
+ debug = false
+ lto = true
+ panic = "abort"
diff --git a/.github/FUNDING.yml b/.github/FUNDING.yml
new file mode 100644
index 0000000..3ba6945
--- /dev/null
+++ b/.github/FUNDING.yml
+ open_collective: playform-cloud-collective
diff --git a/.github/workflows/Dependabot.yml b/.github/workflows/Dependabot.yml
index cfa5b96..387fece 100644
--- a/.github/workflows/Dependabot.yml
+++ b/.github/workflows/Dependabot.yml
-             - uses: dependabot/fetch-metadata@v2.0.0
+             - uses: dependabot/fetch-metadata@v2.2.0
-             - uses: dependabot/fetch-metadata@v2.0.0
+             - uses: dependabot/fetch-metadata@v2.2.0
diff --git a/.github/workflows/GitHub.yml b/.github/workflows/GitHub.yml
index 0be30ba..7b1e399 100644
--- a/.github/workflows/GitHub.yml
+++ b/.github/workflows/GitHub.yml
+             TERRAFORM_TELEMETRY: 0
-             - uses: pozil/auto-assign-issue@v1.14.0
+             - uses: pozil/auto-assign-issue@v2.0.0
diff --git a/.github/workflows/Rust.yml b/.github/workflows/Rust.yml
index cfdee9a..9edf181 100644
--- a/.github/workflows/Rust.yml
+++ b/.github/workflows/Rust.yml
+             TERRAFORM_TELEMETRY: 0
-             - uses: actions/checkout@v4.1.2
+             - uses: actions/checkout@v4.1.7
diff --git a/.gitignore b/.gitignore
index cbb2f71..34f0334 100644
--- a/.gitignore
+++ b/.gitignore
- Cargo.lock
+ /Target/*
- /target/*
- !/target/release
- /target/release/*
- !/target/release/*.deb
- !/target/release/*.exe
- !/target/release/Inn
- !/target/release/Innkeeper
+ !/Target/release
+ /Target/release/*
+ 
+ !/Target/release/*.deb
+ !/Target/release/*.exe
+ !/Target/release/PRun
+ !/Target/release/Run
diff --git a/build.rs b/build.rs
index 550762c..73ccc94 100644
--- a/build.rs
+++ b/build.rs
+ use serde::Deserialize;
+ #[derive(Deserialize)]
+ struct Toml {
+ 	package: Package,
+ }
+ 
+ #[derive(Deserialize)]
+ struct Package {
+ 	version: String,
+ }
+ 
+ 
- 		fs::read_to_string("Cargo.toml")
- 			.expect("Cannot Cargo.toml.")
- 			.lines()
- 			.find(|Line| Line.starts_with("version"))
- 			.expect("Cannot Version.")
- 			.split('=')
- 			.nth(1)
- 			.expect("Cannot nth.")
- 			.trim()
- 			.trim_matches('"')
+ 		(toml::from_str::<Toml>(&fs::read_to_string("Cargo.toml").expect("Cannot Cargo.toml."))
+ 			.expect("Cannot toml."))
+ 		.package
+ 		.version
diff --git a/Cargo.toml b/Cargo.toml
index b8e9a72..7185e08 100644
--- a/Cargo.toml
+++ b/Cargo.toml
+ [[bin]]
+ name = "PRun"
+ path = "Source/Library.rs"
+ 
+ [[bin]]
+ name = "Run"
+ path = "Source/Library.rs"
+ 
- clap = { features = ["derive"], version = "4.5.4" }
- tokio = { features = ["full"], version = "1.36.0" }
+ clap = { features = ["derive"], version = "4.5.11" }
+ futures = "0.3.30"
+ rayon = "1.10.0"
+ tokio = { version = "1.39.2", features = ["full"] }
+ num_cpus = "1.16.0"
+ 
+ [build-dependencies]
+ serde = { version = "1.0.204", features = ["derive"] }
+ toml = "0.8.17"
- default-run = "Inn"
- description = "🍺 Run."
+ default-run = "Run"
+ description = "🍺 Run —"
- name = "innkeeper"
- repository = "https://github.com/NikolaRHristov/Inn.git"
- version = "0.1.9"
+ name = "prun"
+ repository = "https://github.com/PlayForm/Run.git"
+ version = "0.1.0"
+ include = [
+ 	"Source/**/*",
+ 	"LICENSE",
+ 	"README.md",
+ 	"CHANGELOG.md",
+ 	"build.rs",
+ 	"Cargo.toml",
+ ]
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index 13ffed8..01e92b5 100644
--- a/CODE_OF_CONDUCT.md
+++ b/CODE_OF_CONDUCT.md
- community@playform.cloud. All complaints will be reviewed and investigated
+ Community@PlayForm.Cloud. All complaints will be reviewed and investigated
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
+ [homepage]: HTTPS://www.contributor-covenant.org
+ [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
+ [FAQ]: HTTPS://www.contributor-covenant.org/faq
+ [translations]: HTTPS://www.contributor-covenant.org/translations
diff --git a/CONTRIBUTING.md b/CONTRIBUTING.md
index 4700843..c390eae 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- community@playform.cloud. All complaints will be reviewed and investigated
+ Community@PlayForm.Cloud. All complaints will be reviewed and investigated
- [homepage]: https://www.contributor-covenant.org
- [v2.1]: https://www.contributor-covenant.org/version/2/1/code_of_conduct.html
- [Mozilla CoC]: https://github.com/mozilla/diversity
- [FAQ]: https://www.contributor-covenant.org/faq
- [translations]: https://www.contributor-covenant.org/translations
+ [homepage]: HTTPS://www.contributor-covenant.org
+ [v2.1]: HTTPS://www.contributor-covenant.org/version/2/1/code_of_conduct.html
+ [Mozilla CoC]: HTTPS://github.com/mozilla/diversity
+ [FAQ]: HTTPS://www.contributor-covenant.org/faq
+ [translations]: HTTPS://www.contributor-covenant.org/translations
diff --git a/LICENSE b/LICENSE
index c47b9fa..f236d76 100644
--- a/LICENSE
+++ b/LICENSE
- Copyright (c) 2023-2024 Nikola R. Hristov
+ Copyright (c) 2023-2024 PlayForm
diff --git a/README.md b/README.md
index f9edcac..706161c 100644
--- a/README.md
+++ b/README.md
- # 🍺 [Inn]
+ # 🍺 [Run] —
- Inn is a command-line tool designed to execute a specified command in all
- directories that match a certain pattern within a given root directory. It
- provides flexibility and efficiency in running commands across multiple
- directories with customizable patterns.
+ `Run` is a command-line tool that executes commands in multiple directories
+ simultaneously. It leverages parallel processing and concurrent `I/O` to
+ efficiently run tasks across directories.
- [Inn]: https://crates.io/crates/innkeeper
+ [Run]: HTTPS://crates.io/crates/prun
- ## Benchmark
+ ## Bench
- 			<pre>Inn -P .git ls</pre>
+ 			<pre>find -iname .git -execdir ls \;</pre>
- 			<pre>real    0m9.441s
- user    0m0.030s
- sys     0m0.046s</pre>
+ 			<pre>real    0m14.476s
+ user    0m5.260s
+ sys     0m7.526s</pre>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m14.293s +5s
- user    0m4.645s +4s
- sys     0m8.937s +8s</pre>
+ 			<pre>Run -P .git ls</pre>
- 	</tr>
- 	<tr>
- 			<pre>Inn -P .git git status</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m24.146s
+ 			<pre>real    0m7.194s
- sys     0m0.062s</pre>
- 		</td>
- 	</tr>
- 	<tr>
- 		<td>
- 			<pre>find -iname .git -type d -execdir ls \;</pre>
- 		</td>
- 		<td>
- 			<pre>real    0m28.584s +4s
- user    0m4.695s +4s
- sys     0m8.354s +8s</pre>
+ sys     0m0.045s</pre>
- 			<pre>Inn -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
+ 			<pre>find -iname .git -type d -execdir git status \;</pre>
- 			<pre>real    0m33.813s
- user    0m0.015s
- sys     0m0.060s</pre>
+ 			<pre>real    1m1.242s
+ user    0m4.080s
+ sys     0m6.354s</pre>
- 			<pre>find -iname .git -type d -execdir \
- bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
+ 			<pre>Run -P .git git status</pre>
- 			<pre>real    0m53.122s +20s
- user    0m9.449s +9s
- sys     0m14.442s +14s</pre>
+ 			<pre>real    0m21.947s
+ user    0m0.045s
+ sys     0m0.031s</pre>
- cargo install innkeeper
+ cargo install prun
- Inn .git git fetch upstream
+ Run .git git fetch upstream
- This command will fetch from `upstream` for all the `.git` repositories inside
- the current directory. Basically, it replaces the following command:
+ This command will fetch from upstream for all `.git` repositories inside the
+ current directory. It essentially replaces the following command:
- You can hide the command output by specifying an `-H` or `--Hide` parameter:
+ ## Options
- ```sh
- Inn -H -F package.json ncu -u
- ```
+ #### --File or -F:
- To specify a `--File` argument or `-F`, if you would like to search for a file
- instead of a directory, use:
+ Limit execution to files matching a certain pattern:
- Inn -F astro.config.ts npx astro add @playform/compress
+ Run -F astro.config.ts npx astro add @playform/compress
- Additionally, you can provide a `--Root` argument or `-R` to set the current
- working directory to a different folder. The default is `.`.
+ #### --Root or -R:
+ 
+ Set the current working directory to a different folder (default is `.`):
- Inn -R D:\Developer .git git fetch upstream
+ Run -R D:\Developer .git git fetch upstream
- Specify a `--Parallel` argument or `-P` if you would like to run commands in
- parallel. The default is sequential.
+ #### --Parallel or -P:
+ 
+ Run commands in `parallel` (default is `sequential`):
- Inn -P -R D:\Developer .git git fetch upstream
+ Run -P -R D:\Developer .git git fetch upstream
+ #### --Exclude:
+ 
+ Exclude certain files or directories (defailt is
+ `node_modules .git target dist vendor`)
+ 
+ #### --Pattern:
+ 
+ Specify a custom pattern for matching
+ 
- The code imports several crates:
+ `Run` relies on several Rust crates to provide its functionality:
+ 
+ -   `clap` - Parses command-line arguments
+ -   `rayon` - Enables parallel processing
+ -   `tokio` - Provides an asynchronous runtime
+ -   `walkdir` - Facilitates efficient filesystem traversal
- -   `clap` - For parsing command-line arguments.
- -   `tokio` - Enables parallel execution of tasks.
- -   `walkdir` - Facilitates filesystem traversal.
+ [Run]: HTTPS://crates.io/crates/prun
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index f301f4f..5eca35e 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
+ /// This function defines and configures command line arguments for the "Run" command.
+ /// It sets up various arguments such as File, Parallel, Root, Exclude, Pattern, and Command.
+ /// Each argument has specific properties like short and long flags, display order, value names, required status, help messages, and default values.
+ /// The function returns the parsed command line arguments using ArgMatches.
- 	Command::new("Inn")
+ 	Command::new("Run")
- 		.author("Nikola R. Hristov <nikola@playform.cloud>")
- 		.about("Run a command in all directories having a certain pattern.")
- 		.arg(
- 			Arg::new("Hide")
- 				.short('H')
- 				.long("Hide")
- 				.action(SetTrue)
- 				.display_order(1)
- 				.value_name("HIDE")
- 				.required(false)
- 				.help("Hide output."),
- 		)
+ 		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
+ 		.about("🍺 Run —")
- 				.help("Search file."),
+ 				.help("📝 File —"),
- 				.help("Execute code in parallel."),
+ 				.help("⏩ Parallel —"),
- 				.help("Current working directory.")
+ 				.help("📂 Root —")
- 				.help("Exclude pattern.")
+ 				.help("🚫 Exclude —")
- 				.help("Search pattern.")
+ 				.help("🔍 Pattern —")
- 				.help("Command to run."),
+ 				.help("🖥️ Command —"),
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 847b4ea..b771f11 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
+ /// This Rust function walks through a directory, filters out certain files based on exclusion criteria,
+ /// and returns a collection of paths.
+ ///
+ /// Arguments:
+ ///
+ /// some file system operations based on the provided configuration. Here's a breakdown of the
+ /// parameters:
+ ///
+ /// Returns:
+ ///
+ /// a `Vec<String>` containing paths that meet the specified criteria after processing the entries from
+ /// the directory specified by the `Root` parameter.
+ 		.follow_links(false)
- 					true => {
- 						std::fs::metadata(std::path::PathBuf::from(&Path))
- 							.expect("Cannot Metadata.")
- 							.is_dir() && Match
- 					}
+ 					true => match std::fs::metadata(std::path::PathBuf::from(&Path)) {
+ 						Ok(Metadata) => Metadata.is_dir() && Match,
+ 						Err(_Error) => false,
+ 					},
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 11cc07e..5e71ca9 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
- pub fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
- 	let mut Queue = Vec::new();
- 
+ /// The function takes an Option containing Entry, Separator, Pattern, Command, and other values,
+ /// processes the Entry based on the Pattern and Separator, executes a Command with the processed Entry
+ /// as the current directory, and prints the output of each Command execution.
+ ///
+ /// Arguments:
+ ///
+ /// The `Option` enum has fields named `Entry`, `Separator`, `Pattern`, `Command`, and possibly other
+ /// fields.
+ pub async fn Fn(Option { Entry, Separator, Pattern, Command, .. }: Option) {
+ 	let Queue: Vec<String> = futures::stream::iter(
- 		.into_iter()
+ 			.into_par_iter()
- 		.for_each(|Entry| {
- 			let Output = Command::new(Command.get(0).expect("Cannot Command."))
+ 			.collect::<Vec<String>>(),
+ 	)
+ 	.map(|Entry| {
+ 		let Command = Command.clone();
+ 
+ 		async move {
+ 			String::from_utf8_lossy(
+ 				&tokio::process::Command::new(Command.get(0).expect("Cannot Command."))
- 				.output();
- 
- 			Queue.push(async move { Output.await.expect("Cannot await.").stdout });
- 		});
- 
- 	tokio::runtime::Builder::new_multi_thread()
- 		.enable_all()
- 		.build()
- 		.expect("Cannot Runtime.")
- 		.block_on(async {
- 			for Queue in Queue {
- 				println!("{}", String::from_utf8_lossy(&Queue.await));
+ 					.output()
+ 					.await
+ 					.expect("Cannot Output.")
+ 					.stdout,
+ 			)
+ 			.to_string()
+ 	.buffer_unordered(num_cpus::get())
+ 	.collect()
+ 	.await;
+ 
+ 	Queue.par_iter().for_each(|Output| println!("{}", Output));
- use tokio::process::Command;
+ use futures::stream::StreamExt;
+ use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 9a9d6fa..2c80c56 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
+ /// Executes a command with arguments in a specific directory for each entry in the given list.
+ ///
+ /// # Arguments
+ ///
+ /// * `Option` - A struct containing `Command`, `Entry`, `Pattern`, `Separator`, and other optional fields.
+ ///
+ /// # Example
+ ///
+ /// ```
+ /// use std::process::{Command, Stdio};
+ ///
+ /// let options = Option { Command: vec!["ls".to_string()], Entry: vec!["/path/to/dir".to_string()], Pattern: "pattern", Separator: '/'.to_string() };
+ /// Fn(options);
+ /// ```
diff --git a/Source/Library.rs b/Source/Library.rs
index 982f1a3..62cfaff 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
- fn main() {
- 	(Struct::Binary::Command::Struct::Fn().Fn)()
+ #[tokio::main]
+ async fn main() {
+ 	(Struct::Binary::Command::Struct::Fn().Fn)().await
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index faff4af..2a79ad1 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
- pub mod Entry;
- pub mod Option;
- 
- use crate::Fn::Binary::Command::{Parallel, Sequential};
- 
- #[derive(Debug)]
- 	pub Fn: fn(),
+ 	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
- 			Fn: || {
+ 			Fn: Box::new(|| {
+ 				Box::pin(async move {
- 						Parallel::Fn(Option);
+ 							Parallel::Fn(Option).await;
- 			},
+ 				})
+ 			}),
+ 
+ pub mod Entry;
+ pub mod Option;
+ 
+ use crate::Fn::Binary::Command::{Parallel, Sequential};
+ 
+ use futures::Future;
+ use std::pin::Pin;
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c3e16f8..5b62589 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
- 	pub Hide: Hide,
- 			Hide: Option.Hide.clone(),
- 	Command, Hide, Parallel, Pattern, Separator, Struct as Option,
+ 	Command, Parallel, Pattern, Separator, Struct as Option,
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 82d1dc4..e1294e3 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
- pub type Hide = bool;
+ 
- 	pub Hide: Hide,
- 			Hide: Fn().get_flag("Hide"),

