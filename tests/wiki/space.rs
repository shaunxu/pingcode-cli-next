use crate::common::pc;
use predicates::prelude::*;

#[test]
fn space_help_lists_operations() {
    pc().arg("wiki")
        .arg("space")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_space_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("list")
        .arg("--scope-type")
        .arg("user_group")
        .arg("--scope-id")
        .arg("grp-1")
        .arg("--keywords")
        .arg("demo")
        .arg("--include-archived")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces?",
        ))
        .stderr(predicate::str::contains("scope_type=user_group"))
        .stderr(predicate::str::contains("scope_id=grp-1"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("include_archived=true"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn space_list_requires_member_id_with_member_type() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("list")
        .arg("--member-type")
        .arg("user")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--member-id"));
}

#[test]
fn dry_run_space_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("get")
        .arg("spc-123")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_space_create_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Demo","scope_type":"organization","identifier":"DEMO"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces",
        ))
        .stderr(predicate::str::contains("\"identifier\": \"DEMO\""));
}

#[test]
fn dry_run_space_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("update")
        .arg("spc-123")
        .arg("--data")
        .arg(r#"{"name":"New name"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123",
        ))
        .stderr(predicate::str::contains("\"name\": \"New name\""));
}

#[test]
fn dry_run_space_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("space")
        .arg("delete")
        .arg("spc-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/spaces/spc-123",
        ));
}
