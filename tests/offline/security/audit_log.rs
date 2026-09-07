use crate::common::pc;
use predicates::prelude::*;

#[test]
fn audit_log_help_lists_operations() {
    pc().arg("security")
        .arg("audit-log")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn dry_run_audit_log_list_previews_request() {
    pc().arg("--dry-run")
        .arg("security")
        .arg("audit-log")
        .arg("list")
        .arg("--operated-between")
        .arg("2026-01-01T00:00:00Z,2026-02-01T00:00:00Z")
        .arg("--operated-bys")
        .arg("user-1,user-2")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/security/audit_logs?",
        ))
        .stderr(predicate::str::contains("operated_between="))
        .stderr(predicate::str::contains("operated_bys=user-1%2Cuser-2"))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn audit_log_list_requires_operated_between() {
    pc().arg("--dry-run")
        .arg("security")
        .arg("audit-log")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--operated-between"));
}

#[test]
fn audit_log_list_rejects_more_than_20_operator_ids() {
    let ids = (0..21)
        .map(|i| format!("u{i}"))
        .collect::<Vec<_>>()
        .join(",");
    pc().arg("--dry-run")
        .arg("security")
        .arg("audit-log")
        .arg("list")
        .arg("--operated-between")
        .arg("2026-01-01,2026-02-01")
        .arg("--operated-bys")
        .arg(ids)
        .assert()
        .failure()
        .stderr(predicate::str::contains("at most 20 ids"));
}
