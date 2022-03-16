use std::fs;

use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn fails_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("rust_echo")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
    Ok(())
}

#[test]
fn pass() -> TestResult {
    let mut cmd = Command::cargo_bin("rust_echo")?;
    cmd.arg("Hello, world!").assert().success();
    Ok(())
}

fn run(args: &[&str], output_file: &str) -> TestResult {
    let expected_output = fs::read_to_string(output_file)?;
    let mut cmd = Command::cargo_bin("rust_echo")?;
    cmd.args(args).assert().success().stdout(expected_output);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello, world!"], "tests/expected/hello1.out")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello,", "world!"], "tests/expected/hello2.out")
}

#[test]
fn hello1_n() -> TestResult {
    run(&["-n", "Hello,    world!"], "tests/expected/hello1_n.out")
}

#[test]
fn hello2_n() -> TestResult {
    run(&["Hello,", "world!", "-n"], "tests/expected/hello2_n.out")
}
