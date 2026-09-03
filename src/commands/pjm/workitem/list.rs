use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm workitem list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Work item identifier (short code, e.g. "SCR-1")
    #[arg(long, value_name = "IDENTIFIER")]
    pub identifier: Option<String>,

    /// Filter by project id
    #[arg(long, value_name = "ID")]
    pub project_id: Option<String>,

    /// Filter by work item type id or system type enum (e.g. story, bug)
    #[arg(long, value_name = "ID")]
    pub type_id: Option<String>,

    /// Filter by parent work item id
    #[arg(long, value_name = "ID")]
    pub parent_id: Option<String>,

    /// Filter by assignee id
    #[arg(long, value_name = "ID")]
    pub assignee_id: Option<String>,

    /// Filter by work item state id
    #[arg(long, value_name = "ID")]
    pub state_id: Option<String>,

    /// Filter by priority id
    #[arg(long, value_name = "ID")]
    pub priority_id: Option<String>,

    /// Filter by bug type id
    #[arg(long, value_name = "ID")]
    pub bug_type_id: Option<String>,

    /// Filter by tag id
    #[arg(long, value_name = "ID")]
    pub tag_id: Option<String>,

    /// Filter by sprint id
    #[arg(long, value_name = "ID")]
    pub sprint_id: Option<String>,

    /// Filter by board id
    #[arg(long, value_name = "ID")]
    pub board_id: Option<String>,

    /// Filter by board entry id
    #[arg(long, value_name = "ID")]
    pub entry_id: Option<String>,

    /// Filter by swimlane id
    #[arg(long, value_name = "ID")]
    pub swimlane_id: Option<String>,

    /// Filter by phase (plan) id
    #[arg(long, value_name = "ID")]
    pub phase_id: Option<String>,

    /// Filter by release id
    #[arg(long, value_name = "ID")]
    pub release_id: Option<String>,

    /// Filter by creator id
    #[arg(long, value_name = "ID")]
    pub created_by: Option<String>,

    /// Filter by participant (watcher) id
    #[arg(long, value_name = "ID")]
    pub participant_id: Option<String>,

    /// Filter by keyword; matches work item identifier and title
    #[arg(long)]
    pub keywords: Option<String>,

    /// Fields whose rich-text image tokens should be included, comma-separated (e.g. "description,properties.prop_b")
    #[arg(long, value_name = "FIELDS")]
    pub include_public_image_token: Option<String>,

    /// Include deleted work items
    #[arg(long)]
    pub include_deleted: bool,

    /// Include archived work items
    #[arg(long)]
    pub include_archived: bool,
}

/// 分页获取工作项列表：`GET /v1/pjm/workitems`（scope: `pcp:read:pjm:workitem`）。
///
/// 复杂组合、日期或自定义属性过滤请使用「搜索工作项列表」
/// （`POST /v1/pjm/workitems/search`），本命令仅支持文档列出的简单过滤参数。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmWorkitems
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(identifier) = &args.identifier {
        query.insert("identifier".into(), json!(identifier));
    }
    if let Some(project_id) = &args.project_id {
        query.insert("project_id".into(), json!(project_id));
    }
    if let Some(type_id) = &args.type_id {
        query.insert("type_id".into(), json!(type_id));
    }
    if let Some(parent_id) = &args.parent_id {
        query.insert("parent_id".into(), json!(parent_id));
    }
    if let Some(assignee_id) = &args.assignee_id {
        query.insert("assignee_id".into(), json!(assignee_id));
    }
    if let Some(state_id) = &args.state_id {
        query.insert("state_id".into(), json!(state_id));
    }
    if let Some(priority_id) = &args.priority_id {
        query.insert("priority_id".into(), json!(priority_id));
    }
    if let Some(bug_type_id) = &args.bug_type_id {
        query.insert("bug_type_id".into(), json!(bug_type_id));
    }
    if let Some(tag_id) = &args.tag_id {
        query.insert("tag_id".into(), json!(tag_id));
    }
    if let Some(sprint_id) = &args.sprint_id {
        query.insert("sprint_id".into(), json!(sprint_id));
    }
    if let Some(board_id) = &args.board_id {
        query.insert("board_id".into(), json!(board_id));
    }
    if let Some(entry_id) = &args.entry_id {
        query.insert("entry_id".into(), json!(entry_id));
    }
    if let Some(swimlane_id) = &args.swimlane_id {
        query.insert("swimlane_id".into(), json!(swimlane_id));
    }
    if let Some(phase_id) = &args.phase_id {
        query.insert("phase_id".into(), json!(phase_id));
    }
    if let Some(release_id) = &args.release_id {
        query.insert("release_id".into(), json!(release_id));
    }
    if let Some(created_by) = &args.created_by {
        query.insert("created_by".into(), json!(created_by));
    }
    if let Some(participant_id) = &args.participant_id {
        query.insert("participant_id".into(), json!(participant_id));
    }
    if let Some(keywords) = &args.keywords {
        query.insert("keywords".into(), json!(keywords));
    }
    if let Some(fields) = &args.include_public_image_token {
        query.insert("include_public_image_token".into(), json!(fields));
    }
    if args.include_deleted {
        query.insert("include_deleted".into(), json!(true));
    }
    if args.include_archived {
        query.insert("include_archived".into(), json!(true));
    }

    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/workitems", &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
