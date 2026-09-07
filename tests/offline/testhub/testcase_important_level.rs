use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testcase_important_level_help_lists_operations() {
    pc().arg("testhub")
        .arg("testcase-important-level")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_testcase_important_level_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-important-level")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_important_levels",
        ));
}

#[test]
fn dry_run_testcase_important_level_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-important-level")
        .arg("get")
        .arg("il-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_important_levels/il-1",
        ));
}
