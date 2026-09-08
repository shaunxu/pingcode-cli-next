use thiserror::Error;

/// PingCode API 客户端错误
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// HTTP 请求失败，消息中的敏感信息（如 `client_secret`）已脱敏。
    #[error("HTTP request failed: {message}")]
    HttpRedacted { message: String },

    #[error("API returned error status {status}: {body}")]
    Api { status: u16, body: String },

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
}
