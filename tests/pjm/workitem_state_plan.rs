use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_state_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-state-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-state"))
        .stdout(predicate::str::contains("list-states"))
        .stdout(predicate::str::contains("get-state"))
        .stdout(predicate::str::contains("remove-state"))
        .stdout(predicate::str::contains("add-flow"))
        .stdout(predicate::str::contains("list-flows"))
        .stdout(predicate::str::contains("get-flow"))
        .stdout(predicate::str::contains("remove-flow"));
}

#[test]
fn dry_run_workitem_state_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans",
        ));
}

#[test]
fn dry_run_workitem_state_plan_add_state_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("add-state")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"workitem_state_id":"st-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_plan_get_state_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("get-state")
        .arg("sp-1")
        .arg("st-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states/st-2",
        ));
}

#[test]
fn dry_run_workitem_state_plan_remove_state_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("remove-state")
        .arg("sp-1")
        .arg("st-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states/st-2",
        ));
}

#[test]
fn dry_run_workitem_state_plan_add_flow_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("add-flow")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"from_state_id":"st-1","to_state_id":"st-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows",
        ))
        .stderr(predicate::str::contains("\"to_state_id\": \"st-2\""));
}

#[test]
fn dry_run_workitem_state_plan_list_flows_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("list-flows")
        .arg("sp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows",
        ));
}

#[test]
fn dry_run_workitem_state_plan_get_flow_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("get-flow")
        .arg("sp-1")
        .arg("flow-9")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows/flow-9",
        ));
}

#[test]
fn dry_run_workitem_state_plan_remove_flow_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("remove-flow")
        .arg("sp-1")
        .arg("flow-9")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows/flow-9",
        ));
}
