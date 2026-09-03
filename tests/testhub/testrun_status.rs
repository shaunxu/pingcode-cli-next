use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testrun_status_help_lists_operations() {
    pc().arg("testhub")
        .arg("testrun-status")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-library"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_testrun_status_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun-status")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testrun_statuses",
        ));
}

#[test]
fn dry_run_testrun_status_list_for_library() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun-status")
        .arg("list-for-library")
        .arg("--library-id")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testrun/statuses?",
        ))
        .stderr(predicate::str::contains("library_id=lib-1"));
}

#[test]
fn dry_run_testrun_status_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun-status")
        .arg("get")
        .arg("trs-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testrun_statuses/trs-1",
        ));
}
