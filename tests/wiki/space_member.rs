use crate::common::pc;
use predicates::prelude::*;

#[test]
fn space_member_help_lists_operations() {
    pc().arg("wiki")
        .arg("space-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_space_member_list_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space-member")
        .arg("list")
        .arg("spc-123")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123/members?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_space_member_get_previews_path() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space-member")
        .arg("get")
        .arg("spc-123")
        .arg("usr-456")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123/members/usr-456",
        ));
}

#[test]
fn dry_run_space_member_add_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space-member")
        .arg("add")
        .arg("spc-123")
        .arg("--data")
        .arg(r#"{"member":{"type":"user","id":"usr-456"},"role_id":"role-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123/members",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"role-1\""));
}

#[test]
fn dry_run_space_member_remove_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space-member")
        .arg("remove")
        .arg("spc-123")
        .arg("usr-456")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123/members/usr-456",
        ));
}
