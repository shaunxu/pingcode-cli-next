//! 旅程 G：评审（review）+ 评审内容（principal）。
//!
//! review 是容器（title + pilot_id + principal_type），被评实体通过 add-principal 关联。
//! 注意：真实环境创建评审要求「评审规则阶段的评审人」（错误 100071），该规则字段未在公开
//! 文档中暴露，客户端凭据下无法设置；此时评审步骤优雅跳过（不影响其他旅程）。
//! review 有删除接口，成功创建时完整自清理；工作项在 cleanup best-effort 删除。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};
use crate::journeys::support;

#[test]
fn journey_reviews() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("reviews live journey failed");
}

#[derive(Default)]
struct State {
    project_id: String,
    workitem_id: String,
    review_id: String,
    principal_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    s.project_id = support::shared_project_id(ctx);
    let type_id = support::first_workitem_type_id(ctx, &s.project_id);
    s.workitem_id = support::create_workitem(ctx, &s.project_id, &type_id, &ctx.unique_name("rwi"));

    // 1. 创建评审容器（pilot = 项目，principal_type = workitem）。
    //    受未公开的评审规则阶段约束（100071），创建不了就整体跳过。
    let title = ctx.unique_name("rev");
    let body = json!({
        "title": title,
        "pilot_id": s.project_id,
        "principal_type": "workitem",
    });
    let Some(review) = ctx.run_try(&["reviews", "create", "--data", &body.to_string()]) else {
        eprintln!(
            "skip review steps: review rule-stage reviewers required (undocumented server rule)"
        );
        return Ok(());
    };
    s.review_id = str_field(&review, "id")
        .map(str::to_string)
        .ok_or("create review has no id")?;
    assert_eq!(str_field(&review, "title"), Some(title.as_str()));
    eprintln!("created review {title} (id={})", s.review_id);

    // 2. 评审列表（按 pilot + principal_type 过滤）包含。
    let list = ctx.run_ok(&[
        "reviews",
        "list",
        "--principal-type",
        "workitem",
        "--pilot-id",
        &s.project_id,
    ]);
    assert!(find_by_id(&values(&list), &s.review_id).is_some());

    // 3. 评审详情。
    let fetched = ctx.run_ok(&[
        "reviews",
        "get",
        &s.review_id,
        "--principal-type",
        "workitem",
    ]);
    assert_eq!(str_field(&fetched, "id"), Some(s.review_id.as_str()));

    // 4. 向评审添加被评工作项（add-principal）→ list-principals 包含 → get → remove。
    let body = json!({ "principal_type": "workitem", "principal_id": s.workitem_id });
    let Some(principal) = ctx.run_try(&[
        "reviews",
        "add-principal",
        &s.review_id,
        "--data",
        &body.to_string(),
    ]) else {
        eprintln!("skip add-principal: server rejected adding review content");
        return Ok(());
    };
    s.principal_id = str_field(&principal, "id")
        .map(str::to_string)
        .ok_or("add-principal has no id")?;
    let list = ctx.run_ok(&[
        "reviews",
        "list-principals",
        &s.review_id,
        "--principal-type",
        "workitem",
    ]);
    assert!(find_by_id(&values(&list), &s.principal_id).is_some());
    ctx.run_ok(&[
        "reviews",
        "get-principal",
        &s.review_id,
        &s.principal_id,
        "--principal-type",
        "workitem",
    ]);
    ctx.run_ok(&[
        "reviews",
        "remove-principal",
        &s.review_id,
        &s.principal_id,
        "--principal-type",
        "workitem",
    ]);
    s.principal_id.clear();
    eprintln!("review principal add/list/get/remove verified");

    // 5. 负面用例：不存在的评审应失败。
    let stderr = ctx.run_fail(&[
        "reviews",
        "get",
        "pcl-nope-review",
        "--principal-type",
        "workitem",
    ]);
    assert!(!stderr.is_empty());

    // 6. 删除评审，之后 get 应失败。
    ctx.run_ok(&[
        "reviews",
        "delete",
        &s.review_id,
        "--principal-type",
        "workitem",
    ]);
    ctx.run_fail(&[
        "reviews",
        "get",
        &s.review_id,
        "--principal-type",
        "workitem",
    ]);
    s.review_id.clear();
    eprintln!("review deleted (journey complete)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup");
        return;
    }
    if !s.principal_id.is_empty() {
        ctx.run_ok_ignored(&[
            "reviews",
            "remove-principal",
            &s.review_id,
            &s.principal_id,
            "--principal-type",
            "workitem",
        ]);
    }
    if !s.review_id.is_empty() {
        ctx.run_ok_ignored(&[
            "reviews",
            "delete",
            &s.review_id,
            "--principal-type",
            "workitem",
        ]);
    }
    if !s.workitem_id.is_empty() {
        ctx.run_ok_ignored(&["pjm", "workitem", "delete", &s.workitem_id]);
    }
}
