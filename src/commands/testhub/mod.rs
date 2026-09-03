//! Testhub（测试管理）模块：`pc testhub <resource> <operation>`。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/testhub/` 下新建资源目录（如 `testcase/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`TesthubCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod library;
pub mod library_member;
pub mod library_suite;
pub mod testcase;
pub mod testcase_important_level;
pub mod testcase_property;
pub mod testcase_property_plan;
pub mod testcase_state;
pub mod testcase_type;
pub mod testplan;
pub mod testplan_state;
pub mod testplan_type;
pub mod testrun;
pub mod testrun_status;

use library::LibraryCommand;
use library_member::LibraryMemberCommand;
use library_suite::LibrarySuiteCommand;
use testcase::TestcaseCommand;
use testcase_important_level::TestcaseImportantLevelCommand;
use testcase_property::TestcasePropertyCommand;
use testcase_property_plan::TestcasePropertyPlanCommand;
use testcase_state::TestcaseStateCommand;
use testcase_type::TestcaseTypeCommand;
use testplan::TestplanCommand;
use testplan_state::TestplanStateCommand;
use testplan_type::TestplanTypeCommand;
use testrun::TestrunCommand;
use testrun_status::TestrunStatusCommand;

/// `pc testhub` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum TesthubCommand {
    /// Test libraries
    Library {
        #[command(subcommand)]
        command: LibraryCommand,
    },
    /// Test case suites (folder tree under a library)
    LibrarySuite {
        #[command(subcommand)]
        command: LibrarySuiteCommand,
    },
    /// Test library members
    LibraryMember {
        #[command(subcommand)]
        command: LibraryMemberCommand,
    },
    /// Test cases
    Testcase {
        #[command(subcommand)]
        command: TestcaseCommand,
    },
    /// Test plans
    Testplan {
        #[command(subcommand)]
        command: TestplanCommand,
    },
    /// Test plan types
    TestplanType {
        #[command(subcommand)]
        command: TestplanTypeCommand,
    },
    /// Test runs (execution records)
    Testrun {
        #[command(subcommand)]
        command: TestrunCommand,
    },
    /// Test case properties (custom fields)
    TestcaseProperty {
        #[command(subcommand)]
        command: TestcasePropertyCommand,
    },
    /// Test case property plans (library field configuration)
    TestcasePropertyPlan {
        #[command(subcommand)]
        command: TestcasePropertyPlanCommand,
    },
    /// Test case important levels (read-only)
    TestcaseImportantLevel {
        #[command(subcommand)]
        command: TestcaseImportantLevelCommand,
    },
    /// Test case types (read-only)
    TestcaseType {
        #[command(subcommand)]
        command: TestcaseTypeCommand,
    },
    /// Test case states (read-only)
    TestcaseState {
        #[command(subcommand)]
        command: TestcaseStateCommand,
    },
    /// Test plan states (read-only)
    TestplanState {
        #[command(subcommand)]
        command: TestplanStateCommand,
    },
    /// Test run result statuses (read-only)
    TestrunStatus {
        #[command(subcommand)]
        command: TestrunStatusCommand,
    },
}

pub async fn run(ctx: &Ctx, command: TesthubCommand) -> anyhow::Result<()> {
    match command {
        TesthubCommand::Library { command } => library::run(ctx, command).await,
        TesthubCommand::LibrarySuite { command } => library_suite::run(ctx, command).await,
        TesthubCommand::LibraryMember { command } => library_member::run(ctx, command).await,
        TesthubCommand::Testcase { command } => testcase::run(ctx, command).await,
        TesthubCommand::Testplan { command } => testplan::run(ctx, command).await,
        TesthubCommand::TestplanType { command } => testplan_type::run(ctx, command).await,
        TesthubCommand::Testrun { command } => testrun::run(ctx, command).await,
        TesthubCommand::TestcaseProperty { command } => testcase_property::run(ctx, command).await,
        TesthubCommand::TestcasePropertyPlan { command } => {
            testcase_property_plan::run(ctx, command).await
        }
        TesthubCommand::TestcaseImportantLevel { command } => {
            testcase_important_level::run(ctx, command).await
        }
        TesthubCommand::TestcaseType { command } => testcase_type::run(ctx, command).await,
        TesthubCommand::TestcaseState { command } => testcase_state::run(ctx, command).await,
        TesthubCommand::TestplanState { command } => testplan_state::run(ctx, command).await,
        TesthubCommand::TestrunStatus { command } => testrun_status::run(ctx, command).await,
    }
}
