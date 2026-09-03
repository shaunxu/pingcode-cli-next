use crate::common::pc;
use predicates::prelude::*;

#[test]
fn release_stage_help_lists_operations() {
    pc().arg("pjm")
        .arg("release-stage")
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
fn dry_run_release_stage_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages",
        ));
}

#[test]
fn dry_run_release_stage_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("get")
        .arg("stage-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages/stage-1",
        ));
}

#[test]
fn dry_run_release_stage_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Dev","type":"in_progress"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages",
        ))
        .stderr(predicate::str::contains("\"name\": \"Dev\""));
}

#[test]
fn dry_run_release_stage_delete_previews_replace_id() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("delete")
        .arg("stage-1")
        .arg("--data")
        .arg(r#"{"replace_id":"stage-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages/stage-1",
        ))
        .stderr(predicate::str::contains("\"replace_id\": \"stage-2\""));
}
