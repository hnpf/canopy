use crate::modules::tree::TreeNode;
use std::io::Write;

// dump everything to json, easy peasy
pub fn export_json(tree: &TreeNode) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tree)?;
    // just print it to stdout, user can pipe it if they want :)
    println!("{}", json);
    Ok(())
}

// csv for the spreadsheet lovers (literally who needs this lol)
pub fn export_csv(tree: &TreeNode) -> std::io::Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    // header row
    wtr.write_record(&["path", "type", "size"])?;
    
    // recursive helper to flatten the tree
    collect_entries(tree, "", &mut |path, typ, size| {
        let size_str = size.map(|s| s.to_string()).unwrap_or_default();
        // ignore result because if stdout fails we have bigger problems
        let _ = wtr.write_record(&[path, typ, &size_str]);
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
