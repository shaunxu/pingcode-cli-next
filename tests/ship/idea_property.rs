use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_property_help_lists_operations() {
    pc().arg("ship")
        .arg("idea-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"));
}

#[test]
fn dry_run_idea_property_list_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_properties",
        ));
}

#[test]
fn dry_run_idea_property_list_for_product_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-property")
        .arg("list-for-product")
        .arg("prod-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea/properties?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"));
}

#[test]
fn dry_run_idea_property_get_previews_path() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-property")
        .arg("get")
        .arg("prp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_properties/prp-1",
        ));
}

#[test]
fn dry_run_idea_property_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"demo","type":"text"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_properties",
        ));
}

#[test]
fn dry_run_idea_property_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea-property")
        .arg("update")
        .arg("prp-1")
        .arg("--data")
        .arg(r#"{"name":"new"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/idea_properties/prp-1",
        ));
}
