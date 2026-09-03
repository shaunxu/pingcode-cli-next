use crate::common::pc;
use predicates::prelude::*;

#[test]
fn library_help_lists_operations() {
    pc().arg("testhub")
        .arg("library")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_library_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library")
        .arg("list")
        .arg("--scope-type")
        .arg("organization")
        .arg("--scope-id")
        .arg("team-1")
        .arg("--keywords")
        .arg("demo")
        .arg("--member-type")
        .arg("user")
        .arg("--member-id")
        .arg("member-1")
        .arg("--created-between")
        .arg("100")
        .arg("--updated-between")
        .arg("200")
        .arg("--include-deleted")
        .arg("--include-archived")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries?",
        ))
        .stderr(predicate::str::contains("scope_type=organization"))
        .stderr(predicate::str::contains("scope_id=team-1"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("member_type=user"))
        .stderr(predicate::str::contains("member_id=member-1"))
        .stderr(predicate::str::contains("created_between=100"))
        .stderr(predicate::str::contains("updated_between=200"))
        .stderr(predicate::str::contains("include_deleted=true"))
        .stderr(predicate::str::contains("include_archived=true"));
}

#[test]
fn dry_run_library_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library")
        .arg("get")
        .arg("lib-1")
        .arg("--include-deleted")
        .arg("--include-archived")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1?",
        ))
        .stderr(predicate::str::contains("include_deleted=true"))
        .stderr(predicate::str::contains("include_archived=true"));
}

#[test]
fn dry_run_library_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"x","identifier":"LIB"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}

#[test]
fn dry_run_library_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library")
        .arg("update")
        .arg("lib-1")
        .arg("--data")
        .arg(r#"{"name":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}
