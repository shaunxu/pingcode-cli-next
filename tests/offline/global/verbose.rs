use crate::common::pc;
use predicates::prelude::*;

#[test]
fn verbose_help_documents_http_logging() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--verbose"))
        .stdout(predicate::str::contains("HTTP request and response"));
}

#[test]
fn verbose_logs_token_request_to_stderr_without_leaking_secret() {
    // 指向不可路由的地址：令牌换取必然失败，但失败前应先把请求日志打到 stderr。
    let secret = "super-secret-value-123";
    pc().arg("-v")
        .arg("--base-url")
        .arg("http://127.0.0.1:9")
        .arg("--client-id")
        .arg("test-client")
        .arg("--client-secret")
        .arg(secret)
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("] REQUEST GET"))
        .stderr(predicate::str::contains("/v1/auth/token"))
        .stderr(predicate::str::contains("client_secret=***"))
        .stderr(predicate::str::contains("Headers"))
        .stderr(predicate::str::contains("user-agent"))
        .stderr(predicate::str::contains(secret).not());
}

#[test]
fn verbose_dry_run_keeps_dry_run_preview_only() {
    // dry-run 不发网络，只输出 [dry-run] 预览，不应出现 REQUEST/RESPONSE 日志。
    pc().arg("-v")
        .arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run]"))
        .stderr(predicate::str::contains("RESPONSE").not());
}
