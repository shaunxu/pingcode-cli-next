use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testcase_property_help_lists_operations() {
    pc().arg("testhub")
        .arg("testcase-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-library"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_testcase_property_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_properties",
        ));
}

#[test]
fn dry_run_testcase_property_list_for_library() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property")
        .arg("list-for-library")
        .arg("--library-id")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase/properties?",
        ))
        .stderr(predicate::str::contains("library_id=lib-1"));
}

#[test]
fn dry_run_testcase_property_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property")
        .arg("get")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_properties/prop-1",
        ));
}

#[test]
fn dry_run_testcase_property_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"x","type":"text"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_properties",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}

#[test]
fn dry_run_testcase_property_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property")
        .arg("update")
        .arg("prop-1")
        .arg("--data")
        .arg(r#"{"name":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_properties/prop-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}
