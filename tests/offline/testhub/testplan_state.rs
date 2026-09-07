use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testplan_state_help_lists_operations() {
    pc().arg("testhub")
        .arg("testplan-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_testplan_state_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan-state")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testplan_states",
        ));
}

#[test]
fn dry_run_testplan_state_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testplan_states/st-1",
        ));
}
