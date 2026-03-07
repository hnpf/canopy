use colored::{Color, Colorize};
use crate::modules::tree::TreeNode;
use std::path::Path;

// pretty print the tree, make it look like a 'pro hacker tool' lol
pub fn print_tree(node: &TreeNode, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    // dirs are blue, files are white. standard stuff
    let color = if node.is_dir { Color::Blue } else { Color::White };
    
    // get a cute icon lol
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

// we are bad at big numbers, so let's make it readable
pub fn format_size(bytes: u64) -> String {
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

// warn if file is chunky
fn get_size_color(size: u64) -> Color {
    if size < 1024 {
        Color::Green // aww cute
    } else if size < 1024 * 1024 {
        Color::Yellow // u-uh.. is it t-too big..?
    } else {
        Color::Red // OH MY G-GOD~ >-<
    }
}

pub fn get_icon_for_name(name: &str) -> String {
    if let Some(ext) = Path::new(name).extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        // some cool icons
        match ext_str.as_str() {
            "py" => "[🐍] ".to_string(),
            "rs" => "[🦀] ".to_string(),
            "js" => "[📜] ".to_string(),
            "ts" => "[📘] ".to_string(),
            "html" => "[🌐] ".to_string(),
            "css" => "[🎨] ".to_string(),
            "md" => "[📝] ".to_string(),
            "txt" => "[📄] ".to_string(),
            "exe" | "bat" | "cmd" | "sh" => "[🚀] ".to_string(),
            "json" | "toml" | "yaml" => "[⚙️] ".to_string(),
            "png" | "jpg" | "jpeg" | "gif" => "[🖼️] ".to_string(),
            _ => format!("[{}] ", ext_str),
        }
    } else {
        "[📄] ".to_string()
    }
}

pub fn test_colors() {
    println!("{}", "Testing colors...".bold());
    println!("{}", "Red like the errors you ignore".red());
    println!("{}", "Green like the grass you don't touch".green());
    println!("{}", "Blue like the screen of death".blue());
    println!("{}", "Yellow like.. idk bananas? wtf".yellow());
    println!("{}", "Cyan is just light blue".cyan());
    println!("{}", "Magenta is ok".magenta());
    println!("{}", "White is kinda plain.".white());
}
