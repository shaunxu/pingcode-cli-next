//! 旅程 F：pjm 工作项扩展资源——关联（relation）、标签字典（workitem-tag）、
//! 关注人（participant）、工时（workload）。
//! 工作项/标签/关注人/工时都可清理；共享测试项目保留。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};
use crate::journeys::support::{self, now_secs};

#[test]
fn journey_pjm_workitem_extras() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("pjm extras live journey failed");
}

#[derive(Default)]
struct State {
    project_id: String,
    workitem_a: String,
    workitem_b: String,
    relation_id: String,
    tag_id: String,
    participant_workitem: String,
    participant_id: String,
    workload_id: String,
    workload_workitem: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    s.project_id = support::shared_project_id(ctx);
    let type_id = support::first_workitem_type_id(ctx, &s.project_id);
    s.workitem_a = support::create_workitem(ctx, &s.project_id, &type_id, &ctx.unique_name("rela"));
    s.workitem_b = support::create_workitem(ctx, &s.project_id, &type_id, &ctx.unique_name("relb"));

    // 1. 工作项关联：a 关联 b（relate）→ list 包含 → get → delete。
    let body = json!({ "target_workitem_id": s.workitem_b, "relation_type": "relate" });
    let relation = ctx.run_ok(&[
        "pjm",
        "workitem-relation",
        "create",
        &s.workitem_a,
        "--data",
        &body.to_string(),
    ]);
    s.relation_id = str_field(&relation, "id")
        .map(str::to_string)
        .ok_or("create relation has no id")?;
    let list = ctx.run_ok(&["pjm", "workitem-relation", "list", &s.workitem_a]);
    assert!(find_by_id(&values(&list), &s.relation_id).is_some());
    let fetched = ctx.run_ok(&[
        "pjm",
        "workitem-relation",
        "get",
        &s.workitem_a,
        &s.relation_id,
    ]);
    assert_eq!(str_field(&fetched, "id"), Some(s.relation_id.as_str()));
    ctx.run_ok(&[
        "pjm",
        "workitem-relation",
        "delete",
        &s.workitem_a,
        &s.relation_id,
    ]);
    s.relation_id.clear();
    eprintln!("workitem-relation CRUD verified");

    // 2. 标签字典（企业级）：create → get → list 包含 → update → add 到工作项 → remove → delete。
    let tag_name = ctx.unique_name("tag");
    let tag = ctx.run_ok(&[
        "pjm",
        "workitem-tag",
        "create",
        "--data",
        &json!({ "name": tag_name }).to_string(),
    ]);
    s.tag_id = str_field(&tag, "id")
        .map(str::to_string)
        .ok_or("create workitem-tag has no id")?;
    let fetched = ctx.run_ok(&["pjm", "workitem-tag", "get", &s.tag_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.tag_id.as_str()));
    let all_tags = ctx.run_ok(&["pjm", "workitem-tag", "list"]);
    assert!(find_by_id(&values(&all_tags), &s.tag_id).is_some());
    let tag_renamed = ctx.unique_name("tag");
    let updated = ctx.run_ok(&[
        "pjm",
        "workitem-tag",
        "update",
        &s.tag_id,
        "--data",
        &json!({ "name": tag_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(tag_renamed.as_str()));

    // add-tag / get-tag / remove-tag（覆盖工作项上的标签绑定）。
    ctx.run_ok(&[
        "pjm",
        "workitem",
        "add-tag",
        &s.workitem_a,
        "--data",
        &json!({ "tag_id": s.tag_id }).to_string(),
    ]);
    ctx.run_ok(&["pjm", "workitem", "get-tag", &s.workitem_a, &s.tag_id]);
    ctx.run_ok(&["pjm", "workitem", "remove-tag", &s.workitem_a, &s.tag_id]);
    eprintln!("workitem-tag CRUD + add/remove verified");

    // 3. 关注人（participant）+ 工时（workload）：需要一个用户；无用户则跳过。
    if let Some(user_id) = support::first_user_id(ctx) {
        // 关注人：add → list 包含 → get → remove。
        let body = json!({
            "principal_type": "workitem",
            "principal_id": s.workitem_b,
            "type": "user",
            "participant_id": user_id,
        });
        let added = ctx.run_ok(&["participants", "add", "--data", &body.to_string()]);
        s.participant_id = str_field(&added, "id")
            .map(str::to_string)
            .ok_or("add participant has no id")?;
        s.participant_workitem = s.workitem_b.clone();
        let list = ctx.run_ok(&[
            "participants",
            "list",
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.workitem_b,
        ]);
        assert!(find_by_id(&values(&list), &s.participant_id).is_some());
        ctx.run_ok(&[
            "participants",
            "get",
            &s.participant_id,
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.workitem_b,
        ]);
        ctx.run_ok(&[
            "participants",
            "remove",
            &s.participant_id,
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.workitem_b,
        ]);
        s.participant_id.clear();
        eprintln!("participant add/list/get/remove verified");

        // 工时：create → list 包含 → get → update → delete（删除后不假设 get 立即失败）。
        let body = json!({
            "principal_type": "workitem",
            "principal_id": s.workitem_b,
            "duration": 2.5,
            "report_at": now_secs(),
            "report_by_id": user_id,
        });
        let workload = ctx.run_ok(&["workload", "create", "--data", &body.to_string()]);
        s.workload_id = str_field(&workload, "id")
            .map(str::to_string)
            .ok_or("create workload has no id")?;
        s.workload_workitem = s.workitem_b.clone();
        let list = ctx.run_ok(&[
            "workload",
            "list",
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.workitem_b,
        ]);
        assert!(find_by_id(&values(&list), &s.workload_id).is_some());
        let fetched = ctx.run_ok(&["workload", "get", &s.workload_id]);
        assert_eq!(str_field(&fetched, "id"), Some(s.workload_id.as_str()));
        let updated = ctx.run_ok(&[
            "workload",
            "update",
            &s.workload_id,
            "--data",
            &json!({ "duration": 3.0 }).to_string(),
        ]);
        assert!(updated.is_object());
        ctx.run_ok(&["workload", "delete", &s.workload_id]);
        s.workload_id.clear();
        eprintln!("workload CRUD verified");

        // 工时类型列表冒烟（只读字典）。
        let wtypes = ctx.run_ok(&["workload-type", "list"]);
        let _ = values(&wtypes);
    } else {
        eprintln!("skip participant/workload steps: tenant has no users");
    }

    // 4. 清理标签字典（工作项在 cleanup 中删除）。
    ctx.run_ok(&["pjm", "workitem-tag", "delete", &s.tag_id]);
    ctx.run_fail(&["pjm", "workitem-tag", "get", &s.tag_id]);
    s.tag_id.clear();

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup");
        return;
    }
    if !s.workload_id.is_empty() {
        ctx.run_ok_ignored(&["workload", "delete", &s.workload_id]);
    }
    if !s.participant_id.is_empty() {
        ctx.run_ok_ignored(&[
            "participants",
            "remove",
            &s.participant_id,
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.participant_workitem,
        ]);
    }
    if !s.tag_id.is_empty() {
        ctx.run_ok_ignored(&["pjm", "workitem-tag", "delete", &s.tag_id]);
    }
    if !s.relation_id.is_empty() {
        ctx.run_ok_ignored(&[
            "pjm",
            "workitem-relation",
            "delete",
            &s.workitem_a,
            &s.relation_id,
        ]);
    }
    if !s.workitem_a.is_empty() {
        ctx.run_ok_ignored(&["pjm", "workitem", "delete", &s.workitem_a]);
    }
    if !s.workitem_b.is_empty() {
        ctx.run_ok_ignored(&["pjm", "workitem", "delete", &s.workitem_b]);
    }
}
