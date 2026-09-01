use serde::de::DeserializeOwned;

mod error;

pub use error::ClientError;

use crate::config::Config;

/// PingCode REST API 客户端
#[derive(Debug, Clone)]
pub struct PingCodeClient {
    http: reqwest::Client,
    base_url: String,
}

impl PingCodeClient {
    pub fn new(config: &Config) -> Result<Self, ClientError> {
        let http = reqwest::Client::builder()
            .user_agent(concat!("pc/", env!("CARGO_PKG_VERSION")))
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", config.token))
                        .expect("token 包含非法的 HTTP 头字符"),
                ))
                .collect(),
            )
            .build()?;

        Ok(Self {
            http,
            base_url: config.base_url.clone(),
        })
    }

    /// 对 `{base_url}{path}` 发起 GET 请求并将响应体反序列化为 `T`。
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        self.handle(resp).await
    }

    async fn handle<T: DeserializeOwned>(&self, resp: reqwest::Response) -> Result<T, ClientError> {
        let status = resp.status();
        let body = resp.text().await?;

        if !status.is_success() {
            return Err(ClientError::Api {
                status: status.as_u16(),
                body,
            });
        }

        let value: T = serde_json::from_str(&body)?;
        Ok(value)
    }
}
