//! Central configuration loader.
//!
//! Precedence (lowest → highest): defaults → optional TOML file (`MAILIFY_CONFIG` env) → env vars (`MAILIFY_*`).

pub mod theme;

use std::path::PathBuf;

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use mailify_core::smtp_override::TlsMode;
use serde::{Deserialize, Serialize};

pub use theme::Theme;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("figment error: {0}")]
    Figment(#[from] Box<figment::Error>),
}

impl From<figment::Error> for ConfigError {
    fn from(e: figment::Error) -> Self {
        Self::Figment(Box::new(e))
    }
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub smtp: SmtpConfig,
    pub auth: AuthConfig,
    pub queue: QueueConfig,
    pub templates: TemplatesConfig,
    pub theme: Theme,
    pub i18n: I18nConfig,
    pub observability: ObservabilityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
    pub body_limit_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub tls: TlsMode,
    pub default_from_email: String,
    pub default_from_name: Option<String>,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_ttl_secs: u64,
    /// API keys (argon2 hashes), key = identifier, value = hash.
    #[serde(default)]
    pub api_keys: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub worker_concurrency: usize,
    pub max_retries: usize,
    pub retry_backoff_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatesConfig {
    /// Directory containing compiled React Email HTML files (one subdir per template).
    pub path: PathBuf,
    /// Strict mode: fail startup if any built-in template id is missing for default locale.
    pub strict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nConfig {
    pub default_locale: String,
    pub fallback_chain: Vec<String>,
    pub supported_locales: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservabilityConfig {
    pub log_level: String,
    pub log_format: LogFormat,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Pretty,
    Json,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".into(),
                port: 8080,
                request_timeout_secs: 30,
                body_limit_bytes: 10 * 1024 * 1024,
            },
            database: DatabaseConfig {
                url: "postgres://mailify:mailify@localhost:5432/mailify".into(),
                max_connections: 10,
                min_connections: 1,
            },
            smtp: SmtpConfig {
                host: "localhost".into(),
                port: 1025,
                username: None,
                password: None,
                tls: TlsMode::None,
                default_from_email: "no-reply@mailify.local".into(),
                default_from_name: Some("Mailify".into()),
                timeout_secs: 30,
            },
            auth: AuthConfig {
                jwt_secret: "CHANGE_ME_IN_PRODUCTION".into(),
                jwt_issuer: "mailify".into(),
                jwt_ttl_secs: 3600,
                api_keys: Default::default(),
            },
            queue: QueueConfig {
                worker_concurrency: 4,
                max_retries: 5,
                retry_backoff_secs: 30,
            },
            templates: TemplatesConfig {
                path: PathBuf::from("./templates-parser/out"),
                strict: false,
            },
            theme: Theme::default(),
            i18n: I18nConfig {
                default_locale: "en".into(),
                fallback_chain: vec!["en".into()],
                supported_locales: vec!["en".into(), "fr".into()],
            },
            observability: ObservabilityConfig {
                log_level: "info".into(),
                log_format: LogFormat::Pretty,
            },
        }
    }
}

impl AppConfig {
    /// Load config: defaults → optional TOML → env vars prefixed `MAILIFY_` (nested via `__`).
    pub fn load() -> Result<Self> {
        let mut fig = Figment::from(Serialized::defaults(AppConfig::default()));

        if let Ok(path) = std::env::var("MAILIFY_CONFIG") {
            fig = fig.merge(Toml::file(path));
        }

        fig = fig.merge(Env::prefixed("MAILIFY_").split("__"));

        let cfg: AppConfig = fig.extract()?;
        Ok(cfg)
    }
}
