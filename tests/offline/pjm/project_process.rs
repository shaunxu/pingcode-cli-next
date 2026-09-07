use crate::common::pc;
use predicates::prelude::*;

#[test]
fn project_process_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-process")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_project_process_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-process")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/processes",
        ));
}

#[test]
fn dry_run_project_process_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-process")
        .arg("get")
        .arg("proc-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/processes/proc-1",
        ));
}
