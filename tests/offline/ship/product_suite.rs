use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_suite_help_lists_operations() {
    pc().arg("ship")
        .arg("product-suite")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_suite_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-suite")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/suites",
        ));
}

#[test]
fn dry_run_product_suite_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-suite")
        .arg("get")
        .arg("prod-1")
        .arg("SUI-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/suites/SUI-1",
        ));
}

#[test]
fn dry_run_product_suite_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-suite")
        .arg("create")
        .arg("prod-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/suites",
        ));
}

#[test]
fn dry_run_product_suite_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-suite")
        .arg("delete")
        .arg("prod-1")
        .arg("SUI-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/suites/SUI-1",
        ));
}
