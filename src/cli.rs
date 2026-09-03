use clap::{Parser, Subcommand};

use crate::commands::{organization, pjm, ship, testhub, wiki};

/// PingCode Open API command line client
#[derive(Debug, Parser)]
#[command(name = "pc", version, about, long_about = None)]
pub struct Cli {
    /// PingCode Open API base URL (env: PC_OPEN_API_BASE_URL)
    #[arg(long, env = "PC_OPEN_API_BASE_URL", global = true)]
    pub base_url: Option<String>,

    /// Access token (env: PC_TOKEN); skips the client-credentials exchange when set
    #[arg(long, env = "PC_TOKEN", global = true)]
    pub token: Option<String>,

    /// Application Client ID (env: PC_CLIENT_ID), used for the client-credentials flow
    #[arg(long, env = "PC_CLIENT_ID", global = true)]
    pub client_id: Option<String>,

    /// Application Client Secret (env: PC_CLIENT_SECRET), used for the client-credentials flow
    #[arg(long, env = "PC_CLIENT_SECRET", global = true)]
    pub client_secret: Option<String>,

    /// Increase verbosity of log output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Print the HTTP request that would be sent without sending it; skips authentication
    #[arg(long, global = true)]
    pub dry_run: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Organization module: enterprise/team, members, departments, groups, roles and jobs
    Organization {
        #[command(subcommand)]
        command: organization::OrganizationCommand,
    },
    /// Project management module: work items, sprints, releases, projects and more
    Pjm {
        #[command(subcommand)]
        command: pjm::PjmCommand,
    },
    /// Ship module: products, ideas, tickets and their configuration
    Ship {
        #[command(subcommand)]
        command: ship::ShipCommand,
    },
    /// Testhub (test management) module: libraries, test cases, test plans, test runs and configuration
    Testhub {
        #[command(subcommand)]
        command: testhub::TesthubCommand,
    },
    /// Wiki (knowledge management) module: spaces, space members and pages
    Wiki {
        #[command(subcommand)]
        command: wiki::WikiCommand,
    },
    /// Show authentication status along with the current team (enterprise) and user info
    State,
}
