# 🍺 [Run] —

`Run` is a command-line tool that executes commands in multiple directories
concurrently.

[Run]: HTTPS://crates.io/crates/prun

## Bench

<table>
	<tr>
		<th>Command:</th>
		<th>Time:</th>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -execdir ls \;</pre>
		</td>
		<td>
			<pre>real    0m17.340s
user    0m6.214s
sys     0m9.138s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git -C ls</pre>
		</td>
		<td>
			<pre>real    0m8.480s
user    0m0.046s
sys     0m0.046s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -execdir git status \;</pre>
		</td>
		<td>
			<pre>real    1m19.070s
user    0m5.385s
sys     0m7.357s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git -C 'git status'</pre>
		</td>
		<td>
			<pre>real    0m26.170s
user    0m0.030s
sys     0m0.046s</pre>
		</td>
	</tr>
</table>

## 🚀 Installation

```sh
cargo install prun
```

## 🛠️ Usage

`Run` can be used with various options:

```sh
🍺 Run —

Usage: Run [OPTIONS] --Command <COMMAND> <PATTERN>

Arguments:
  <PATTERN>  🔍 Pattern — [default: .]

Options:
  -F, --File               📝 File —
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: "node_modules .git target dist vendor"]
  -C, --Command <COMMAND>  🖥️ Command —
  -h, --help               Print help
  -V, --version            Print version
```

```sh
Run .git -C 'git fetch upstream'
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

## Options

#### --Command or -C:

The command to execute:

```sh
Run .git -C 'git status'
```

or multiple commands:

```sh
Run .git -C 'git status' -C 'git add .' -C 'git commit'
```

#### --File or -F:

Limit execution to files matching a certain pattern:

```sh
Run -F astro.config.ts -C 'npx astro add @playform/compress'
```

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Run -R D:\Developer .git -C 'git fetch upstream'
```

#### --Parallel or -P:

Run commands in `parallel` (default is `sequential`):

```sh
Run -P -R D:\Developer .git -C 'git fetch upstream'
```

#### --Exclude:

Exclude certain files or directories (defailt is
`node_modules .git target dist vendor`)

#### --Pattern:

Specify a custom pattern for matching

## Dependencies

`Run` relies on several Rust crates to provide its functionality:

-   [`clap`](https://crates.io/crates/clap) (v4.5.17) - A powerful and flexible
    command-line argument parser. The "derive" feature is used to simplify the
    creation of command-line interfaces through derive macros.

-   [`walkdir`](https://crates.io/crates/walkdir) (v2.5.0) - Provides an efficient
    and cross-platform way to recursively traverse directories. This is useful
    for filesystem operations and searching.

-   [`futures`](https://crates.io/crates/futures) (v0.3.30) - Offers abstractions
    for asynchronous programming in Rust. It's used in conjunction with tokio to
    handle asynchronous operations effectively.

-   [`rayon`](https://crates.io/crates/rayon) (v1.10.0) - Enables easy parallelism
    for data-parallel tasks. It's used to parallelize CPU-bound operations,
    improving performance on multi-core systems.

-   [`tokio`](https://crates.io/crates/tokio) (v1.40.0) - An asynchronous runtime
    for Rust, providing essential building blocks for writing reliable
    asynchronous applications. The "full" feature set is used to enable all
    tokio functionality.

-   [`num_cpus`](https://crates.io/crates/num_cpus) (v1.16.0) - A small crate that
    determines the number of CPUs on the current system. This is useful for
    optimizing parallel workloads.

-   [`once_cell`](https://crates.io/crates/once_cell) (v1.19.0) - Provides a way
    to perform lazy static initialization. It's often used for global variables
    or singletons that need to be initialized only once.

These dependencies work together to provide a robust, efficient, and
user-friendly command-line tool capable of handling parallel and asynchronous
operations while efficiently traversing filesystems.

[Run]: HTTPS://crates.io/crates/prun

## Changelog

See [`CHANGELOG.md`](CHANGELOG.md) for a history of changes to this CLI.
