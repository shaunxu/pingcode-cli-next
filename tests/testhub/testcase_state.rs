use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testcase_state_help_lists_operations() {
    pc().arg("testhub")
        .arg("testcase-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-library"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_testcase_state_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-state")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_states",
        ));
}

#[test]
fn dry_run_testcase_state_list_for_library() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-state")
        .arg("list-for-library")
        .arg("--library-id")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase/states?",
        ))
        .stderr(predicate::str::contains("library_id=lib-1"));
}

#[test]
fn dry_run_testcase_state_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_states/st-1",
        ));
}
