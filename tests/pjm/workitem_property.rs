use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_property_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_workitem_property_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom field"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("update")
        .arg("prop-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed field"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties/prop-1",
        ));
}

#[test]
fn dry_run_workitem_property_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-type-id")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/properties?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_property_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("get")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties/prop-1",
        ));
}
