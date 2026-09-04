use crate::common::pc;
use predicates::prelude::*;

#[test]
fn reviews_help_lists_operations() {
    pc().arg("reviews")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("list-principals"))
        .stdout(predicate::str::contains("add-principal"))
        .stdout(predicate::str::contains("get-principal"))
        .stdout(predicate::str::contains("remove-principal"));
}

#[test]
fn dry_run_reviews_list_previews_query() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--pilot-id")
        .arg("proj-1")
        .arg("--status")
        .arg("in_progress")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"))
        .stderr(predicate::str::contains("pilot_id=proj-1"))
        .stderr(predicate::str::contains("status=in_progress"));
}

#[test]
fn dry_run_reviews_create_previews_post_body() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("create")
        .arg("--data")
        .arg(r#"{"title":"first review","pilot_id":"proj-1","principal_type":"workitem","description":"please review"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews",
        ))
        .stderr(predicate::str::contains("first review"))
        .stderr(predicate::str::contains("proj-1"));
}

#[test]
fn dry_run_reviews_get_previews_query() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("get")
        .arg("rev-1")
        .arg("--principal-type")
        .arg("idea")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews/rev-1?",
        ))
        .stderr(predicate::str::contains("principal_type=idea"));
}

#[test]
fn dry_run_reviews_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("delete")
        .arg("rev-1")
        .arg("--principal-type")
        .arg("testcase")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews/rev-1?",
        ))
        .stderr(predicate::str::contains("principal_type=testcase"));
}

#[test]
fn dry_run_reviews_list_principals_previews_query() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("list-principals")
        .arg("rev-1")
        .arg("--principal-type")
        .arg("workitem")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews/rev-1/principals?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"));
}

#[test]
fn dry_run_reviews_add_principal_previews_post_body() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("add-principal")
        .arg("rev-1")
        .arg("--data")
        .arg(r#"{"principal_type":"workitem","principal_id":"wi-42"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews/rev-1/principals",
        ))
        .stderr(predicate::str::contains("wi-42"));
}

#[test]
fn dry_run_reviews_remove_principal_previews_delete() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("remove-principal")
        .arg("rev-1")
        .arg("wi-42")
        .arg("--principal-type")
        .arg("workitem")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/reviews/rev-1/principals/wi-42?",
        ))
        .stderr(predicate::str::contains("principal_type=workitem"));
}

#[test]
fn reviews_list_rejects_unknown_status() {
    pc().arg("--dry-run")
        .arg("reviews")
        .arg("list")
        .arg("--principal-type")
        .arg("workitem")
        .arg("--pilot-id")
        .arg("proj-1")
        .arg("--status")
        .arg("bogus")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
