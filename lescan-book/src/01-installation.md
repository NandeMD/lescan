# Installation

There are three ways to install LeScan. You can download the latest release from the
[GitHub releases page](https://github.com/NandeMD/lescan/releases) (recommended), install the application with cargo or
build the application from source.

> [!NOTE]
> LeScan GitHub releases are available for Windows, macOS, and Linux.
> Since the releases are automated by GitHub Actions, the latest releases run on:
> - Ubuntu release: `ubuntu-latest` (requires `glibc` version 2.39 or later)
> - macOS release: `macos-latest`
> - Windows release: `windows-latest`

## Install with Cargo

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already.
2. Run the following command:

    ```bash
    cargo install --git https://github.com/NandeMD/lescan.git
    ```
3. Run the application:

    ```bash
    lescan
    ```

## Building from Source

1. Clone the repository:

    ```bash
    git clone https://github.com/NandeMD/lescan.git
    cd lescan
    ```
2. Build the application:

    ```bash
    cargo build --release
    # The compiled binary will be located at `target/release/lescan`
    ```
3. Run the application:

    ```bash
    cargo run --release
    ```
