//! 表达式（expression）资源：`pc expression <operation>`。
//!
//! 对应 `/v1/expression/...` 的 REST 接口（开放平台文档中的「全局 › 表达式」分组）：
//! 表达式语法分析（analyse）与表达式计算（evaluate）。
//!
//! 新增操作（operation）：
//! 1. 在本目录新建操作文件（如 `analyse.rs`），定义 clap 参数结构体与 `run(ctx, args)`；
//! 2. 在 [`ExpressionCommand`] 枚举加一个变体，并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod analyse;
pub mod evaluate;

use analyse::AnalyseArgs;
use evaluate::EvaluateArgs;

/// `pc expression` 的操作级子命令。
///
/// 操作级变体直接持有参数结构体（实现 `clap::Args`），
/// 不再有下一级子命令。
#[derive(Debug, Subcommand)]
pub enum ExpressionCommand {
    /// Analyse and validate the syntax of a set of expressions (POST /v1/expression/analyse)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postExpressionAnalyse
    Analyse(AnalyseArgs),

    /// Evaluate a set of expressions and return the results (POST /v1/expression/evaluate)
    ///
    /// Docs: https://developer.alpha.pingcode.live/restapi/pingcode/postExpressionEvaluate
    Evaluate(EvaluateArgs),
}

pub async fn run(ctx: &Ctx, command: ExpressionCommand) -> anyhow::Result<()> {
    match command {
        ExpressionCommand::Analyse(args) => analyse::run(ctx, &args).await,
        ExpressionCommand::Evaluate(args) => evaluate::run(ctx, &args).await,
    }
}
