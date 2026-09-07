use crate::common::pc;
use predicates::prelude::*;

#[test]
fn entity_properties_help_lists_operations() {
    pc().arg("entity-properties")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_entity_properties_list_previews_query() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("list")
        .arg("--entity-type")
        .arg("workitem")
        .arg("--entity-id")
        .arg("wi-123")
        .arg("--page-index")
        .arg("0")
        .arg("--page-size")
        .arg("50")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/entity_properties?",
        ))
        .stderr(predicate::str::contains("entity_type=workitem"))
        .stderr(predicate::str::contains("entity_id=wi-123"))
        .stderr(predicate::str::contains("page_index=0"))
        .stderr(predicate::str::contains("page_size=50"));
}

#[test]
fn dry_run_entity_properties_get_previews_query() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("get")
        .arg("prop-1")
        .arg("--entity-type")
        .arg("idea")
        .arg("--entity-id")
        .arg("idea-7")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/entity_properties/prop-1?",
        ))
        .stderr(predicate::str::contains("entity_type=idea"))
        .stderr(predicate::str::contains("entity_id=idea-7"));
}

#[test]
fn dry_run_entity_properties_create_previews_post_body() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("create")
        .arg("--data")
        .arg(
            r#"{"entity_type":"workitem","entity_id":"wi-1","property_key":"app:demo:state","value":{"synced":true},"overwrite":1}"#,
        )
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/entity_properties",
        ))
        .stderr(predicate::str::contains("app:demo:state"));
}

#[test]
fn dry_run_entity_properties_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("delete")
        .arg("prop-2")
        .arg("--entity-type")
        .arg("testcase")
        .arg("--entity-id")
        .arg("tc-3")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/entity_properties/prop-2?",
        ))
        .stderr(predicate::str::contains("entity_type=testcase"));
}

#[test]
fn entity_properties_list_requires_entity_type_and_id() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--entity-type"))
        .stderr(predicate::str::contains("--entity-id"));
}

#[test]
fn entity_properties_list_rejects_unknown_entity_type() {
    pc().arg("--dry-run")
        .arg("entity-properties")
        .arg("list")
        .arg("--entity-type")
        .arg("bogus")
        .arg("--entity-id")
        .arg("x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}
