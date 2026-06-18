use std::fs;
use std::path::{Path, PathBuf};

pub fn read_tagline(directory: &Path) -> Option<String> {
    readme_candidates(directory)
        .into_iter()
        .find_map(|path| tagline_from_file(&path))
}

fn readme_candidates(directory: &Path) -> Vec<PathBuf> {
    vec![
        directory.join("README.md"),
        directory.join("README"),
        directory.join("readme.md"),
    ]
}

fn tagline_from_file(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;

    content
        .lines()
        .map(clean_line)
        .find(|line| !line.is_empty())
}

fn clean_line(line: &str) -> String {
    line.trim().trim_start_matches('#').trim().to_string()
}

//単体テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_line_removes_markdown_heading_marker() {
        let result = clean_line("# peekls");

        assert_eq!(result, "peekls");
    }

    #[test]
    fn clean_line_trims_spaces() {
        let result = clean_line("   peekls   ");

        assert_eq!(result, "peekls");
    }

    #[test]
    fn clean_line_removes_heading_marker_and_spaces() {
        let result = clean_line("##   README tagline   ");

        assert_eq!(result, "README tagline");
    }
}