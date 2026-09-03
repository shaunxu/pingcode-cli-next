use crate::common::pc;
use predicates::prelude::*;

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
