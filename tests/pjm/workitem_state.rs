use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_state_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-all"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_workitem_state_list_all_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("list-all")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom state"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("update")
        .arg("st-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states/st-1",
        ));
}

#[test]
fn dry_run_workitem_state_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-type-id")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/states?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"))
        .stderr(predicate::str::contains("workitem_type_id=wt-1"));
}

#[test]
fn dry_run_workitem_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states/st-1",
        ));
}
