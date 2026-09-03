use assert_cmd::Command;
use predicates::prelude::*;

fn pc() -> Command {
    let mut cmd = Command::cargo_bin("pc").unwrap();
    // 隔离宿主环境，避免开发者本机的认证信息影响断言
    cmd.env_remove("PC_TOKEN")
        .env_remove("PC_CLIENT_ID")
        .env_remove("PC_CLIENT_SECRET")
        .env_remove("PC_OPEN_API_BASE_URL");
    // 切换到不含 .env 的目录，避免仓库根目录的 .env 被自动加载污染断言
    cmd.current_dir(std::env::temp_dir());
    cmd
}

#[test]
fn help_succeeds() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("PingCode"));
}

#[test]
fn help_lists_state_command() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("state"));
}

#[test]
fn version_succeeds() {
    pc().arg("--version").assert().success();
}

#[test]
fn missing_credentials_fails_with_hint() {
    pc().arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_ID"))
        .stderr(predicate::str::contains("PC_TOKEN"));
}

#[test]
fn client_id_without_secret_fails() {
    pc().arg("--client-id")
        .arg("cid")
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_SECRET"));
}

#[test]
fn client_secret_without_id_fails() {
    pc().arg("--client-secret")
        .arg("secret")
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("PC_CLIENT_ID"));
}

#[test]
fn invalid_base_url_fails() {
    pc().arg("--base-url")
        .arg("not-a-url")
        .arg("--token")
        .arg("dummy")
        .arg("state")
        .assert()
        .failure()
        .stderr(predicate::str::contains("http://"));
}

#[test]
fn pjm_help_lists_workitem_resource() {
    pc().arg("pjm")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("workitem"));
}

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
fn dry_run_state_works_without_credentials() {
    pc().arg("state")
        .arg("--dry-run")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains("/v1/directory/team"))
        .stderr(predicate::str::contains("/v1/myself"));
}

#[test]
fn project_help_lists_operations() {
    pc().arg("pjm")
        .arg("project")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("enable-local-config"));
}

#[test]
fn dry_run_project_list_previews_request_without_credentials() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("list")
        .arg("--type")
        .arg("scrum")
        .arg("--keywords")
        .arg("demo")
        .arg("--include-archived")
        .arg("--page-index")
        .arg("0")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects?",
        ))
        .stderr(predicate::str::contains("type=scrum"))
        .stderr(predicate::str::contains("keywords=demo"))
        .stderr(predicate::str::contains("include_archived=true"))
        .stderr(predicate::str::contains("page_index=0"));
}

#[test]
fn project_list_requires_member_id_with_member_type() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("list")
        .arg("--member-type")
        .arg("user")
        .assert()
        .failure()
        .stderr(predicate::str::contains("--member-id"));
}

#[test]
fn dry_run_project_get_previews_path_and_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("get")
        .arg("prj-123")
        .arg("--include-deleted")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123",
        ))
        .stderr(predicate::str::contains("include_deleted=true"));
}

#[test]
fn dry_run_project_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Demo","type":"scrum","identifier":"DEMO"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects",
        ))
        .stderr(predicate::str::contains("\"identifier\": \"DEMO\""));
}

#[test]
fn dry_run_project_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("update")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"New name"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123",
        ))
        .stderr(predicate::str::contains("\"name\": \"New name\""));
}

#[test]
fn dry_run_project_clone_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("clone")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"identifier":"COPY"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/clone",
        ))
        .stderr(predicate::str::contains("\"identifier\": \"COPY\""));
}

#[test]
fn dry_run_project_enable_local_config_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("enable-local-config")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/local_config/enable",
        ));
}

#[test]
fn dry_run_project_progress_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project")
        .arg("progress")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/progress",
        ));
}

#[test]
fn project_member_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-member")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("remove"));
}

#[test]
fn dry_run_project_member_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members",
        ));
}

#[test]
fn dry_run_project_member_add_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("add")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"member":{"type":"user","id":"u1"},"role_id":"r1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r1\""));
}

#[test]
fn dry_run_project_member_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("update")
        .arg("prj-123")
        .arg("u1")
        .arg("--data")
        .arg(r#"{"role_id":"r2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members/u1",
        ))
        .stderr(predicate::str::contains("\"role_id\": \"r2\""));
}

#[test]
fn dry_run_project_member_remove_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-member")
        .arg("remove")
        .arg("prj-123")
        .arg("u1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/members/u1",
        ));
}

#[test]
fn project_property_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("add-to-project"))
        .stdout(predicate::str::contains("remove-from-project"));
}

#[test]
fn dry_run_project_property_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties",
        ));
}

#[test]
fn dry_run_project_property_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Priority","type":"select"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties",
        ))
        .stderr(predicate::str::contains("\"name\": \"Priority\""));
}

#[test]
fn dry_run_project_property_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("update")
        .arg("prop-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_properties/prop-1",
        ));
}

#[test]
fn dry_run_project_property_list_for_project_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("list-for-project")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties",
        ));
}

#[test]
fn dry_run_project_property_add_to_project_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("add-to-project")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"property_id":"prop-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties",
        ))
        .stderr(predicate::str::contains("\"property_id\": \"prop-1\""));
}

#[test]
fn dry_run_project_property_remove_from_project_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-property")
        .arg("remove-from-project")
        .arg("prj-123")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/project_properties/prop-1",
        ));
}

#[test]
fn project_state_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_project_state_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-state")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project/states?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_project_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/project_states/st-1",
        ));
}

#[test]
fn board_help_lists_operations() {
    pc().arg("pjm")
        .arg("board")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_board_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards",
        ));
}

#[test]
fn dry_run_board_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ));
}

#[test]
fn dry_run_board_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"Sprint board"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards",
        ))
        .stderr(predicate::str::contains("\"name\": \"Sprint board\""));
}

#[test]
fn dry_run_board_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed board"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"Renamed board\""));
}

#[test]
fn dry_run_board_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1",
        ));
}

#[test]
fn board_entry_help_lists_operations() {
    pc().arg("pjm")
        .arg("board-entry")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_board_entry_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("list")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries",
        ));
}

#[test]
fn dry_run_board_entry_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ));
}

#[test]
fn dry_run_board_entry_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("create")
        .arg("prj-123")
        .arg("brd-1")
        .arg("--data")
        .arg(r#"{"name":"To do","wip_limit":5}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries",
        ))
        .stderr(predicate::str::contains("\"wip_limit\": 5"));
}

#[test]
fn dry_run_board_entry_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .arg("--data")
        .arg(r#"{"is_split":true}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ))
        .stderr(predicate::str::contains("\"is_split\": true"));
}

#[test]
fn dry_run_board_entry_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-entry")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .arg("ent-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/entries/ent-1",
        ));
}

#[test]
fn board_swimlane_help_lists_operations() {
    pc().arg("pjm")
        .arg("board-swimlane")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_board_swimlane_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("list")
        .arg("prj-123")
        .arg("brd-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes",
        ));
}

#[test]
fn dry_run_board_swimlane_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("get")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ));
}

#[test]
fn dry_run_board_swimlane_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("update")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .arg("--data")
        .arg(r#"{"name":"Team B"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ))
        .stderr(predicate::str::contains("\"name\": \"Team B\""));
}

#[test]
fn dry_run_board_swimlane_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("board-swimlane")
        .arg("delete")
        .arg("prj-123")
        .arg("brd-1")
        .arg("sw-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/boards/brd-1/swimlanes/sw-1",
        ));
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

#[test]
fn workitem_tag_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-tag")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_tag_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Blocked"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags",
        ))
        .stderr(predicate::str::contains("\"name\": \"Blocked\""));
}

#[test]
fn dry_run_workitem_tag_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("list")
        .arg("--name")
        .arg("block")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags?",
        ))
        .stderr(predicate::str::contains("name=block"));
}

#[test]
fn dry_run_workitem_tag_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/tags?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_tag_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("get")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}

#[test]
fn dry_run_workitem_tag_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("update")
        .arg("tag-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}

#[test]
fn dry_run_workitem_tag_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-tag")
        .arg("delete")
        .arg("tag-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_tags/tag-1",
        ));
}

#[test]
fn workitem_relation_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-relation")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_relation_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("create")
        .arg("wi-1")
        .arg("--data")
        .arg(r#"{"target_workitem_id":"wi-2","relation_type":"relate"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations",
        ))
        .stderr(predicate::str::contains("\"relation_type\": \"relate\""));
}

#[test]
fn dry_run_workitem_relation_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("list")
        .arg("wi-1")
        .arg("--relation-type")
        .arg("block")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations?",
        ))
        .stderr(predicate::str::contains("relation_type=block"));
}

#[test]
fn dry_run_workitem_relation_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("get")
        .arg("wi-1")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations/rel-1",
        ));
}

#[test]
fn dry_run_workitem_relation_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation")
        .arg("delete")
        .arg("wi-1")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/relations/rel-1",
        ));
}

#[test]
fn workitem_relation_type_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-relation-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_relation_type_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation-type")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_relation_types",
        ));
}

#[test]
fn dry_run_workitem_relation_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-relation-type")
        .arg("get")
        .arg("rt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_relation_types/rt-1",
        ));
}

#[test]
fn workitem_transition_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-transition")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_transition_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-transition")
        .arg("list")
        .arg("wi-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/transition_histories",
        ));
}

#[test]
fn dry_run_workitem_transition_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-transition")
        .arg("get")
        .arg("wi-1")
        .arg("th-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitems/wi-1/transition_histories/th-1",
        ));
}

#[test]
fn deliverable_help_lists_operations() {
    pc().arg("pjm")
        .arg("deliverable")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_deliverable_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("create")
        .arg("--data")
        .arg(r#"{"workitem_id":"wi-1","name":"Spec doc"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables",
        ))
        .stderr(predicate::str::contains("\"name\": \"Spec doc\""));
}

#[test]
fn dry_run_deliverable_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-id")
        .arg("wi-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"))
        .stderr(predicate::str::contains("workitem_id=wi-1"));
}

#[test]
fn dry_run_deliverable_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("get")
        .arg("d-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}

#[test]
fn dry_run_deliverable_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("update")
        .arg("d-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}

#[test]
fn dry_run_deliverable_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("deliverable")
        .arg("delete")
        .arg("d-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/deliverables/d-1",
        ));
}

#[test]
fn workitem_type_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-type")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_workitem_type_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom type"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("update")
        .arg("wt-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}

#[test]
fn dry_run_workitem_type_delete_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("delete")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}

#[test]
fn dry_run_workitem_type_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/types?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_type_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type")
        .arg("get")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_types/wt-1",
        ));
}

#[test]
fn workitem_state_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-state")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list-all"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_workitem_state_list_all_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("list-all")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom state"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("update")
        .arg("st-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states/st-1",
        ));
}

#[test]
fn dry_run_workitem_state_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-type-id")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/states?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"))
        .stderr(predicate::str::contains("workitem_type_id=wt-1"));
}

#[test]
fn dry_run_workitem_state_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state")
        .arg("get")
        .arg("st-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_states/st-1",
        ));
}

#[test]
fn workitem_property_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-property")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"));
}

#[test]
fn dry_run_workitem_property_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Custom field"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("update")
        .arg("prop-1")
        .arg("--data")
        .arg(r#"{"name":"Renamed field"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties/prop-1",
        ));
}

#[test]
fn dry_run_workitem_property_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .arg("--workitem-type-id")
        .arg("wt-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/properties?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_property_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property")
        .arg("get")
        .arg("prop-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_properties/prop-1",
        ));
}

#[test]
fn workitem_priority_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-priority")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("list-for-project"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_workitem_priority_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_priorities",
        ));
}

#[test]
fn dry_run_workitem_priority_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("list-for-project")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem/priorities?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_priority_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-priority")
        .arg("get")
        .arg("pri-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_priorities/pri-1",
        ));
}

#[test]
fn release_help_lists_operations() {
    pc().arg("pjm")
        .arg("release")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"))
        .stdout(predicate::str::contains("bulk-create"));
}

#[test]
fn dry_run_release_list_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("list")
        .arg("prj-123")
        .arg("--name")
        .arg("v1.0")
        .arg("--status")
        .arg("in_progress")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases?",
        ))
        .stderr(predicate::str::contains("name=v1.0"))
        .stderr(predicate::str::contains("status=in_progress"));
}

#[test]
fn dry_run_release_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("get")
        .arg("prj-123")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"v1.0","assignee_id":"u-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases",
        ))
        .stderr(predicate::str::contains("\"name\": \"v1.0\""));
}

#[test]
fn dry_run_release_update_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("update")
        .arg("prj-123")
        .arg("rel-1")
        .arg("--data")
        .arg(r#"{"name":"v1.0.1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("delete")
        .arg("prj-123")
        .arg("rel-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/releases/rel-1",
        ));
}

#[test]
fn dry_run_release_bulk_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release")
        .arg("bulk-create")
        .arg("--data")
        .arg(r#"{"releases":[{"project_id":"prj-1","name":"v1.0"}]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/releases/bulk",
        ))
        .stderr(predicate::str::contains("\"releases\""));
}

#[test]
fn release_stage_help_lists_operations() {
    pc().arg("pjm")
        .arg("release-stage")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_release_stage_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages",
        ));
}

#[test]
fn dry_run_release_stage_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("get")
        .arg("stage-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages/stage-1",
        ));
}

#[test]
fn dry_run_release_stage_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("create")
        .arg("--data")
        .arg(r#"{"name":"Dev","type":"in_progress"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages",
        ))
        .stderr(predicate::str::contains("\"name\": \"Dev\""));
}

#[test]
fn dry_run_release_stage_delete_previews_replace_id() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-stage")
        .arg("delete")
        .arg("stage-1")
        .arg("--data")
        .arg(r#"{"replace_id":"stage-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/release_stages/stage-1",
        ))
        .stderr(predicate::str::contains("\"replace_id\": \"stage-2\""));
}

#[test]
fn release_section_help_lists_operations() {
    pc().arg("pjm")
        .arg("release-section")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_release_section_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections",
        ));
}

#[test]
fn dry_run_release_section_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"2026 H1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections",
        ))
        .stderr(predicate::str::contains("\"name\": \"2026 H1\""));
}

#[test]
fn dry_run_release_section_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-section")
        .arg("delete")
        .arg("prj-123")
        .arg("sec-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_sections/sec-1",
        ));
}

#[test]
fn release_category_help_lists_operations() {
    pc().arg("pjm")
        .arg("release-category")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("create"))
        .stdout(predicate::str::contains("update"))
        .stdout(predicate::str::contains("delete"));
}

#[test]
fn dry_run_release_category_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-category")
        .arg("list")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_categories",
        ));
}

#[test]
fn dry_run_release_category_create_previews_request() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-category")
        .arg("create")
        .arg("prj-123")
        .arg("--data")
        .arg(r#"{"name":"Mobile","section_id":"sec-1"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_categories",
        ))
        .stderr(predicate::str::contains("\"name\": \"Mobile\""));
}

#[test]
fn dry_run_release_category_delete_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("release-category")
        .arg("delete")
        .arg("prj-123")
        .arg("cat-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/projects/prj-123/release_categories/cat-1",
        ));
}

#[test]
fn project_process_help_lists_operations() {
    pc().arg("pjm")
        .arg("project-process")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"));
}

#[test]
fn dry_run_project_process_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-process")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/processes",
        ));
}

#[test]
fn dry_run_project_process_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("project-process")
        .arg("get")
        .arg("proc-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/processes/proc-1",
        ));
}

#[test]
fn workitem_type_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-type-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-type"))
        .stdout(predicate::str::contains("list-types"))
        .stdout(predicate::str::contains("get-type"))
        .stdout(predicate::str::contains("update-type"))
        .stdout(predicate::str::contains("remove-type"));
}

#[test]
fn dry_run_workitem_type_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans",
        ));
}

#[test]
fn dry_run_workitem_type_plan_list_for_project_previews_query() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list")
        .arg("--project-id")
        .arg("prj-123")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans?",
        ))
        .stderr(predicate::str::contains("project_id=prj-123"));
}

#[test]
fn dry_run_workitem_type_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("get")
        .arg("tp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1",
        ));
}

#[test]
fn dry_run_workitem_type_plan_add_type_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("add-type")
        .arg("tp-1")
        .arg("--data")
        .arg(r#"{"workitem_type_id":"wt-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_plan_list_types_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("list-types")
        .arg("tp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types",
        ));
}

#[test]
fn dry_run_workitem_type_plan_get_type_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("get-type")
        .arg("tp-1")
        .arg("wt-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}

#[test]
fn dry_run_workitem_type_plan_update_type_previews_patch() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("update-type")
        .arg("tp-1")
        .arg("wt-2")
        .arg("--data")
        .arg(r#"{"default":true}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] PATCH"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}

#[test]
fn dry_run_workitem_type_plan_remove_type_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-type-plan")
        .arg("remove-type")
        .arg("tp-1")
        .arg("wt-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_type_plans/tp-1/workitem_types/wt-2",
        ));
}

#[test]
fn workitem_state_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-state-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-state"))
        .stdout(predicate::str::contains("list-states"))
        .stdout(predicate::str::contains("get-state"))
        .stdout(predicate::str::contains("remove-state"))
        .stdout(predicate::str::contains("add-flow"))
        .stdout(predicate::str::contains("list-flows"))
        .stdout(predicate::str::contains("get-flow"))
        .stdout(predicate::str::contains("remove-flow"));
}

#[test]
fn dry_run_workitem_state_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans",
        ));
}

#[test]
fn dry_run_workitem_state_plan_add_state_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("add-state")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"workitem_state_id":"st-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states",
        ));
}

#[test]
fn dry_run_workitem_state_plan_get_state_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("get-state")
        .arg("sp-1")
        .arg("st-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states/st-2",
        ));
}

#[test]
fn dry_run_workitem_state_plan_remove_state_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("remove-state")
        .arg("sp-1")
        .arg("st-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_states/st-2",
        ));
}

#[test]
fn dry_run_workitem_state_plan_add_flow_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("add-flow")
        .arg("sp-1")
        .arg("--data")
        .arg(r#"{"from_state_id":"st-1","to_state_id":"st-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows",
        ))
        .stderr(predicate::str::contains("\"to_state_id\": \"st-2\""));
}

#[test]
fn dry_run_workitem_state_plan_list_flows_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("list-flows")
        .arg("sp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows",
        ));
}

#[test]
fn dry_run_workitem_state_plan_get_flow_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("get-flow")
        .arg("sp-1")
        .arg("flow-9")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows/flow-9",
        ));
}

#[test]
fn dry_run_workitem_state_plan_remove_flow_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-state-plan")
        .arg("remove-flow")
        .arg("sp-1")
        .arg("flow-9")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_state_plans/sp-1/workitem_state_flows/flow-9",
        ));
}

#[test]
fn workitem_property_plan_help_lists_operations() {
    pc().arg("pjm")
        .arg("workitem-property-plan")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("get"))
        .stdout(predicate::str::contains("add-property"))
        .stdout(predicate::str::contains("list-properties"))
        .stdout(predicate::str::contains("get-property"))
        .stdout(predicate::str::contains("remove-property"));
}

#[test]
fn dry_run_workitem_property_plan_list_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("list")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans",
        ));
}

#[test]
fn dry_run_workitem_property_plan_get_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("get")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1",
        ));
}

#[test]
fn dry_run_workitem_property_plan_add_property_previews_post() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("add-property")
        .arg("pp-1")
        .arg("--data")
        .arg(r#"{"workitem_property_id":"prop-2"}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_plan_list_properties_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("list-properties")
        .arg("pp-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties",
        ));
}

#[test]
fn dry_run_workitem_property_plan_get_property_previews_path() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("get-property")
        .arg("pp-1")
        .arg("prop-2")
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties/prop-2",
        ));
}

#[test]
fn dry_run_workitem_property_plan_remove_property_previews_delete() {
    pc().arg("--dry-run")
        .arg("pjm")
        .arg("workitem-property-plan")
        .arg("remove-property")
        .arg("pp-1")
        .arg("prop-2")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] DELETE"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/pjm/workitem_property_plans/pp-1/workitem_properties/prop-2",
        ));
}
