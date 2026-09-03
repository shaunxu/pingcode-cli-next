use crate::common::pc;
use predicates::prelude::*;

#[test]
fn release_help_lists_operations() {
    pc().arg("pjm")
        .arg("release")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("bulk-create"));
}

#[test]
fn dry_run_release_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("list")
        .arg("prj-123")
        .arg("--name")
        .arg("v1.0")
        .arg("--status")
        .arg("in_progress")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases?",
        ))
        .stderr(predicate::str::contains("name=v1.0"))
        .stderr(predicate::str::contains("status=in_progress"));
}

#[test]
fn dry_run_release_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("get")
        .arg("prj-123")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"v1.0","assignee_id":"u-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases",
        ))
        .stderr(predicate::str::contains("\"name\": \"v1.0\""));
}

#[test]
fn dry_run_release_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("update")
        .arg("prj-123")
        .arg("rel-1")
        .arg("--data")
        .arg(r#"{"name":"v1.0.1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("delete")
        .arg("prj-123")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_bulk_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("bulk-create")
        .arg("--data")
        .arg(r#"{"releases":[{"project_id":"prj-1","name":"v1.0"}]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/releases/bulk",
        ))
        .stderr(predicate::str::contains("\"releases\""));
}
