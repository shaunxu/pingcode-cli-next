use crate::common::pc;
use predicates::prelude::*;

#[test]
fn user_help_lists_operations() {
    pc().arg("organization")
        .arg("user")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("bulk-update"));
}

#[test]
fn dry_run_user_list_previews_query() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("user")
        .arg("list")
        .arg("--keywords")
        .arg("alice")
        .arg("--department-ids")
        .arg("dep-1,dep-2")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/users?",
        ))
        .stderr(predicate::str::contains("keywords=alice"))
        .stderr(predicate::str::contains("department_ids=dep-1%2Cdep-2"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_user_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("user")
        .arg("get")
        .arg("usr-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/users/usr-123",
        ));
}

#[test]
fn dry_run_user_create_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("user")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"alice","display_name":"Alice","email":"alice@example.com"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/users",
        ))
        .stderr(predicate::str::contains("\"display_name\": \"Alice\""));
}

#[test]
fn dry_run_user_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("user")
        .arg("update")
        .arg("usr-123")
        .arg("--data")
        .arg(r#"{"status":"disabled"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/users/usr-123",
        ))
        .stderr(predicate::str::contains("\"status\": \"disabled\""));
}

#[test]
fn dry_run_user_bulk_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("user")
        .arg("bulk-update")
        .arg("--data")
        .arg(r#"{"user_ids":["usr-1","usr-2"],"property_name":"status","property_value":"disabled"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/users/bulk",
        ))
        .stderr(predicate::str::contains("\"property_name\": \"status\""));
}
