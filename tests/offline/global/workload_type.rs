use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workload_type_help_lists_operations() {
    pc().arg("workload-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workload_type_list_previews_query() {
    pc().arg("--dry-run")
        .arg("workload-type")
        .arg("list")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workload_types?",
        ))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn dry_run_workload_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("workload-type")
        .arg("get")
        .arg("wt-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/workload_types/wt-123",
        ));
}
