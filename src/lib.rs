pub mod cli;
pub mod entry;
pub mod ignore;

use std::error::Error;
use std::fs;
use std::path::Path;

use entry::Entry;

pub fn list_directory(
    path: &Path,
    show_hidden: bool,
    show_ignored: bool,
) -> Result<Vec<Entry>, Box<dyn Error>> {
    let gitignore = ignore::build_gitignore(path);
    let mut entries = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        let entry_path = item.path();
        let entry = Entry::new(entry_path.clone())?;

        if !show_ignored && ignore::is_gitignored(&gitignore, &entry_path) {
            continue;
        }

        if show_hidden || !entry.is_hidden() {
            entries.push(entry);
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}
