use assert_cmd::Command;
use predicates::prelude::*;

fn pc() -> Command {
    let mut cmd = Command::cargo_bin("pc").unwrap();
    // 隔离宿主环境，避免开发者本机的认证信息影响断言
    cmd.env_remove("PC_TOKEN")
        .env_remove("PC_CLIENT_ID")
        .env_remove("PC_CLIENT_SECRET")
        .env_remove("PC_OPEN_API_BASE_URL");
    // 切换到不含 .env 的目录，避免仓库根目录的 .env 被自动加载污染断言
    cmd.current_dir(std::env::temp_dir());
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
fn help_lists_state_command() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("state"));
}

#[test]
fn version_succeeds() {
    pc().arg("--version").assert().success();
}

#[test]
fn missing_credentials_fails_with_hint() {
    pc().arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_ID"))
        .stderr(predicate::str::contains("PC_TOKEN"));
}

#[test]
fn client_id_without_secret_fails() {
    pc().arg("--client-id")
        .arg("cid")
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_SECRET"));
}

#[test]
fn client_secret_without_id_fails() {
    pc().arg("--client-secret")
        .arg("secret")
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_ID"));
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
