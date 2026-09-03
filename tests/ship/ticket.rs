use crate::common::pc;
use predicates::prelude::*;

#[test]
fn ticket_help_lists_operations() {
    pc().arg("ship")
        .arg("ticket")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("search"));
}

#[test]
fn dry_run_ticket_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket")
        .arg("list")
        .arg("--product-id")
        .arg("prod-1")
        .arg("--type-id")
        .arg("tt-1")
        .arg("--keywords")
        .arg("demo")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets?",
        ))
        .stderr(predicate::str::contains("product_id=prod-1"))
        .stderr(predicate::str::contains("type_id=tt-1"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn dry_run_ticket_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket")
        .arg("get")
        .arg("tkt-1")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets/tkt-1",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_ticket_create_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket")
        .arg("create")
        .arg("--data")
        .arg(r#"{"product_id":"prod-1","title":"Demo","type_id":"tt-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets",
        ))
        .stderr(predicate::str::contains("\"title\": \"Demo\""));
}

#[test]
fn dry_run_ticket_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket")
        .arg("update")
        .arg("tkt-1")
        .arg("--data")
        .arg(r#"{"title":"New title"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets/tkt-1",
        ))
        .stderr(predicate::str::contains("\"title\": \"New title\""));
}

#[test]
fn dry_run_ticket_search_previews_request() {
    pc().arg("--dry-run")
        .arg("ship")
        .arg("ticket")
        .arg("search")
        .arg("--data")
        .arg(r#"{"mode":"query","payload":{"filter":{"title":"demo"}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/ship/tickets/search",
        ))
        .stderr(predicate::str::contains("\"mode\": \"query\""));
}
