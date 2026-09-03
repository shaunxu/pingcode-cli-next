use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_type_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_ticket_type_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-type")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_types?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}
#[test]
fn dry_run_ticket_type_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-type")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket/types?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}
#[test]
fn dry_run_ticket_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-type")
        .arg("get")
        .arg("TT-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_types/TT-1",
        ));
}
