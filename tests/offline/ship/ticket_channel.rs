use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_channel_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-channel")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-for-product"));
}

#[test]
fn dry_run_ticket_channel_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-channel")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket/channels?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}
