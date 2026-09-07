use crate::common::pc;
use predicates::prelude::*;

#[test]
fn project_member_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_project_member_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members",
        ));
}

#[test]
fn dry_run_project_member_add_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("add")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"member":{"type":"user","id":"u1"},"role_id":"r1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r1\""));
}

#[test]
fn dry_run_project_member_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("update")
        .arg("prj-123")
        .arg("u1")
        .arg("--data")
        .arg(r#"{"role_id":"r2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members/u1",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r2\""));
}

#[test]
fn dry_run_project_member_remove_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("remove")
        .arg("prj-123")
        .arg("u1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members/u1",
        ));
}
