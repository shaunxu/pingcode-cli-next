use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_suite_help_lists_operations() {
    pc().arg("ship")
        .arg("idea-suite")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-for-product"));
}

#[test]
fn dry_run_idea_suite_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-suite")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea/suites?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}
