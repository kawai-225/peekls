use assert_cmd::Command;
use predicates::str::contains;
use std::fs;

#[test]
fn cli_lists_visible_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    fs::write(temp_dir.path().join("README.md"), "# test").unwrap();

    let mut command = Command::cargo_bin("peekls").unwrap();

    command
        .arg(temp_dir.path())
        .assert()
        .success()
        .stdout(contains("README.md"));
}