use assert_cmd::Command;
use predicates::prelude::*;

fn pc() -> Command {
    let mut cmd = Command::cargo_bin("pc").unwrap();
    // 隔离宿主环境，避免开发者本机的 PINGCODE_TOKEN 影响断言
    cmd.env_remove("PINGCODE_TOKEN")
        .env_remove("PINGCODE_BASE_URL");
    cmd
}

#[test]
fn help_succeeds() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("PingCode"));
}

#[test]
fn version_succeeds() {
    pc().arg("--version").assert().success();
}

#[test]
fn missing_token_fails_with_hint() {
    pc().arg("whoami")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PINGCODE_TOKEN"));
}

#[test]
fn invalid_base_url_fails() {
    pc().arg("--base-url")
        .arg("not-a-url")
        .arg("--token")
        .arg("dummy")
        .arg("whoami")
        .assert()
        .failure()
        .stderr(predicate::str::contains("http://"));
}
