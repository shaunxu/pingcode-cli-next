use crate::common::pc;
use predicates::prelude::*;

#[test]
fn library_suite_help_lists_operations() {
    pc().arg("testhub")
        .arg("library-suite")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_library_suite_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-suite")
        .arg("list")
        .arg("lib-1")
        .arg("--parent-id")
        .arg("root")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/suites?",
        ))
        .stderr(predicate::str::contains("parent_id=root"));
}

#[test]
fn dry_run_library_suite_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-suite")
        .arg("get")
        .arg("lib-1")
        .arg("suite-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/suites/suite-1",
        ));
}

#[test]
fn dry_run_library_suite_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-suite")
        .arg("create")
        .arg("lib-1")
        .arg("--data")
        .arg(r#"{"name":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/suites",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}

#[test]
fn dry_run_library_suite_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-suite")
        .arg("update")
        .arg("lib-1")
        .arg("suite-1")
        .arg("--data")
        .arg(r#"{"name":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/suites/suite-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"x\""));
}

#[test]
fn dry_run_library_suite_remove() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("library-suite")
        .arg("remove")
        .arg("lib-1")
        .arg("suite-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/suites/suite-1",
        ));
}
