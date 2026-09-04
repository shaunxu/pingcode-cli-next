use clap::Args;
use serde_json::{json, Value};

use crate::commands::participants::PrincipalType;
use crate::commands::Ctx;
use crate::output;

/// `pc participants get` 的参数。
#[derive(Debug, Args)]
pub struct GetArgs {
    /// Participant id (user id or team id)
    #[arg(value_name = "PARTICIPANT_ID")]
    pub participant_id: String,

    /// Type of the principal the participant belongs to
    #[arg(long, value_enum, value_name = "TYPE")]
    pub principal_type: PrincipalType,

    /// Id of the principal (work item, test case, idea, ticket, page, ...)
    #[arg(long, value_name = "ID")]
    pub principal_id: String,
}

/// 获取一个关注人：`GET /v1/participants/{participant_id}`（scope 依赖关注人
/// 所属主体）。
///
/// 查询参数 `principal_type`（主体类型）、`principal_id`（主体 id）。
/// 返回对象中 `type` 为 `user` 时带 `user` 字段，为 `user_group` 时带
/// `user_group` 字段。
///
/// 文档：https://developer.alpha.pingcode.live/restapi/pingcode/getParticipantsByParticipantId
pub async fn run(ctx: &Ctx, args: &GetArgs) -> anyhow::Result<()> {
    let mut query = serde_json::Map::new();
    query.insert("principal_type".into(), json!(args.principal_type.as_str()));
    query.insert("principal_id".into(), json!(&args.principal_id));

    let path = format!("/v1/participants/{}", args.participant_id);
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
