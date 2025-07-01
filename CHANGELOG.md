## 0.1.8

### Improved

- **Exclusion Pattern Defaults**: Updated the default exclusion pattern to use glob syntax (`**/{node_modules,.git,target,dist,vendor}/**/*`) for more precise matching of entire directories
- **Optimized Directory Traversal**: Refactored entry filtering logic to:
  - Use `filter_entry` for early exclusion of entire directories (pruning)
  - Simplify file filtering with direct path checks
  - Eliminate redundant pattern matching operations

## 0.1.7

### Improved

- **Massive Performance Overhaul**: The application's core logic has been
  re-architected for maximum performance, resulting in a **~40% reduction in
  total CPU time** and a **~86% reduction in system call overhead**. The
  application is now definitively I/O bound on the filesystem walk, its ideal
  state.
- **Efficient Exclusion Filtering**: Replaced naive string matching with the
  high-performance `globset` crate for path exclusions. This compiles patterns
  once and matches them against thousands of paths far more efficiently.
- **Optimized Command Processing**: Commands are now pre-parsed and analyzed for
  GPG requirements _once_ at startup, eliminating redundant string splitting and
  analysis inside the hot execution loops.
- **Correct Async I/O in Sequential Mode**: Fixed a critical performance bug in
  sequential mode by replacing blocking `std::process::Command` with the
  non-blocking `tokio::process::Command`, preventing the async runtime from
  stalling.
- **Idiomatic Caching**: Replaced a manual, two-flag atomic cache for GPG status
  with the industry-standard `once_cell::sync::OnceCell`, resulting in cleaner,
  safer, and more readable code.
- **Robust Error Handling**: Replaced most `.expect()` calls and panics with
  proper `Result` propagation and descriptive `eprintln!` messages, making the
  application more resilient to failures like file permission errors or failed
  command executions.

### Refactored

- **Consistent Code Style**: Refactored the entire codebase to use `PascalCase`
  for all variables and types as per the new style guide, enhancing readability
  and consistency.
- **Enhanced Documentation**: Added comprehensive, high-quality documentation
  (doc comments) to all public functions and structs, clearly explaining their
  purpose, arguments, and behavior.
- **Streamlined Path Logic**: The logic for finding target directories has been
  improved to use efficient `Path` and `PathBuf` methods instead of lossy and
  wasteful string splitting and joining.
- **Centralized CLI Argument Parsing**: Encapsulated `clap` argument parsing
  within a `once_cell::sync::Lazy` static to guarantee it runs only once and
  provides a clean, global access point to configuration.

## 0.1.6

### Removed

- Removed `Source/Graph.md` documentation with Mermaid diagrams as the execution
  flow documentation was deprecated.

### Change

- Updated dependencies:
    - Removed `unbug` dependency
    - Simplified `rayon` dependency syntax in Cargo.toml

### Improved

- Refactored parallel execution in `Command/Parallel.rs`:
    - Reintroduced `rayon` parallel iterators combined with `crossbeam-queue`
      for efficient work distribution
    - Implemented hybrid async/parallel model using `tokio::sync::mpsc` for
      output handling
    - Simplified resource management with atomic queue-based entry distribution
    - Added proper thread pool sizing based on available CPU cores
    - Improved error handling for channel communication failures

## 0.1.5

### Add

- Added `Source/Graph.md` with Mermaid diagrams documenting execution flow and
  command processing.

### Change

- Updated dependencies:
    - `clap` to 4.5.35
    - `once_cell` to 1.21.3
    - `tokio` to 1.44.1
- Updated author email address to `<Source/Open@PlayForm.Cloud>` in CLI output.

### Improved

- Optimized exclusion filtering in `Command/Entry.rs`:
    - Avoid unnecessary cloning of exclude patterns
    - Improved variable naming and code clarity
- Refactored parallel execution in `Command/Parallel.rs`:
    - Replaced `crossbeam-queue` and `rayon` with `futures::stream` for better
      async integration
    - Simplified concurrency model and improved resource management
- Enhanced error handling in sequential command execution:
    - Added status code checking and error reporting in `Command/Sequential.rs`

## 0.1.4

### Add

- Added Knowledge.dot to visualize project structure.

### Change

- Updated dependencies:
    - `clap` to 4.5.32
    - `crossbeam-queue` to 0.3.12
    - `futures` to 0.3.31
    - `once_cell` to 1.21.0
    - `tokio` to 1.44.0
    - `unbug` to 0.4.0
- Updated build-dependencies:
    - `serde` to 1.0.219
    - `toml` to 0.8.20
- Modified project description to "Run 🍺" and updated the edition to "2024" in
  Cargo.toml.
- Updated author information in Source/Fn/Binary/Command.rs to "Source ✍🏻 Open
  👐🏻 <Source/Open@PlayForm.Cloud>".
- Updated help messages in Source/Fn/Binary/Command.rs for brevity.

### Improved

- Enhanced parallel execution in Source/Fn/Binary/Command/Parallel.rs:
    - Refactored for improved performance and clarity, using par_iter for
      parallel processing.
    - Improved error handling and queue management.
    - Removed unnecessary mutex locking.
    - Streamlined worker task execution.
- Removed redundant filtering in Source/Fn/Binary/Command/Entry.rs.

## 0.1.3

### Improved

- Enhanced parallel execution in Source/Fn/Binary/Command/Parallel.rs:
    - Leverages `tokio::sync::mpsc` for improved channel communication.
    - Refactored for clarity and conciseness.

### Change

- Updated documentation for the Fn function in
  Source/Fn/Binary/Command/Parallel.rs:
    - Provides a more concise and informative description of the function's
      purpose.
    - Includes a revised, more practical example of how to use the function.
- Minor code style and formatting adjustments for consistency.

## 0.1.2

### Change

- Updated once_cell dependency to version 1.19.0
- Updated version number to 0.1.2 in Cargo.toml
- Simplified project description in README.md
- Updated benchmark results in README.md
- Changed find command example in README.md to use -iname instead of -name
- Added detailed usage instructions and options to README.md
- Updated dependencies section in README.md with more detailed information
- Changed `use tokio::sync::Mutex;` to `use std::sync::{Arc, Mutex};` for better
  compatibility

### Add

- Added `crossbeam-queue = "0.3.11"` dependency

### Improved

- Enhanced parallel execution in Source/Fn/Binary/Command/Parallel.rs:
    - Leverages `crossbeam-queue` for managing entries concurrently.
    - Utilizes a thread pool for parallel processing of entries.
- Updated documentation for the Fn function in
  Source/Fn/Binary/Command/Parallel.rs:
    - Provides clearer explanation of the function's purpose.
    - Includes an example of how to use the function.

## 0.1.1

### Add

- Added support for multiple commands using the `-C` or `--Command` flag.
- Introduced a new GPG module to handle Git commit signing.
- Added a Process module to encapsulate asynchronous command execution.

### Change

- Updated various dependencies to their latest versions:
    - `clap` to 4.5.17
    - `tokio` to 1.40.0
    - `serde` to 1.0.210
    - `toml` to 0.8.19
- Updated the project repository URL to `https://GitHub.Com/PlayForm/Run.git`.
- Improved the README with clearer usage instructions and examples.
- Renamed the binary target from `Innkeeper` to `InnKeeper` for consistency.
- Enhanced error handling during parallel command execution.

### Fix

- Resolved an issue where parallel execution could deadlock when Git commit
  signing was enabled.

## 0.1.0

### Breaking Changes

- Removed the `--Separator` option.

### Change

- Updated version number from 0.0.7 to 0.1.0 in `Cargo.toml`

### Improved

- Enhanced `README.md` with more detailed information and updated benchmarks:
    - Changed "Benchmark" section to "Bench"
    - Updated benchmark results with new timings
    - Removed one benchmark example
    - Updated description for the `--Exclude` option
    - Removed the `--Separator` option description
- Refined command-line argument descriptions in `Source/Fn/Binary/Command.rs`
- Updated author information in `Source/Fn/Binary/Command.rs`

### Fix

- Corrected import in `Source/Fn/Binary/Command/Parallel.rs`:
    - Changed `stream::iter` to `futures::stream::iter`
    - Updated import statement for `StreamExt`

## 0.0.7

### Breaking Changes

- Updated `clap` dependency to version 4.5.11: Dependency updates can sometimes
  introduce breaking changes if the new version is not backward compatible.
- Updated `tokio` dependency to version 1.39.1 with full features: Similar to
  the `clap` update, this could introduce breaking changes if the new version of
  `tokio` is not backward compatible.
- Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
  support: Major refactoring often introduces breaking changes, especially if
  the public API or expected behavior changes.
- Updated `Source/Library.rs` to use async main function: Changing the main
  function to async can be a breaking change if users were not expecting this
  and need to adjust their code accordingly.
- Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
  async support: Refactoring for type safety and async support can introduce
  breaking changes if the function signatures or expected usage patterns change.

### Add

- New `.cargo/Config.toml` file with build and profile configurations
- Added `futures = "0.3.30"` dependency
- Added `rayon = "1.10.0"` dependency
- Added `num_cpus = "1.16.0"` dependency

### Change

- Updated `.gitignore` to use "Target" instead of "target" for consistency
- Updated `clap` dependency to version 4.5.11
- Updated `tokio` dependency to version 1.39.1 with full features
- Changed project description to "🍺 Run —"
- Updated repository URL to "https://github.com/PlayForm/Run.git"

### Improved

- Enhanced `README.md` with more detailed feature descriptions and usage
  instructions
- Added documentation for command-line options in `README.md`
- Improved error handling and added documentation in various source files

### Refactored

- Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
  support
- Updated `Source/Library.rs` to use async main function
- Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
  async support

## 0.0.6

### Breaking Changes

- Updated `clap` dependency to version 4.5.11: As mentioned before, dependency
  updates can introduce breaking changes.
- Updated `tokio` dependency to version 1.39.1: Similar to the `clap` update,
  this could introduce breaking changes.
- Refactored Parallel execution to use async/await and `tokio`: This is likely a
  breaking change as it changes the execution model.
- Updated main function to use `tokio` runtime: Changing the main function to
  use a specific runtime can be a breaking change.

### Add

- New `.github/FUNDING.yml` file with Open Collective funding information
- Added `serde = { version = "1.0.204", features = ["derive"] }` as a build
  dependency
- Added `toml = "0.8.16"` as a build dependency

### Change

- Updated various GitHub Action workflows
- Updated `clap` dependency to version 4.5.11
- Updated `tokio` dependency to version 1.39.1
- Changed copyright holder in `LICENSE` file to PlayForm
- Removed `Cargo.lock` from `.gitignore`

### Improved

- Enhanced `build.rs` script to use `serde` for parsing `Cargo.toml`
- Improved `README.md` with more detailed feature descriptions
- Added extensive documentation to various source files

### Refactored

- Major refactoring of `Source/Fn/Binary/Command/Parallel.rs` for better async
  support
- Updated `Source/Library.rs` to use async main function
- Refactored `Source/Struct/Binary/Command.rs` for improved type safety and
  async support

## 0.0.5

### Breaking Changes

- Updated `tokio` dependency to version 1.37.0: Dependency updates can introduce
  breaking changes.

### Change

- Updated `tokio` dependency to version 1.37.0
- Changed project description to "🍺 Run"
- Updated version number to 0.0.5

### Improved

- Enhanced `README.md` with more detailed information about the tool's features
  and usage
- Improved command-line argument descriptions in `Source/Fn/Binary/Command.rs`
- Added documentation to `Source/Fn/Binary/Command/Entry.rs` and
  `Source/Fn/Binary/Command/Parallel.rs`

### Fix

- Updated links in `CODE_OF_CONDUCT.md` and `CONTRIBUTING.md` to use HTTPS

## 0.0.4

### Change

- Updated version number to 0.0.4

### Improved

- Enhanced `README.md` with more detailed information about the tool's features
  and usage
- Refined command-line argument descriptions in `Source/Fn/Binary/Command.rs`

## 0.0.3

### Change

- Updated version number to 0.0.3

## 0.0.2

### Breaking Changes

- Added new binary targets: "Inn" and "InnKeeper": Adding new binaries is not a
  breaking change, but if it involved changes to existing binaries or their
  interfaces, it could be.

### Add

- New binary targets: "Inn" and "InnKeeper"

### Change

- Updated version number from 0.0.1 to 0.0.2

## 0.0.1

- Initial release
