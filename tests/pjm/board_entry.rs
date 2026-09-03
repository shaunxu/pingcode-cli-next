use crate::common::pc;
use predicates::prelude::*;

#[test]
fn board_entry_help_lists_operations() {
    pc().arg("pjm")
        .arg("board-entry")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_board_entry_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("list")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries",
        ));
}

#[test]
fn dry_run_board_entry_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ));
}

#[test]
fn dry_run_board_entry_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("create")
        .arg("prj-123")
        .arg("brd-1")
        .arg("--data")
        .arg(r#"{"name":"To do","wip_limit":5}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries",
        ))
        .stderr(predicate::str::contains("\"wip_limit\": 5"));
}

#[test]
fn dry_run_board_entry_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .arg("--data")
        .arg(r#"{"is_split":true}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ))
        .stderr(predicate::str::contains("\"is_split\": true"));
}

#[test]
fn dry_run_board_entry_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ));
}
