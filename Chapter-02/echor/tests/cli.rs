use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_wrong_flag() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("-y").assert().failure();
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert().success();
    Ok(())
}

fn input_tests(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello1() -> TestResult {
    input_tests(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    input_tests(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello3() -> TestResult {
    input_tests(&["-n", "Hello there"], "tests/expected/hello3.txt")
}

#[test]
fn hello4() -> TestResult {
    input_tests(&["-n", "Hello", "there"], "tests/expected/hello4.txt")
}