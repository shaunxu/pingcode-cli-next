use crate::common::pc;
use predicates::prelude::*;

#[test]
fn team_help_lists_operations() {
    pc().arg("organization")
        .arg("team")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_team_get_previews_request() {
    pc().arg("--dry-run")
        .arg("organization")
        .arg("team")
        .arg("get")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/directory/team",
        ));
}
