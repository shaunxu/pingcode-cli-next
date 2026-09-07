use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_relation_type_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-relation-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_relation_type_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation-type")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_relation_types",
        ));
}

#[test]
fn dry_run_workitem_relation_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation-type")
        .arg("get")
        .arg("rt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_relation_types/rt-1",
        ));
}
