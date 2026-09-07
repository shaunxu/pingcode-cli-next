use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_tag_help_lists_operations() {
    pc().arg("ship")
        .arg("product-tag")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_tag_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-tag")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/tags",
        ));
}

#[test]
fn dry_run_product_tag_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-tag")
        .arg("get")
        .arg("prod-1")
        .arg("TAG-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/tags/TAG-1",
        ));
}

#[test]
fn dry_run_product_tag_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-tag")
        .arg("create")
        .arg("prod-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/tags",
        ));
}

#[test]
fn dry_run_product_tag_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-tag")
        .arg("delete")
        .arg("prod-1")
        .arg("TAG-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/tags/TAG-1",
        ));
}
