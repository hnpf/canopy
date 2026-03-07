# Canopy

[![Crates.io version](https://img.shields.io/crates/v/virex-canopy)](https://crates.io/crates/v/virex-canopy)
[![Crates.io downloads](https://img.shields.io/crates/d/virex-canopy)](https://crates.io/crates/v/virex-canopy)
[![Repo size](https://img.shields.io/github/repo-size/hnpf/canopy)](https://github.com/hnpf/canopy)
[![Made with love](https://img.shields.io/badge/made%20with-love-red)](https://github.com/hnpf/canopy)
[![Rust](https://img.shields.io/badge/rust-stable-orange)](https://www.rust-lang.org/)

![meow](github/demos/meow.png)
A command-line tool for generating and visualizing directory tree structures in the terminal!


## Usage

```
canopy <path>
```

Example:

```
canopy /path/to/directory
```

This will print a tree-like structure of the directory and its subdirectories.

## Screenshots!
The `--interactive` flag
![demo main](github/demos/demomain.png)
`virex-canopy . --depth 0`

![demo2](github/demos/demo2.png)
`--test-colors`

![democolors](github/demos/democolors.png)
The `--help` flag command!

![demohelp](github/demos/demohelp.png)



## Installation

     
### Option 1, Using the Installer

1. Download the installer binary for your platform from the releases page.

2. Run the installer:

   - Windows: `canopy-installer.exe`
   - Linux: `./canopy-installer`

   The installer will copy the canopy binary to your home directory and add it to your PATH.

### Option 2, From Source

1. Ensure you have Rust installed. Download from [rustup.rs](https://rustup.rs/).

2. Clone the repository:

   ```
   git clone <repository-url>
   cd canopy
   ```

3. Build the project:

   ```
   cargo build --release
   ```

4. The binary will be located at `target/release/canopy`.

5. To install manually, copy the binary to a directory in your PATH, e.g., `~/bin/` or `%USERPROFILE%\bin\`.

### Pre-built Binaries

Download the latest release for your platform from the releases page.

## Cross-Platform Support

Canopy is built with Rust and supports multiple platforms:

- Windows (x86_64)
- Linux (x86_64)

To build for a specific target:

- For Linux: `cargo build --release --target x86_64-unknown-linux-gnu`
- For Windows: `cargo build --release --target x86_64-pc-windows-gnu`

## Features

- Recursive directory traversal
- Clean tree visualization with Unicode box-drawing characters
- Sorted output for consistent results
- Error handling for invalid paths

## Contributing

Contributions are welcome! Please feel free to submit a PR.

## License

This project is licensed under the GPL-3.0 License. For more info, check the LICENSE File.


