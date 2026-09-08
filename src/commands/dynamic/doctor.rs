//! `pc doctor` — 诊断配置与连通性。
//!
//! 与普通命令不同，doctor 不走 [`crate::config::Config::from_cli`]（配置缺失时会直接
//! bail）也不经过 [`crate::commands::Ctx`]（构造客户端时会急切换取令牌）：配置本身有
//! 问题正是 doctor 要报告的对象，因此它宽松读取 CLI/env 原始值，自行发起探针请求，
//! 把每一项检查结果聚合为结构化 JSON 报告输出到 stdout，人类可读的勾叉清单输出到
//! stderr。
//!
//! 退出码：`0` = 全部检查通过（warn/info/skipped 不算失败）；`1` = 至少一项检查失败
//! （stdout 报告含 remediation）；`2` = 诊断流程自身发生意外错误。

use std::io::Write;

use clap::parser::ValueSource;
use clap::{ArgMatches, CommandFactory};
use serde::Serialize;
use serde_json::{json, Value};

use crate::cli::Cli;
use crate::client::{self, ClientError, Team, User};

const DEFAULT_BASE_URL: &str = "https://api.pingcode.com";

/// 检查结论等级。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Pass,
    Fail,
    Warn,
    Info,
    Skipped,
}

/// 一条修复建议。
#[derive(Debug, Clone, Serialize)]
struct Remediation {
    title: String,
    steps: Vec<String>,
}

/// 单项检查结果。
#[derive(Debug, Clone, Serialize)]
struct Check {
    id: &'static str,
    status: Status,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    remediation: Option<Remediation>,
}

impl Check {
    fn passed(id: &'static str, message: impl Into<String>) -> Self {
        Self {
            id,
            status: Status::Pass,
            message: message.into(),
            detail: None,
            remediation: None,
        }
    }

    fn info(id: &'static str, message: impl Into<String>) -> Self {
        Self {
            id,
            status: Status::Info,
            message: message.into(),
            detail: None,
            remediation: None,
        }
    }

    fn skipped(id: &'static str, message: impl Into<String>) -> Self {
        Self {
            id,
            status: Status::Skipped,
            message: message.into(),
            detail: None,
            remediation: None,
        }
    }

    fn failed(
        id: &'static str,
        message: impl Into<String>,
        remediation: Remediation,
        detail: Option<Value>,
    ) -> Self {
        Self {
            id,
            status: Status::Fail,
            message: message.into(),
            detail,
            remediation: Some(remediation),
        }
    }

    fn warned(id: &'static str, message: impl Into<String>, remediation: Remediation) -> Self {
        Self {
            id,
            status: Status::Warn,
            message: message.into(),
            detail: None,
            remediation: Some(remediation),
        }
    }
}

/// 宽松读取到的原始配置（值与来源）。
struct RawConfig {
    base_url: String,
    base_url_source: &'static str,
    token: Option<String>,
    token_source: Option<&'static str>,
    client_id: Option<String>,
    client_id_source: Option<&'static str>,
    client_secret: Option<String>,
    client_secret_source: Option<&'static str>,
    verbose: bool,
    dry_run: bool,
}

fn source_label(source: Option<ValueSource>) -> Option<&'static str> {
    match source {
        Some(ValueSource::CommandLine) => Some("cli"),
        Some(ValueSource::EnvVariable) => Some("env"),
        Some(ValueSource::DefaultValue) => Some("default"),
        _ => None,
    }
}

/// 从 CLI/env 宽松读取配置：任何缺失/不成对都不报错，留给检查项报告。
fn read_raw_config(cli: &Cli) -> RawConfig {
    let matches: ArgMatches = Cli::command().get_matches();

    let value_source = |id: &str| source_label(matches.value_source(id));

    let base_url_source = value_source("base_url").unwrap_or("default");
    let base_url = cli
        .base_url
        .clone()
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
        .trim_end_matches('/')
        .to_string();

    let token = cli.token.clone().filter(|v| !v.trim().is_empty());
    let token_source = value_source("token");
    let client_id = cli.client_id.clone().filter(|v| !v.trim().is_empty());
    let client_id_source = value_source("client_id");
    let client_secret = cli.client_secret.clone().filter(|v| !v.trim().is_empty());
    let client_secret_source = value_source("client_secret");

    RawConfig {
        base_url,
        base_url_source,
        token,
        token_source,
        client_id,
        client_id_source,
        client_secret,
        client_secret_source,
        verbose: cli.verbose,
        dry_run: cli.dry_run,
    }
}

/// `pc doctor` 入口。返回进程退出码（0/1/2 由本模块语义决定）。
pub async fn run(cli: &Cli) -> anyhow::Result<u8> {
    let raw = read_raw_config(cli);

    let mut checks: Vec<Check> = Vec::new();

    // ---- 阶段 A：静态检查（不接触网络）----

    let base_url_ok = if raw.base_url.starts_with("http://") || raw.base_url.starts_with("https://")
    {
        checks.push(Check::passed(
            "base_url_format",
            format!("Base URL {} is a valid http(s) URL.", raw.base_url),
        ));
        true
    } else {
        checks.push(Check::failed(
            "base_url_format",
            format!(
                "Base URL {:?} must start with http:// or https://.",
                raw.base_url
            ),
            Remediation {
                title: "Set a valid base URL".to_string(),
                steps: vec![
                    format!(
                        "Leave PC_OPEN_API_BASE_URL unset to use the default ({DEFAULT_BASE_URL})."
                    ),
                    "Or set it to an https URL, e.g. PC_OPEN_API_BASE_URL=https://api.pingcode.com"
                        .to_string(),
                    "Re-run: pc doctor".to_string(),
                ],
            },
            Some(json!({ "base_url": raw.base_url })),
        ));
        false
    };

    let has_token = raw.token.is_some();
    let has_client_pair = raw.client_id.is_some() && raw.client_secret.is_some();
    let has_any_credentials = has_token || raw.client_id.is_some() || raw.client_secret.is_some();

    let credentials_ok = if !has_any_credentials {
        checks.push(Check::failed(
            "credentials_present",
            "No credentials found: neither an access token nor client credentials are configured.".to_string(),
            Remediation {
                title: "Provide PingCode Open API credentials".to_string(),
                steps: vec![
                    "Option 1 (recommended): set PC_CLIENT_ID and PC_CLIENT_SECRET (or pass --client-id / --client-secret) to use the client-credentials flow.".to_string(),
                    "Option 2: set PC_TOKEN (or pass --token) to an existing access token.".to_string(),
                    "See README 'Authentication' for how to obtain application credentials.".to_string(),
                    "Re-run: pc doctor".to_string(),
                ],
            },
            None,
        ));
        false
    } else {
        checks.push(Check::passed(
            "credentials_present",
            "At least one credential source is configured.".to_string(),
        ));
        true
    };

    // client_id / client_secret 必须成对出现。
    let pair_ok = match (&raw.client_id, &raw.client_secret) {
        (Some(_), Some(_)) => {
            checks.push(Check::passed(
                "credential_pair",
                "Client ID and Client Secret are both provided.".to_string(),
            ));
            true
        }
        (Some(_), None) => {
            checks.push(Check::failed(
                "credential_pair",
                "Client ID is provided but Client Secret is missing.".to_string(),
                Remediation {
                    title: "Provide the matching Client Secret".to_string(),
                    steps: vec![
                        "Set PC_CLIENT_SECRET (or pass --client-secret) alongside the Client ID."
                            .to_string(),
                        "Re-run: pc doctor".to_string(),
                    ],
                },
                None,
            ));
            false
        }
        (None, Some(_)) => {
            checks.push(Check::failed(
                "credential_pair",
                "Client Secret is provided but Client ID is missing.".to_string(),
                Remediation {
                    title: "Provide the matching Client ID".to_string(),
                    steps: vec![
                        "Set PC_CLIENT_ID (or pass --client-id) alongside the Client Secret."
                            .to_string(),
                        "Re-run: pc doctor".to_string(),
                    ],
                },
                None,
            ));
            false
        }
        (None, None) => {
            checks.push(Check::skipped(
                "credential_pair",
                "Client credentials are not used; skipped.".to_string(),
            ));
            true
        }
    };

    // token 与 client 凭据同时存在：token 优先，client 凭据被忽略。
    if has_token && (raw.client_id.is_some() || raw.client_secret.is_some()) {
        checks.push(Check::warned(
            "credential_conflict",
            "Both an access token and client credentials are set; the access token takes precedence and client credentials are ignored.".to_string(),
            Remediation {
                title: "Use only one authentication method".to_string(),
                steps: vec![
                    "Keep PC_TOKEN and unset PC_CLIENT_ID / PC_CLIENT_SECRET, or vice versa.".to_string(),
                    "Re-run: pc doctor".to_string(),
                ],
            },
        ));
    } else {
        checks.push(Check::passed(
            "credential_conflict",
            "Only one authentication method is configured.".to_string(),
        ));
    }

    let auth_method = if has_token {
        "access_token"
    } else if has_client_pair {
        "client_credentials"
    } else {
        "none"
    };

    // ---- 阶段 B：网络探针 ----
    //
    // 先跑探针、把结论收集为局部变量，最后按固定 id 顺序 push 检查结果，
    // 保证 checks 数组顺序稳定（AI agent 可按 id 索引，也可按顺序消费）。

    let network_blocked = raw.dry_run || !base_url_ok || !credentials_ok || !pair_ok;

    let (team, user) = if network_blocked {
        let reason = if raw.dry_run {
            "--dry-run is set; no network requests are sent."
        } else {
            "Static configuration checks failed; network probes cannot run."
        };
        checks.extend([
            Check::skipped("base_url_reachable", reason),
            Check::skipped("token_exchange", reason),
            Check::skipped("api_auth", reason),
            Check::skipped("token_kind", reason),
        ]);
        (None, None)
    } else {
        let (network_checks, team, user) = network_checks(&raw, has_token).await?;
        checks.extend(network_checks);
        (team, user)
    };

    let exit_code = print_report(&raw, &checks, auth_method, team.as_ref(), user.as_ref())?;
    Ok(exit_code)
}

/// 令牌获取阶段的结论：要么拿到可用 token（同时给出 base_url_reachable / token_exchange
/// 两项检查），要么失败（后续探针全部跳过）。
enum TokenAcquired {
    Yes {
        token: String,
        reachable: Option<Check>,
        exchange: Check,
    },
    No {
        reachable: Check,
        exchange: Check,
    },
}

/// 网络阶段探针：返回按固定顺序排列的四项检查（base_url_reachable / token_exchange /
/// api_auth / token_kind）以及探测到的企业/用户信息。
async fn network_checks(
    raw: &RawConfig,
    has_token: bool,
) -> anyhow::Result<([Check; 4], Option<Team>, Option<User>)> {
    let connect_remediation = || {
        Remediation {
        title: "Fix connectivity to the PingCode API host".to_string(),
        steps: vec![
            format!("Verify PC_OPEN_API_BASE_URL points to a reachable host (default: {DEFAULT_BASE_URL})."),
            "Check network access, DNS, proxy settings (HTTPS_PROXY/HTTP_PROXY) and firewalls.".to_string(),
            "Re-run: pc doctor".to_string(),
        ],
    }
    };

    // 第一步：拿到一个可用 token（client 模式先换取），同时判定主机可达性与令牌有效性。
    let acquired = if has_token {
        TokenAcquired::Yes {
            token: raw.token.clone().unwrap(),
            // token 模式的可达性留到第二步用真实 API 调用判定。
            reachable: None,
            exchange: Check::skipped(
                "token_exchange",
                "Using an access token directly; client-credentials exchange skipped.",
            ),
        }
    } else {
        match client::fetch_enterprise_token(
            &raw.base_url,
            raw.client_id.as_deref().unwrap(),
            raw.client_secret.as_deref().unwrap(),
            raw.verbose,
        )
        .await
        {
            Ok(token) => TokenAcquired::Yes {
                token,
                reachable: Some(Check::passed(
                    "base_url_reachable",
                    format!("API host {} is reachable.", raw.base_url),
                )),
                exchange: Check::passed(
                    "token_exchange",
                    "Client credentials were accepted and an enterprise token was issued.",
                ),
            },
            Err(ClientError::HttpRedacted { .. }) | Err(ClientError::Request(_)) => {
                TokenAcquired::No {
                    reachable: Check::failed(
                        "base_url_reachable",
                        format!("Cannot connect to {} (DNS/TCP/TLS/timeout).", raw.base_url),
                        connect_remediation(),
                        Some(json!({ "kind": "connect" })),
                    ),
                    exchange: Check::skipped(
                        "token_exchange",
                        "Host unreachable; token exchange could not be attempted.",
                    ),
                }
            }
            Err(ClientError::Api { status, .. }) => TokenAcquired::No {
                reachable: Check::passed(
                    "base_url_reachable",
                    format!(
                        "API host {} responded (HTTP {status}); credentials were rejected.",
                        raw.base_url
                    ),
                ),
                exchange: Check::failed(
                    "token_exchange",
                    format!("Token endpoint returned HTTP {status}: client credentials were rejected."),
                    Remediation {
                        title: "Provide valid application credentials".to_string(),
                        steps: vec![
                            "Open the PingCode admin console and create or reset the Open API application to get a valid Client ID and Secret.".to_string(),
                            "Update PC_CLIENT_ID and PC_CLIENT_SECRET (or pass --client-id / --client-secret).".to_string(),
                            "Re-run: pc doctor".to_string(),
                        ],
                    },
                    Some(json!({ "http_status": status })),
                ),
            },
            Err(ClientError::Parse(err)) => {
                anyhow::bail!("failed to parse token endpoint response: {err}");
            }
        }
    };

    let (token, reachable_from_exchange, exchange) = match acquired {
        TokenAcquired::Yes {
            token,
            reachable,
            exchange,
        } => (Some(token), reachable, exchange),
        TokenAcquired::No {
            reachable,
            exchange,
        } => {
            // 拿不到 token：api_auth / token_kind 无需再探测。
            return Ok((
                [
                    reachable,
                    exchange,
                    Check::skipped("api_auth", "No valid token available; API probe skipped."),
                    Check::skipped(
                        "token_kind",
                        "No valid token available; token kind probe skipped.",
                    ),
                ],
                None,
                None,
            ));
        }
    };
    let token = token.unwrap();

    // 第二步：用 token 调 /v1/directory/team 验证鉴权（token 模式同时判定可达性）。
    let probe = client::PingCodeClient::with_token(&raw.base_url, &token, raw.verbose, false)?;
    let mut team: Option<Team> = None;
    let reachable;
    let auth;
    match probe.get::<Team>("/v1/directory/team").await {
        Ok(team_info) => {
            // client 模式的可达性已在第一步判定；token 模式在这里判定。
            reachable = reachable_from_exchange.unwrap_or_else(|| {
                Check::passed(
                    "base_url_reachable",
                    format!("API host {} is reachable.", raw.base_url),
                )
            });
            team = Some(team_info);
            auth = Check::passed(
                "api_auth",
                "Authenticated successfully; enterprise (team) info fetched.",
            );
        }
        Err(ClientError::HttpRedacted { .. }) | Err(ClientError::Request(_)) => {
            reachable = Check::failed(
                "base_url_reachable",
                if has_token {
                    format!("Cannot connect to {} (DNS/TCP/TLS/timeout).", raw.base_url)
                } else {
                    format!(
                        "Token issued but {} became unreachable (DNS/TCP/TLS/timeout).",
                        raw.base_url
                    )
                },
                connect_remediation(),
                Some(json!({ "kind": "connect" })),
            );
            auth = Check::skipped("api_auth", "Host unreachable; API probe skipped.");
        }
        Err(ClientError::Api { status, .. }) => {
            reachable = reachable_from_exchange.unwrap_or_else(|| {
                Check::passed(
                    "base_url_reachable",
                    format!("API host {} responded (HTTP {status}).", raw.base_url),
                )
            });
            let message = if has_token {
                format!("GET /v1/directory/team returned HTTP {status}: the access token is invalid, expired or lacks permission.")
            } else {
                format!(
                    "GET /v1/directory/team returned HTTP {status} even though a token was issued."
                )
            };
            let remediation = if status == 401 || status == 403 {
                if has_token {
                    Remediation {
                        title: "Provide a valid access token".to_string(),
                        steps: vec![
                            "The token in PC_TOKEN (or --token) was rejected; obtain a fresh access token.".to_string(),
                            "Alternatively, switch to client credentials via PC_CLIENT_ID and PC_CLIENT_SECRET.".to_string(),
                            "Re-run: pc doctor".to_string(),
                        ],
                    }
                } else {
                    Remediation {
                        title: "Check the application's API permissions".to_string(),
                        steps: vec![
                            "The issued enterprise token was rejected on a basic endpoint; check the application's permissions/scopes in the PingCode admin console.".to_string(),
                            "Re-run with -v/--verbose to see the full HTTP response.".to_string(),
                            "Re-run: pc doctor".to_string(),
                        ],
                    }
                }
            } else {
                Remediation {
                    title: "Investigate the API error response".to_string(),
                    steps: vec![
                        "Re-run with -v/--verbose to see the full HTTP response.".to_string(),
                        "Check the PingCode Open API status and your base URL.".to_string(),
                        "Re-run: pc doctor".to_string(),
                    ],
                }
            };
            auth = Check::failed(
                "api_auth",
                message,
                remediation,
                Some(json!({ "http_status": status })),
            );
        }
        Err(ClientError::Parse(err)) => {
            anyhow::bail!("failed to parse team response: {err}");
        }
    }

    // 第三步：鉴权通过后探测 token 类型（/v1/myself 仅用户令牌可访问）。
    let mut user: Option<User> = None;
    let kind = if team.is_some() {
        let probe = client::PingCodeClient::with_token(&raw.base_url, &token, raw.verbose, false)?;
        match probe.get::<User>("/v1/myself").await {
            Ok(user_info) => {
                user = Some(user_info);
                Check::info(
                    "token_kind",
                    "This is a user access token (bound to an individual user).",
                )
            }
            Err(ClientError::Api { status, .. }) if status == 401 || status == 403 => Check::info(
                "token_kind",
                "This is an enterprise token (not bound to an individual user; /v1/myself is unavailable).",
            ),
            Err(ClientError::Api { status, .. }) => Check::skipped(
                "token_kind",
                format!("Token kind probe returned unexpected HTTP {status}; skipped."),
            ),
            Err(_) => Check::skipped(
                "token_kind",
                "Token kind probe could not connect; skipped.",
            ),
        }
    } else {
        Check::skipped(
            "token_kind",
            "API authentication failed; token kind probe skipped.",
        )
    };

    Ok(([reachable, exchange, auth, kind], team, user))
}

/// 汇总并输出报告（stdout JSON + stderr 勾叉清单），返回退出码。
fn print_report(
    raw: &RawConfig,
    checks: &[Check],
    auth_method: &str,
    team: Option<&Team>,
    user: Option<&User>,
) -> anyhow::Result<u8> {
    let mut counts = Summary::default();
    for check in checks {
        match check.status {
            Status::Pass => counts.pass += 1,
            Status::Fail => counts.fail += 1,
            Status::Warn => counts.warn += 1,
            Status::Info => counts.info += 1,
            Status::Skipped => counts.skipped += 1,
        }
    }
    let ok = counts.fail == 0;

    let report = json!({
        "ok": ok,
        "summary": {
            "pass": counts.pass,
            "fail": counts.fail,
            "warn": counts.warn,
            "info": counts.info,
            "skipped": counts.skipped,
        },
        "config": {
            "base_url": raw.base_url,
            "authentication_method": auth_method,
            "credential_sources": {
                "base_url": raw.base_url_source,
                "token": raw.token_source,
                "client_id": raw.client_id_source,
                "client_secret": raw.client_secret_source,
            },
        },
        "identity": {
            "team": team,
            "user": user,
        },
        "checks": checks,
    });

    // 人类可读清单到 stderr；stdout 保持纯 JSON 供 AI agent 解析。
    let mut err = std::io::stderr().lock();
    for check in checks {
        let (symbol, label) = match check.status {
            Status::Pass => ("✓", "PASS"),
            Status::Fail => ("✗", "FAIL"),
            Status::Warn => ("!", "WARN"),
            Status::Info => ("i", "INFO"),
            Status::Skipped => ("-", "SKIP"),
        };
        let _ = writeln!(err, "{symbol} [{label}] {} — {}", check.id, check.message);
        if check.status == Status::Fail {
            if let Some(remediation) = &check.remediation {
                let _ = writeln!(err, "  → {}:", remediation.title);
                for step in &remediation.steps {
                    let _ = writeln!(err, "    - {step}");
                }
            }
        }
    }
    let _ = writeln!(
        err,
        "doctor: {} passed, {} failed, {} warnings, {} info, {} skipped",
        counts.pass, counts.fail, counts.warn, counts.info, counts.skipped
    );

    crate::output::print_json(&report)?;

    Ok(if ok { 0 } else { 1 })
}

#[derive(Default)]
struct Summary {
    pass: u32,
    fail: u32,
    warn: u32,
    info: u32,
    skipped: u32,
}
