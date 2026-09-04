use crate::common::pc;
use predicates::prelude::*;

#[test]
fn participants_help_lists_operations() {
    pc().arg("participants")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_participants_list_previews_query() {
    pc().arg("--dry-run")
        .arg("participants")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--principal-id")
        .arg("wi-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/participants?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("principal_id=wi-123"));
}

#[test]
fn dry_run_participants_add_previews_post_body() {
    pc().arg("--dry-run")
        .arg("participants")
        .arg("add")
        .arg("--data")
        .arg(r#"{"principal_type":"ticket","principal_id":"t-1","type":"user","participant_id":"u-9"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/participants",
        ))
        .stderr(predicate::str::contains("\"type\": \"user\""))
        .stderr(predicate::str::contains("u-9"));
}

#[test]
fn dry_run_participants_remove_previews_delete() {
    pc().arg("--dry-run")
        .arg("participants")
        .arg("remove")
        .arg("u-9")
        .arg("--principal-type")
        .arg("idea")
        .arg("--principal-id")
        .arg("idea-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/participants/u-9?",
        ))
        .stderr(predicate::str::contains("principal_type=idea"));
}

#[test]
fn participants_list_rejects_unknown_principal_type() {
    pc().arg("--dry-run")
        .arg("participants")
        .arg("list")
        .arg("--principal-type")
        .arg("bogus")
        .arg("--principal-id")
        .arg("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
