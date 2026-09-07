use crate::common::pc;
use predicates::prelude::*;

#[test]
fn expression_help_lists_operations() {
    pc().arg("expression")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("analyse"))
        .stdout(predicate::str::contains("evaluate"));
}

#[test]
fn dry_run_expression_analyse_previews_request() {
    pc().arg("--dry-run")
        .arg("expression")
        .arg("analyse")
        .arg("--data")
        .arg(r#"{"expressions":["1 + 1","context.workitem.title"]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/expression/analyse",
        ))
        .stderr(predicate::str::contains("\"expressions\""));
}

#[test]
fn dry_run_expression_evaluate_previews_request() {
    pc().arg("--dry-run")
        .arg("expression")
        .arg("evaluate")
        .arg("--data")
        .arg(r#"{"expressions":["1 + 1"]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/expression/evaluate",
        ))
        .stderr(predicate::str::contains("1 + 1"));
}

#[test]
fn dry_run_expression_evaluate_expand_previews_query() {
    pc().arg("--dry-run")
        .arg("expression")
        .arg("evaluate")
        .arg("--expand")
        .arg("--data")
        .arg(r#"{"expressions":["1 + 1"],"context":{"custom":{"foo":"bar"}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/expression/evaluate?",
        ))
        .stderr(predicate::str::contains("expand=true"));
}
