use std::fs;

use assert_cmd::Command;

const BIN: &str = "fetch-sac";

#[test]
fn test_empty_args() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.assert().success();
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-h").assert().success();
}

#[test]
fn test_version() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-V").assert().success();
}

#[test]
fn test_json() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-J").assert().success();
}

#[test]
fn test_csv() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-C").assert().success();
}

#[test]
fn test_output_file() {
    let mut cmd = Command::cargo_bin(BIN).unwrap();
    cmd.arg("-o").arg("foo.txt").assert().success();
    fs::remove_file("foo.txt").unwrap()
}
