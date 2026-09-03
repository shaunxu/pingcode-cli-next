use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_solution_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-solution")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_ticket_solution_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-solution")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_solutions?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}
#[test]
fn dry_run_ticket_solution_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-solution")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket/solutions?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}
#[test]
fn dry_run_ticket_solution_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-solution")
        .arg("get")
        .arg("SOL-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_solutions/SOL-1",
        ));
}
