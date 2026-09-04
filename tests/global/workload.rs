use crate::common::pc;
use predicates::prelude::*;

#[test]
fn top_level_help_lists_workload_commands() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("workload"))
        .stdout(predicate::str::contains("workload-type"));
}

#[test]
fn workload_help_lists_operations() {
    pc().arg("workload")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workload_list_previews_query() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--pilot-id")
        .arg("prj-123")
        .arg("--principal-id")
        .arg("wi-456")
        .arg("--start-at")
        .arg("1704067200")
        .arg("--end-at")
        .arg("1711929600")
        .arg("--report-by-id")
        .arg("usr-789")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workloads?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("pilot_id=prj-123"))
        .stderr(predicate::str::contains("principal_id=wi-456"))
        .stderr(predicate::str::contains("start_at=1704067200"))
        .stderr(predicate::str::contains("end_at=1711929600"))
        .stderr(predicate::str::contains("report_by_id=usr-789"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn workload_list_requires_principal_type_with_principal_id() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("list")
        .arg("--principal-id")
        .arg("wi-456")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--principal-type"));
}

#[test]
fn workload_list_requires_principal_type_with_pilot_id() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("list")
        .arg("--pilot-id")
        .arg("prj-123")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--principal-type"));
}

#[test]
fn workload_list_requires_end_at_with_start_at() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("list")
        .arg("--start-at")
        .arg("1704067200")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--end-at"));
}

#[test]
fn dry_run_workload_get_previews_path() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("get")
        .arg("wl-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workloads/wl-123",
        ));
}

#[test]
fn dry_run_workload_create_previews_request() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("create")
        .arg("--data")
        .arg(r#"{"principal_id":"wi-456","principal_type":"workitem","duration":2.5,"report_at":1704067200}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workloads",
        ))
        .stderr(predicate::str::contains("\"duration\": 2.5"))
        .stderr(predicate::str::contains("\"principal_type\": \"workitem\""));
}

#[test]
fn workload_create_rejects_invalid_json() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("create")
        .arg("--data")
        .arg("not-json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid JSON in --data"));
}

#[test]
fn workload_create_rejects_non_object_body() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("create")
        .arg("--data")
        .arg("[1,2,3]")
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a JSON object"));
}

#[test]
fn dry_run_workload_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("update")
        .arg("wl-123")
        .arg("--data")
        .arg(r#"{"duration":3.0}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workloads/wl-123",
        ))
        .stderr(predicate::str::contains("\"duration\": 3.0"));
}

#[test]
fn dry_run_workload_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("workload")
        .arg("delete")
        .arg("wl-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workloads/wl-123",
        ));
}
