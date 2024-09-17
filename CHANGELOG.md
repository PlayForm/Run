## 0.1.2

### Changed

-   Updated `once_cell` dependency to version 1.19.0
-   Updated version number to 0.1.2 in Cargo.toml
-   Simplified project description in README.md
-   Updated benchmark results in README.md
-   Changed find command example in README.md to use `-iname` instead of `-name`
-   Added detailed usage instructions and options to README.md
-   Updated dependencies section in README.md with more detailed information

### Improved

-   Enhanced parallel execution in `Source/Fn/Binary/Command/Parallel.rs`:
    -   Added thread pool and semaphore for better resource management
    -   Improved error handling and asynchronous execution
-   Updated documentation for the `Fn` function in
    `Source/Fn/Binary/Command/Parallel.rs`

### Fixed

-   Changed `use tokio::sync::Mutex;` to `use std::sync::{Arc, Mutex};` for
    better compatibility

## 0.1.1

### Added

-   Added support for multiple commands using the `-C` or `--Command` flag.
-   Introduced a new GPG module to handle Git commit signing.
-   Added a Process module to encapsulate asynchronous command execution.

### Changed

-   Updated various dependencies to their latest versions:
    -   `clap` to 4.5.17
    -   `tokio` to 1.40.0
    -   `serde` to 1.0.210
    -   `toml` to 0.8.19
-   Updated the project repository URL to `HTTPS://GitHub.Com/PlayForm/Run.git`.
-   Improved the README with clearer usage instructions and examples.
-   Renamed the binary target from `Innkeeper` to `InnKeeper` for consistency.
-   Enhanced error handling during parallel command execution.

### Fixed

-   Resolved an issue where parallel execution could deadlock when Git commit
    signing was enabled.

## 0.1.0

### Breaking Changes

-   Removed the `--Separator` option.

### Changed

-   Updated version number from 0.0.7 to 0.1.0 in `Cargo.toml`

### Improved

-   Enhanced `README.md` with more detailed information and updated benchmarks:
    -   Changed "Benchmark" section to "Bench"
    -   Updated benchmark results with new timings
    -   Removed one benchmark example
    -   Updated description for the `--Exclude` option
    -   Removed the `--Separator` option description
-   Refined command-line argument descriptions in `Source/Fn/Binary/Command.rs`
-   Updated author information in `Source/Fn/Binary/Command.rs`

### Fixed

-   Corrected import in `Source/Fn/Binary/Command/Parallel.rs`:
    -   Changed `stream::iter` to `futures::stream::iter`
    -   Updated import statement for `StreamExt`

## 0.0.7

### Breaking Changes

-   Updated `clap` dependency to version 4.5.11: Dependency updates can
    sometimes introduce breaking changes if the new version is not backward
    compatible.
-   Updated `tokio` dependency to version 1.39.1 with full features: Similar to
    the `clap` update, this could introduce breaking changes if the new version
    of `tokio` is not backward compatible.
-   Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
    support: Major refactoring often introduces breaking changes, especially if
    the public API or expected behavior changes.
-   Updated `Source/Library.rs` to use async main function: Changing the main
    function to async can be a breaking change if users were not expecting this
    and need to adjust their code accordingly.
-   Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
    async support: Refactoring for type safety and async support can introduce
    breaking changes if the function signatures or expected usage patterns
    change.

### Added

-   New `.cargo/Config.toml` file with build and profile configurations
-   Added `futures = "0.3.30"` dependency
-   Added `rayon = "1.10.0"` dependency
-   Added `num_cpus = "1.16.0"` dependency

### Changed

-   Updated `.gitignore` to use "Target" instead of "target" for consistency
-   Updated `clap` dependency to version 4.5.11
-   Updated `tokio` dependency to version 1.39.1 with full features
-   Changed project description to "🍺 Run —"
-   Updated repository URL to "https://github.com/PlayForm/Run.git"

### Improved

-   Enhanced `README.md` with more detailed feature descriptions and usage
    instructions
-   Added documentation for command-line options in `README.md`
-   Improved error handling and added documentation in various source files

### Refactored

-   Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
    support
-   Updated `Source/Library.rs` to use async main function
-   Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
    async support

## 0.0.6

### Breaking Changes

-   Updated `clap` dependency to version 4.5.11: As mentioned before, dependency
    updates can introduce breaking changes.
-   Updated `tokio` dependency to version 1.39.1: Similar to the `clap` update,
    this could introduce breaking changes.
-   Refactored Parallel execution to use async/await and `tokio`: This is likely
    a breaking change as it changes the execution model.
-   Updated main function to use `tokio` runtime: Changing the main function to
    use a specific runtime can be a breaking change.

### Added

-   New `.github/FUNDING.yml` file with Open Collective funding information
-   Added `serde = { version = "1.0.204", features = ["derive"] }` as a build
    dependency
-   Added `toml = "0.8.16"` as a build dependency

### Changed

-   Updated various GitHub Action workflows
-   Updated `clap` dependency to version 4.5.11
-   Updated `tokio` dependency to version 1.39.1
-   Changed copyright holder in `LICENSE` file to PlayForm
-   Removed `Cargo.lock` from `.gitignore`

### Improved

-   Enhanced `build.rs` script to use `serde` for parsing `Cargo.toml`
-   Improved `README.md` with more detailed feature descriptions
-   Added extensive documentation to various source files

### Refactored

-   Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
    support
-   Updated `Source/Library.rs` to use async main function
-   Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
    async support

## 0.0.5

### Breaking Changes

-   Updated `tokio` dependency to version 1.37.0: Dependency updates can
    introduce breaking changes.

### Changed

-   Updated `tokio` dependency to version 1.37.0
-   Changed project description to "🍺 Run"
-   Updated version number to 0.0.5

### Improved

-   Enhanced `README.md` with more detailed information about the tool's
    features and usage
-   Improved command-line argument descriptions in `Source/Fn/Binary/Command.rs`
-   Added documentation to `Source/Fn/Binary/Command/Entry.rs` and
    `Source/Fn/Binary/Command/Parallel.rs`

### Fixed

-   Updated links in `CODE_OF_CONDUCT.md` and `CONTRIBUTING.md` to use HTTPS

## 0.0.4

### Changed

-   Updated version number to 0.0.4

### Improved

-   Enhanced `README.md` with more detailed information about the tool's
    features and usage
-   Refined command-line argument descriptions in `Source/Fn/Binary/Command.rs`

## 0.0.3

### Changed

-   Updated version number to 0.0.3

## 0.0.2

### Breaking Changes

-   Added new binary targets: "Inn" and "InnKeeper": Adding new binaries is not
    a breaking change, but if it involved changes to existing binaries or their
    interfaces, it could be.

### Added

-   New binary targets: "Inn" and "InnKeeper"

### Changed

-   Updated version number from 0.0.1 to 0.0.2

## 0.0.1

-   Initial release
