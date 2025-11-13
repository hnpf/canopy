use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(name = "canopy")]
#[command(about = "Generate and visualize directory tree structures")]
struct Args {
    /// Path to the directory to visualize
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: Path does not exist: {}", args.path);
        std::process::exit(1);
    }

    println!("{}", path.display());

    if let Err(e) = print_tree(path, "") {
        eprintln!("Error reading directory: {}", e);
        std::process::exit(1);
    }
}

fn print_tree(path: &Path, prefix: &str) -> std::io::Result<()> {
    if path.is_file() {
        return Ok(());
    }

    let entries = std::fs::read_dir(path)?;
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.path());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        let connector = if is_last { "└── " } else { "├── " };
        println!("{}{}{}", prefix, connector, entry.file_name().to_string_lossy());

        if entry.file_type()?.is_dir() {
            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            print_tree(&entry.path(), &new_prefix)?;
        }
    }

    Ok(())
}
