mod modules;

use clap::Parser;
use std::path::Path;
use modules::{tree, render, export, tui, utils};

#[derive(Parser)]
#[command(name = "virex-canopy")]
#[command(about = "Generate and visualize directory tree structures")]
#[command(disable_help_flag = true)]
struct Args {
    path: Option<String>,
    #[arg(long)]
    depth: Option<usize>,
    #[arg(long)]
    hidden: bool,
    #[arg(long)]
    collapse: bool,
    #[arg(long)]
    export: Option<String>,
    #[arg(long)]
    json: bool,
    #[arg(long)]
    interactive: bool,
    #[arg(long)]
    filter: Option<String>,
    #[arg(long)]
    test_colors: bool,
    #[arg(long)]
    check_path: bool,
    #[arg(long, short)]
    help: bool,
}

fn print_welcome() {
    println!("╔══════════════════════════╗");
    println!("║     Welcome to Canopy!   ║");
    println!("╚══════════════════════════╝");
    println!();
    println!("Commands:");
    println!("  [x] virex-canopy <PATH>       Show tree for a folder");
    println!("  [x] --depth <N>               Limit tree depth");
    println!("  [x] --hidden                  Include hidden files");
    println!("  [x] --json                    Export tree as JSON");
    println!("  [x] --interactive             Enable interactive TUI mode");
    println!("  [x] --filter <PATTERN>        Filter files with glob pattern");
    println!("  [x] --test-colors             Test color output");
    println!("  [x] --check-path              Check if exe is in PATH");
    println!();
    println!("Tip: Try `virex-canopy . --depth 2` to explore your current folder!");
}

fn main() {
    utils::enable_ansi();
    let args = match Args::try_parse() {
        Ok(a) => a,
        Err(e) => {
            // e.print().unwrap();
            // check if it's a help request
            if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
                print_welcome();
                return;
            }
             // otherwise print error
            eprintln!("{}", e);
            return;
        }
    };

    if args.help {
        print_welcome();
        return;
    }

    if args.check_path {
        utils::check_and_install_to_path();
        return;
    }

    if args.test_colors {
        render::test_colors();
        return;
    }

    let mut interactive = args.interactive;
    let path_str = if let Some(p) = args.path {
        p
    } else {
        // no path? default to current dir or interactive?
        // my original logic: interactive = true, path = C:\
        // thats weird for linux.
        // lets default to current dir if not interactive, or interactive on current dir?
        // original: interactive = true, "C:\\"
        // change that to "."
        interactive = true;
        ".".to_string()
    };
    
    let path = Path::new(&path_str);
    if !path.exists() {
        eprintln!("Error: Path does not exist: {}", path_str);
        std::process::exit(1);
    }

    if interactive {
        if let Err(e) = tui::run_tui(path, args.hidden, args.filter.as_deref()) {
            eprintln!("TUI error: {}", e);
            std::process::exit(1);
        }
    } else {
        let tree = match tree::build_tree(path, args.depth, args.hidden, args.filter.as_deref()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                std::process::exit(1);
            }
        };

        let tree = if args.collapse {
            tree::collapse_tree(tree)
        } else {
            tree
        };

        let export_format = if args.json { Some("json") } else { args.export.as_deref() };

        if let Some(format) = export_format {
            match format {
                "json" => {
                    if let Err(e) = export::export_json(&tree) {
                        eprintln!("Error exporting to JSON: {}", e);
                        std::process::exit(1);
                    }
                }
                "csv" => {
                    if let Err(e) = export::export_csv(&tree) {
                        eprintln!("Error exporting to CSV: {}", e);
                        std::process::exit(1);
                    }
                }
                _ => {
                    eprintln!("Invalid export format: {}", format);
                    std::process::exit(1);
                }
            }
        } else {
            println!("{}", path.display());
            render::print_tree(&tree, "", true);
        }
    }
}
