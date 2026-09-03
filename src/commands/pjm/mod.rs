//! PJM（项目管理）模块：`pc pjm <resource> <operation>`。
//!
//! 新增资源（resource）：
//! 1. 在 `src/commands/pjm/` 下新建资源目录（如 `workitem/`），
//!    在其中按操作（operation）拆分文件；
//! 2. 在本文件的 [`PjmCommand`] 枚举加一个变体，
//!    并在 [`run`] 的 match 中加一行分发。

use clap::Subcommand;

use crate::commands::Ctx;

pub mod board;
pub mod board_entry;
pub mod board_swimlane;
pub mod deliverable;
pub mod project;
pub mod project_member;
pub mod project_process;
pub mod project_property;
pub mod project_state;
pub mod release;
pub mod release_category;
pub mod release_section;
pub mod release_stage;
pub mod sprint;
pub mod sprint_category;
pub mod sprint_section;
pub mod workitem;
pub mod workitem_priority;
pub mod workitem_property;
pub mod workitem_property_plan;
pub mod workitem_relation;
pub mod workitem_relation_type;
pub mod workitem_state;
pub mod workitem_state_plan;
pub mod workitem_tag;
pub mod workitem_transition;
pub mod workitem_type;
pub mod workitem_type_plan;

use board::BoardCommand;
use board_entry::BoardEntryCommand;
use board_swimlane::BoardSwimlaneCommand;
use deliverable::DeliverableCommand;
use project::ProjectCommand;
use project_member::ProjectMemberCommand;
use project_process::ProjectProcessCommand;
use project_property::ProjectPropertyCommand;
use project_state::ProjectStateCommand;
use release::ReleaseCommand;
use release_category::ReleaseCategoryCommand;
use release_section::ReleaseSectionCommand;
use release_stage::ReleaseStageCommand;
use sprint::SprintCommand;
use sprint_category::SprintCategoryCommand;
use sprint_section::SprintSectionCommand;
use workitem::WorkitemCommand;
use workitem_priority::WorkitemPriorityCommand;
use workitem_property::WorkitemPropertyCommand;
use workitem_property_plan::WorkitemPropertyPlanCommand;
use workitem_relation::WorkitemRelationCommand;
use workitem_relation_type::WorkitemRelationTypeCommand;
use workitem_state::WorkitemStateCommand;
use workitem_state_plan::WorkitemStatePlanCommand;
use workitem_tag::WorkitemTagCommand;
use workitem_transition::WorkitemTransitionCommand;
use workitem_type::WorkitemTypeCommand;
use workitem_type_plan::WorkitemTypePlanCommand;

/// `pc pjm` 的资源级子命令。
#[derive(Debug, Subcommand)]
pub enum PjmCommand {
    /// Projects
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    /// Project members
    ProjectMember {
        #[command(subcommand)]
        command: ProjectMemberCommand,
    },
    /// Project properties (global definitions and per-project configuration)
    ProjectProperty {
        #[command(subcommand)]
        command: ProjectPropertyCommand,
    },
    /// Project states
    ProjectState {
        #[command(subcommand)]
        command: ProjectStateCommand,
    },
    /// Project processes (project configuration, read-only)
    ProjectProcess {
        #[command(subcommand)]
        command: ProjectProcessCommand,
    },
    /// Sprints (iterations)
    Sprint {
        #[command(subcommand)]
        command: SprintCommand,
    },
    /// Sprint sections (iteration groups)
    SprintSection {
        #[command(subcommand)]
        command: SprintSectionCommand,
    },
    /// Sprint categories (iteration categories)
    SprintCategory {
        #[command(subcommand)]
        command: SprintCategoryCommand,
    },
    /// Releases
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
    },
    /// Release sections (release groups)
    ReleaseSection {
        #[command(subcommand)]
        command: ReleaseSectionCommand,
    },
    /// Release categories
    ReleaseCategory {
        #[command(subcommand)]
        command: ReleaseCategoryCommand,
    },
    /// Release stages (enterprise-level configuration)
    ReleaseStage {
        #[command(subcommand)]
        command: ReleaseStageCommand,
    },
    /// Work items (requirements, tasks, bugs, ...)
    Workitem {
        #[command(subcommand)]
        command: WorkitemCommand,
    },
    /// Work item tags (tag dictionary)
    WorkitemTag {
        #[command(subcommand)]
        command: WorkitemTagCommand,
    },
    /// Work item relations between work items
    WorkitemRelation {
        #[command(subcommand)]
        command: WorkitemRelationCommand,
    },
    /// Work item relation types (read-only)
    WorkitemRelationType {
        #[command(subcommand)]
        command: WorkitemRelationTypeCommand,
    },
    /// Work item transition histories (read-only)
    WorkitemTransition {
        #[command(subcommand)]
        command: WorkitemTransitionCommand,
    },
    /// Work item deliverable targets
    Deliverable {
        #[command(subcommand)]
        command: DeliverableCommand,
    },
    /// Work item types (work item configuration)
    WorkitemType {
        #[command(subcommand)]
        command: WorkitemTypeCommand,
    },
    /// Work item type plans (work item configuration)
    WorkitemTypePlan {
        #[command(subcommand)]
        command: WorkitemTypePlanCommand,
    },
    /// Work item states (work item configuration)
    WorkitemState {
        #[command(subcommand)]
        command: WorkitemStateCommand,
    },
    /// Work item state plans and state transitions (work item configuration)
    WorkitemStatePlan {
        #[command(subcommand)]
        command: WorkitemStatePlanCommand,
    },
    /// Work item properties / custom fields (work item configuration)
    WorkitemProperty {
        #[command(subcommand)]
        command: WorkitemPropertyCommand,
    },
    /// Work item property plans (work item configuration)
    WorkitemPropertyPlan {
        #[command(subcommand)]
        command: WorkitemPropertyPlanCommand,
    },
    /// Work item priorities (read-only)
    WorkitemPriority {
        #[command(subcommand)]
        command: WorkitemPriorityCommand,
    },
    /// Boards (kanban)
    Board {
        #[command(subcommand)]
        command: BoardCommand,
    },
    /// Board entries (kanban columns)
    BoardEntry {
        #[command(subcommand)]
        command: BoardEntryCommand,
    },
    /// Board swimlanes
    BoardSwimlane {
        #[command(subcommand)]
        command: BoardSwimlaneCommand,
    },
}

pub async fn run(ctx: &Ctx, command: PjmCommand) -> anyhow::Result<()> {
    match command {
        PjmCommand::Project { command } => project::run(ctx, command).await,
        PjmCommand::ProjectMember { command } => project_member::run(ctx, command).await,
        PjmCommand::ProjectProperty { command } => project_property::run(ctx, command).await,
        PjmCommand::ProjectState { command } => project_state::run(ctx, command).await,
        PjmCommand::ProjectProcess { command } => project_process::run(ctx, command).await,
        PjmCommand::Sprint { command } => sprint::run(ctx, command).await,
        PjmCommand::SprintSection { command } => sprint_section::run(ctx, command).await,
        PjmCommand::SprintCategory { command } => sprint_category::run(ctx, command).await,
        PjmCommand::Release { command } => release::run(ctx, command).await,
        PjmCommand::ReleaseSection { command } => release_section::run(ctx, command).await,
        PjmCommand::ReleaseCategory { command } => release_category::run(ctx, command).await,
        PjmCommand::ReleaseStage { command } => release_stage::run(ctx, command).await,
        PjmCommand::Workitem { command } => workitem::run(ctx, command).await,
        PjmCommand::WorkitemTag { command } => workitem_tag::run(ctx, command).await,
        PjmCommand::WorkitemRelation { command } => workitem_relation::run(ctx, command).await,
        PjmCommand::WorkitemRelationType { command } => {
            workitem_relation_type::run(ctx, command).await
        }
        PjmCommand::WorkitemTransition { command } => workitem_transition::run(ctx, command).await,
        PjmCommand::Deliverable { command } => deliverable::run(ctx, command).await,
        PjmCommand::WorkitemType { command } => workitem_type::run(ctx, command).await,
        PjmCommand::WorkitemTypePlan { command } => workitem_type_plan::run(ctx, command).await,
        PjmCommand::WorkitemState { command } => workitem_state::run(ctx, command).await,
        PjmCommand::WorkitemStatePlan { command } => workitem_state_plan::run(ctx, command).await,
        PjmCommand::WorkitemProperty { command } => workitem_property::run(ctx, command).await,
        PjmCommand::WorkitemPropertyPlan { command } => {
            workitem_property_plan::run(ctx, command).await
        }
        PjmCommand::WorkitemPriority { command } => workitem_priority::run(ctx, command).await,
        PjmCommand::Board { command } => board::run(ctx, command).await,
        PjmCommand::BoardEntry { command } => board_entry::run(ctx, command).await,
        PjmCommand::BoardSwimlane { command } => board_swimlane::run(ctx, command).await,
    }
}
