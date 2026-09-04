use clap::Args;
use serde_json::{json, Value};

use crate::commands::activities::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc activities get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Activity record id
    #[arg(value_name = "ACTIVITY_ID")]
    pub activity_id: String,

    /// Type of the principal the activity record belongs to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test run, test case, idea, ticket)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,
}

/// 获取一条活动记录：`GET /v1/activities/{activity_id}`（scope 依赖活动记录
/// 所属主体）。
///
/// 查询参数 `principal_type`（主体类型：`workitem`/`testrun`/`testcase`/
/// `idea`/`ticket`）、`principal_id`（主体 id）必填。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getActivitiesByActivityId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(&args.principal_id));

    let path = format!("/v1/activities/{}", args.activity_id);
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
