mod audit_log;
mod login_log;

use crate::common::pc;
use predicates::prelude::*;

#[test]
fn security_help_lists_log_resources() {
    pc().arg("security")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("login-log"))
        .stdout(predicate::str::contains("audit-log"));
}
