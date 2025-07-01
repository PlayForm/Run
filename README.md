# [Run] 🍺 (`prun`)

`Run` is a blazingly fast, concurrent command-line utility for executing
commands across multiple directories that match a specific pattern.

It's designed as a modern, high-performance replacement for complex
`find ... -execdir` pipelines, leveraging Rust's full potential for safe,
parallel, and asynchronous operations.

[Run]: https://crates.io/crates/prun

## Key Features

- **Blazing Fast**: Drastically out-performs traditional shell equivalents by
  using an optimized parallel directory walker and asynchronous command
  execution.
- **Concurrent by Default**: Runs tasks in parallel using a hybrid `rayon` and
  `tokio` approach to maximize CPU and I/O efficiency.
- **Intuitive Syntax**: Replaces complex `find` and `xargs` syntax with simple,
  readable flags.
- **Smart Exclusion**: Comes with sensible defaults to automatically ignore
  common directories like `node_modules`, `.git`, and `target`.
- **Cross-Platform**: Built with Rust, `Run` works consistently across Windows,
  macOS, and Linux.

---

## Performance Benchmarks

`Run` is significantly faster than its `find -execdir` equivalent. The
benchmarks below were performed on a Windows machine with an NVMe SSD and 16 CPU
cores, scanning a large developer directory with hundreds of Git repositories.

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
			<pre>real    0m10.562s
user    0m3.092s
sys     0m6.789s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git -C 'ls'</pre>
		</td>
		<td>
			<pre>real    0m3.484s
user    0m0.045s
sys     0m0.031s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -execdir git status \;</pre>
		</td>
		<td>
			<pre>real    0m30.352s
user    0m3.218s
sys     0m7.286s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git -C 'git status'</pre>
		</td>
		<td>
			<pre>real    0m7.412s
user    0m0.045s
sys     0m0.031s</pre>
		</td>
	</tr>
</table>

**Why is it so much faster?**

- `find` executes commands sequentially, one after another.
- `Run` walks the directory tree once and then executes all commands in
  parallel, taking full advantage of modern multi-core processors and fast I/O.

---

## Installation

You can install `Run` directly from [Crates.io](https://crates.io/crates/prun)
using Cargo:

```sh
cargo install prun
```

The installed binary is `Run`. You may want to create a symlink or alias like
`prun` for convenience.

---

## Usage

The core idea is to define a `Pattern` (like a file or directory name) that
identifies the locations where you want to execute one or more `Command`s.

```
A utility to run commands in directories matching a pattern.

Usage: Run [OPTIONS] --Pattern <PATTERN> --Command <COMMAND>...

Options:
  -F, --File               Target files directly instead of directories containing a pattern
  -P, --Parallel           Execute commands in parallel across all found directories
  -R, --Root <DIRECTORY>   The root directory to start the search from [default: .]
  -E, --Exclude <PATTERNS> A space-separated list of glob patterns to exclude from the search [default: **/{node_modules,.git,target,dist,vendor}/**/*]
  -C, --Command <COMMAND>  The command to execute. Can be specified multiple times
  -h, --help               Print help
  -V, --version            Print version
```

### Basic Examples

**1. Fetch all Git repositories in the current directory.**

This will find every directory containing a `.git` subfolder and run `git fetch`
inside it.

```sh
Run --Pattern .git --Command "git fetch --all"
```

_This is the high-performance equivalent of:_
`find . -type d -name .git -execdir git fetch --all \;`

**2. Run multiple commands.**

Clean the `target` directory for all Rust projects.

```sh
Run --Pattern Cargo.toml --Command "cargo clean" --Command "echo Cleaned"
```

**3. Run `npm install` in all projects with a `package.json`.**

The default `--Exclude` rules will automatically prevent it from running inside
`node_modules`.

```sh
Run --Pattern package.json --Command "npm install"
```

### Advanced Options

- **`-R, --Root <DIRECTORY>`**: Start searching from a different directory.

    ```sh
    Run --Root ~/projects --Pattern .git --Command "git status -s"
    ```

- **`-P, --Parallel`**: While parallel is the default behavior, this flag exists
  for clarity. You can run things sequentially by omitting it if sequential
  execution is ever implemented as the default. _(Currently, it defaults to
  parallel)_.

- **`-E, --Exclude <PATTERNS>`**: Override the default exclusion patterns. Note
  that patterns are space-separated.

    ```sh
    # Also exclude 'build' and 'cache' directories
    Run --Pattern .git -E "**/{node_modules,target,dist,build,cache}/**/*" -C "git status"
    ```

- **`-F, --File`**: Target files directly based on the pattern. This is less
  common. For example, to run a linter on all TypeScript config files:
    ```sh
    Run -F --Pattern "tsconfig.json" --Command "npx eslint tsconfig.json"
    ```

---

## Dependencies

`Run` stands on the shoulders of giants. It is built with these excellent crates
from the Rust ecosystem:

- **[`clap`](https://crates.io/crates/clap)**: For robust and ergonomic
  command-line argument parsing.
- **[`walkdir`](https://crates.io/crates/walkdir)**: For efficient,
  cross-platform directory traversal.
- **[`globset`](https://crates.io/crates/globset)**: For high-performance file
  path matching against glob patterns.
- **[`tokio`](https://crates.io/crates/tokio)**: The de-facto asynchronous
  runtime for writing fast and reliable network and I/O-bound applications.
- **[`rayon`](https://crates.io/crates/rayon)**: A data-parallelism library that
  makes it easy to convert sequential computations into parallel ones.
- **[`futures`](https://crates.io/crates/futures)**: Provides core abstractions
  for asynchronous programming.
- **[`crossbeam-queue`](https://crates.io/crates/crossbeam-queue)**: For
  high-performance, lock-free queues used in our concurrent pipeline.
- **[`once_cell`](https://crates.io/crates/once_cell)**: For safe, efficient,
  and thread-safe one-time initialization of global state.

## Changelog

For a detailed history of changes, see [`CHANGELOG.md`](CHANGELOG.md).
