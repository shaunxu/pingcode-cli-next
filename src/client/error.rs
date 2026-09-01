use thiserror::Error;

/// PingCode API 客户端错误
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API returned error status {status}: {body}")]
    Api { status: u16, body: String },

    #[error("Failed to parse response: {0}")]
    Parse(#[from] serde_json::Error),
}
