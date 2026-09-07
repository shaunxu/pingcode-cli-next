use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_transition_help_lists_operations() {
    pc().arg("ship")
        .arg("idea-transition")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_idea_transition_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-transition")
        .arg("list")
        .arg("idea-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas/idea-1/transition_histories",
        ));
}

#[test]
fn dry_run_idea_transition_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-transition")
        .arg("get")
        .arg("idea-1")
        .arg("th-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas/idea-1/transition_histories/th-1",
        ));
}
