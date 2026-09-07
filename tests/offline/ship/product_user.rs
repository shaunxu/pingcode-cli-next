use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_user_help_lists_operations() {
    pc().arg("ship")
        .arg("product-user")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_user_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-user")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/users",
        ));
}

#[test]
fn dry_run_product_user_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-user")
        .arg("get")
        .arg("prod-1")
        .arg("USR-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/users/USR-1",
        ));
}

#[test]
fn dry_run_product_user_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-user")
        .arg("create")
        .arg("prod-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/users",
        ));
}

#[test]
fn dry_run_product_user_update_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-user")
        .arg("update")
        .arg("prod-1")
        .arg("USR-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/users/USR-1",
        ));
}

#[test]
fn dry_run_product_user_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-user")
        .arg("delete")
        .arg("prod-1")
        .arg("USR-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/users/USR-1",
        ));
}
