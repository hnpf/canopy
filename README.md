# Canopy

A command-line tool made in Rust for generating and visualizing directory tree structures in the terminal!

## Usage

```
canopy <path>
```

Example:

```
canopy C:\Windows\System32\
```

This will print a tree-like structure of the directory and its subdirectories :)

## Installation

### Using the Installer

1. Download the installer binary for your platform from the releases page!

2. Install from Crate:
   -  `cargo install virex-canopy`
   - Launch with virex-canopy!
     
3. Run the installer:

   - Windows: `canopy-installer.exe`
   - Linux: `./canopy-installer`

   The installer will copy the canopy binary to your home directory and add it to your PATH!
   
### From Source

1. Make sure you have Rust installed! If not, just download it from [rustup.rs](https://rustup.rs/).

2. Clone the repository:

   ```
   git clone https://github.com/hnpf/canopy
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
- A clean tree visualization with Unicode box-drawing characters
- Sorted output for consistent results
- Actual handling for invalid paths!!

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GNU GENERAL PUBLIC LICENSE.
