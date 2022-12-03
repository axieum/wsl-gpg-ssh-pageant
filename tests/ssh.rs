use assert_cmd::Command;

/// Tests that the SSH pageant proxy is launched
#[test]
fn launch_ssh_pageant() {
    Command::cargo_bin("wsl-gpg-ssh-pageant")
        .unwrap()
        .arg("--ssh")
        .assert()
        .success()
        .stdout("Launching the ssh-pageant proxy...\n");
}
