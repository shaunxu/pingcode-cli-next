use crate::common::pc;
use predicates::prelude::*;

#[test]
fn role_help_lists_operations() {
    pc().arg("organization")
        .arg("role")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_role_list_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("role")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/roles?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_role_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("role")
        .arg("get")
        .arg("role-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/roles/role-123",
        ));
}
