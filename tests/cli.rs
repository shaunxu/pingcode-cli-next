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
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("http://"));
}

#[test]
fn pjm_help_lists_workitem_resource() {
    pc().arg("pjm")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("workitem"));
}

#[test]
fn workitem_help_lists_create_operation() {
    pc().arg("pjm")
        .arg("workitem")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("create"));
}

#[test]
fn create_help_mentions_data_and_endpoint() {
    pc().arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--data"))
        .stdout(predicate::str::contains("/v1/pjm/workitems"));
}

#[test]
fn dry_run_create_works_without_credentials() {
    // dry-run 不发网络请求，因此无凭据也应成功，并在 stderr 预览请求。
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg(r#"{"project_id":"p1","type_id":"t1","title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems",
        ))
        .stderr(predicate::str::contains("\"title\": \"x\""));
}

#[test]
fn dry_run_flag_works_after_subcommand() {
    // 全局参数放在子命令之后同样生效。
    pc().arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--dry-run")
        .arg("--data")
        .arg(r#"{"project_id":"p1","type_id":"t1","title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"));
}

#[test]
fn create_rejects_invalid_json_data() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg("not-json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid JSON in --data"));
}

#[test]
fn create_rejects_non_object_data() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg("[1,2,3]")
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a JSON object"));
}

#[test]
fn dry_run_state_works_without_credentials() {
    pc().arg("state")
        .arg("--dry-run")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains("/v1/directory/team"))
        .stderr(predicate::str::contains("/v1/myself"));
}
