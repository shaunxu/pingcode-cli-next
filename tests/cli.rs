mod common;
mod dynamic;
mod pjm;
mod ship;
mod testhub;

use common::pc;
use predicates::prelude::*;

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
