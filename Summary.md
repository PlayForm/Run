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

🗣️ Summary from Run/v0.0.7 to last commit in .
diff --git a/Cargo.toml b/Cargo.toml
index 8fc9504..d9635d8 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- version = "0.0.7"
+ version = "0.1.0"
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

🗣️ Summary from first commit to Run/v0.0.7 in .
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
index b5d4b17..8fc9504 100644
--- a/Cargo.toml
+++ b/Cargo.toml
- clap = { features = ["derive"], version = "4.5.9" }
- tokio = { features = ["full"], version = "1.38.1" }
+ clap = { features = ["derive"], version = "4.5.11" }
+ futures = "0.3.30"
+ rayon = "1.10.0"
+ tokio = { version = "1.39.1", features = ["full"] }
+ num_cpus = "1.16.0"
- toml = "0.8.15"
+ toml = "0.8.16"
- description = "🍺 Run"
+ description = "🍺 Run —"
- version = "0.0.5"
+ version = "0.0.7"
diff --git a/CODE_OF_CONDUCT.md b/CODE_OF_CONDUCT.md
index d35c668..01e92b5 100644
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
index c749fbe..c390eae 100644
--- a/CONTRIBUTING.md
+++ b/CONTRIBUTING.md
- community@playform.cloud. All complaints will be reviewed and investigated
+ Community@PlayForm.Cloud. All complaints will be reviewed and investigated
diff --git a/README.md b/README.md
index 4732518..77f3dee 100644
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
- -   `tokio` - Enables parallel execution of tasks.
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
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 7f8a2ca..b771f11 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
- /// * ``: It looks like you have a function that takes an `Option` struct as a parameter and performs
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index a0b67b8..ff9548b 100644
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
- 			Queue.push(async move { Output.await.expect("Cannot Output.").stdout });
- 		});
- 
- 	tokio::runtime::Builder::new_multi_thread()
- 		.enable_all()
- 		.build()
- 		.expect("Cannot Runtime.")
- 		.block_on(async {
- 			println!("{}", String::from_utf8_lossy(&Queue.remove(0).await));
- 
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
index 8e9ef64..62cfaff 100644
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

