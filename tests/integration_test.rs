use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn create_test_file(dir: &TempDir, name: &str, content: &str) -> String {
    let file_path = dir.path().join(name);
    fs::write(&file_path, content).unwrap();
    file_path.to_str().unwrap().to_string()
}

#[test]
fn test_python_execution() {
    let temp_dir = TempDir::new().unwrap();
    let file = create_test_file(&temp_dir, "test.py", "print('Hello, World!')");
    
    Command::cargo_bin("code-runner")
        .unwrap()
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, World!"));
}

#[test]
fn test_javascript_execution() {
    let temp_dir = TempDir::new().unwrap();
    let file = create_test_file(&temp_dir, "test.js", "console.log('Hello, JS!');");
    
    Command::cargo_bin("code-runner")
        .unwrap()
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, JS!"));
}

#[test]
fn test_file_with_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let file = create_test_file(&temp_dir, "hello world.py", "print('Spaces work!')");
    
    Command::cargo_bin("code-runner")
        .unwrap()
        .arg(&file)
        .assert()
        .success()
        .stdout(predicate::str::contains("Spaces work!"));
}

#[test]
fn test_nonexistent_file() {
    Command::cargo_bin("code-runner")
        .unwrap()
        .arg("nonexistent.py")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_unsupported_extension() {
    let temp_dir = TempDir::new().unwrap();
    let file = create_test_file(&temp_dir, "test.xyz", "some content");
    
    Command::cargo_bin("code-runner")
        .unwrap()
        .arg(&file)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported file type"));
}

#[test]
fn test_no_arguments() {
    Command::cargo_bin("code-runner")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
