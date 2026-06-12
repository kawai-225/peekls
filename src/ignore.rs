use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;

pub type IgnoreRules = Gitignore;

pub fn build_gitignore(path: &Path) -> Option<IgnoreRules> {
    let gitignore_path = path.join(".gitignore");

    if !gitignore_path.exists() {
        return None;
    }

    let mut builder = GitignoreBuilder::new(path);
    builder.add(gitignore_path);
    builder.build().ok()
}

pub fn is_gitignored(gitignore: &Option<IgnoreRules>, path: &Path) -> bool {
    if let Some(gitignore) = gitignore {
        return gitignore.matched(path, path.is_dir()).is_ignore();
    }

    false
}
