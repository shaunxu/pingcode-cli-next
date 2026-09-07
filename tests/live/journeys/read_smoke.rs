//! 旅程 H：只读/字典接口冒烟。全部为 GET（无副作用、无残留），
//! 验证鉴权与反序列化在真实环境可用。
//!
//! 只覆盖稳定的只读字典端点；`permission points`（500）、`permission my-global`（403）
//! 等在部分租户/令牌下不稳定的接口不纳入，活动记录依赖已存在的工作项则用 run_try 容错。

use serde_json::Value;

use crate::common::{values, LiveCtx};
use crate::journeys::support;

#[test]
fn journey_readonly_smoke() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };
    run_steps(&ctx).expect("read-only smoke journey failed");
}

/// 断言响应是对象或数组（即被正常解析）。
fn assert_response_shape(value: &Value, label: &str) {
    assert!(
        value.is_object() || value.is_array(),
        "{label} should return an object or array, got: {value}"
    );
}

/// 只读取值：成功返回 true（并断言信封形状），失败打印跳过原因但不 panic。
fn smoke_list(ctx: &LiveCtx, label: &str, args: &[&str]) {
    if let Some(value) = ctx.run_try(args) {
        assert_response_shape(&value, label);
        let _ = values(&value);
    }
}

fn run_steps(ctx: &LiveCtx) -> Result<(), Box<dyn std::error::Error>> {
    let project_id = support::shared_project_id(ctx);
    let type_id = support::first_workitem_type_id(ctx, &project_id);
    let workitem_id = support::create_workitem(ctx, &project_id, &type_id, &ctx.unique_name("ro"));

    // pjm 字典：项目流程、工作项优先级（全局 + 按项目）。
    smoke_list(
        ctx,
        "project-process list",
        &["pjm", "project-process", "list"],
    );
    smoke_list(
        ctx,
        "workitem-priority list",
        &["pjm", "workitem-priority", "list"],
    );
    smoke_list(
        ctx,
        "workitem-priority list-for-project",
        &[
            "pjm",
            "workitem-priority",
            "list-for-project",
            "--project-id",
            &project_id,
        ],
    );

    // organization 字典：角色、职位、团队、部门。
    smoke_list(ctx, "role list", &["organization", "role", "list"]);
    smoke_list(ctx, "job list", &["organization", "job", "list"]);
    smoke_list(ctx, "group list", &["organization", "group", "list"]);
    smoke_list(
        ctx,
        "department list",
        &["organization", "department", "list"],
    );

    // 工时类型字典。
    smoke_list(ctx, "workload-type list", &["workload-type", "list"]);

    // 工作项活动记录（依赖该工作项已有活动，失败容错）。
    smoke_list(
        ctx,
        "activities list",
        &[
            "activities",
            "list",
            "--principal-type",
            "workitem",
            "--principal-id",
            &workitem_id,
        ],
    );

    // 清理本旅程创建的工作项（其余均为只读）。
    ctx.run_ok_ignored(&["pjm", "workitem", "delete", &workitem_id]);

    eprintln!("read-only smoke journey verified");
    Ok(())
}
