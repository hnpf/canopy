use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub children: Vec<TreeNode>,
}

// recursive tree builder, eats directories
pub fn build_tree(path: &Path, max_depth: Option<usize>, show_hidden: bool, filter: Option<&str>) -> std::io::Result<TreeNode> {
    let entries = match std::fs::read_dir(path) {
        Ok(dir) => dir.filter_map(|e| e.ok()).collect::<Vec<_>>(),
        Err(e) => {
            // silent fail, move on. nobody likes error messages cluttering a nice interface!
            return Ok(TreeNode {
                name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                is_dir: true,
                size: None,
                children: Vec::new(),
            });
        }
    };
    let mut entries = entries;

    // hidden files? no, unless show_hidden is true tho
    if !show_hidden {
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with('.'));
    }
    entries.sort_by_key(|e| e.path());

    // glob filter logic, kinda shit but works
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
                name,
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

// squash empty folders
pub fn collapse_tree(node: TreeNode) -> TreeNode {
    let mut new_children = Vec::new();
    for child in node.children {
        new_children.push(collapse_tree(child));
    }
    
    // if only one child and it's a dir, merge them!
    if new_children.len() == 1 && new_children[0].is_dir {
        let child = new_children.into_iter().next().unwrap();
        TreeNode {
            name: format!("{}/{}", node.name, child.name),
            is_dir: true, // still a dir dw
            size: None,
            children: child.children,
        }
    } else {
        TreeNode {
            children: new_children,
            ..node
        }
    }
}
