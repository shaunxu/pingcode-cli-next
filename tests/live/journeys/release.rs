//! 旅程 E：pjm 发布分组（section）/ 类别（category）全套 CRUD + 发布（release）CRUD。
//! 分组、类别、发布都有删除接口，完整自清理；共享测试项目保留。
//! 发布创建需要 assignee_id（企业成员），无可用用户时跳过发布步骤；
//! bulk-create 依赖 release_stage 字典（stage_id 必填），不在此覆盖。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};
use crate::journeys::support::{self, now_secs};

#[test]
fn journey_pjm_release() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("release live journey failed");
}

#[derive(Default)]
struct State {
    project_id: String,
    release_id: String,
    category_id: String,
    section_id: String,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    s.project_id = support::shared_project_id(ctx);

    // 1. 发布分组（section）：create → list 包含 → get → update → delete。
    let section_name = ctx.unique_name("rsec");
    let section = ctx.run_ok(&[
        "pjm",
        "release-section",
        "create",
        &s.project_id,
        "--data",
        &json!({ "name": section_name, "description": "pc live" }).to_string(),
    ]);
    s.section_id = str_field(&section, "id")
        .map(str::to_string)
        .ok_or("create release-section has no id")?;
    let list = ctx.run_ok(&["pjm", "release-section", "list", &s.project_id]);
    assert!(find_by_id(&values(&list), &s.section_id).is_some());
    let section_renamed = ctx.unique_name("rsec");
    let updated = ctx.run_ok(&[
        "pjm",
        "release-section",
        "update",
        &s.project_id,
        &s.section_id,
        "--data",
        &json!({ "name": section_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(section_renamed.as_str()));
    ctx.run_ok(&[
        "pjm",
        "release-section",
        "delete",
        &s.project_id,
        &s.section_id,
    ]);
    ctx.run_fail(&[
        "pjm",
        "release-section",
        "get",
        &s.project_id,
        &s.section_id,
    ]);
    s.section_id.clear();
    eprintln!("release-section CRUD verified");

    // 2. 发布类别（category）：create → list 包含 → get → update → delete。
    let category_name = ctx.unique_name("rcat");
    let category = ctx.run_ok(&[
        "pjm",
        "release-category",
        "create",
        &s.project_id,
        "--data",
        &json!({ "name": category_name }).to_string(),
    ]);
    s.category_id = str_field(&category, "id")
        .map(str::to_string)
        .ok_or("create release-category has no id")?;
    let list = ctx.run_ok(&["pjm", "release-category", "list", &s.project_id]);
    assert!(find_by_id(&values(&list), &s.category_id).is_some());
    let category_renamed = ctx.unique_name("rcat");
    let updated = ctx.run_ok(&[
        "pjm",
        "release-category",
        "update",
        &s.project_id,
        &s.category_id,
        "--data",
        &json!({ "name": category_renamed }).to_string(),
    ]);
    assert_eq!(str_field(&updated, "name"), Some(category_renamed.as_str()));
    ctx.run_ok(&[
        "pjm",
        "release-category",
        "delete",
        &s.project_id,
        &s.category_id,
    ]);
    ctx.run_fail(&[
        "pjm",
        "release-category",
        "get",
        &s.project_id,
        &s.category_id,
    ]);
    s.category_id.clear();
    eprintln!("release-category CRUD verified");

    // 3. 发布（release）：需要 assignee_id，无用户则跳过。
    if let Some(assignee) = support::first_user_id(ctx) {
        let name = ctx.unique_name("rel");
        let start = now_secs();
        let body = json!({
            "name": name,
            "start_at": start,
            "end_at": start + 7 * 24 * 3600,
            "assignee_id": assignee,
        });
        let release = ctx.run_ok(&[
            "pjm",
            "release",
            "create",
            &s.project_id,
            "--data",
            &body.to_string(),
        ]);
        s.release_id = str_field(&release, "id")
            .map(str::to_string)
            .ok_or("create release has no id")?;
        assert_eq!(str_field(&release, "name"), Some(name.as_str()));
        eprintln!("created release {name} (id={})", s.release_id);

        let list = ctx.run_ok(&["pjm", "release", "list", &s.project_id]);
        assert!(find_by_id(&values(&list), &s.release_id).is_some());
        let fetched = ctx.run_ok(&["pjm", "release", "get", &s.project_id, &s.release_id]);
        assert_eq!(str_field(&fetched, "id"), Some(s.release_id.as_str()));

        let renamed = ctx.unique_name("rel");
        let updated = ctx.run_ok(&[
            "pjm",
            "release",
            "update",
            &s.project_id,
            &s.release_id,
            "--data",
            &json!({ "name": renamed }).to_string(),
        ]);
        assert_eq!(str_field(&updated, "name"), Some(renamed.as_str()));

        ctx.run_ok(&["pjm", "release", "delete", &s.project_id, &s.release_id]);
        // release 是软删除：按 id GET 仍可能返回 200，但已从列表过滤，故用列表断言删除生效。
        let list = ctx.run_ok(&["pjm", "release", "list", &s.project_id]);
        assert!(find_by_id(&values(&list), &s.release_id).is_none());
        s.release_id.clear();
        eprintln!("release CRUD verified");
    } else {
        eprintln!("skip release steps: tenant has no users for assignee_id");
    }

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup; release={}", s.release_id);
        return;
    }
    if !s.release_id.is_empty() {
        ctx.run_ok_ignored(&["pjm", "release", "delete", &s.project_id, &s.release_id]);
    }
    if !s.category_id.is_empty() {
        ctx.run_ok_ignored(&[
            "pjm",
            "release-category",
            "delete",
            &s.project_id,
            &s.category_id,
        ]);
    }
    if !s.section_id.is_empty() {
        ctx.run_ok_ignored(&[
            "pjm",
            "release-section",
            "delete",
            &s.project_id,
            &s.section_id,
        ]);
    }
}
