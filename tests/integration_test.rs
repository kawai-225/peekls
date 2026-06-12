use std::fs;

#[test]
fn list_directory_returns_visible_files() {
    let temp_dir = tempfile::tempdir().unwrap();

    fs::write(temp_dir.path().join("README.md"), "# test").unwrap();

    let entries = peekls::list_directory(
        temp_dir.path(),
        false,
        false,
        &[],
    )
    .unwrap();

    let names: Vec<String> = entries.into_iter().map(|entry| entry.name).collect();

    assert_eq!(names, vec!["README.md"]);
}