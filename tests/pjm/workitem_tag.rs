use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_tag_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-tag")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_tag_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Blocked"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags",
        ))
        .stderr(predicate::str::contains("\"name\": \"Blocked\""));
}

#[test]
fn dry_run_workitem_tag_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("list")
        .arg("--name")
        .arg("block")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags?",
        ))
        .stderr(predicate::str::contains("name=block"));
}

#[test]
fn dry_run_workitem_tag_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/tags?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_tag_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("get")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}

#[test]
fn dry_run_workitem_tag_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("update")
        .arg("tag-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}

#[test]
fn dry_run_workitem_tag_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("delete")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}
