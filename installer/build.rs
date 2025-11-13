use std::process::Command;

fn main() {
    // Build the canopy binary first!
    println!("cargo:rerun-if-changed=../src/main.rs");
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let status = Command::new("cargo")
        .args(&["build", "--release", "--manifest-path", "../Cargo.toml"])
        .status()
        .expect("Failed to build canopy :(");
    if !status.success() {
        panic!("Uh oh! The Canopy build failed.");
    }
    let src = "../target/release/canopy.exe"; /// Lowk just change this to whatever
    let dest = "canopy.exe";
    std::fs::copy(src, dest).expect("copy canopy binary failed.");

}
