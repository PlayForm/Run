# 🍺 [Run] —

`Run` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

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
			<pre>real    0m14.476s
user    0m5.260s
sys     0m7.526s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git ls</pre>
		</td>
		<td>
			<pre>real    0m7.194s
user    0m0.030s
sys     0m0.045s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -type d -execdir git status \;</pre>
		</td>
		<td>
			<pre>real    1m1.242s
user    0m4.080s
sys     0m6.354s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git git status</pre>
		</td>
		<td>
			<pre>real    0m21.947s
user    0m0.045s
sys     0m0.031s</pre>
		</td>
	</tr>
</table>

## Installation

```sh
cargo install prun
```

## Usage

```sh
Run .git git fetch upstream
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

## Options

#### --File or -F:

Limit execution to files matching a certain pattern:

```sh
Run -F astro.config.ts npx astro add @playform/compress
```

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Run -R D:\Developer .git git fetch upstream
```

#### --Parallel or -P:

Run commands in `parallel` (default is `sequential`):

```sh
Run -P -R D:\Developer .git git fetch upstream
```

#### --Exclude:

Exclude certain files or directories (defailt is
`node_modules .git target dist vendor`)

#### --Pattern:

Specify a custom pattern for matching

## Dependencies

`Run` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Run]: HTTPS://crates.io/crates/prun

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
