use crate::common::pc;
use predicates::prelude::*;

#[test]
fn login_log_help_lists_operations() {
    pc().arg("security")
        .arg("login-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn dry_run_login_log_list_previews_request() {
    pc().arg("--dry-run")
        .arg("security")
        .arg("login-log")
        .arg("list")
        .arg("--logged-between")
        .arg("2026-01-01T00:00:00Z,2026-02-01T00:00:00Z")
        .arg("--user-ids")
        .arg("user-1,user-2")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/security/login_logs?",
        ))
        .stderr(predicate::str::contains("logged_between="))
        .stderr(predicate::str::contains("user_ids=user-1%2Cuser-2"))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn login_log_list_requires_logged_between() {
    pc().arg("--dry-run")
        .arg("security")
        .arg("login-log")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--logged-between"));
}

#[test]
fn login_log_list_rejects_more_than_20_user_ids() {
    let ids = (0..21)
        .map(|i| format!("u{i}"))
        .collect::<Vec<_>>()
        .join(",");
    pc().arg("--dry-run")
        .arg("security")
        .arg("login-log")
        .arg("list")
        .arg("--logged-between")
        .arg("2026-01-01,2026-02-01")
        .arg("--user-ids")
        .arg(ids)
        .assert()
        .failure()
        .stderr(predicate::str::contains("at most 20 ids"));
}
