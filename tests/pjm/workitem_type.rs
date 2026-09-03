use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_type_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_type_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom type"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("update")
        .arg("wt-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}

#[test]
fn dry_run_workitem_type_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("delete")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}

#[test]
fn dry_run_workitem_type_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/types?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("get")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}
