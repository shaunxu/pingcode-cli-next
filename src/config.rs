use anyhow::{bail, Result};

use crate::cli::Cli;

/// 运行期配置，由命令行参数与环境变量合并而来
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub verbose: bool,
}

const DEFAULT_BASE_URL: &str = "https://api.pingcode.com";

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        let base_url = cli
            .base_url
            .clone()
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();

        let token = match cli.token.clone() {
            Some(t) if !t.trim().is_empty() => t,
            _ => bail!(
                "缺少访问令牌：请通过 --token 参数或 PINGCODE_TOKEN 环境变量提供 PingCode 访问令牌"
            ),
        };

        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            bail!("base-url 必须以 http:// 或 https:// 开头，当前值为：{base_url}");
        }

        let config = Config {
            base_url,
            token,
            verbose: cli.verbose,
        };
        Ok(config)
    }
}
