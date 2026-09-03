use clap::{Args, ValueEnum};
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// 发布状态（查询参数 `status`）。
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "snake_case")]
pub enum ReleaseStatus {
    /// Not started
    Pending,
    /// In progress
    InProgress,
    /// Published
    Published,
}

impl ReleaseStatus {
    fn as_str(self) -> &'static str {
        match self {
            ReleaseStatus::Pending => "pending",
            ReleaseStatus::InProgress => "in_progress",
            ReleaseStatus::Published => "published",
        }
    }
}

/// `pc pjm release list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Filter by release name keyword
    #[arg(long)]
    pub name: Option<String>,

    /// Filter by release status
    #[arg(long, value_enum)]
    pub status: Option<ReleaseStatus>,

    /// Filter by creation time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE")]
    pub created_between: Option<String>,

    /// Filter by last-update time range, comma-separated "<start>,<end>"
    #[arg(long, value_name = "RANGE")]
    pub updated_between: Option<String>,
}

/// 获取发布列表：`GET /v1/pjm/projects/{project_id}/releases`（分页，
/// scope: `pcp:read:pjm:release`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectsByProjectIdReleases
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    if let Some(name) = &args.name {
        query.insert("name".into(), json!(name));
    }
    if let Some(status) = args.status {
        query.insert("status".into(), json!(status.as_str()));
    }
    if let Some(range) = &args.created_between {
        query.insert("created_between".into(), json!(range));
    }
    if let Some(range) = &args.updated_between {
        query.insert("updated_between".into(), json!(range));
    }

    let path = format!("/v1/pjm/projects/{}/releases", args.project_id);
    let response: Value = ctx
        .client
        .get_with_query(&path, &Value::Object(query))
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
