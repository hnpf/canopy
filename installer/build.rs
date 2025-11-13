use std::process::Command;

fn main() {
    // Build the canopy binary first
    println!("cargo:rerun-if-changed=../src/main.rs");
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let status = Command::new("cargo")
        .args(&["build", "--release", "--manifest-path", "../Cargo.toml"])
        .status()
        .expect("Failed to build canopy");

    if !status.success() {
        panic!("Canopy build failed");
    }

    // Copy the built binary to the installer directory
    let src = "../target/release/canopy.exe";
    let dest = "canopy.exe";

    std::fs::copy(src, dest).expect("Failed to copy canopy binary");
}