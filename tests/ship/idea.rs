use crate::common::pc;
use predicates::prelude::*;

#[test]
fn idea_help_lists_operations() {
    pc().arg("ship")
        .arg("idea")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("search"));
}

#[test]
fn dry_run_idea_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea")
        .arg("list")
        .arg("--product-id")
        .arg("prod-1")
        .arg("--state-id")
        .arg("st-1")
        .arg("--keywords")
        .arg("demo")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"))
        .stderr(predicate::str::contains("state_id=st-1"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_idea_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea")
        .arg("get")
        .arg("idea-1")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas/idea-1",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_idea_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea")
        .arg("create")
        .arg("--data")
        .arg(r#"{"product_id":"prod-1","title":"Demo"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas",
        ))
        .stderr(predicate::str::contains("\"title\": \"Demo\""));
}

#[test]
fn dry_run_idea_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea")
        .arg("update")
        .arg("idea-1")
        .arg("--data")
        .arg(r#"{"title":"New title"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas/idea-1",
        ))
        .stderr(predicate::str::contains("\"title\": \"New title\""));
}

#[test]
fn dry_run_idea_search_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("idea")
        .arg("search")
        .arg("--data")
        .arg(r#"{"mode":"query","payload":{"filter":{"title":"demo"}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/ideas/search",
        ))
        .stderr(predicate::str::contains("\"mode\": \"query\""));
}
