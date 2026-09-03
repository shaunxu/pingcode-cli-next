use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_state_help_lists_operations() {
    pc().arg("ship")
        .arg("idea-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_idea_state_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-state")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_states?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_idea_state_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-state")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea/states?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}

#[test]
fn dry_run_idea_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-state")
        .arg("get")
        .arg("ST-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_states/ST-1",
        ));
}
