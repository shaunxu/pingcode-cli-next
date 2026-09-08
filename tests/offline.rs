#[path = "offline/common/mod.rs"]
mod common;
#[path = "offline/dynamic/mod.rs"]
mod dynamic;
#[path = "offline/global/mod.rs"]
mod global;
#[path = "offline/organization/mod.rs"]
mod organization;
#[path = "offline/pjm/mod.rs"]
mod pjm;
#[path = "offline/security/mod.rs"]
mod security;
#[path = "offline/ship/mod.rs"]
mod ship;
#[path = "offline/testhub/mod.rs"]
mod testhub;
#[path = "offline/wiki/mod.rs"]
mod wiki;

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
fn help_lists_doctor_command() {
    pc().arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("doctor"));
}

#[test]
fn version_succeeds() {
    pc().arg("--version").assert().success();
}

// 配置错误（缺凭据/凭据不成对/base-url 非法）不再以普通错误退出，而是由
// `doctor` 命令输出结构化诊断报告；对应断言见 tests/offline/dynamic/doctor.rs。
