use crate::common::pc;
use predicates::prelude::*;

#[test]
fn group_help_lists_operations() {
    pc().arg("organization")
        .arg("group")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_group_list_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_group_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group")
        .arg("get")
        .arg("grp-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123",
        ));
}

#[test]
fn dry_run_group_create_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Dev Team","visibility":"public"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups",
        ))
        .stderr(predicate::str::contains("\"visibility\": \"public\""));
}

#[test]
fn dry_run_group_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group")
        .arg("update")
        .arg("grp-123")
        .arg("--data")
        .arg(r#"{"description":"updated"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123",
        ));
}
