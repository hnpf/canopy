#!/bin/bash
# canopy build script for linux/windows distros
set -e
echo "Cleaning target..."
cargo clean
cd installer && cargo clean && cd ..
echo "Building Linux (x64) canopy & installer..."
cd installer
cargo build --release --target x86_64-unknown-linux-gnu
cd ..

mkdir -p dist/linux
cp installer/target/x86_64-unknown-linux-gnu/release/canopy-installer dist/linux/canopy-installer-linux
echo "Building Windows (x64) canopy & installer..."
rustup target add x86_64-pc-windows-gnu || true
cd installer
cargo build --release --target x86_64-pc-windows-gnu
cd ..
mkdir -p dist/windows
cp installer/target/x86_64-pc-windows-gnu/release/canopy-installer.exe dist/windows/canopy-installer-windows.exe

echo "Dist built successfully in dist/ folder!"
