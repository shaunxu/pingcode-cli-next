use crate::common::pc;
use predicates::prelude::*;

#[test]
fn testcase_help_lists_operations() {
    pc().arg("testhub")
        .arg("testcase")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("bulk-create"))
        .stdout(predicate::str::contains("bulk-update"))
        .stdout(predicate::str::contains("histories"));
}

#[test]
fn dry_run_testcase_list() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("list")
        .arg("--library-id")
        .arg("lib-1")
        .arg("--maintenance-id")
        .arg("m-1")
        .arg("--state-id")
        .arg("st-1")
        .arg("--important-level-id")
        .arg("il-1")
        .arg("--tag-id")
        .arg("tag-1")
        .arg("--keywords")
        .arg("demo")
        .arg("--include-public-image-token")
        .arg("description")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases?",
        ))
        .stderr(predicate::str::contains("library_id=lib-1"))
        .stderr(predicate::str::contains("maintenance_id=m-1"))
        .stderr(predicate::str::contains("state_id=st-1"))
        .stderr(predicate::str::contains("important_level_id=il-1"))
        .stderr(predicate::str::contains("tag_id=tag-1"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains(
            "include_public_image_token=description",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_testcase_get() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("get")
        .arg("tc-1")
        .arg("--include-public-image-token")
        .arg("description")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/tc-1?",
        ))
        .stderr(predicate::str::contains(
            "include_public_image_token=description",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_testcase_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("create")
        .arg("--data")
        .arg(r#"{"library_id":"lib-1","title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases",
        ))
        .stderr(predicate::str::contains("\"title\": \"x\""));
}

#[test]
fn dry_run_testcase_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("update")
        .arg("tc-1")
        .arg("--data")
        .arg(r#"{"title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/tc-1",
        ))
        .stderr(predicate::str::contains("\"title\": \"x\""));
}

#[test]
fn dry_run_testcase_delete() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("delete")
        .arg("tc-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/tc-1",
        ));
}

#[test]
fn dry_run_testcase_search() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("search")
        .arg("--data")
        .arg(r#"{"mode":"query","payload":{"filter":{}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/search",
        ))
        .stderr(predicate::str::contains("\"mode\": \"query\""));
}

#[test]
fn dry_run_testcase_bulk_create() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("bulk-create")
        .arg("--data")
        .arg(r#"{"testcases":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/bulk",
        ))
        .stderr(predicate::str::contains("\"testcases\": []"));
}

#[test]
fn dry_run_testcase_bulk_update() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("bulk-update")
        .arg("--data")
        .arg(r#"{"testcases":[]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/bulk",
        ))
        .stderr(predicate::str::contains("\"testcases\": []"));
}

#[test]
fn dry_run_testcase_histories() {
    pc().arg("--dry-run")
        .arg("testhub")
        .arg("testcase")
        .arg("histories")
        .arg("tc-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/testhub/testcases/tc-1/histories",
        ));
}
