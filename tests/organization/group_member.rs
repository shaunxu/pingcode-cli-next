use crate::common::pc;
use predicates::prelude::*;

#[test]
fn group_member_help_lists_operations() {
    pc().arg("organization")
        .arg("group-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_group_member_list_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group-member")
        .arg("list")
        .arg("grp-123")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123/members?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_group_member_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group-member")
        .arg("get")
        .arg("grp-123")
        .arg("usr-456")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123/members/usr-456",
        ));
}

#[test]
fn dry_run_group_member_add_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group-member")
        .arg("add")
        .arg("grp-123")
        .arg("--data")
        .arg(r#"{"user_id":"usr-456","role":"manager"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123/members",
        ))
        .stderr(predicate::str::contains("\"role\": \"manager\""));
}

#[test]
fn dry_run_group_member_remove_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("group-member")
        .arg("remove")
        .arg("grp-123")
        .arg("usr-456")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/groups/grp-123/members/usr-456",
        ));
}
