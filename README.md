# 🍺 [Inn]

Innkeeper is a command-line tool designed to execute a specified command in all
directories that match a certain pattern within a given root directory. It
provides flexibility and efficiency in running commands across multiple
directories with customizable patterns.

[Inn]: https://crates.io/crates/innkeeper

## Benchmark

<table>
	<tr>
		<th>Command:</th>
		<th>Time:</th>
	</tr>
	<tr>
		<td>
			<pre>Inn -P .git ls</pre>
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
user    0m4.645s
sys     0m8.937s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Inn -P .git git status</pre>
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
user    0m4.695s
sys     0m8.354s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>Inn -P .git 'git add . && git commit -m "squash!" && git sync'</pre>
		</td>
		<td>
			<pre>real    0m33.813s
user    0m0.015s
sys     0m0.060s</pre>
		</td>
	</tr>
	<tr>
		<td>
			<pre>find -iname .git -type d -execdir
bash -c 'git add . && git commit -m "squash!" && git sync' \;</pre>
		</td>
		<td>
			<pre>real    0m53.122s
user    0m9.449s
sys     0m14.442s</pre>
		</td>
	</tr>
</table>

## Installation

```sh
cargo install innkeeper
```

## Usage

```sh
Inn .git git fetch upstream
```

This command will fetch from upstream for all the .git repositories inside the
current directory. Essentially, it replaces the following shell command:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

To specify a `--File` argument or `-F`, if you would like to search for a file
instead of a directory, use:

```sh
Inn -F astro.config.ts npx astro add astro-compress
```

Additionally, you can provide a `--Root` argument or `-R` to set the current
working directory to a different folder. The default is `.`.

```sh
Inn -R D:\Developer .git git fetch upstream
```

Specify a `--Parallel` argument or `-P` if you would like to run commands in
parallel. The default is sequential.

```sh
Inn -P -R D:\Developer .git git fetch upstream
```

## Dependencies

The code imports several crates:

-   `clap` - For parsing command-line arguments.
-   `tokio` - Enables parallel execution of tasks.
-   `walkdir` - Facilitates filesystem traversal.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
