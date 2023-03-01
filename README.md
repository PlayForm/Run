# [inn] 🍺

Inn is a little Rust utility designed to execute commmands for multiple
directories in parallel.

[inn]: https://crates.io/crates/innkeeper

## Installation

```sh
cargo install innkeeper
```

## Usage

```sh
inn .git git fetch upstream
```

This will fetch from upstream for all the .git repositories inside the current
directory. Basically it replaces:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

You can also provide a `--root` argument or `-r` which sets the current working
directory to a different folder.

```sh
inn -r D:\Developer .git git fetch upstream
```

Specify a `--parallel` argument or `-p` if you would like to run functions in
parallel.

```sh
inn -p -r D:\Developer .git git fetch upstream
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
