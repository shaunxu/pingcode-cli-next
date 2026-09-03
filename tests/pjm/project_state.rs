use crate::common::pc;
use predicates::prelude::*;

#[test]
fn project_state_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_project_state_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-state")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project/states?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_project_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_states/st-1",
        ));
}
