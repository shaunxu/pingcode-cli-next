use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_property_plan_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-property-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-properties"))
        .stdout(predicate::str::contains("add-property"));
}

#[test]
fn dry_run_property_plan_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans",
        ));
}

#[test]
fn dry_run_property_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("get")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans/pp-1",
        ));
}

#[test]
fn dry_run_property_plan_add_property_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("add-property")
        .arg("pp-1")
        .arg("--data")
        .arg(r#"{"property_id":"prp-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans/pp-1/ticket_properties",
        ));
}

#[test]
fn dry_run_property_plan_list_properties_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("list-properties")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans/pp-1/ticket_properties",
        ));
}

#[test]
fn dry_run_property_plan_get_property_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("get-property")
        .arg("pp-1")
        .arg("prp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans/pp-1/ticket_properties/prp-1",
        ));
}

#[test]
fn dry_run_property_plan_remove_property_previews_delete() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-property-plan")
        .arg("remove-property")
        .arg("pp-1")
        .arg("prp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_property_plans/pp-1/ticket_properties/prp-1",
        ));
}
