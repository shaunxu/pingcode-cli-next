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
        self.request(reqwest::Method::GET, path, None, None).await
    }

    /// 对 `{base_url}{path}` 发起带查询参数的 GET 请求。
    ///
    /// `query` 必须是 JSON object，值仅支持字符串/数字/布尔等扁平类型。
    pub async fn get_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::GET, path, Some(query), None)
            .await
    }

    /// 对 `{base_url}{path}` 发起 POST 请求，请求体为 JSON。
    pub async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::POST, path, None, Some(body))
            .await
    }

    /// 对 `{base_url}{path}` 发起 PATCH 请求，请求体为 JSON。
    pub async fn patch<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::PATCH, path, None, Some(body))
            .await
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
        self.request(reqwest::Method::PUT, path, None, Some(body))
            .await
    }

    /// 对 `{base_url}{path}` 发起 DELETE 请求。
    ///
    /// 框架方法：供后续新增的三级操作使用，当前尚无调用方。
    #[allow(dead_code)]
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        self.request(reqwest::Method::DELETE, path, None, None)
            .await
    }

    /// 对 `{base_url}{path}` 发起带 JSON 请求体的 DELETE 请求。
    ///
    /// 个别端点（如删除已被发布引用的发布阶段）要求在 DELETE 时携带请求体
    /// （例如 `{"replace_id": "..."}`），故与无请求体的 [`delete`](Self::delete) 区分。
    pub async fn delete_with_body<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::DELETE, path, None, Some(body))
            .await
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<&Value>,
        body: Option<&Value>,
    ) -> Result<T, ClientError> {
        let query_string = query.and_then(encode_query);
        let url = match &query_string {
            Some(qs) => format!("{}{}?{}", self.base_url, path, qs),
            None => format!("{}{}", self.base_url, path),
        };

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

/// 将 JSON object 编码为 `application/x-www-form-urlencoded` 查询字符串。
///
/// 仅接受 object；值按 `true/false`、数字原样、字符串转义的方式编码，
/// 嵌套对象/数组会被 JSON 序列化后转义。空对象返回 `None`。
fn encode_query(query: &Value) -> Option<String> {
    let object = query.as_object()?;
    if object.is_empty() {
        return None;
    }

    let mut parts = Vec::new();
    for (key, value) in object {
        if value.is_null() {
            continue;
        }
        let raw = match value {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            other => other.to_string(),
        };
        parts.push(format!("{}={}", percent_encode(key), percent_encode(&raw)));
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join("&"))
    }
}

/// RFC 3986 百分号编码（未保留字符不转义），与 reqwest 的 query 编码行为一致。
fn percent_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        let unreserved = matches!(byte,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~');
        if unreserved {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
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
