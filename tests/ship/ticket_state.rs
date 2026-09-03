use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_state_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_ticket_state_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_states?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}
#[test]
fn dry_run_ticket_state_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket/states?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}
#[test]
fn dry_run_ticket_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state")
        .arg("get")
        .arg("TS-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_states/TS-1",
        ));
}
#[test]
fn dry_run_ticket_state_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_states",
        ));
}
#[test]
fn dry_run_ticket_state_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state")
        .arg("update")
        .arg("TS-1")
        .arg("--data")
        .arg(r#"{"name":"new"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_states/TS-1",
        ));
}
