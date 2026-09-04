use clap::Args;
use serde_json::{json, Value};

use crate::commands::reviews::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc reviews get-principal` 的参数。
#[derive(Debug, Args)]
pub struct GetPrincipalArgs {
    /// Review id
    #[arg(value_name = "REVIEW_ID")]
    pub review_id: String,

    /// Principal id (id of the reviewed work item, test case or idea)
    #[arg(value_name = "PRINCIPAL_ID")]
    pub principal_id: String,

    /// Type of the reviewed principals (workitem, testcase or idea)
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,
}

/// 获取评审中的一个评审内容：
/// `GET /v1/reviews/{review_id}/principals/{principal_id}`
/// （scope 依赖评审所属主体）。
///
/// 查询参数 `principal_type`（评审主体类型：`workitem`/`testcase`/`idea`）
/// 必填。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getReviewsByReviewIdPrincipalsByPrincipalIdByPrincipalType
pub async fn run(ctx: &Ctx, args: &GetPrincipalArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));

    let path = format!(
        "/v1/reviews/{}/principals/{}",
        args.review_id, args.principal_id
    );
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
