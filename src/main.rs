mod cli;
mod client;
mod commands;
mod config;
mod output;

use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> ExitCode {
    // 自动加载工作目录下的 .env；已设置的真实环境变量优先，不会被 .env 覆盖
    let _ = dotenvy::dotenv();

    let cli = Cli::parse();

    match commands::run(cli).await {
        // 0：成功；1：doctor 检查发现配置问题（报告已输出到 stdout）
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("Error: {err:#}");
            // 2：命令执行过程中发生意外错误（区别于 doctor 的配置问题诊断）
            ExitCode::from(2)
        }
    }
}
