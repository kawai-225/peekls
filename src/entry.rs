use std::fs;
use std::io;
use std::path::PathBuf;

pub struct Entry {
    pub name: String,
    size: u64,
}

impl Entry {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;

        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Ok(Self {
            name,
            size: metadata.len(),
        })
    }

    pub fn format(&self, long: bool) -> String {
        if long {
            format!("{:>10} {}", self.size, self.name)
        } else {
            self.name.clone()
        }
    }
}
