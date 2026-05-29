pub mod cli;
pub mod entry;

use std::error::Error;
use std::fs;
use std::path::Path;

use entry::Entry;

pub fn list_directory(path: &Path, show_hidden: bool) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut entries = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        let entry = Entry::new(item.path())?;

        if show_hidden || !entry.is_hidden() {
            entries.push(entry);
        }
    }

    entries.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(entries)
}
