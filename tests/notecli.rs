use std::fs;
use std::process::Command;

const TEST_FILE: &str = "test_notes.json";

fn run_cmd(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_notecli"))
        .args(args)
        .env("RUST_BACKTRACE", "1")
        .output()
        .expect("failed to execute notecli");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn clean_test_file() {
    let _ = fs::remove_file(TEST_FILE);
}

#[test]
fn test_add_and_list_notes() {
    clean_test_file();

    let _ = run_cmd(&["new", "Integration test note"]);
    let list_output = run_cmd(&["list"]);

    assert!(list_output.contains("Integration test note"));
}
