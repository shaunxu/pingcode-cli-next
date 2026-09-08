use crate::common::pc;
use predicates::prelude::*;
use serde_json::Value;

/// 运行 doctor，返回退出码与解析后的 stdout JSON。
fn doctor_json(args: &[&str]) -> (Option<i32>, Value) {
    let mut cmd = pc();
    cmd.arg("doctor");
    for arg in args {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("doctor runs");
    let stdout = String::from_utf8(output.stdout.clone()).expect("stdout is utf8");
    let json: Value = serde_json::from_str(&stdout).expect("doctor stdout is valid JSON");
    (output.status.code(), json)
}

/// 按 id 取出检查项。
fn check<'a>(report: &'a Value, id: &str) -> &'a Value {
    report["checks"]
        .as_array()
        .expect("checks is an array")
        .iter()
        .find(|c| c["id"] == id)
        .unwrap_or_else(|| panic!("check {id} not found"))
}

fn status_of<'a>(report: &'a Value, id: &str) -> &'a str {
    check(report, id)["status"].as_str().unwrap()
}

#[test]
fn doctor_works_without_credentials_and_reports_failure() {
    let (code, report) = doctor_json(&[]);
    // 配置有问题 -> 退出码 1，但命令本身成功运行并产出 JSON。
    assert_eq!(code, Some(1));

    assert_eq!(report["ok"], Value::Bool(false));
    assert_eq!(status_of(&report, "credentials_present"), "fail");
    assert_eq!(status_of(&report, "base_url_format"), "pass");
    assert_eq!(status_of(&report, "credential_pair"), "skipped");
    assert_eq!(status_of(&report, "credential_conflict"), "pass");

    // 无有效配置，网络探针全部跳过。
    for id in [
        "base_url_reachable",
        "token_exchange",
        "api_auth",
        "token_kind",
    ] {
        assert_eq!(status_of(&report, id), "skipped", "check {id}");
    }

    // 失败项必须带可执行的修复建议。
    let failed = check(&report, "credentials_present");
    assert!(failed["remediation"]["steps"].is_array());
    assert!(failed["remediation"]["steps"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s.as_str().unwrap().contains("PC_CLIENT_ID")));

    // config 段反映认证方式与来源。
    assert_eq!(
        report["config"]["authentication_method"],
        Value::String("none".to_string())
    );
    assert_eq!(
        report["config"]["base_url"],
        Value::String("https://api.pingcode.com".to_string())
    );
}

#[test]
fn doctor_reports_missing_client_secret() {
    let (code, report) = doctor_json(&["--client-id", "cid"]);
    assert_eq!(code, Some(1));
    assert_eq!(status_of(&report, "credential_pair"), "fail");
    assert_eq!(status_of(&report, "credentials_present"), "pass");
    let pair = check(&report, "credential_pair");
    assert!(pair["message"].as_str().unwrap().contains("Client Secret"));
    assert!(pair["remediation"]["steps"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s.as_str().unwrap().contains("PC_CLIENT_SECRET")));
}

#[test]
fn doctor_reports_invalid_base_url_format() {
    // 配合 --dry-run 保证不发网络请求；base_url 格式错误在静态阶段即失败。
    let (code, report) = doctor_json(&["--base-url", "ftp://example.com", "--dry-run"]);
    assert_eq!(code, Some(1));
    assert_eq!(status_of(&report, "base_url_format"), "fail");
    assert!(check(&report, "base_url_format")["message"]
        .as_str()
        .unwrap()
        .contains("http:// or https://"));
}

#[test]
fn doctor_dry_run_with_client_credentials_passes_static_checks() {
    let (code, report) = doctor_json(&[
        "--client-id",
        "cid",
        "--client-secret",
        "csecret",
        "--dry-run",
    ]);
    assert_eq!(code, Some(0));
    assert_eq!(report["ok"], Value::Bool(true));
    assert_eq!(status_of(&report, "base_url_format"), "pass");
    assert_eq!(status_of(&report, "credentials_present"), "pass");
    assert_eq!(status_of(&report, "credential_pair"), "pass");
    assert_eq!(status_of(&report, "credential_conflict"), "pass");
    for id in [
        "base_url_reachable",
        "token_exchange",
        "api_auth",
        "token_kind",
    ] {
        assert_eq!(status_of(&report, id), "skipped", "check {id}");
    }
    assert_eq!(
        report["config"]["authentication_method"],
        Value::String("client_credentials".to_string())
    );
    assert_eq!(
        report["config"]["credential_sources"]["client_id"],
        Value::String("cli".to_string())
    );
}

#[test]
fn doctor_warns_on_credential_conflict() {
    let (code, report) = doctor_json(&[
        "--token",
        "tok",
        "--client-id",
        "cid",
        "--client-secret",
        "csecret",
        "--dry-run",
    ]);
    // warn 不影响退出码。
    assert_eq!(code, Some(0));
    assert_eq!(report["ok"], Value::Bool(true));
    assert_eq!(status_of(&report, "credential_conflict"), "warn");
    assert_eq!(
        report["config"]["authentication_method"],
        Value::String("access_token".to_string())
    );
}

#[test]
fn doctor_prints_human_readable_checklist_to_stderr() {
    pc().arg("doctor")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("[FAIL] credentials_present"))
        .stderr(predicate::str::contains("[SKIP] api_auth"));
}

#[test]
fn doctor_reports_unreachable_host() {
    // 指向一个本机 discard 端口，连接会快速被拒（不依赖外网/DNS）。
    let (code, report) = doctor_json(&[
        "--base-url",
        "http://127.0.0.1:9",
        "--client-id",
        "cid",
        "--client-secret",
        "csecret",
    ]);
    assert_eq!(code, Some(1));
    assert_eq!(status_of(&report, "base_url_format"), "pass");
    assert_eq!(status_of(&report, "base_url_reachable"), "fail");
    assert_eq!(
        check(&report, "base_url_reachable")["detail"]["kind"],
        Value::String("connect".to_string())
    );
}
