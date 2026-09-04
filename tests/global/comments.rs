use crate::common::pc;
use predicates::prelude::*;

#[test]
fn comments_help_lists_operations() {
    pc().arg("comments")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"));
}

#[test]
fn dry_run_comments_list_previews_query() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--principal-id")
        .arg("wi-123")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/comments?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("principal_id=wi-123"))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn comments_list_requires_principal_type_and_id() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--principal-type"))
        .stderr(predicate::str::contains("--principal-id"));
}

#[test]
fn comments_list_rejects_unknown_principal_type() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("list")
        .arg("--principal-type")
        .arg("bogus")
        .arg("--principal-id")
        .arg("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
