use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testplan_type_help_lists_operations() {
    pc().arg("testhub")
        .arg("testplan-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_testplan_type_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan-type")
        .arg("list")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplan_types",
        ));
}

#[test]
fn dry_run_testplan_type_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan-type")
        .arg("get")
        .arg("lib-1")
        .arg("tpt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplan_types/tpt-1",
        ));
}
