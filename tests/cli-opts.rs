use std::fs;

use assert_cmd::cargo_bin;
use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_empty_args() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.assert().success();
}

#[test]
fn test_help() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.arg("-h").assert().success();
}

#[test]
fn test_version() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.arg("-V").assert().success();
}

#[test]
fn test_json() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.arg("-J").assert().success();
}

#[test]
fn test_csv() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.arg("-C").assert().success();
}

#[test]
fn test_output_file() {
    let mut cmd = Command::new(cargo_bin!());
    cmd.arg("-o").arg("foo.txt").assert().success();
    fs::remove_file("foo.txt").unwrap()
}
