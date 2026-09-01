use clap::{Parser, Subcommand};

/// PingCode Open API 命令行客户端
#[derive(Debug, Parser)]
#[command(name = "pc", version, about, long_about = None)]
pub struct Cli {
    /// PingCode API 基础地址（也可通过环境变量 PINGCODE_BASE_URL 设置）
    #[arg(long, env = "PINGCODE_BASE_URL", global = true)]
    pub base_url: Option<String>,

    /// 访问令牌（也可通过环境变量 PINGCODE_TOKEN 设置）
    #[arg(long, env = "PINGCODE_TOKEN", global = true)]
    pub token: Option<String>,

    /// 输出更详细的日志信息
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// 验证凭据并输出当前认证用户信息
    Whoami,
}
