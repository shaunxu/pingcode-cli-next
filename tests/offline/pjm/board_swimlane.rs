use crate::common::pc;
use predicates::prelude::*;

#[test]
fn board_swimlane_help_lists_operations() {
    pc().arg("pjm")
        .arg("board-swimlane")
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
fn dry_run_board_swimlane_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("list")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes",
        ));
}

#[test]
fn dry_run_board_swimlane_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ));
}

#[test]
fn dry_run_board_swimlane_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .arg("--data")
        .arg(r#"{"name":"Team B"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"Team B\""));
}

#[test]
fn dry_run_board_swimlane_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ));
}
