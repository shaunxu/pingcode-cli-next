use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_priority_help_lists_operations() {
    pc().arg("ship")
        .arg("idea-priority")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_idea_priority_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-priority")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_priorities?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_idea_priority_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-priority")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea/priorities?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}

#[test]
fn dry_run_idea_priority_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-priority")
        .arg("get")
        .arg("PRI-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_priorities/PRI-1",
        ));
}
