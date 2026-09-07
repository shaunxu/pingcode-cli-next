use crate::common::pc;
use predicates::prelude::*;

#[test]
fn library_member_help_lists_operations() {
    pc().arg("testhub")
        .arg("library-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_library_member_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-member")
        .arg("list")
        .arg("lib-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/members",
        ));
}

#[test]
fn dry_run_library_member_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-member")
        .arg("get")
        .arg("lib-1")
        .arg("member-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/members/member-1",
        ));
}

#[test]
fn dry_run_library_member_add() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-member")
        .arg("add")
        .arg("lib-1")
        .arg("--data")
        .arg(r#"{"role_id":"r-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/members",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r-1\""));
}

#[test]
fn dry_run_library_member_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-member")
        .arg("update")
        .arg("lib-1")
        .arg("member-1")
        .arg("--data")
        .arg(r#"{"role_id":"r-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/members/member-1",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r-1\""));
}

#[test]
fn dry_run_library_member_remove() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-member")
        .arg("remove")
        .arg("lib-1")
        .arg("member-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/members/member-1",
        ));
}
