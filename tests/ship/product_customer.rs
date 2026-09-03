use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_customer_help_lists_operations() {
    pc().arg("ship")
        .arg("product-customer")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_customer_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-customer")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/customers",
        ));
}

#[test]
fn dry_run_product_customer_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-customer")
        .arg("get")
        .arg("prod-1")
        .arg("CUS-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/customers/CUS-1",
        ));
}

#[test]
fn dry_run_product_customer_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-customer")
        .arg("create")
        .arg("prod-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/customers",
        ));
}

#[test]
fn dry_run_product_customer_update_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-customer")
        .arg("update")
        .arg("prod-1")
        .arg("CUS-1")
        .arg("--data")
        .arg(r#"{"customer_id":"cus-9"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/customers/CUS-1",
        ));
}
