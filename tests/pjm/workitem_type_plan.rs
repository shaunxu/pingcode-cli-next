use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_type_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-type-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-type"))
        .stdout(predicate::str::contains("list-types"))
        .stdout(predicate::str::contains("get-type"))
        .stdout(predicate::str::contains("update-type"))
        .stdout(predicate::str::contains("remove-type"));
}

#[test]
fn dry_run_workitem_type_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans",
        ));
}

#[test]
fn dry_run_workitem_type_plan_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_type_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("get")
        .arg("tp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1",
        ));
}

#[test]
fn dry_run_workitem_type_plan_add_type_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("add-type")
        .arg("tp-1")
        .arg("--data")
        .arg(r#"{"workitem_type_id":"wt-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_plan_list_types_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list-types")
        .arg("tp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_plan_get_type_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("get-type")
        .arg("tp-1")
        .arg("wt-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}

#[test]
fn dry_run_workitem_type_plan_update_type_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("update-type")
        .arg("tp-1")
        .arg("wt-2")
        .arg("--data")
        .arg(r#"{"default":true}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}

#[test]
fn dry_run_workitem_type_plan_remove_type_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("remove-type")
        .arg("tp-1")
        .arg("wt-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}
