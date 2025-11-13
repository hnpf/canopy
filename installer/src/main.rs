use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
static CANOPY_BINARY: &[u8] = include_bytes!("../canopy.exe");
fn main() {
    println!("Installing Canopy.");
    let os = env::consts::OS;
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).expect("Could not find home dir. You should probably get that checked out.");

    let install_dir = if os == "windows" {
        Path::new(&home).join("canopy")
    } else {
        Path::new(&home).join("canopy")
    };
    // make install dir
    if let Err(e) = fs::create_dir_all(&install_dir) {
        eprintln!("Failed to create install directory: {}", e);
        return;
    }

    // embedded binary stuff
    let binary_name = if os == "windows" { "canopy.exe" } else { "canopy" };
    let dest = install_dir.join(binary_name);
    if let Err(e) = fs::write(&dest, CANOPY_BINARY) {
        eprintln!("Failed to write binary: {}", e);
        return;
    }
    
    println!("Binary installed to: {}", dest.display());
    if os == "windows" {
        add_to_path_windows(install_dir.to_string_lossy().as_ref());
    } else {
        add_to_path_linux(install_dir.to_string_lossy().as_ref());
    }
    println!("Installation finished! Please restart your terminal or run 'source ~/.bashrc' (Linux) to use canopy! Have fun :)");
}

fn add_to_path_windows(dir: &str) {
    let output = Command::new("setx")
        .args(&["PATH", &format!("%PATH%;{}", dir)])
        .output();
    match output {
        Ok(_) => println!("Added {} to PATH. Restart your command prompt to apply.", dir),
        Err(e) => eprintln!("Failed to add PATH variable: {}. You may need to add {} to your PATH manually.", e, dir),
    }
}

fn add_to_path_linux(dir: &str) {
    let bashrc = env::var("HOME").unwrap() + "/.bashrc";
    let export_line = format!("\nexport PATH=\"$PATH:{}\"\n", dir);
    match fs::OpenOptions::new().append(true).open(&bashrc) {
        Ok(mut file) => {
            use std::io::Write;
            if file.write_all(export_line.as_bytes()).is_ok() {
                println!("Added {} to PATH in ~/.bashrc. Run 'source ~/.bashrc' to apply.", dir);
            } else {
                eprintln!("Failed to write to ~/.bashrc. Add 'export PATH=\"$PATH:{}\"' manually.", dir);
            }
        }
        Err(e) => eprintln!("Failed to open ~/.bashrc: {}. Add 'export PATH=\"$PATH:{}\"' manually.", e, dir),
    }
}
