use crate::common::pc;
use predicates::prelude::*;

#[test]
fn deliverable_help_lists_operations() {
    pc().arg("pjm")
        .arg("deliverable")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_deliverable_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("create")
        .arg("--data")
        .arg(r#"{"workitem_id":"wi-1","name":"Spec doc"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables",
        ))
        .stderr(predicate::str::contains("\"name\": \"Spec doc\""));
}

#[test]
fn dry_run_deliverable_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-id")
        .arg("wi-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"))
        .stderr(predicate::str::contains("workitem_id=wi-1"));
}

#[test]
fn dry_run_deliverable_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("get")
        .arg("d-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}

#[test]
fn dry_run_deliverable_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("update")
        .arg("d-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}

#[test]
fn dry_run_deliverable_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("delete")
        .arg("d-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}
