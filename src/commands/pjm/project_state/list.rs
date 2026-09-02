use clap::Args;
use serde_json::{json, Value};

use crate::commands::Ctx;
use crate::output;

/// `pc pjm project-state list` 的参数。
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Project id
    #[arg(long, value_name = "ID")]
    pub project_id: String,
}

/// 分页获取某项目中的项目状态列表：
/// `GET /v1/pjm/project/states?project_id={project_id}`
/// （scope: `pcp:read:pjm:project`）。
///
/// 注意路径为单数 `project`，`project_id` 通过查询参数传递。
/// 响应为分页结构（`page_index` / `page_size` / `total` / `values`）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getPjmProjectStatesByProjectId
pub async fn run(ctx: &Ctx, args: &ListArgs) -> anyhow::Result<()> {
    let query = json!({ "project_id": args.project_id });
    let response: Value = ctx
        .client
        .get_with_query("/v1/pjm/project/states", &query)
        .await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
