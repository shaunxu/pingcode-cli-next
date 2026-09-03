use crate::common::pc;
use predicates::prelude::*;

#[test]
fn project_property_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("add-to-project"))
        .stdout(predicate::str::contains("remove-from-project"));
}

#[test]
fn dry_run_project_property_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties",
        ));
}

#[test]
fn dry_run_project_property_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Priority","type":"select"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties",
        ))
        .stderr(predicate::str::contains("\"name\": \"Priority\""));
}

#[test]
fn dry_run_project_property_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("update")
        .arg("prop-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties/prop-1",
        ));
}

#[test]
fn dry_run_project_property_list_for_project_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("list-for-project")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties",
        ));
}

#[test]
fn dry_run_project_property_add_to_project_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("add-to-project")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"property_id":"prop-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties",
        ))
        .stderr(predicate::str::contains("\"property_id\": \"prop-1\""));
}

#[test]
fn dry_run_project_property_remove_from_project_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("remove-from-project")
        .arg("prj-123")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties/prop-1",
        ));
}
