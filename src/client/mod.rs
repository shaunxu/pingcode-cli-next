use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod error;

pub use error::ClientError;

use crate::config::{Config, Credentials};
use crate::output;

/// PingCode REST API 客户端
#[derive(Debug, Clone)]
pub struct PingCodeClient {
    http: reqwest::Client,
    base_url: String,
    dry_run: bool,
}

/// `/v1/auth/token` 响应
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

/// `/v1/directory/team` 响应（企业信息）
#[derive(Debug, Clone, Deserialize, Serialize)]
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
#[derive(Debug, Clone, Deserialize, Serialize)]
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
        // dry-run 模式不换取令牌、不发任何网络请求，使用占位 Authorization 头即可。
        let token = if config.dry_run {
            "dry-run".to_string()
        } else {
            match &config.credentials {
                Credentials::Token(token) => token.clone(),
                Credentials::Client {
                    client_id,
                    client_secret,
                } => fetch_enterprise_token(&config.base_url, client_id, client_secret).await?,
                Credentials::Anonymous => "dry-run".to_string(),
            }
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
            dry_run: config.dry_run,
        })
    }

    /// 对 `{base_url}{path}` 发起 GET 请求并将响应体反序列化为 `T`。
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        self.request(reqwest::Method::GET, path, None).await
    }

    /// 对 `{base_url}{path}` 发起 POST 请求，请求体为 JSON。
    pub async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::POST, path, Some(body)).await
    }

    /// 对 `{base_url}{path}` 发起 PATCH 请求，请求体为 JSON。
    ///
    /// 框架方法：供后续新增的三级操作使用，当前尚无调用方。
    #[allow(dead_code)]
    pub async fn patch<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::PATCH, path, Some(body)).await
    }

    /// 对 `{base_url}{path}` 发起 PUT 请求，请求体为 JSON。
    ///
    /// 框架方法：供后续新增的三级操作使用，当前尚无调用方。
    #[allow(dead_code)]
    pub async fn put<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::PUT, path, Some(body)).await
    }

    /// 对 `{base_url}{path}` 发起 DELETE 请求。
    ///
    /// 框架方法：供后续新增的三级操作使用，当前尚无调用方。
    #[allow(dead_code)]
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        self.request(reqwest::Method::DELETE, path, None).await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&Value>,
    ) -> Result<T, ClientError> {
        let url = format!("{}{}", self.base_url, path);

        if self.dry_run {
            // 预览写入 stderr；写入失败（如管道关闭）直接忽略，不值得中断 dry-run。
            let _ = output::print_dry_run(method.as_str(), &url, body);
            // dry-run 不接触网络：返回空值，调用方不应继续依赖响应内容。
            return Ok(serde_json::from_value(Value::Null)?);
        }

        let mut req = self.http.request(method, &url);
        if let Some(body) = body {
            req = req.json(body);
        }
        let resp = req.send().await?;
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
