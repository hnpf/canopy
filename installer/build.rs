use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // we build the main app for the SAME target as the installer
    let target = env::var("TARGET").unwrap();
    let profile = env::var("PROFILE").unwrap();
    
    println!("cargo:rerun-if-changed=../src");
    println!("cargo:rerun-if-changed=../Cargo.toml");

    let status = Command::new("cargo")
        .args(&[
            "build", 
            if profile == "release" { "--release" } else { "" },
            "--target", &target,
            "--manifest-path", "../Cargo.toml"
        ])
        .status()
        .expect("Failed to build canopy binary");

    if !status.success() {
        panic!("Canopy build failed for target {}", target);
    }

    // binary name changes on windows
    let binary_name = if target.contains("windows") {
        "virex-canopy.exe"
    } else {
        "virex-canopy"
    };

    // find the built binary
    let src_path = Path::new("..")
        .join("target")
        .join(&target)
        .join(&profile)
        .join(binary_name);
        
    // if not found in target dir (maybe it was built for host?), try host dir
    let src_path = if !src_path.exists() {
        Path::new("..")
            .join("target")
            .join(&profile)
            .join(binary_name)
    } else {
        src_path
    };

    let dest_path = Path::new("src").join("payload.bin");
    std::fs::copy(&src_path, &dest_path)
        .expect(&format!("Failed to copy binary from {} to {}", src_path.display(), dest_path.display()));
}
