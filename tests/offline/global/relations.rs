use crate::common::pc;
use predicates::prelude::*;

#[test]
fn relations_help_lists_operations() {
    pc().arg("relations")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_relations_list_previews_query() {
    pc().arg("--dry-run")
        .arg("relations")
        .arg("list")
        .arg("--principal-type")
        .arg("idea")
        .arg("--principal-id")
        .arg("idea-1")
        .arg("--target-type")
        .arg("ticket")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/relations?",
        ))
        .stderr(predicate::str::contains("principal_type=idea"))
        .stderr(predicate::str::contains("principal_id=idea-1"))
        .stderr(predicate::str::contains("target_type=ticket"));
}

#[test]
fn dry_run_relations_get_previews_include_deleted_flag() {
    pc().arg("--dry-run")
        .arg("relations")
        .arg("get")
        .arg("rel-1")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/relations/rel-1?",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_relations_create_previews_post_body() {
    pc().arg("--dry-run")
        .arg("relations")
        .arg("create")
        .arg("--data")
        .arg(r#"{"principal_type":"idea","principal_id":"idea-1","target_type":"ticket","target_id":"t-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/relations",
        ))
        .stderr(predicate::str::contains("target_id"))
        .stderr(predicate::str::contains("t-2"));
}

#[test]
fn dry_run_relations_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("relations")
        .arg("delete")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/relations/rel-1",
        ));
}

#[test]
fn relations_list_requires_principal_and_target() {
    pc().arg("--dry-run")
        .arg("relations")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--principal-type"))
        .stderr(predicate::str::contains("--target-type"));
}
