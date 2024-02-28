# 🍺 [Inn]

Inn is a tiny Rust utility that lets execute commmands in different directories
concurrently.

[Inn]: https://crates.io/crates/innkeeper

## Installation

```sh
cargo install innkeeper
```

## Usage

```sh
Inn .git git fetch upstream
```

This will fetch from upstream for all the `.git` repositories inside the current
directory. Basically it replaces:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

Specify a `--File` argument or `-F` if you would like to search for file instead
of a directory. Default is `false` or no flag at all.

```sh
Inn -F astro.config.ts npx astro add astro-compress
```

You can also provide a `--Root` argument or `-R` which sets the current working
directory to a different folder. Default is `.`.

```sh
Inn -R D:\Developer .git git fetch upstream
```

Specify a `--Parallel` argument or `-P` if you would like to run commands in
parallel. Default is sequential.

```sh
Inn -P -R D:\Developer .git git fetch upstream
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
