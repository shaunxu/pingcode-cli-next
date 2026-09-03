use crate::common::pc;
use predicates::prelude::*;

#[test]
fn release_section_help_lists_operations() {
    pc().arg("pjm")
        .arg("release-section")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_release_section_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections",
        ));
}

#[test]
fn dry_run_release_section_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"2026 H1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections",
        ))
        .stderr(predicate::str::contains("\"name\": \"2026 H1\""));
}

#[test]
fn dry_run_release_section_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("delete")
        .arg("prj-123")
        .arg("sec-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections/sec-1",
        ));
}
