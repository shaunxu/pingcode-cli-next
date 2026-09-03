mod department;
mod group;
mod group_member;
mod job;
mod role;
mod team;
mod user;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn organization_help_lists_resources() {
    pc().arg("organization")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("team"))
        .stdout(predicate::str::contains("user"))
        .stdout(predicate::str::contains("department"))
        .stdout(predicate::str::contains("group"))
        .stdout(predicate::str::contains("group-member"))
        .stdout(predicate::str::contains("role"))
        .stdout(predicate::str::contains("job"));
}
