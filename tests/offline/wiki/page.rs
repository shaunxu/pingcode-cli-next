use crate::common::pc;
use predicates::prelude::*;

#[test]
fn page_help_lists_operations() {
    pc().arg("wiki")
        .arg("page")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("get-content"))
        .stdout(predicate::str::contains("update-content"))
        .stdout(predicate::str::contains("list-versions"))
        .stdout(predicate::str::contains("get-version"))
        .stdout(predicate::str::contains("restore-version"));
}

#[test]
fn dry_run_page_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("list")
        .arg("--space-id")
        .arg("spc-1")
        .arg("--parent-id")
        .arg("pg-9")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages?",
        ))
        .stderr(predicate::str::contains("space_id=spc-1"))
        .stderr(predicate::str::contains("parent_id=pg-9"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn page_list_rejects_parent_and_ancestor_together() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("list")
        .arg("--parent-id")
        .arg("pg-1")
        .arg("--ancestor-id")
        .arg("pg-2")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--parent-id"))
        .stderr(predicate::str::contains("--ancestor-id"));
}

#[test]
fn dry_run_page_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("get")
        .arg("pg-123")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_page_create_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("create")
        .arg("--data")
        .arg(r#"{"space_id":"spc-1","name":"Demo","content":"hi","format_type":"markdown"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages",
        ))
        .stderr(predicate::str::contains("\"name\": \"Demo\""));
}

#[test]
fn dry_run_page_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("update")
        .arg("pg-123")
        .arg("--data")
        .arg(r#"{"name":"New title","lock":1}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123",
        ))
        .stderr(predicate::str::contains("\"name\": \"New title\""));
}

#[test]
fn dry_run_page_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("delete")
        .arg("pg-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123",
        ));
}

#[test]
fn dry_run_page_get_content_previews_query() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("get-content")
        .arg("pg-123")
        .arg("--format-type")
        .arg("markdown")
        .arg("--version-id")
        .arg("ver-7")
        .arg("--include-public-image-token")
        .arg("content")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123/content?",
        ))
        .stderr(predicate::str::contains("format_type=markdown"))
        .stderr(predicate::str::contains("version_id=ver-7"))
        .stderr(predicate::str::contains(
            "include_public_image_token=content",
        ));
}

#[test]
fn dry_run_page_update_content_previews_put() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("update-content")
        .arg("pg-123")
        .arg("--data")
        .arg(r#"{"content":"hello body","format_type":"markdown"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PUT"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123/content",
        ))
        .stderr(predicate::str::contains("\"format_type\": \"markdown\""))
        .stderr(predicate::str::contains("hello body"));
}

#[test]
fn dry_run_page_list_versions_previews_request() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("list-versions")
        .arg("pg-123")
        .arg("--page-size")
        .arg("20")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123/versions?",
        ))
        .stderr(predicate::str::contains("page_size=20"));
}

#[test]
fn dry_run_page_get_version_previews_path() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("get-version")
        .arg("pg-123")
        .arg("ver-7")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123/versions/ver-7",
        ));
}

#[test]
fn dry_run_page_restore_version_previews_post() {
    pc().arg("--dry-run")
        .arg("wiki")
        .arg("page")
        .arg("restore-version")
        .arg("pg-123")
        .arg("ver-7")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/wiki/pages/pg-123/versions/ver-7/restore",
        ));
}
