use crate::common::pc;
use predicates::prelude::*;

#[test]
fn workitem_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("search"))
        .stdout(predicate::str::contains("batch-update"))
        .stdout(predicate::str::contains("add-tag"))
        .stdout(predicate::str::contains("get-tag"))
        .stdout(predicate::str::contains("remove-tag"));
}

#[test]
fn create_help_mentions_data_and_endpoint() {
    pc().arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--data"))
        .stdout(predicate::str::contains("/v1/pjm/workitems"));
}

#[test]
fn dry_run_create_works_without_credentials() {
    // dry-run 不发网络请求，因此无凭据也应成功，并在 stderr 预览请求。
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg(r#"{"project_id":"p1","type_id":"t1","title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems",
        ))
        .stderr(predicate::str::contains("\"title\": \"x\""));
}

#[test]
fn dry_run_flag_works_after_subcommand() {
    // 全局参数放在子命令之后同样生效。
    pc().arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--dry-run")
        .arg("--data")
        .arg(r#"{"project_id":"p1","type_id":"t1","title":"x"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"));
}

#[test]
fn create_rejects_invalid_json_data() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg("not-json")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid JSON in --data"));
}

#[test]
fn create_rejects_non_object_data() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("create")
        .arg("--data")
        .arg("[1,2,3]")
        .assert()
        .failure()
        .stderr(predicate::str::contains("expected a JSON object"));
}

#[test]
fn dry_run_workitem_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--keywords")
        .arg("demo")
        .arg("--include-archived")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("include_archived=true"));
}

#[test]
fn dry_run_workitem_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("get")
        .arg("wi-123")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_workitem_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("update")
        .arg("wi-123")
        .arg("--data")
        .arg(r#"{"title":"New title"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123",
        ))
        .stderr(predicate::str::contains("\"title\": \"New title\""));
}

#[test]
fn dry_run_workitem_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("delete")
        .arg("wi-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123",
        ));
}

#[test]
fn dry_run_workitem_search_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("search")
        .arg("--data")
        .arg(r#"{"mode":"query","payload":{"filter":{"project_id":"p1"}}}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/search",
        ))
        .stderr(predicate::str::contains("\"mode\": \"query\""));
}

#[test]
fn dry_run_workitem_batch_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("batch-update")
        .arg("--data")
        .arg(r#"{"ids":["wi-1","wi-2"],"property_name":"priority_id","property_value":"pr-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems",
        ))
        .stderr(predicate::str::contains(
            "\"property_name\": \"priority_id\"",
        ));
}

#[test]
fn dry_run_workitem_add_tag_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("add-tag")
        .arg("wi-123")
        .arg("--data")
        .arg(r#"{"tag_id":"tag-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123/tags",
        ))
        .stderr(predicate::str::contains("\"tag_id\": \"tag-1\""));
}

#[test]
fn dry_run_workitem_get_tag_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("get-tag")
        .arg("wi-123")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123/tags/tag-1",
        ));
}

#[test]
fn dry_run_workitem_remove_tag_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem")
        .arg("remove-tag")
        .arg("wi-123")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-123/tags/tag-1",
        ));
}
