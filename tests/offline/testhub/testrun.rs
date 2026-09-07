use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testrun_help_lists_operations() {
    pc().arg("testhub")
        .arg("testrun")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("replace"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("bulk-create"))
        .stdout(predicate::str::contains("bulk-update"))
        .stdout(predicate::str::contains("histories"))
        .stdout(predicate::str::contains("get-history"))
        .stdout(predicate::str::contains("plan-batch"));
}

#[test]
fn dry_run_testrun_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("list")
        .arg("--testplan-id")
        .arg("tp-1")
        .arg("--testcase-id")
        .arg("tc-1")
        .arg("--suite-id")
        .arg("suite-1")
        .arg("--status-id")
        .arg("trs-1")
        .arg("--keywords")
        .arg("demo")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns?",
        ))
        .stderr(predicate::str::contains("testplan_id=tp-1"))
        .stderr(predicate::str::contains("testcase_id=tc-1"))
        .stderr(predicate::str::contains("suite_id=suite-1"))
        .stderr(predicate::str::contains("status_id=trs-1"))
        .stderr(predicate::str::contains("keywords=demo"));
}

#[test]
fn dry_run_testrun_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("get")
        .arg("tr-1")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/tr-1?",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_testrun_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("create")
        .arg("--data")
        .arg(r#"{"library_id":"lib-1","testplan_id":"tp-1","testcase_id":"tc-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns",
        ))
        .stderr(predicate::str::contains("\"testplan_id\": \"tp-1\""));
}

#[test]
fn dry_run_testrun_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("update")
        .arg("tr-1")
        .arg("--data")
        .arg(r#"{"status_id":"trs-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/tr-1",
        ))
        .stderr(predicate::str::contains("\"status_id\": \"trs-1\""));
}

#[test]
fn dry_run_testrun_replace() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("replace")
        .arg("tr-1")
        .arg("--data")
        .arg(r#"{"status_id":"trs-1","steps":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PUT"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/tr-1",
        ))
        .stderr(predicate::str::contains("\"status_id\": \"trs-1\""));
}

#[test]
fn dry_run_testrun_search() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("search")
        .arg("--data")
        .arg(r#"{"mode":"query","payload":{"filter":{}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/search",
        ))
        .stderr(predicate::str::contains("\"mode\": \"query\""));
}

#[test]
fn dry_run_testrun_bulk_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("bulk-create")
        .arg("--data")
        .arg(r#"{"testruns":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/bulk",
        ))
        .stderr(predicate::str::contains("\"testruns\": []"));
}

#[test]
fn dry_run_testrun_bulk_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("bulk-update")
        .arg("--data")
        .arg(r#"{"testruns":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/bulk",
        ))
        .stderr(predicate::str::contains("\"testruns\": []"));
}

#[test]
fn dry_run_testrun_histories() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("histories")
        .arg("tr-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/tr-1/histories",
        ));
}

#[test]
fn dry_run_testrun_get_history() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("get-history")
        .arg("tr-1")
        .arg("hist-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testruns/tr-1/histories/hist-1",
        ));
}

#[test]
fn dry_run_testrun_plan_batch() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testrun")
        .arg("plan-batch")
        .arg("lib-1")
        .arg("tp-1")
        .arg("--data")
        .arg(r#"{"inserts":[],"updates":[],"deletes":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/libraries/lib-1/testplans/tp-1/testruns/bulk",
        ))
        .stderr(predicate::str::contains("\"inserts\": []"));
}
