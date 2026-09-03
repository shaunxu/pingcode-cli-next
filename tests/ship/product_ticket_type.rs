use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_ticket_type_help_lists_operations() {
    pc().arg("ship")
        .arg("product-ticket-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_ticket_type_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-ticket-type")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/ticket_types",
        ));
}

#[test]
fn dry_run_product_ticket_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-ticket-type")
        .arg("get")
        .arg("prod-1")
        .arg("TT-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/ticket_types/TT-1",
        ));
}
