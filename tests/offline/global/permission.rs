use crate::common::pc;
use predicates::prelude::*;

#[test]
fn permission_help_lists_operations() {
    pc().arg("permission")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("points"))
        .stdout(predicate::str::contains("my-global"))
        .stdout(predicate::str::contains("my-pilot"))
        .stdout(predicate::str::contains("my-principal"))
        .stdout(predicate::str::contains("check-global"))
        .stdout(predicate::str::contains("check-pilot"))
        .stdout(predicate::str::contains("check-principal"));
}

#[test]
fn dry_run_permission_points_previews_request() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("points")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/points",
        ));
}

#[test]
fn dry_run_permission_my_global_previews_request() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("my-global")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/my/global",
        ));
}

#[test]
fn dry_run_permission_my_pilot_previews_query() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("my-pilot")
        .arg("--type")
        .arg("project")
        .arg("--id")
        .arg("proj-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/my/pilot?",
        ))
        .stderr(predicate::str::contains("type=project"))
        .stderr(predicate::str::contains("id=proj-1"));
}

#[test]
fn permission_my_pilot_rejects_invalid_type() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("my-pilot")
        .arg("--type")
        .arg("workitem")
        .arg("--id")
        .arg("wi-1")
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn dry_run_permission_my_principal_previews_query() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("my-principal")
        .arg("--type")
        .arg("workitem")
        .arg("--id")
        .arg("wi-1")
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] GET"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/my/principal?",
        ))
        .stderr(predicate::str::contains("type=workitem"))
        .stderr(predicate::str::contains("id=wi-1"));
}

#[test]
fn dry_run_permission_check_global_previews_request() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("check-global")
        .arg("--data")
        .arg(r#"{"permissions":["global:team:manage"]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/check/global",
        ))
        .stderr(predicate::str::contains("\"permissions\""));
}

#[test]
fn dry_run_permission_check_pilot_previews_request() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("check-pilot")
        .arg("--data")
        .arg(r#"{"permissions":["pjm:workitem:read"],"type":"project","ids":["proj-1","proj-2"]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/check/pilot",
        ))
        .stderr(predicate::str::contains("\"type\": \"project\""))
        .stderr(predicate::str::contains("\"ids\""));
}

#[test]
fn dry_run_permission_check_principal_previews_request() {
    pc().arg("--dry-run")
        .arg("permission")
        .arg("check-principal")
        .arg("--data")
        .arg(r#"{"permissions":["pjm:workitem:edit"],"type":"workitem","ids":["wi-1"]}"#)
        .assert()
        .success()
        .stderr(predicate::str::contains("[dry-run] POST"))
        .stderr(predicate::str::contains(
            "https://api.pingcode.com/v1/permission/check/principal",
        ))
        .stderr(predicate::str::contains("\"type\": \"workitem\""));
}
