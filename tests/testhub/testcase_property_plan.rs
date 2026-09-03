use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testcase_property_plan_help_lists_operations() {
    pc().arg("testhub")
        .arg("testcase-property-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("list-properties"))
        .stdout(predicate::str::contains("get-property"))
        .stdout(predicate::str::contains("add-property"))
        .stdout(predicate::str::contains("remove-property"));
}

#[test]
fn dry_run_testcase_property_plan_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("list")
        .arg("--library-id")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_property_plans?",
        ))
        .stderr(predicate::str::contains("library_id=lib-1"));
}

#[test]
fn dry_run_testcase_property_plan_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("get")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_property_plans/pp-1",
        ));
}

#[test]
fn dry_run_testcase_property_plan_list_properties() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("list-properties")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_property_plans/pp-1/testcase_properties",
        ));
}

#[test]
fn dry_run_testcase_property_plan_get_property() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("get-property")
        .arg("pp-1")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains("https://api.pingcode.com/v1/testhub/testcase_property_plans/pp-1/testcase_properties/prop-1"))
;
}

#[test]
fn dry_run_testcase_property_plan_add_property() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("add-property")
        .arg("pp-1")
        .arg("--data")
        .arg(r#"{"property_id":"prop-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcase_property_plans/pp-1/testcase_properties",
        ))
        .stderr(predicate::str::contains("\"property_id\": \"prop-1\""));
}

#[test]
fn dry_run_testcase_property_plan_remove_property() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase-property-plan")
        .arg("remove-property")
        .arg("pp-1")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains("https://api.pingcode.com/v1/testhub/testcase_property_plans/pp-1/testcase_properties/prop-1"))
;
}
