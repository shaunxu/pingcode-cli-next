use clap::Args;
use serde_json::Value;

use crate::commands::Ctx;
use crate::output;

/// `pc reviews add-principal` 的参数。
#[derive(Debug, Args)]
pub struct AddPrincipalArgs {
    /// Review id
    #[arg(value_name = "REVIEW_ID")]
    pub review_id: String,

    /// Request body as JSON: inline string, @file.json, or @- for stdin
    ///
    /// Required fields: principal_type (workitem|testcase|idea),
    /// principal_id (id of the reviewed work item, test case or idea).
    #[arg(long, value_name = "JSON")]
    pub data: String,
}

/// 向评审中添加一个评审内容：`POST /v1/reviews/{review_id}/principals`
/// （scope 依赖评审所属主体）。
///
/// 请求体必填 `principal_type`（评审主体类型：`workitem`/`testcase`/
/// `idea`）、`principal_id`（被评工作项/需求/用例的 id）。注意这里的
/// `principal_id` 是被评实体的 id，与评审列表的 `pilot_id`（容器 id）语义
/// 不同。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/postReviewsByReviewIdPrincipals
pub async fn run(ctx: &Ctx, args: &AddPrincipalArgs) -> anyhow::Result<()> {
    let body = output::ensure_object(output::read_data(&args.data)?)?;

    let path = format!("/v1/reviews/{}/principals", args.review_id);
    let response: Value = ctx.client.post(&path, &body).await?;

    if ctx.config.dry_run {
        return Ok(());
    }

    output::print_json(&response)?;
    Ok(())
}
