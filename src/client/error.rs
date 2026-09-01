use thiserror::Error;

/// PingCode API 客户端错误
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP 请求失败：{0}")]
    Request(#[from] reqwest::Error),

    #[error("API 返回错误状态 {status}：{body}")]
    Api { status: u16, body: String },

    #[error("响应解析失败：{0}")]
    Parse(#[from] serde_json::Error),
}
