use assert_cmd::Command;
use predicates::prelude::*;

/// Tests that at least one of `--gpg` and `--ssh` flags must be specified
#[test]
fn must_specify_at_least_one_proxy() {
    Command::cargo_bin("wsl-gpg-ssh-pageant")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "error: The following required arguments were not provided:
  <--gpg|--ssh>",
        ));
}

/// Tests that both the gpg-agent and ssh-pageant proxies can be launched at the same time
#[test]
fn can_launch_both_gpg_and_ssh_proxies() {
    Command::cargo_bin("wsl-gpg-ssh-pageant")
        .unwrap()
        .args(&["--gpg", "--ssh"])
        .assert()
        .success()
        .stdout(
            "Launching the gpg-agent proxy...
Launching the ssh-pageant proxy...\n",
        );
}
