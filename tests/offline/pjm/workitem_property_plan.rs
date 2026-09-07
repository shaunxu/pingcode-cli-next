use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_property_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-property-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-property"))
        .stdout(predicate::str::contains("list-properties"))
        .stdout(predicate::str::contains("get-property"))
        .stdout(predicate::str::contains("remove-property"));
}

#[test]
fn dry_run_workitem_property_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans",
        ));
}

#[test]
fn dry_run_workitem_property_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("get")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1",
        ));
}

#[test]
fn dry_run_workitem_property_plan_add_property_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("add-property")
        .arg("pp-1")
        .arg("--data")
        .arg(r#"{"workitem_property_id":"prop-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_plan_list_properties_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("list-properties")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_plan_get_property_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("get-property")
        .arg("pp-1")
        .arg("prop-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties/prop-2",
        ));
}

#[test]
fn dry_run_workitem_property_plan_remove_property_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("remove-property")
        .arg("pp-1")
        .arg("prop-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties/prop-2",
        ));
}
