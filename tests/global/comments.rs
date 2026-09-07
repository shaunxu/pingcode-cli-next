use crate::common::pc;
use predicates::prelude::*;

#[test]
fn comments_help_lists_operations() {
    pc().arg("comments")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_comments_get_previews_query() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("get")
        .arg("cmt-1")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--principal-id")
        .arg("wi-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/comments/cmt-1?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("principal_id=wi-123"));
}

#[test]
fn dry_run_comments_create_previews_post_body() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("create")
        .arg("--data")
        .arg(r#"{"principal_type":"workitem","principal_id":"wi-1","content":"hello"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/comments",
        ))
        .stderr(predicate::str::contains("hello"));
}

#[test]
fn dry_run_comments_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("comments")
        .arg("delete")
        .arg("cmt-2")
        .arg("--principal-type")
        .arg("ticket")
        .arg("--principal-id")
        .arg("t-3")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/comments/cmt-2?",
        ))
        .stderr(predicate::str::contains("principal_type=ticket"));
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
