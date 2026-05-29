use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Entry {
    pub name: String,
    size: u64,
    kind: EntryKind,
}

enum EntryKind {
    File,
    Directory,
    Other,
}

impl Entry {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;
        let name = entry_name(&path);
        let kind = EntryKind::from_metadata(&metadata);

        Ok(Self {
            name,
            size: metadata.len(),
            kind,
        })
    }

    pub fn format(&self, long: bool) -> String {
        if long {
            self.format_long()
        } else {
            self.name.clone()
        }
    }

    fn format_long(&self) -> String {
        format!("{:<6} {:>10} {}", self.kind.label(), self.size, self.name)
    }
}

impl EntryKind {
    fn from_metadata(metadata: &fs::Metadata) -> Self {
        if metadata.is_dir() {
            Self::Directory
        } else if metadata.is_file() {
            Self::File
        } else {
            Self::Other
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::File => "FILE",
            Self::Directory => "DIR",
            Self::Other => "OTHER",
        }
    }
}

fn entry_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
