//! 旅程 C：组织架构——部门与团队（user_group）。
//!
//! 部门：create → get → list 包含 → update → get 反映 → delete → get 失败（自清理）。
//! 团队：create → get → list 包含 → update → 成员 add/list/remove。
//! 团队没有删除接口，创建后保留（名字带 pc-live- 前缀，便于人工清理）。
//! 不测试 user create（会向真实邮箱发送邀请）。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};

#[test]
fn journey_organization_department_and_group() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("organization live journey failed");
}

#[derive(Default)]
struct State {
    department_id: String,
    group_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // ---- 部门全 CRUD ----
    let dept_name = ctx.unique_name("dept");
    let body = json!({ "name": dept_name });
    let dept = ctx.run_ok(&[
        "organization",
        "department",
        "create",
        "--data",
        &body.to_string(),
    ]);
    s.department_id = str_field(&dept, "id")
        .ok_or("create department response has no id")?
        .to_string();
    assert_eq!(str_field(&dept, "name"), Some(dept_name.as_str()));
    eprintln!("created department {dept_name} (id={})", s.department_id);

    let fetched = ctx.run_ok(&["organization", "department", "get", &s.department_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.department_id.as_str()));

    let list = ctx.run_ok(&["organization", "department", "list", "--page-size", "100"]);
    assert!(
        find_by_id(&values(&list), &s.department_id).is_some(),
        "department list should contain created department"
    );

    let new_dept_name = ctx.unique_name("dept");
    let body = json!({ "name": new_dept_name });
    let updated = ctx.run_ok(&[
        "organization",
        "department",
        "update",
        &s.department_id,
        "--data",
        &body.to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(new_dept_name.as_str()));
    let fetched = ctx.run_ok(&["organization", "department", "get", &s.department_id]);
    assert_eq!(str_field(&fetched, "name"), Some(new_dept_name.as_str()));

    // ---- 团队 create/get/list/update + 成员 ----
    let group_name = ctx.unique_name("group");
    let body = json!({ "name": group_name, "visibility": "private" });
    let group = ctx.run_ok(&[
        "organization",
        "group",
        "create",
        "--data",
        &body.to_string(),
    ]);
    s.group_id = str_field(&group, "id")
        .ok_or("create group response has no id")?
        .to_string();
    assert_eq!(str_field(&group, "name"), Some(group_name.as_str()));
    eprintln!("created group {group_name} (id={})", s.group_id);

    let fetched = ctx.run_ok(&["organization", "group", "get", &s.group_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.group_id.as_str()));

    let list = ctx.run_ok(&["organization", "group", "list", "--page-size", "100"]);
    assert!(
        find_by_id(&values(&list), &s.group_id).is_some(),
        "group list should contain created group"
    );

    let new_group_name = ctx.unique_name("group");
    let body = json!({ "name": new_group_name });
    let updated = ctx.run_ok(&[
        "organization",
        "group",
        "update",
        &s.group_id,
        "--data",
        &body.to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(new_group_name.as_str()));

    // 成员：取企业第一个用户 add → list 包含 → get → remove。
    let users = ctx.run_ok(&["organization", "user", "list", "--page-size", "10"]);
    if let Some(user_id) = values(&users).first().and_then(|u| str_field(u, "id")) {
        let user_id = user_id.to_string();
        let body = json!({ "user_id": user_id, "role": "member" });
        let membership = ctx.run_ok(&[
            "organization",
            "group-member",
            "add",
            &s.group_id,
            "--data",
            &body.to_string(),
        ]);
        let member_id = str_field(&membership, "id")
            .ok_or("add group member response has no id")?
            .to_string();

        let list = ctx.run_ok(&["organization", "group-member", "list", &s.group_id]);
        assert!(
            find_by_id(&values(&list), &member_id).is_some(),
            "group member list should contain added member"
        );

        ctx.run_ok(&[
            "organization",
            "group-member",
            "get",
            &s.group_id,
            &member_id,
        ]);
        ctx.run_ok(&[
            "organization",
            "group-member",
            "remove",
            &s.group_id,
            &member_id,
        ]);
        eprintln!("group member add/get/remove verified for user {user_id}");
    } else {
        eprintln!("skip group-member steps: tenant has no users");
    }

    // 负面用例：不存在的部门应失败。
    let stderr = ctx.run_fail(&["organization", "department", "get", "pcl-nope-dept"]);
    assert!(!stderr.is_empty());

    // 删除部门，之后 get 应失败（自清理）。
    ctx.run_ok(&["organization", "department", "delete", &s.department_id]);
    ctx.run_fail(&["organization", "department", "get", &s.department_id]);
    s.department_id.clear();
    eprintln!("deleted department (journey steps complete)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!(
            "PC_LIVE_KEEP=1: skipping cleanup; department={} group={}",
            s.department_id, s.group_id
        );
        return;
    }
    // best-effort 收尾：部门正常流程已在步骤内删除；这里兜底中途失败的残留。
    if !s.department_id.is_empty() {
        ctx.run_ok_ignored(&["organization", "department", "delete", &s.department_id]);
    }
    // 团队（user_group）无删除接口：保留，名字带 pc-live- 前缀便于人工清理。
    if !s.group_id.is_empty() {
        eprintln!(
            "group {} has no delete API; left in tenant with pc-live- prefix for manual cleanup",
            s.group_id
        );
    }
}
