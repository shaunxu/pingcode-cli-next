//! 命令分发。
//!
//! 两类命令：
//! - **三级命令**：`pc <module> <resource> <operation>`（如 `pc pjm workitem create`）。
//!   每个模块一个目录（如 `pjm/`），模块内按资源建子目录，资源目录内按操作拆文件。
//! - **自由命令**：不遵循三级模式的命令（如 `doctor`），
//!   放在 `dynamic/` 下，每个命令一个文件，直接在本文件的 match 中分发。
//!
//! `run` 返回进程退出码：普通命令成功为 `0`；`doctor` 在检查到配置问题时返回 `1`，
//! 诊断流程自身发生意外错误时返回 `2`（见 [`dynamic::doctor`]）。

pub mod activities;
pub mod attachments;
pub mod comments;
pub mod context;
pub mod dynamic;
pub mod entity_properties;
pub mod expression;
pub mod organization;
pub mod participants;
pub mod permission;
pub mod pjm;
pub mod relations;
pub mod reviews;
pub mod security;
pub mod ship;
pub mod testhub;
pub mod wiki;
pub mod workload;
pub mod workload_type;

use context::Ctx;

use crate::cli::{Cli, Command};
use crate::config::Config;

/// 执行命令并返回进程退出码。
///
/// 普通命令成功返回 `0`；`doctor` 的退出码语义见 [`dynamic::doctor`]。
pub async fn run(cli: Cli) -> anyhow::Result<u8> {
    // doctor 不经过常规配置解析/客户端初始化（配置坏了也要能出诊断报告）。
    if matches!(cli.command, Command::Doctor) {
        return dynamic::doctor::run(&cli).await;
    }

    let config = Config::from_cli(&cli)?;
    let ctx = Ctx::new(config.clone()).await?;
    let command = cli.command;

    match command {
        // 三级命令：module -> resource -> operation
        Command::Organization {
            command: organization_command,
        } => {
            organization::run(&ctx, organization_command).await?;
            Ok(0)
        }

        // 三级命令：module -> resource -> operation
        Command::Pjm {
            command: pjm_command,
        } => {
            pjm::run(&ctx, pjm_command).await?;
            Ok(0)
        }

        // 三级命令：module -> resource -> operation
        Command::Ship {
            command: ship_command,
        } => {
            ship::run(&ctx, ship_command).await?;
            Ok(0)
        }

        // 三级命令：module -> resource -> operation
        Command::Testhub {
            command: testhub_command,
        } => {
            testhub::run(&ctx, testhub_command).await?;
            Ok(0)
        }

        // 三级命令：module -> resource -> operation
        Command::Wiki {
            command: wiki_command,
        } => {
            wiki::run(&ctx, wiki_command).await?;
            Ok(0)
        }

        // 三级命令：module -> resource -> operation
        Command::Security {
            command: security_command,
        } => {
            security::run(&ctx, security_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：权限直接挂在顶层（pc permission <operation>）
        Command::Permission {
            command: permission_command,
        } => {
            permission::run(&ctx, permission_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：表达式直接挂在顶层（pc expression <operation>）
        Command::Expression {
            command: expression_command,
        } => {
            expression::run(&ctx, expression_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：工时直接挂在顶层（pc workload <operation>）
        Command::Workload {
            command: workload_command,
        } => {
            workload::run(&ctx, workload_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：工时类型直接挂在顶层（pc workload-type <operation>）
        Command::WorkloadType {
            command: workload_type_command,
        } => {
            workload_type::run(&ctx, workload_type_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：评论直接挂在顶层（pc comments <operation>）
        Command::Comments {
            command: comments_command,
        } => {
            comments::run(&ctx, comments_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：实体扩展属性直接挂在顶层（pc entity-properties <operation>）
        Command::EntityProperties {
            command: entity_properties_command,
        } => {
            entity_properties::run(&ctx, entity_properties_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：附件直接挂在顶层（pc attachments <operation>）
        Command::Attachments {
            command: attachments_command,
        } => {
            attachments::run(&ctx, attachments_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：关注人直接挂在顶层（pc participants <operation>）
        Command::Participants {
            command: participants_command,
        } => {
            participants::run(&ctx, participants_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：关联直接挂在顶层（pc relations <operation>）
        Command::Relations {
            command: relations_command,
        } => {
            relations::run(&ctx, relations_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：活动记录直接挂在顶层（pc activities <operation>）
        Command::Activities {
            command: activities_command,
        } => {
            activities::run(&ctx, activities_command).await?;
            Ok(0)
        }

        // 跨模块全局资源：评审直接挂在顶层（pc reviews <operation>）
        Command::Reviews {
            command: reviews_command,
        } => {
            reviews::run(&ctx, reviews_command).await?;
            Ok(0)
        }

        // 自由命令：不遵循 module/resource/operation 模式（已在上面提前处理）
        Command::Doctor => unreachable!("doctor is dispatched before context creation"),
    }
}
