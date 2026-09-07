use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_relation_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-relation")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_relation_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("create")
        .arg("wi-1")
        .arg("--data")
        .arg(r#"{"target_workitem_id":"wi-2","relation_type":"relate"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations",
        ))
        .stderr(predicate::str::contains("\"relation_type\": \"relate\""));
}

#[test]
fn dry_run_workitem_relation_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("list")
        .arg("wi-1")
        .arg("--relation-type")
        .arg("block")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations?",
        ))
        .stderr(predicate::str::contains("relation_type=block"));
}

#[test]
fn dry_run_workitem_relation_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("get")
        .arg("wi-1")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations/rel-1",
        ));
}

#[test]
fn dry_run_workitem_relation_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("delete")
        .arg("wi-1")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations/rel-1",
        ));
}
