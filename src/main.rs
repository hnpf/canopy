use clap::Parser;
use colored::{Color, Colorize};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color as TuiColor, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct TreeNode {
    name: String,
    is_dir: bool,
    size: Option<u64>,
    children: Vec<TreeNode>,
}

#[derive(Parser)]
#[command(name = "virex-canopy")]
#[command(about = "Generate and visualize directory tree structures")]
#[command(disable_help_flag = true)]
struct Args {
    /// === path to the dir to visualize === ///
    path: Option<String>,

    /// === maximum depth to traverse, now u can limit that shit lolw === ///

    #[arg(long)]
    depth: Option<usize>,

    /// === include hidden files starting with ., screw the dots :fire: === ///
    #[arg(long)]
    hidden: bool,

    /// === collapse empty folders and 1-file chains, make it meaningful === ///
    #[arg(long)]
    collapse: bool,

    /// === export to json or csv, for programmatic use === ///
    #[arg(long)]
    export: Option<String>,

    /// === export to json
    #[arg(long)]
    json: bool,

    /// === enable interactive TUI mode, navigate with arrows and enter === ///
    #[arg(long)]
    interactive: bool,

    /// === filter files with glob pattern, e.g. *.rs === ///
    #[arg(long)]
    filter: Option<String>,

    /// === this tests color output === ///
    #[arg(long)]
    test_colors: bool,

    /// === check if exe is in PATH === ///
    #[arg(long)]
    check_path: bool,
}

fn print_welcome() {
    println!("╔══════════════════════════╗");
    println!("║     Welcome to Canopy!   ║");
    println!("╚══════════════════════════╝");
    println!();
    println!("Your directory tree visualizer is ready to go!");
    println!();
    println!("Commands:");
    println!("  [x] virex-canopy <PATH>       Show tree for a folder");
    println!("  [x] --depth <N>               Limit tree depth");
    println!("  [x] --hidden                  Include hidden files");
    println!("  [x] --json                    Export tree as JSON");
    println!("  [x] --help                    Show this message");
    println!("  [x] --interactive             Enable interactive TUI mode");
    println!("  [x] --filter <PATTERN>        Filter files with glob pattern, e.g. *.rs");
    println!("  [x] --test-colors             Test color output");
    println!("  [x] --check-path              Check if exe is in PATH");
    println!();
    println!("Tip: Try `virex-canopy . --depth 2` to explore your current folder!");
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

#[cfg(windows)]
fn is_executable(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "exe" | "bat" | "cmd" | "com")
    } else {
        false
    }
}
fn enable_ansi() {
    #[cfg(windows)] {
        use winapi::um::processenv::GetStdHandle;
        use winapi::um::winbase::STD_OUTPUT_HANDLE;
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let mut mode: winapi::shared::minwindef::DWORD = 0;
            winapi::um::consoleapi::GetConsoleMode(handle, &mut mode);
            winapi::um::consoleapi::SetConsoleMode(handle, mode | winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

fn format_size(bytes: u64) -> String {
    // format bytes to human readable, damn big numbers
    if bytes < 1024 {
        format!("{}B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn get_size_color(size: u64) -> Color {
    if size < 1024 {
        Color::Green
    } else if size < 1024 * 1024 {
        Color::Yellow
    } else {
        Color::Red
    }
}

fn main() {
    enable_ansi();

    if std::env::args().any(|arg| arg == "--help" || arg == "-h") {
        print_welcome();
        return;
    }

    // check if in PATH
    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap().to_string_lossy();
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let paths: Vec<std::path::PathBuf> = std::env::split_paths(&path_var).collect();
    let in_path = paths.iter().any(|p| p.join(exe_name.as_ref()).exists());
    if !in_path {
        print_welcome();
        thread::sleep(Duration::from_secs(4));
        let appdata = std::env::var("APPDATA").unwrap();
        let dest = Path::new(&appdata).join("canopy.exe");
        if let Err(e) = std::fs::copy(&exe_path, &dest) {
            eprintln!("Failed to copy to appdata: {}", e);
        } else {
            let log_path = Path::new(&appdata).join("canopy.log");
            let mut log = std::fs::OpenOptions::new().append(true).create(true).open(log_path).unwrap();
            writeln!(log, "Copied to appdata at {}", dest.display()).unwrap();
        }
    }

    let args = Args::parse();

    let export_format = if args.json { Some("json") } else { args.export.as_deref() };

    if args.check_path {
        let exe_path = std::env::current_exe().unwrap();
        let exe_name = exe_path.file_name().unwrap().to_string_lossy();
        let path_var = std::env::var_os("PATH").unwrap_or_default();
        let paths: Vec<std::path::PathBuf> = std::env::split_paths(&path_var).collect();
        let found_path = paths.iter().find(|p| p.join(exe_name.as_ref()).exists());
        if let Some(p) = found_path {
            println!("yes {}", p.join(exe_name.as_ref()).display());
        } else {
            println!("no");
        }
        return;
    }

    if args.test_colors {
        test_colors();
        return;
    }

    let mut interactive = args.interactive;
    let path_str = if let Some(p) = args.path {
        p
    } else {
        interactive = true;
        "C:\\".to_string()
    };
    let path = Path::new(&path_str);

    if !path.exists() {
        eprintln!("Error: Path does not exist: {}", path_str);
        std::process::exit(1);
    }

    if interactive {
        if let Err(e) = run_tui(path, args.hidden, args.filter.as_deref()) {
            eprintln!("TUI error: {}", e);
            std::process::exit(1);
        }
    } else {
        let tree = match build_tree(path, args.depth, args.hidden, args.filter.as_deref()) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error reading directory: {}", e);
                std::process::exit(1);
            }
        };

        let tree = if args.collapse {
            collapse_tree(tree)
        } else {
            tree
        };

        if let Some(format) = export_format {
            match format {
                "json" => {
                    if let Err(e) = export_json(&tree) {
                        eprintln!("Error exporting to JSON: {}", e);
                        std::process::exit(1);
                    }
                }
                "csv" => {
                    if let Err(e) = export_csv(&tree) {
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
            print_tree(&tree, "", true);
        }
    }
}

fn build_tree(path: &Path, max_depth: Option<usize>, show_hidden: bool, filter: Option<&str>) -> std::io::Result<TreeNode> {
    let entries = match std::fs::read_dir(path) {
        Ok(dir) => dir.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Warning: access denied for {}: {}", path.display(), e);
            Vec::new()
        }
    };
    let mut entries = entries;
    // skip hidden files if not showing them, dotfiles smh
    if !show_hidden {
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
    }
    entries.sort_by_key(|e| e.path());

    if let Some(pattern) = filter {
        let pat = glob::Pattern::new(pattern).unwrap_or(glob::Pattern::new("*").unwrap());
        entries.retain(|e| {
            let is_dir = e.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            is_dir || pat.matches(&e.file_name().to_string_lossy())
        });
    }

    let mut children = Vec::new();
    for entry in entries {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let size = if is_dir {
            None
        } else {
            entry.metadata().ok().map(|m| m.len())
        };

        let child = if is_dir && max_depth.map_or(true, |d| d > 0) {
            let new_depth = max_depth.map(|d| d - 1);
            build_tree(&entry.path(), new_depth, show_hidden, filter)?
        } else {
            TreeNode {
                name: name.clone(),
                is_dir,
                size,
                children: Vec::new(),
            }
        };

        children.push(child);
    }

    Ok(TreeNode {
        name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
        is_dir: true,
        size: None,
        children,
    })
}

fn collapse_tree(node: TreeNode) -> TreeNode {
    let name = node.name;
    let is_dir = node.is_dir;
    let size = node.size;
    let children = node.children;
    let mut new_children = Vec::new();
    for child in children {
        let collapsed = collapse_tree(child);
        new_children.push(collapsed);
    }
    if new_children.len() == 1 && new_children[0].is_dir {
        let child = new_children.into_iter().next().unwrap();
        TreeNode {
            name: format!("{}/{}", name, child.name),
            is_dir: true,
            size: None,
            children: child.children,
        }
    } else {
        TreeNode {
            name,
            is_dir,
            size,
            children: new_children,
        }
    }
}

fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    let color = if node.is_dir { Color::Blue } else { Color::White };
    let icon = if node.is_dir { "[DIR] ".to_string() } else { get_icon_for_name(&node.name) };
    let icon_colored = icon.color(color);
    let name_colored = node.name.color(color);
    let mut display = format!("{}{}", icon_colored, name_colored);
    if let Some(size) = node.size {
        let size_str = format_size(size);
        let size_color = get_size_color(size);
        display.push_str(&format!(" ({})", size_str.color(size_color)));
    }
    println!("{}{}{}", prefix, connector, display);

    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
    for (i, child) in node.children.iter().enumerate() {
        let child_is_last = i == node.children.len() - 1;
        print_tree(child, &new_prefix, child_is_last);
    }
}

fn get_icon_for_name(name: &str) -> String {
    if let Some(ext) = Path::new(name).extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        match ext_str.as_str() {
            "py" => "[.py] ".to_string(),
            "rs" => "[.rs] ".to_string(),
            "js" => "[.js] ".to_string(),
            "ts" => "[.ts] ".to_string(),
            "html" => "[.html] ".to_string(),
            "css" => "[.css] ".to_string(),
            "md" => "[.md] ".to_string(),
            "txt" => "[.txt] ".to_string(),
            "exe" | "bat" | "cmd" => "[EXEC] ".to_string(),
            _ => format!("[{}] ", ext_str),
        }
    } else {
        "[FILE] ".to_string()
    }
}

fn export_json(tree: &TreeNode) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tree)?;
    println!("{}", json);
    Ok(())
}

fn test_colors() {
    println!("{}", "Testing colors:".bold());
    println!("{}", "Red text".red());
    println!("{}", "Green text".green());
    println!("{}", "Blue text".blue());
    println!("{}", "Yellow text".yellow());
    println!("{}", "Cyan text".cyan());
    println!("{}", "Magenta text".magenta());
    println!("{}", "White text".white());
    println!("{}", "Black text".black());
    println!("{}", "Bright red".bright_red());
    println!("{}", "Bright green".bright_green());
    println!("{}", "Bright blue".bright_blue());
    println!("{}", "Bright yellow".bright_yellow());
    println!("{}", "Bright cyan".bright_cyan());
    println!("{}", "Bright magenta".bright_magenta());
    println!("{}", "Bright white".bright_white());
}

fn export_csv(tree: &TreeNode) -> std::io::Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&["path", "type", "size"])?;
    collect_entries(tree, "", &mut |path, typ, size| {
        let size_str = size.map(|s| s.to_string()).unwrap_or_default();
        wtr.write_record(&[path, typ, &size_str]).unwrap();
    });
    wtr.flush()?;
    Ok(())
}

fn collect_entries<F>(node: &TreeNode, current_path: &str, func: &mut F)
where
    F: FnMut(&str, &str, Option<u64>),
{
    let path = if current_path.is_empty() {
        node.name.clone()
    } else {
        format!("{}/{}", current_path, node.name)
    };
    let typ = if node.is_dir { "directory" } else { "file" };
    func(&path, typ, node.size);
    for child in &node.children {
        collect_entries(child, &path, func);
    }
}

fn get_entries(path: &Path, show_hidden: bool, filter: Option<&str>) -> io::Result<Vec<std::fs::DirEntry>> {
    let mut entries = match std::fs::read_dir(path) {
        Ok(dir) => dir.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Warning: access denied for {}: {}", path.display(), e);
            Vec::new()
        }
    };
    if !show_hidden {
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
    }
    entries.sort_by_key(|e| e.path());

    if let Some(pattern) = filter {
        let pat = glob::Pattern::new(pattern).unwrap_or(glob::Pattern::new("*").unwrap());
        entries.retain(|e| {
            let is_dir = e.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            is_dir || pat.matches(&e.file_name().to_string_lossy())
        });
    }

    Ok(entries)
}

fn get_icon_for_entry(entry: &std::fs::DirEntry) -> String {
    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
    if is_dir {
        "[DIR] ".to_string()
    } else {
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > 1024 * 1024 {
                return "[BIG] ".to_string();
            }
        }
        let path = entry.path();
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            match ext_str.as_str() {
                "py" => "[.py] ".to_string(),
                "rs" => "[.rs] ".to_string(),
                "js" => "[.js] ".to_string(),
                "ts" => "[.ts] ".to_string(),
                "html" => "[.html] ".to_string(),
                "css" => "[.css] ".to_string(),
                "md" => "[.md] ".to_string(),
                "txt" => "[.txt] ".to_string(),
                "exe" | "bat" | "cmd" | "sh" => "[EXEC] ".to_string(),
                _ => format!("[{}] ", ext_str),
            }
        } else {
            "[FILE] ".to_string()
        }
    }
}

fn get_color_for_entry(entry: &std::fs::DirEntry) -> TuiColor {
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();
    if file_name_str.starts_with('.') {
        TuiColor::Gray
    } else {
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if is_dir {
            TuiColor::Blue
        } else if is_executable(&entry.path()) {
            TuiColor::Green
        } else {
            TuiColor::White
        }
    }
}

fn run_tui(path: &Path, show_hidden: bool, filter: Option<&str>) -> io::Result<()> {
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnableMouseCapture, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_path = path.to_path_buf();
    let mut selected = 0;
    let mut entries = get_entries(&current_path, show_hidden, filter)?;

    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                let size = f.size();
                let items: Vec<ListItem> = entries.iter().map(|e| {
                    let icon = get_icon_for_entry(e);
                    let color = get_color_for_entry(e);
                    let name = e.file_name().to_string_lossy().to_string();
                    ListItem::new(Line::from(vec![
                        Span::styled(icon, Style::default().fg(color)),
                        Span::styled(name, Style::default()),
                    ]))
                }).collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title(format!("{} ({} items)", current_path.display(), entries.len())))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> ");

                let mut state = ListState::default();
                state.select(Some(selected));

                f.render_stateful_widget(list, size, &mut state);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if selected < entries.len().saturating_sub(1) {
                                selected += 1;
                            }
                        }
                        KeyCode::Right => {
                            if let Some(entry) = entries.get(selected) {
                                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                                if is_dir {
                                    current_path.push(entry.file_name());
                                    entries = get_entries(&current_path, show_hidden, filter)?;
                                    selected = 0;
                                }
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(entry) = entries.get(selected) {
                                let path_str = entry.path().to_string_lossy().to_string();
                                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                                if is_dir {
                                    std::process::Command::new("explorer").arg(&path_str).spawn().ok();
                                } else {
                                    std::process::Command::new("cmd").args(&["/c", "start", "", &path_str]).spawn().ok();
                                }
                            }
                        }
                        KeyCode::Left | KeyCode::Backspace | KeyCode::Esc => {
                            if current_path != path {
                                current_path.pop();
                                entries = get_entries(&current_path, show_hidden, filter)?;
                                selected = 0;
                            }
                        }
                        KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }));

    crossterm::execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        crossterm::terminal::LeaveAlternateScreen
    )?;
    match res {
        Ok(inner) => inner,
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "TUI Panicked!!!")),
    }
}
