use crate::common::pc;
use predicates::prelude::*;

#[test]
fn dry_run_state_works_without_credentials() {
    pc().arg("state")
        .arg("--dry-run")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains("/v1/directory/team"))
        .stderr(predicate::str::contains("/v1/myself"));
}
