use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization department update` 的参数。
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Department id
    #[arg(value_name = "DEPARTMENT_ID")]
    pub department_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 部分更新一个部门：`PATCH /v1/directory/departments/{department_id}`
/// （scope: `pcp:write:global:team`）。
///
/// 请求体可选 `name`、`parent_id`（父部门 id）、`head_id`（负责人 id）。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/patchDirectoryDepartmentsByDepartmentId
pub async fn run(ctx: &Ctx, args: &UpdateArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/directory/departments/{}", args.department_id);
    let response: Value = ctx.client.patch(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
