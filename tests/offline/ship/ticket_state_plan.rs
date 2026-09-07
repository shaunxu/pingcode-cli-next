use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_state_plan_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket-state-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-states"))
        .stdout(predicate::str::contains("add-flow"));
}

#[test]
fn dry_run_state_plan_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans",
        ));
}

#[test]
fn dry_run_state_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("get")
        .arg("sp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1",
        ));
}

#[test]
fn dry_run_state_plan_add_state_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("add-state")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"state_id":"ts-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_states",
        ))
        .stderr(predicate::str::contains("\"state_id\": \"ts-1\""));
}

#[test]
fn dry_run_state_plan_list_states_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("list-states")
        .arg("sp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_states",
        ));
}

#[test]
fn dry_run_state_plan_get_state_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("get-state")
        .arg("sp-1")
        .arg("ts-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_states/ts-1",
        ));
}

#[test]
fn dry_run_state_plan_remove_state_previews_delete() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("remove-state")
        .arg("sp-1")
        .arg("ts-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_states/ts-1",
        ));
}

#[test]
fn dry_run_state_plan_add_flow_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("add-flow")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"from_state_id":"ts-1","to_state_id":"ts-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_state_flows",
        ))
        .stderr(predicate::str::contains("\"to_state_id\": \"ts-2\""));
}

#[test]
fn dry_run_state_plan_list_flows_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("list-flows")
        .arg("sp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_state_flows",
        ));
}

#[test]
fn dry_run_state_plan_get_flow_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("get-flow")
        .arg("sp-1")
        .arg("fl-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_state_flows/fl-1",
        ));
}

#[test]
fn dry_run_state_plan_remove_flow_previews_delete() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket-state-plan")
        .arg("remove-flow")
        .arg("sp-1")
        .arg("fl-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ticket_state_plans/sp-1/ticket_state_flows/fl-1",
        ));
}
