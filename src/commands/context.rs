use crate::client::PingCodeClient;
use crate::config::Config;

/// 命令执行上下文：持有已初始化的 API 客户端与运行时配置。
///
/// 三级命令（module/resource/operation）与自由命令都接收 `&Ctx`，
/// 避免每个命令重复构造客户端或层层传递 `config`。
pub struct Ctx {
    pub client: PingCodeClient,
    pub config: Config,
}

impl Ctx {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let client = PingCodeClient::new(&config).await?;
        Ok(Self { client, config })
    }
}
