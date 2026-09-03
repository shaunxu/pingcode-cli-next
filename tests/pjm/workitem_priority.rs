use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_priority_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-priority")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_priority_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_priorities",
        ));
}

#[test]
fn dry_run_workitem_priority_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/priorities?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_priority_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("get")
        .arg("pri-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_priorities/pri-1",
        ));
}
