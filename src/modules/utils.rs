use std::path::{Path, PathBuf};
use std::io;

#[cfg(unix)]
pub fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
}

#[cfg(windows)]
pub fn is_executable(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(ext.as_str(), "exe" | "bat" | "cmd" | "com")
    } else {
        false
    }
}

pub fn enable_ansi() {
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

pub fn list_dir(path: &Path, show_hidden: bool, filter: Option<&str>) -> io::Result<Vec<std::fs::DirEntry>> {
    let entries = match std::fs::read_dir(path) {
        Ok(dir) => dir.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(e) => {
            return Ok(Vec::new());
        }
    };
    let mut entries = entries;
    
    // hide the secrets if we must
    if !show_hidden {
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
    }
    entries.sort_by_key(|e| e.path());
    // filter some shit
    if let Some(pattern) = filter {
        let pat = glob::Pattern::new(pattern).unwrap_or(glob::Pattern::new("*").unwrap());
        entries.retain(|e| {
            let is_dir = e.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
            is_dir || pat.matches(&e.file_name().to_string_lossy())
        });
    }
    Ok(entries)
}

// self replication logic... almost like a worm but useful
pub fn check_and_install_to_path() {
    let exe_path = std::env::current_exe().unwrap();
    let exe_name = exe_path.file_name().unwrap().to_string_lossy();
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let paths: Vec<PathBuf> = std::env::split_paths(&path_var).collect();
    
    // are we in the (PATH)?
    let in_path = paths.iter().any(|p| p.join(exe_name.as_ref()).exists());
    
    if !in_path {
        println!("Wait, I'm not in PATH? Let me fix that...");
        
        #[cfg(windows)]
        {
            let appdata = std::env::var("APPDATA").unwrap_or(".".to_string());
            let dest = Path::new(&appdata).join("virex-canopy.exe"); // consistent name lol
             if let Err(e) = std::fs::copy(&exe_path, &dest) {
                eprintln!("Failed to copy to appdata: {}", e);
            } else {
                println!("Copied to {}", dest.display());
                // TODO: actually add to PATH registry or something? 
                // the original code didn't actually add to PATH, just copied to AppData which might be in PATH? 
                // wtf was i doing bro
                // usually AppData is NOT in PATH.
                // the original code was copy to APPDATA/canopy.exe. 
                // that logic was flawed if APPDATA isn't in PATH.
                println!("{} {}", {"You're on Windows, you may need to set to PATH manually.. Please consider upgrading your OS."}, appdata);
            }
        }
        #[cfg(unix)]
        {
             // on linux, we usually don't just copy ourselves to /usr/bin without sudo.
             // MAYBE ~/.local/bin?
             let home = std::env::var("HOME").unwrap_or("/tmp".to_string());
             let local_bin = Path::new(&home).join(".local/bin");
             if local_bin.exists() {
                 let dest = local_bin.join("virex-canopy");
                 if let Err(e) = std::fs::copy(&exe_path, &dest) {
                     // oh yeah baby
                 } else {
                     println!("Copied myself to {}", dest.display());
                 }
             }
        }
    }
}
