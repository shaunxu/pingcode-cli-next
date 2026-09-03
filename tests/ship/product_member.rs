use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_member_help_lists_operations() {
    pc().arg("ship")
        .arg("product-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_member_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-member")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/members",
        ));
}

#[test]
fn dry_run_product_member_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-member")
        .arg("get")
        .arg("prod-1")
        .arg("MEM-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/members/MEM-1",
        ));
}

#[test]
fn dry_run_product_member_add_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-member")
        .arg("add")
        .arg("prod-1")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/members",
        ));
}

#[test]
fn dry_run_product_member_remove_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-member")
        .arg("remove")
        .arg("prod-1")
        .arg("MEM-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/members/MEM-1",
        ));
}
