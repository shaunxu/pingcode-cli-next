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
fn workitem_help_lists_create_operation() {
    pc().arg("pjm")
        .arg("workitem")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("create"));
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
