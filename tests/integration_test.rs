use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let expected_output = "Usage: rust-cli <COMMAND>\n\nCommands:\n  first-level-sub-command  \n  help                     Print this message or the help of the given subcommand(s)\n\nOptions:\n  -h, --help     Print help\n  -V, --version  Print version\n";

    let mut cmd = Command::cargo_bin("rust-cli").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_output));
}

#[test]
fn test_version_command() {
    let expected_output = "rust-cli 0.1.0\n";

    let mut cmd = Command::cargo_bin("rust-cli").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_output));
}

#[test]
fn test_run_first_level_command() {
    let expected_output = "Here is the first flag value: hello\n";

    let mut cmd = Command::cargo_bin("rust-cli").unwrap();
    cmd.arg("first-level-sub-command");
    cmd.arg("--first-level-flag");
    cmd.arg("hello");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(expected_output));
}
