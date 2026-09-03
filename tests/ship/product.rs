use crate::common::pc;
use predicates::prelude::*;

#[test]
fn product_help_lists_operations() {
    pc().arg("ship")
        .arg("product")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_product_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product")
        .arg("list")
        .arg("--scope-type")
        .arg("organization")
        .arg("--keywords")
        .arg("demo")
        .arg("--include-archived")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products?",
        ))
        .stderr(predicate::str::contains("scope_type=organization"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("include_archived=true"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn product_list_requires_member_id_with_member_type() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product")
        .arg("list")
        .arg("--member-type")
        .arg("user")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--member-id"));
}

#[test]
fn dry_run_product_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product")
        .arg("get")
        .arg("prod-123")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-123",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_product_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Demo","identifier":"DEMO"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products",
        ))
        .stderr(predicate::str::contains("\"identifier\": \"DEMO\""));
}

#[test]
fn dry_run_product_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product")
        .arg("update")
        .arg("prod-123")
        .arg("--data")
        .arg(r#"{"name":"New name"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-123",
        ))
        .stderr(predicate::str::contains("\"name\": \"New name\""));
}
