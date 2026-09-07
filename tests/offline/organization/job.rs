use crate::common::pc;
use predicates::prelude::*;

#[test]
fn job_help_lists_operations() {
    pc().arg("organization")
        .arg("job")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_job_list_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("job")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/jobs?",
        ))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_job_get_previews_path() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("job")
        .arg("get")
        .arg("job-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/jobs/job-123",
        ));
}
