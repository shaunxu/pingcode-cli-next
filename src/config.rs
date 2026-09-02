use anyhow::{bail, Result};

use crate::cli::Cli;

/// Authentication method
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Use the provided access token directly
    Token(String),
    /// OAuth2 client-credentials flow; exchange client_id/client_secret for an enterprise token at runtime
    Client {
        client_id: String,
        client_secret: String,
    },
    /// No credentials available; only valid with --dry-run, which never sends a request
    Anonymous,
}

impl Credentials {
    /// User-facing name of the authentication method
    pub fn label(&self) -> &'static str {
        match self {
            Credentials::Token(_) => "access token",
            Credentials::Client { .. } => "client credentials (enterprise token)",
            Credentials::Anonymous => "anonymous (dry-run, no request is sent)",
        }
    }
}

/// Runtime configuration, merged from CLI arguments and environment variables
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub credentials: Credentials,
    pub verbose: bool,
    pub dry_run: bool,
}

const DEFAULT_BASE_URL: &str = "https://api.pingcode.com";

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        let base_url = cli
            .base_url
            .clone()
            .unwrap_or_else(|| DEFAULT_BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();

        if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
            bail!("base-url must start with http:// or https://, got: {base_url}");
        }

        let client_id = cli.client_id.clone().filter(|v| !v.trim().is_empty());
        let client_secret = cli.client_secret.clone().filter(|v| !v.trim().is_empty());

        let credentials = match (client_id, client_secret) {
            (Some(_), None) => {
                bail!("Client ID provided but Client Secret is missing: pass --client-secret or set PC_CLIENT_SECRET")
            }
            (None, Some(_)) => {
                bail!("Client Secret provided but Client ID is missing: pass --client-id or set PC_CLIENT_ID")
            }
            (Some(client_id), Some(client_secret)) => Credentials::Client {
                client_id,
                client_secret,
            },
            (None, None) => match cli.token.clone() {
                Some(token) if !token.trim().is_empty() => Credentials::Token(token),
                _ => {
                    // --dry-run 不会发起任何网络请求，因此允许离线运行而不提供凭据。
                    if cli.dry_run {
                        Credentials::Anonymous
                    } else {
                        bail!(
                            "Missing credentials: use the client-credentials flow via PC_CLIENT_ID and PC_CLIENT_SECRET \
                            (or --client-id/--client-secret), or provide an access token via --token / PC_TOKEN"
                        )
                    }
                }
            },
        };

        let config = Config {
            base_url,
            credentials,
            verbose: cli.verbose,
            dry_run: cli.dry_run,
        };
        Ok(config)
    }
}
