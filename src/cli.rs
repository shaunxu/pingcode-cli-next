use clap::{Parser, Subcommand};

use crate::commands::{pjm, ship};

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
    /// Project management module: work items, sprints, releases, projects and more
    Pjm {
        #[command(subcommand)]
        command: pjm::PjmCommand,
    },
    /// Product and ticket management module: products, tickets, ticket configuration and more
    Ship {
        #[command(subcommand)]
        command: ship::ShipCommand,
    },
    /// Show authentication status along with the current team (enterprise) and user info
    State,
}
