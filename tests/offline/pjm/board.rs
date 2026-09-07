use crate::common::pc;
use predicates::prelude::*;

#[test]
fn board_help_lists_operations() {
    pc().arg("pjm")
        .arg("board")
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
fn dry_run_board_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards",
        ));
}

#[test]
fn dry_run_board_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ));
}

#[test]
fn dry_run_board_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"Sprint board"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards",
        ))
        .stderr(predicate::str::contains("\"name\": \"Sprint board\""));
}

#[test]
fn dry_run_board_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed board"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"Renamed board\""));
}

#[test]
fn dry_run_board_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ));
}
