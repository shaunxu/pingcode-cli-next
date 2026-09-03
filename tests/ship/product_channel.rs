use crate::common::pc;

use predicates::prelude::*;

#[test]
fn product_channel_help_lists_operations() {
    pc().arg("ship")
        .arg("product-channel")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_product_channel_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-channel")
        .arg("list")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/channels",
        ));
}

#[test]
fn dry_run_product_channel_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("product-channel")
        .arg("get")
        .arg("prod-1")
        .arg("CHN-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/products/prod-1/channels/CHN-1",
        ));
}
