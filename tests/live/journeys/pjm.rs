//! 旅程 A：pjm 项目 + 工作项全链路。
//!
//! 步骤：state 冒烟 → 项目列表/创建/详情/进度/成员 → 工作项类型 →
//! 工作项创建/列表/详情/更新 → 标签（若项目已有标签）→ 评论 → 附件（文件上传）→
//! 实体扩展属性 → 负面用例（不存在的 id）→ 删除工作项。
//!
//! 项目没有删除接口：设置 `PC_LIVE_PROJECT_ID` 可复用已有测试项目；否则用 `pc-live-`
//! 前缀新建并保留（请在专用测试租户运行并定期人工清理）。

use std::fs;
use std::path::PathBuf;

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};

#[test]
fn journey_pjm_project_and_workitem() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("pjm live journey failed");
}

#[derive(Default)]
struct State {
    project_id: String,
    project_created: bool,
    workitem_id: String,
    /// 旅程中创建、待清理的附件 id。
    attachment_ids: Vec<String>,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // 1. state 冒烟：凭据有效、能取到企业信息。
    let state_out = ctx.run_ok(&["state"]);
    assert_eq!(state_out["authenticated"], true);
    assert!(
        str_field(&state_out["team"], "id").is_some(),
        "state should expose team id: {state_out}"
    );

    // 2. 项目列表（基线，仅断言请求成功且是分页信封）。
    let projects = ctx.run_ok(&["pjm", "project", "list", "--page-size", "50"]);
    let _ = values(&projects);

    // 3. 复用或创建项目。
    if let Some(pid) = &ctx.reuse_project_id {
        s.project_id = pid.clone();
        eprintln!("reusing project {pid} (PC_LIVE_PROJECT_ID)");
    } else {
        let name = ctx.unique_name("proj");
        let identifier = ctx.unique_identifier("PC");
        let body = json!({
            "name": name,
            "identifier": identifier,
            "type": "scrum",
            "visibility": "private",
        });
        let project = ctx.run_ok(&["pjm", "project", "create", "--data", &body.to_string()]);
        s.project_id = str_field(&project, "id")
            .ok_or("create project response has no id")?
            .to_string();
        s.project_created = true;
        eprintln!("created project {} (id={})", name, s.project_id);
    }

    // 4. 项目列表包含新项目（复用场景下跳过名字断言）。
    if s.project_created {
        let list = ctx.run_ok(&[
            "pjm",
            "project",
            "list",
            "--keywords",
            "pcl",
            "--page-size",
            "100",
        ]);
        assert!(
            find_by_id(&values(&list), &s.project_id).is_some(),
            "project list should contain created project {}",
            s.project_id
        );
    }

    // 5. 获取项目详情，字段与创建时一致。
    let project = ctx.run_ok(&["pjm", "project", "get", &s.project_id]);
    assert_eq!(str_field(&project, "id"), Some(s.project_id.as_str()));
    assert!(str_field(&project, "name").is_some());

    // 6. 项目进度、成员列表冒烟。
    let progress = ctx.run_ok(&["pjm", "project", "progress", &s.project_id]);
    assert!(progress.is_object() || progress.is_null());
    let members = ctx.run_ok(&["pjm", "project-member", "list", &s.project_id]);
    let _ = values(&members);

    // 7. 取项目可用的工作项类型。
    let types = ctx.run_ok(&[
        "pjm",
        "workitem-type",
        "list-for-project",
        "--project-id",
        &s.project_id,
    ]);
    let type_id = values(&types)
        .first()
        .and_then(|t| str_field(t, "id"))
        .ok_or("project has no workitem types")?
        .to_string();

    // 8. 创建工作项。
    let title = ctx.unique_name("wi");
    let body = json!({
        "project_id": s.project_id,
        "type_id": type_id,
        "title": title,
    });
    let workitem = ctx.run_ok(&["pjm", "workitem", "create", "--data", &body.to_string()]);
    s.workitem_id = str_field(&workitem, "id")
        .ok_or("create workitem response has no id")?
        .to_string();
    assert_eq!(str_field(&workitem, "title"), Some(title.as_str()));
    eprintln!("created workitem {title} (id={})", s.workitem_id);

    // 9. 工作项列表（按项目过滤）包含新工作项。
    let list = ctx.run_ok(&["pjm", "workitem", "list", "--project-id", &s.project_id]);
    assert!(
        find_by_id(&values(&list), &s.workitem_id).is_some(),
        "workitem list should contain created workitem {}",
        s.workitem_id
    );

    // 10. 获取工作项详情。
    let fetched = ctx.run_ok(&["pjm", "workitem", "get", &s.workitem_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.workitem_id.as_str()));
    assert_eq!(str_field(&fetched, "title"), Some(title.as_str()));

    // 11. 修改工作项标题，详情与列表均反映新值。
    let new_title = ctx.unique_name("wi");
    let body = json!({ "title": new_title });
    let updated = ctx.run_ok(&[
        "pjm",
        "workitem",
        "update",
        &s.workitem_id,
        "--data",
        &body.to_string(),
    ]);
    assert_eq!(str_field(&updated, "title"), Some(new_title.as_str()));
    let fetched = ctx.run_ok(&["pjm", "workitem", "get", &s.workitem_id]);
    assert_eq!(str_field(&fetched, "title"), Some(new_title.as_str()));
    let list = ctx.run_ok(&[
        "pjm",
        "workitem",
        "list",
        "--project-id",
        &s.project_id,
        "--keywords",
        &new_title,
    ]);
    assert!(
        find_by_id(&values(&list), &s.workitem_id).is_some(),
        "workitem list filtered by new title should contain the workitem"
    );

    // 12. 标签：项目标签字典非空时走 add/get/remove；否则跳过（不臆造标签创建）。
    let tags = ctx.run_ok(&[
        "pjm",
        "workitem-tag",
        "list-for-project",
        "--project-id",
        &s.project_id,
    ]);
    if let Some(tag_id) = values(&tags).first().and_then(|t| str_field(t, "id")) {
        let tag_id = tag_id.to_string();
        let body = json!({ "tag_id": tag_id });
        let added = ctx.run_ok(&[
            "pjm",
            "workitem",
            "add-tag",
            &s.workitem_id,
            "--data",
            &body.to_string(),
        ]);
        assert_eq!(added["tag"]["id"], tag_id);
        ctx.run_ok(&["pjm", "workitem", "get-tag", &s.workitem_id, &tag_id]);
        ctx.run_ok(&["pjm", "workitem", "remove-tag", &s.workitem_id, &tag_id]);
        eprintln!("tag add/get/remove verified for tag {tag_id}");
    } else {
        eprintln!("skip tag steps: project has no pre-defined tags");
    }

    // 13. 评论：create → list 包含 → get 内容一致 → delete。
    let comment_body = json!({
        "principal_type": "workitem",
        "principal_id": s.workitem_id,
        "content": "pc live test comment",
    });
    let comment = ctx.run_ok(&["comments", "create", "--data", &comment_body.to_string()]);
    let comment_id = str_field(&comment, "id").ok_or("create comment response has no id")?;
    let list = ctx.run_ok(&[
        "comments",
        "list",
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    assert!(find_by_id(&values(&list), comment_id).is_some());
    let got = ctx.run_ok(&[
        "comments",
        "get",
        comment_id,
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    assert_eq!(str_field(&got, "content"), Some("pc live test comment"));
    ctx.run_ok(&[
        "comments",
        "delete",
        comment_id,
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);

    // 14. 附件（文件，multipart）：upload-file → list 包含 → get → delete。
    // 注：代码段附件 `upload-snippet`（POST /v1/attachments JSON）在测试环境稳定返回
    // 400（文档字段核对无误，疑为环境侧差异），暂不覆盖。
    let file_path = write_temp_file("pc-live-upload.txt", "pc live multipart upload\n")?;
    let uploaded = ctx.run_ok(&[
        "attachments",
        "upload-file",
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
        "--file",
        file_path.to_str().expect("utf8 temp path"),
        "--title",
        "pc-live-upload.txt",
    ]);
    let file_id = str_field(&uploaded, "id").ok_or("upload-file response has no id")?;
    s.attachment_ids.push(file_id.to_string());
    let list = ctx.run_ok(&[
        "attachments",
        "list",
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    assert!(find_by_id(&values(&list), file_id).is_some());
    let got = ctx.run_ok(&[
        "attachments",
        "get",
        file_id,
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    assert_eq!(str_field(&got, "title"), Some("pc-live-upload.txt"));
    ctx.run_ok(&[
        "attachments",
        "delete",
        file_id,
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    s.attachment_ids.retain(|id| id != file_id);
    let list = ctx.run_ok(&[
        "attachments",
        "list",
        "--principal-type",
        "workitem",
        "--principal-id",
        &s.workitem_id,
    ]);
    assert!(find_by_id(&values(&list), file_id).is_none());

    // 15. 实体扩展属性：create → list 包含 → get key 一致 → delete。
    let prop_body = json!({
        "entity_type": "workitem",
        "entity_id": s.workitem_id,
        "property_key": "app:pc-live:test",
        "value": { "ref": "PC-LIVE-1" },
        "overwrite": 1,
    });
    let prop = ctx.run_ok(&[
        "entity-properties",
        "create",
        "--data",
        &prop_body.to_string(),
    ]);
    let prop_id = str_field(&prop, "id").ok_or("create entity-property response has no id")?;
    let list = ctx.run_ok(&[
        "entity-properties",
        "list",
        "--entity-type",
        "workitem",
        "--entity-id",
        &s.workitem_id,
    ]);
    assert!(find_by_id(&values(&list), prop_id).is_some());
    let got = ctx.run_ok(&[
        "entity-properties",
        "get",
        prop_id,
        "--entity-type",
        "workitem",
        "--entity-id",
        &s.workitem_id,
    ]);
    assert_eq!(str_field(&got, "property_key"), Some("app:pc-live:test"));
    ctx.run_ok(&[
        "entity-properties",
        "delete",
        prop_id,
        "--entity-type",
        "workitem",
        "--entity-id",
        &s.workitem_id,
    ]);

    // 16. 负面用例：不存在的工作项 id 应失败。
    let stderr = ctx.run_fail(&["pjm", "workitem", "get", "pcl-nonexistent-id"]);
    assert!(!stderr.is_empty(), "failed get should print an API error");

    // 17. 删除工作项，之后 get 应失败。
    ctx.run_ok(&["pjm", "workitem", "delete", &s.workitem_id]);
    ctx.run_fail(&["pjm", "workitem", "get", &s.workitem_id]);
    let deleted_id = s.workitem_id.clone();
    s.workitem_id.clear();
    eprintln!("deleted workitem {deleted_id} (journey steps complete)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!(
            "PC_LIVE_KEEP=1: skipping cleanup; workitem={}",
            s.workitem_id
        );
        return;
    }
    // best-effort 收尾：正常流程里附件与工作项已在步骤内删除；这里只兜底步骤中途失败的残留。
    for id in &s.attachment_ids {
        ctx.run_ok_ignored(&[
            "attachments",
            "delete",
            id,
            "--principal-type",
            "workitem",
            "--principal-id",
            &s.workitem_id,
        ]);
    }
    if !s.workitem_id.is_empty() {
        ctx.run_ok_ignored(&["pjm", "workitem", "delete", &s.workitem_id]);
    }
    // 项目无删除接口：新建项目保留，名字带 pc-live- 前缀便于人工清理。
    if s.project_created {
        eprintln!(
            "project {} has no delete API; left in tenant with pc-live- prefix for manual cleanup",
            s.project_id
        );
    }
}

fn write_temp_file(name: &str, content: &str) -> std::io::Result<PathBuf> {
    let path = std::env::temp_dir().join(name);
    fs::write(&path, content)?;
    Ok(path)
}
