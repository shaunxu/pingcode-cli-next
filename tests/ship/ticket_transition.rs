use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_transition_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-transition")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_ticket_transition_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-transition")
        .arg("list")
        .arg("tkt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets/tkt-1/transition_histories",
        ));
}

#[test]
fn dry_run_ticket_transition_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-transition")
        .arg("get")
        .arg("tkt-1")
        .arg("th-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets/tkt-1/transition_histories/th-1",
        ));
}
