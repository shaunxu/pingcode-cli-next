use crate::common::pc;
use predicates::prelude::*;

#[test]
fn activities_help_lists_operations() {
    pc().arg("activities")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_activities_list_previews_query() {
    pc().arg("--dry-run")
        .arg("activities")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--principal-id")
        .arg("wi-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/activities?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("principal_id=wi-123"));
}

#[test]
fn dry_run_activities_get_previews_query() {
    pc().arg("--dry-run")
        .arg("activities")
        .arg("get")
        .arg("act-1")
        .arg("--principal-type")
        .arg("ticket")
        .arg("--principal-id")
        .arg("t-5")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/activities/act-1?",
        ))
        .stderr(predicate::str::contains("principal_type=ticket"));
}

#[test]
fn activities_list_rejects_review_principal_type() {
    pc().arg("--dry-run")
        .arg("activities")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem_review")
        .arg("--principal-id")
        .arg("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
