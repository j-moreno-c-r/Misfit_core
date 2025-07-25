use assert_cmd::Command;

const EXIT_MESSAGE: &str = "-> Program finalized 👋\n";

#[test]
fn test_exit_command() {
    Command::cargo_bin("misfit_core")
        .unwrap()
        .write_stdin("exit")
        .assert()
        .success()
        .stdout(EXIT_MESSAGE);
}

#[test]
fn test_help_command() {
    Command::cargo_bin("misfit_core")
        .unwrap()
        .write_stdin("help")
        .write_stdin("exit")
        .assert()
        .success()
        .stdout(EXIT_MESSAGE);
}

#[test]
fn test_clear_command() {
    Command::cargo_bin("misfit_core")
        .unwrap()
        .write_stdin("clear")
        .write_stdin("exit")
        .assert()
        .success()
        .stdout(EXIT_MESSAGE);
}

#[test]
fn test_block_command() {
    Command::cargo_bin("misfit_core")
        .unwrap()
        .write_stdin("block")
        .write_stdin("exit")
        .assert()
        .success()
        .stdout(EXIT_MESSAGE);
}

#[test]
fn test_tx_command() {
    Command::cargo_bin("misfit_core")
        .unwrap()
        .write_stdin("tx")
        .write_stdin("exit")
        .assert()
        .success()
        .stdout(EXIT_MESSAGE);
}