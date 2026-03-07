use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io;
use std::path::{Path, PathBuf};
use crate::modules::utils::{list_dir, is_executable};

// TUI mode: because CLI args are for unemployed nerds
pub fn run_tui(path: &Path, show_hidden: bool, filter: Option<&str>) -> io::Result<()> {
    // setup term
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut current_path = path.to_path_buf();
    let mut selected = 0;
    
    // safe wrapper to ensure we restore terminal even if panic
    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> io::Result<()> {
        loop {
            // refresh entries every frame? no bc thats slow. 
            // but we need to refresh if path changes.
            // for now, just fetch them. logic could be fixed ig
            let entries = list_dir(&current_path, show_hidden, filter)?;
            
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
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
                    .highlight_symbol(">> ");
                let mut state = ListState::default();
                state.select(Some(selected));
                f.render_stateful_widget(list, size, &mut state);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up | KeyCode::Char('k') => {
                            if selected > 0 {
                                selected -= 1;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            if selected < entries.len().saturating_sub(1) {
                                selected += 1;
                            }
                        }
                        KeyCode::Right | KeyCode::Enter | KeyCode::Char('l') => {
                            if let Some(entry) = entries.get(selected) {
                                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                                if is_dir {
                                    current_path.push(entry.file_name());
                                    selected = 0; // reset selection on new dir
                                } else {
                                    // open file? 
                                    open_file(&entry.path());
                                }
                            }
                        }
                        KeyCode::Left | KeyCode::Backspace | KeyCode::Esc | KeyCode::Char('h') => {
                            if current_path != path {
                                current_path.pop();
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

    // restore terminal
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    let _ = crossterm::terminal::disable_raw_mode();
    terminal.show_cursor()?;

    match res {
        Ok(inner) => inner,
        Err(_) => Err(io::Error::new(io::ErrorKind::Other, "TUI Panicked!")),
    }
}

fn get_icon_for_entry(entry: &std::fs::DirEntry) -> String {
    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
    if is_dir {
        "[DIR] ".to_string()
    } else {
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > 1024 * 1024 * 100 { // 100MB is big and this is justifiable
                return "[BIG] ".to_string();
            }
        }
        // reuse the logic from render? no. lets keep it simple here now
        let path = entry.path();
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            match ext_str.as_str() {
                "py" => "[🐍] ".to_string(),
                "rs" => "[🦀] ".to_string(),
                "js" | "ts" => "[📜] ".to_string(),
                "exe" | "bat" | "sh" => "[🚀] ".to_string(),
                _ => format!("[{}] ", ext_str),
            }
        } else {
            "[📄] ".to_string()
        }
    }
}

fn get_color_for_entry(entry: &std::fs::DirEntry) -> Color {
    let file_name = entry.file_name();
    let file_name_str = file_name.to_string_lossy();
    if file_name_str.starts_with('.') {
        Color::DarkGray
    } else {
        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        if is_dir {
            Color::Blue
        } else if is_executable(&entry.path()) {
            Color::Green
        } else {
            Color::White
        }
    }
}

fn open_file(path: &Path) {
    let path_str = path.to_string_lossy().to_string();
    use std::process::Stdio;

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/c", "start", "", &path_str])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path_str)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path_str)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .ok();
    }
}
