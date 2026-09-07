use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_transition_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-transition")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_transition_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-transition")
        .arg("list")
        .arg("wi-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/transition_histories",
        ));
}

#[test]
fn dry_run_workitem_transition_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-transition")
        .arg("get")
        .arg("wi-1")
        .arg("th-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/transition_histories/th-1",
        ));
}
