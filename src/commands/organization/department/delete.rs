use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc organization department delete` 的参数。
#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Department id
    #[arg(value_name = "DEPARTMENT_ID")]
    pub department_id: String,
}

/// 删除一个部门：`DELETE /v1/directory/departments/{department_id}`
/// （scope: `pcp:write:global:team`）。
///
/// 按 id 删除一个部门，返回被删除的部门对象。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/deleteDirectoryDepartmentsByDepartmentId
pub async fn run(ctx: &Ctx, args: &DeleteArgs) -> anyhow::Result<()> {
    let path = format!("/v1/directory/departments/{}", args.department_id);
    let response: Value = ctx.client.delete(&path).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
