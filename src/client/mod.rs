use serde::de::DeserializeOwned;
use serde::Deserialize;

mod error;

pub use error::ClientError;

use crate::config::{Config, Credentials};

/// PingCode REST API 客户端
#[derive(Debug, Clone)]
pub struct PingCodeClient {
    http: reqwest::Client,
    base_url: String,
}

/// `/v1/auth/token` 响应
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

/// `/v1/directory/team` 响应（企业信息）
#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub secondary_domain: Option<String>,
}

/// `/v1/myself` 响应（当前用户个人信息）
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub mobile: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

impl PingCodeClient {
    pub async fn new(config: &Config) -> Result<Self, ClientError> {
        let token = match &config.credentials {
            Credentials::Token(token) => token.clone(),
            Credentials::ClientCredentials {
                client_id,
                client_secret,
            } => fetch_enterprise_token(&config.base_url, client_id, client_secret).await?,
        };

        let http = reqwest::Client::builder()
            .user_agent(concat!("pc/", env!("CARGO_PKG_VERSION")))
            .default_headers(
                std::iter::once((
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {token}"))
                        .expect("token contains invalid HTTP header characters"),
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
        handle(resp).await
    }
}

/// 通过 OAuth2 客户端凭据模式（client_credentials）换取企业令牌。
///
/// 端点：`GET /v1/auth/token?grant_type=client_credentials&client_id=...&client_secret=...`
async fn fetch_enterprise_token(
    base_url: &str,
    client_id: &str,
    client_secret: &str,
) -> Result<String, ClientError> {
    let http = reqwest::Client::builder()
        .user_agent(concat!("pc/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let url = format!("{base_url}/v1/auth/token");
    let resp = http
        .get(&url)
        .query(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await?;

    let token: TokenResponse = handle(resp).await?;
    Ok(token.access_token)
}

async fn handle<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, ClientError> {
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
