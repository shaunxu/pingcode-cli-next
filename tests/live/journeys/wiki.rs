//! 旅程 B：wiki 空间 + 页面全 CRUD（完整自清理，覆盖 PUT 动词）。
//!
//! 步骤：space create → list 包含 → get → page create → list 包含 → get →
//! update-content(PUT) → get-content 反映新内容 → list-versions 非空 →
//! 评论挂到 page（验证 page 主体类型）→ 负面用例 → delete page → delete space → get 失败。

use serde_json::json;

use crate::common::{find_by_id, str_field, values, LiveCtx};

#[test]
fn journey_wiki_space_and_page() {
    let Some(ctx) = LiveCtx::new() else {
        return;
    };

    let mut state = State::default();
    let result = run_steps(&ctx, &mut state);
    cleanup(&ctx, &state);
    result.expect("wiki live journey failed");
}

#[derive(Default)]
struct State {
    space_id: String,
    page_id: String,
    comment_id: Option<String>,
}

fn run_steps(ctx: &LiveCtx, s: &mut State) -> Result<(), Box<dyn std::error::Error>> {
    // 1. 创建空间（企业令牌只能建 organization 空间）。
    let name = ctx.unique_name("wiki");
    let identifier = ctx.unique_identifier("WK");
    let body = json!({
        "scope_type": "organization",
        "name": name,
        "identifier": identifier,
        "visibility": "private",
    });
    let space = ctx.run_ok(&["wiki", "space", "create", "--data", &body.to_string()]);
    s.space_id = str_field(&space, "id")
        .ok_or("create space response has no id")?
        .to_string();
    assert_eq!(str_field(&space, "name"), Some(name.as_str()));
    eprintln!("created wiki space {name} (id={})", s.space_id);

    // 2. 空间列表包含新空间。
    let list = ctx.run_ok(&[
        "wiki",
        "space",
        "list",
        "--keywords",
        "pcl",
        "--page-size",
        "100",
    ]);
    assert!(
        find_by_id(&values(&list), &s.space_id).is_some(),
        "space list should contain created space"
    );

    // 3. 获取空间详情。
    let fetched = ctx.run_ok(&["wiki", "space", "get", &s.space_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.space_id.as_str()));

    // 4. 创建页面（带 markdown 正文）。
    let page_name = ctx.unique_name("page");
    let body = json!({
        "space_id": s.space_id,
        "name": page_name,
        "content": "# pc live\ninitial content",
        "format_type": "markdown",
    });
    let page = ctx.run_ok(&["wiki", "page", "create", "--data", &body.to_string()]);
    s.page_id = str_field(&page, "id")
        .ok_or("create page response has no id")?
        .to_string();
    assert_eq!(str_field(&page, "name"), Some(page_name.as_str()));
    eprintln!("created wiki page {page_name} (id={})", s.page_id);

    // 5. 页面列表（按空间过滤）包含新页面。
    let list = ctx.run_ok(&["wiki", "page", "list", "--space-id", &s.space_id]);
    assert!(
        find_by_id(&values(&list), &s.page_id).is_some(),
        "page list should contain created page"
    );

    // 6. 获取页面详情。
    let fetched = ctx.run_ok(&["wiki", "page", "get", &s.page_id]);
    assert_eq!(str_field(&fetched, "id"), Some(s.page_id.as_str()));

    // 7. 更新正文（PUT），get-content 反映新内容。
    let body = json!({ "content": "# pc live\nupdated content", "format_type": "markdown" });
    let updated = ctx.run_ok(&[
        "wiki",
        "page",
        "update-content",
        &s.page_id,
        "--data",
        &body.to_string(),
    ]);
    assert_eq!(str_field(&updated, "format_type"), Some("markdown"));
    let content = ctx.run_ok(&[
        "wiki",
        "page",
        "get-content",
        &s.page_id,
        "--format-type",
        "markdown",
    ]);
    let content_text = str_field(&content, "content").unwrap_or_default();
    assert!(
        content_text.contains("updated content"),
        "get-content should return updated content, got: {content_text}"
    );

    // 8. 版本列表非空（更新正文产生版本）。
    let versions = ctx.run_ok(&["wiki", "page", "list-versions", &s.page_id]);
    assert!(
        !values(&versions).is_empty(),
        "page should have at least one version after content update"
    );

    // 9. 评论挂到 wiki 页面（principal_type=page），随后删除。
    let comment_body = json!({
        "principal_type": "page",
        "principal_id": s.page_id,
        "content": "pc live wiki comment",
    });
    let comment = ctx.run_ok(&["comments", "create", "--data", &comment_body.to_string()]);
    let comment_id = str_field(&comment, "id").ok_or("create page comment response has no id")?;
    s.comment_id = Some(comment_id.to_string());
    let list = ctx.run_ok(&[
        "comments",
        "list",
        "--principal-type",
        "page",
        "--principal-id",
        &s.page_id,
    ]);
    assert!(find_by_id(&values(&list), comment_id).is_some());
    ctx.run_ok(&[
        "comments",
        "delete",
        comment_id,
        "--principal-type",
        "page",
        "--principal-id",
        &s.page_id,
    ]);
    s.comment_id = None;

    // 10. 负面用例：不存在的空间 id 应失败。
    let stderr = ctx.run_fail(&["wiki", "space", "get", "pcl-nope-space"]);
    assert!(!stderr.is_empty());

    // 11. 删除页面与空间，之后 get 均应失败（完整自清理）。
    ctx.run_ok(&["wiki", "page", "delete", &s.page_id]);
    ctx.run_fail(&["wiki", "page", "get", &s.page_id]);
    s.page_id.clear();
    ctx.run_ok(&["wiki", "space", "delete", &s.space_id]);
    ctx.run_fail(&["wiki", "space", "get", &s.space_id]);
    s.space_id.clear();
    eprintln!("deleted wiki page and space (journey steps complete)");

    Ok(())
}

fn cleanup(ctx: &LiveCtx, s: &State) {
    if ctx.keep {
        eprintln!("PC_LIVE_KEEP=1: skipping cleanup; space={}", s.space_id);
        return;
    }
    // best-effort 收尾：正常流程已在步骤内删除；这里只兜底中途失败的残留。
    if let Some(comment_id) = &s.comment_id {
        ctx.run_ok_ignored(&[
            "comments",
            "delete",
            comment_id,
            "--principal-type",
            "page",
            "--principal-id",
            &s.page_id,
        ]);
    }
    if !s.page_id.is_empty() {
        ctx.run_ok_ignored(&["wiki", "page", "delete", &s.page_id]);
    }
    if !s.space_id.is_empty() {
        ctx.run_ok_ignored(&["wiki", "space", "delete", &s.space_id]);
    }
}
