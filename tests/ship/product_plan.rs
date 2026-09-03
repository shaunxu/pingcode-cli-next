use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_plan_help_lists_operations() {
    pc().arg("ship")
        .arg("product-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_plan_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-plan")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/plans",
        ));
}

#[test]
fn dry_run_product_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-plan")
        .arg("get")
        .arg("prod-1")
        .arg("PLN-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/plans/PLN-1",
        ));
}
