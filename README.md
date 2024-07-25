# 🍺 [Run] —

`Run` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

[Run]: HTTPS://crates.io/crates/prun

## Benchmark

<table>
	<tr>
		<th>Command:</th>
		<th>Time:</th>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git ls</pre>
		</td>
		<td>
			<pre>real    0m9.441s
user    0m0.030s
sys     0m0.046s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -type d -execdir ls \;</pre>
		</td>
		<td>
			<pre>real    0m14.293s +5s
user    0m4.645s +4s
sys     0m8.937s +8s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git git status</pre>
		</td>
		<td>
			<pre>real    0m24.146s
user    0m0.030s
sys     0m0.062s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -type d -execdir ls \;</pre>
		</td>
		<td>
			<pre>real    0m28.584s +4s
user    0m4.695s +4s
sys     0m8.354s +8s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Run -P .git 'git add . && git ecommit && git sync'</pre>
		</td>
		<td>
			<pre>real    0m33.813s
user    0m0.015s
sys     0m0.060s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -type d -execdir \
bash -c 'git add . && git ecommit && git sync' \;</pre>
		</td>
		<td>
			<pre>real    0m53.122s +20s
user    0m9.449s +9s
sys     0m14.442s +14s</pre>
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

#### --Separator:

Define a custom separator

## Dependencies

`Run` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Run]: HTTPS://crates.io/crates/prun

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
