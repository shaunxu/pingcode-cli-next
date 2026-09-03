use crate::common::pc;
use predicates::prelude::*;

#[test]
fn department_help_lists_operations() {
    pc().arg("organization")
        .arg("department")
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
fn dry_run_department_list_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("department")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/departments?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_department_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("department")
        .arg("get")
        .arg("dep-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/departments/dep-123",
        ));
}

#[test]
fn dry_run_department_create_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("department")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Engineering","parent_id":"dep-0"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/departments",
        ))
        .stderr(predicate::str::contains("\"name\": \"Engineering\""));
}

#[test]
fn dry_run_department_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("department")
        .arg("update")
        .arg("dep-123")
        .arg("--data")
        .arg(r#"{"name":"Eng"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/departments/dep-123",
        ));
}

#[test]
fn dry_run_department_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("department")
        .arg("delete")
        .arg("dep-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/departments/dep-123",
        ));
}
