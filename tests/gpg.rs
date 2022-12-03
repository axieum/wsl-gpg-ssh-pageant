use assert_cmd::Command;

/// Tests that the GPG agent proxy is launched
#[test]
fn launch_gpg_agent() {
    Command::cargo_bin("wsl-gpg-ssh-pageant")
        .unwrap()
        .arg("--gpg")
        .assert()
        .success()
        .stdout("Launching the gpg-agent proxy...\n");
}
