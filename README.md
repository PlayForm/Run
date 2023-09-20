# [Inn] 🍺

Inn is a tiny Rust utility that lets execute commmands in different directories at the same time.

[Inn]: https://crates.io/crates/innkeeper

## Installation

```sh
cargo install innkeeper
```

## Usage

```sh
inn .git git fetch upstream
```

This will fetch from upstream for all the `.git` repositories inside the current
directory. Basically it replaces:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

Specify a `--file` argument or `-f` if you would like to search for file instead
of a directory. Default is `false` or no flag at all.

```sh
inn -f astro.config.ts npx astro add astro-compress
```

You can also provide a `--root` argument or `-r` which sets the current working
directory to a different folder. Default is `.`.

```sh
inn -r D:\Developer .git git fetch upstream
```

Specify a `--parallel` argument or `-p` if you would like to run functions in
parallel. Default is sequential.

```sh
inn -p -r D:\Developer .git git fetch upstream
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
