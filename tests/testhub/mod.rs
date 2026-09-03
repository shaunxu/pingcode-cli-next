mod library;
mod library_member;
mod library_suite;
mod testcase;
mod testcase_important_level;
mod testcase_property;
mod testcase_property_plan;
mod testcase_state;
mod testcase_type;
mod testplan;
mod testplan_state;
mod testplan_type;
mod testrun;
mod testrun_status;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testhub_help_lists_resources() {
    pc().arg("testhub")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("library"))
        .stdout(predicate::str::contains("library-suite"))
        .stdout(predicate::str::contains("library-member"))
        .stdout(predicate::str::contains("testcase"))
        .stdout(predicate::str::contains("testplan"))
        .stdout(predicate::str::contains("testplan-type"))
        .stdout(predicate::str::contains("testrun"))
        .stdout(predicate::str::contains("testcase-property"))
        .stdout(predicate::str::contains("testcase-property-plan"))
        .stdout(predicate::str::contains("testcase-important-level"))
        .stdout(predicate::str::contains("testcase-type"))
        .stdout(predicate::str::contains("testcase-state"))
        .stdout(predicate::str::contains("testplan-state"))
        .stdout(predicate::str::contains("testrun-status"));
}
