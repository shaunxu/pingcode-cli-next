use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc pjm release-category update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Project id
    #[arg(value_name = "PROJECT_ID")]
    pub project_id: String,

    /// Release category id
    #[arg(value_name = "RELEASE_CATEGORY_ID")]
    pub release_category_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个发布类别：`PATCH /v1/pjm/projects/{project_id}/release_categories/{release_category_id}`
/// （scope: `pcp:write:pjm:release`）。
///
/// 请求体可选 `name`、`section_id`（所属发布分组 id），完整字段见文档。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchPjmProjectsByProjectIdReleaseCategoriesByReleaseCategoryId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!(
        "/v1/pjm/projects/{}/release_categories/{}",
        args.project_id, args.release_category_id
    );
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
