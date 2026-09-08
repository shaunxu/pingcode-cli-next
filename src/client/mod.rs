use std::time::Instant;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
    verbose: bool,
}

/// `/v1/auth/token` 响应
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
}

/// multipart/form-data 表单字段：文本值或文件。
#[derive(Debug, Clone)]
pub enum MultipartField<'a> {
    /// 普通文本字段（字段名, 值）。
    Text(&'a str, &'a str),
    /// 文件字段（字段名, 文件名, 字节内容）。
    File {
        name: &'a str,
        file_name: &'a str,
        bytes: &'a [u8],
    },
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
                } => {
                    fetch_enterprise_token(
                        &config.base_url,
                        client_id,
                        client_secret,
                        config.verbose,
                    )
                    .await?
                }
                Credentials::Anonymous => "dry-run".to_string(),
            }
        };

        Self::with_token(&config.base_url, &token, config.verbose, config.dry_run)
    }

    /// 用已有的 Bearer token 构造客户端，不再触发 client-credentials 令牌换取。
    ///
    /// 供 `doctor` 命令使用：诊断流程需要把「换取令牌」作为一个独立检查项，
    /// 换取成功后再用拿到的 token 构造探针客户端。
    pub fn with_token(
        base_url: &str,
        token: &str,
        verbose: bool,
        dry_run: bool,
    ) -> Result<Self, ClientError> {
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
            base_url: base_url.to_string(),
            dry_run,
            verbose,
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

    /// 对 `{base_url}{path}` 发起带查询参数的 POST 请求，请求体为 JSON。
    pub async fn post_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Value,
        body: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::POST, path, Some(query), Some(body))
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
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, ClientError> {
        self.request(reqwest::Method::DELETE, path, None, None)
            .await
    }

    /// 对 `{base_url}{path}` 发起带查询参数的 DELETE 请求。
    ///
    /// 通用资源（附件、关注人、评审等）的删除端点要求把 `principal_type` 等
    /// 定位信息放在查询字符串上，故与无查询参数的 [`delete`](Self::delete) 区分。
    pub async fn delete_with_query<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Value,
    ) -> Result<T, ClientError> {
        self.request(reqwest::Method::DELETE, path, Some(query), None)
            .await
    }

    /// 对 `{base_url}{path}` 发起 `multipart/form-data` POST 请求（文件上传）。
    ///
    /// `query` 为查询参数（如附件上传的 `principal_type`/`principal_id`）；
    /// `fields` 为 multipart 表单字段（文本或文件）；
    /// `dry_run_preview` 是 dry-run 时用于预览的 JSON 摘要（不发网络、不读文件）。
    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Value,
        fields: &[MultipartField<'_>],
        dry_run_preview: &Value,
    ) -> Result<T, ClientError> {
        let query_string = encode_query(query);
        let url = match &query_string {
            Some(qs) => format!("{}{}?{}", self.base_url, path, qs),
            None => format!("{}{}", self.base_url, path),
        };

        if self.dry_run {
            let _ = output::print_dry_run("POST", &url, Some(dry_run_preview));
            return Ok(serde_json::from_value(Value::Null)?);
        }

        // 先构建 multipart 表单，以便 verbose 日志能输出实际的 Content-Type（含 boundary）。
        let mut multipart = reqwest::multipart::Form::new();
        for field in fields {
            match field {
                MultipartField::Text(name, value) => {
                    multipart = multipart.text((*name).to_string(), (*value).to_string());
                }
                MultipartField::File {
                    name,
                    file_name,
                    bytes,
                } => {
                    let part = reqwest::multipart::Part::bytes(bytes.to_vec())
                        .file_name((*file_name).to_string());
                    multipart = multipart.part((*name).to_string(), part);
                }
            }
        }

        let boundary = multipart.boundary().to_string();
        let req = self.http.post(&url).multipart(multipart);
        if self.verbose {
            // multipart 不打印文件内容，Body 以 JSON 摘要列出每个字段的名称与文件元信息。
            let content_type = format!("multipart/form-data; boundary={boundary}");
            let headers = request_headers_json(Some(&content_type));
            output::log_http_request("POST", &url, &headers, Some(&multipart_body(fields)));
        }

        let started = Instant::now();
        let resp = req.send().await?;
        handle(resp, self.verbose, &url, started).await
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

        let content_type = body.map(|_| "application/json");
        let mut req = self.http.request(method.clone(), &url);
        if let Some(body) = body {
            req = req.json(body);
        }
        if self.verbose {
            let headers = request_headers_json(content_type);
            output::log_http_request(method.as_str(), &url, &headers, body);
        }

        let started = Instant::now();
        let resp = req.send().await?;
        handle(resp, self.verbose, &url, started).await
    }
}

/// 构造 verbose 日志中的请求头 JSON。
///
/// `content_type` 为 `Some` 时额外带上 Content-Type（JSON 请求 / multipart 上传）；
/// `Authorization` 统一脱敏为 `Bearer ***`，不泄漏令牌。
fn request_headers_json(content_type: Option<&str>) -> Value {
    let mut headers = serde_json::Map::new();
    headers.insert("authorization".to_string(), json!("Bearer ***"));
    headers.insert(
        "user-agent".to_string(),
        json!(concat!("pc/", env!("CARGO_PKG_VERSION"))),
    );
    if let Some(content_type) = content_type {
        headers.insert("content-type".to_string(), json!(content_type));
    }
    Value::Object(headers)
}

/// 将响应头收集为 JSON（同名头聚合为数组；值非 ASCII/UTF-8 时跳过）。
fn response_headers_json(headers: &reqwest::header::HeaderMap) -> Value {
    let mut map = serde_json::Map::new();
    for (name, value) in headers {
        let Ok(text) = value.to_str() else {
            continue;
        };
        map.entry(name.as_str().to_string())
            .and_modify(|existing| match existing {
                Value::Array(items) => items.push(json!(text)),
                Value::String(first) => {
                    *existing = json!([std::mem::take(first), text]);
                }
                _ => {}
            })
            .or_insert(json!(text));
    }
    Value::Object(map)
}

/// 生成 multipart 表单的 JSON 摘要（verbose 日志 Body 段用）：只列字段名与文件元信息，不含文件内容。
fn multipart_body(fields: &[MultipartField<'_>]) -> Value {
    let parts: Vec<Value> = fields
        .iter()
        .map(|field| match field {
            MultipartField::Text(name, value) => {
                json!({ "field": name, "type": "text", "value": value })
            }
            MultipartField::File {
                name,
                file_name,
                bytes,
            } => json!({
                "field": name,
                "type": "file",
                "file_name": file_name,
                "bytes": bytes.len(),
            }),
        })
        .collect();
    json!({ "multipart_fields": parts })
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
///
/// `pub(crate)`：供 `doctor` 命令把令牌换取作为独立检查项单独调用。
pub(crate) async fn fetch_enterprise_token(
    base_url: &str,
    client_id: &str,
    client_secret: &str,
    verbose: bool,
) -> Result<String, ClientError> {
    let http = reqwest::Client::builder()
        .user_agent(concat!("pc/", env!("CARGO_PKG_VERSION")))
        .build()?;

    let url = format!("{base_url}/v1/auth/token");
    let full_url = format!(
        "{url}?grant_type=client_credentials&client_id={}&client_secret={}",
        percent_encode(client_id),
        percent_encode(client_secret),
    );

    let logged_url = redact_query_secret(&full_url);
    if verbose {
        // 令牌请求的 query 中包含 client_secret，日志里必须脱敏；该请求不携带 Authorization 头。
        let headers = json!({
            "user-agent": concat!("pc/", env!("CARGO_PKG_VERSION")),
        });
        output::log_http_request("GET", &logged_url, &headers, None);
    }

    let started = Instant::now();
    let resp = http
        .get(&url)
        .query(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ])
        .send()
        .await
        .map_err(|err| ClientError::HttpRedacted {
            // reqwest 错误消息会回显完整 URL（query 中含 client_secret），必须脱敏后再向上传播。
            message: redact_secret_in_text(&err.to_string(), client_secret),
        })?;

    let status = resp.status();
    let resp_headers = response_headers_json(resp.headers());
    let body = resp.text().await?;
    if verbose {
        // 响应体含 access_token，同样脱敏后再打印。
        output::log_http_response(
            status.as_u16(),
            &logged_url,
            started.elapsed().as_millis(),
            &resp_headers,
            &redact_token_body(&body),
        );
    }

    if !status.is_success() {
        return Err(ClientError::Api {
            status: status.as_u16(),
            body,
        });
    }

    let token: TokenResponse = serde_json::from_str(&body)?;
    Ok(token.access_token)
}

/// 将 URL query 中的 `client_secret` 值替换为 `***`（verbose 日志脱敏）。
fn redact_query_secret(url: &str) -> String {
    let Some((base, query)) = url.split_once('?') else {
        return url.to_string();
    };
    let redacted = query
        .split('&')
        .map(|pair| {
            if pair
                .split('=')
                .next()
                .map(|k| matches!(k, "client_secret"))
                .unwrap_or(false)
            {
                "client_secret=***".to_string()
            } else {
                pair.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("&");
    format!("{base}?{redacted}")
}

/// 将任意文本中出现的 `client_secret` 明文值替换为 `***`（用于 reqwest 错误消息脱敏）。
fn redact_secret_in_text(text: &str, client_secret: &str) -> String {
    text.replace(client_secret, "***")
}

/// 将响应体 JSON 中的 `access_token` 字段值掩码为 `***`（解析失败则原样返回）。
fn redact_token_body(body: &str) -> String {
    match serde_json::from_str::<Value>(body) {
        Ok(Value::Object(mut map)) => {
            if let Some(value) = map.get_mut("access_token") {
                *value = Value::String("***".to_string());
            }
            serde_json::to_string_pretty(&Value::Object(map)).unwrap_or_else(|_| body.to_string())
        }
        _ => body.to_string(),
    }
}

async fn handle<T: DeserializeOwned>(
    resp: reqwest::Response,
    verbose: bool,
    url: &str,
    started: Instant,
) -> Result<T, ClientError> {
    let status = resp.status();
    let headers = response_headers_json(resp.headers());
    let body = resp.text().await?;

    if verbose {
        output::log_http_response(
            status.as_u16(),
            url,
            started.elapsed().as_millis(),
            &headers,
            &body,
        );
    }

    if !status.is_success() {
        return Err(ClientError::Api {
            status: status.as_u16(),
            body,
        });
    }

    let value: T = serde_json::from_str(&body)?;
    Ok(value)
}
