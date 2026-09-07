use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testplan_help_lists_operations() {
    pc().arg("testhub")
        .arg("testplan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_testplan_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan")
        .arg("list")
        .arg("lib-1")
        .arg("--name")
        .arg("smoke")
        .arg("--created-between")
        .arg("100")
        .arg("--updated-between")
        .arg("200")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplans?",
        ))
        .stderr(predicate::str::contains("name=smoke"))
        .stderr(predicate::str::contains("created_between=100"))
        .stderr(predicate::str::contains("updated_between=200"));
}

#[test]
fn dry_run_testplan_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan")
        .arg("get")
        .arg("lib-1")
        .arg("tp-1")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplans/tp-1?",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_testplan_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan")
        .arg("create")
        .arg("lib-1")
        .arg("--data")
        .arg(r#"{"name":"x","type_id":"tt-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplans",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}

#[test]
fn dry_run_testplan_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testplan")
        .arg("update")
        .arg("lib-1")
        .arg("tp-1")
        .arg("--data")
        .arg(r#"{"name":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplans/tp-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}
