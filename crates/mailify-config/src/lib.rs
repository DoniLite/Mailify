//! Central configuration loader.
//!
//! Precedence (lowest → highest): defaults → auto-discovered TOML file → env vars (`MAILIFY_*`).
//!
//! TOML discovery (first match wins): `$MAILIFY_CONFIG` → `./Mailify.toml` →
//! `$XDG_CONFIG_HOME/mailify/config.toml` (fallback `$HOME/.config/mailify/config.toml` on Unix,
//! `%APPDATA%\mailify\config.toml` on Windows) → `/etc/mailify/config.toml` (Unix).

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
    /// API keys (argon2 hashes), key = identifier, value = hash.    make hash-key KEY=CHANGE_ME_IN_PRODUCTION
    #[serde(default)]
    pub api_keys: std::collections::HashMap<String, String>,
    /// When true (default) and `api_keys` is empty at boot, the server generates
    /// an ephemeral API key, prints the plaintext + the env line to paste, and keeps
    /// the hash in memory for the current session. Set `false` to disable (e.g. when
    /// intentionally running with no keys).
    #[serde(default = "default_true")]
    pub bootstrap: bool,
}

fn default_true() -> bool {
    true
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
                bootstrap: true,
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
    /// Load config: dotenv → defaults → auto-discovered TOML → env vars prefixed `MAILIFY_` (nested via `__`).
    ///
    /// TOML discovery order (first found wins):
    ///   1. `MAILIFY_CONFIG` env var (explicit file path)
    ///   2. `./Mailify.toml` in current working directory
    ///   3. `$XDG_CONFIG_HOME/mailify/config.toml` (or `$HOME/.config/mailify/config.toml` on Unix,
    ///      `%APPDATA%\mailify\config.toml` on Windows)
    ///   4. `/etc/mailify/config.toml` (Unix system-wide, last resort)
    ///
    /// Dotenv loading order (first found wins, does not override already-set vars):
    ///   1. `MAILIFY_DOTENV_PATH` (explicit file)
    ///   2. `.env.<MAILIFY_ENV>.local`
    ///   3. `.env.<MAILIFY_ENV>`
    ///   4. `.env.local`
    ///   5. `.env`
    ///
    /// Disable dotenv by setting `MAILIFY_DOTENV=false`.
    pub fn load() -> Result<Self> {
        load_dotenv();

        let mut fig = Figment::from(Serialized::defaults(AppConfig::default()));

        if let Some(path) = discover_config_file() {
            fig = fig.merge(Toml::file(path));
        }

        fig = fig.merge(Env::prefixed("MAILIFY_").split("__"));

        let cfg: AppConfig = fig.extract()?;
        Ok(cfg)
    }
}

/// Resolve the TOML config file to load. Returns `None` when no candidate exists.
///
/// Searches in this order, returning the first existing path:
///   1. `$MAILIFY_CONFIG`
///   2. `./Mailify.toml`
///   3. Per-user config dir: `$XDG_CONFIG_HOME/mailify/config.toml`, else
///      `$HOME/.config/mailify/config.toml` on Unix, `%APPDATA%\mailify\config.toml` on Windows
///   4. `/etc/mailify/config.toml` on Unix
fn discover_config_file() -> Option<PathBuf> {
    config_candidates().into_iter().find(|c| c.is_file())
}

fn config_candidates() -> Vec<PathBuf> {
    let mut out = Vec::new();

    if let Ok(explicit) = std::env::var("MAILIFY_CONFIG") {
        out.push(PathBuf::from(explicit));
    }

    out.push(PathBuf::from("Mailify.toml"));

    if let Some(user_dir) = user_config_dir() {
        out.push(user_dir.join("mailify").join("config.toml"));
    }

    #[cfg(unix)]
    out.push(PathBuf::from("/etc/mailify/config.toml"));

    out
}

fn user_config_dir() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Some(PathBuf::from(xdg));
        }
    }

    #[cfg(unix)]
    {
        if let Ok(home) = std::env::var("HOME") {
            if !home.is_empty() {
                return Some(PathBuf::from(home).join(".config"));
            }
        }
    }

    #[cfg(windows)]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            if !appdata.is_empty() {
                return Some(PathBuf::from(appdata));
            }
        }
    }

    None
}

fn load_dotenv() {
    if std::env::var("MAILIFY_DOTENV")
        .map(|v| v.eq_ignore_ascii_case("false") || v == "0")
        .unwrap_or(false)
    {
        return;
    }

    if let Ok(explicit) = std::env::var("MAILIFY_DOTENV_PATH") {
        if let Err(e) = load_dotenv_from_file(&explicit) {
            tracing::warn!(path = %explicit, error = %e, "MAILIFY_DOTENV_PATH failed");
        } else {
            tracing::info!(path = %explicit, "loaded dotenv from MAILIFY_DOTENV_PATH");
        }
        return;
    }

    let env_name = std::env::var("MAILIFY_ENV").unwrap_or_else(|_| "development".to_string());
    let candidates = [
        format!(".env.{env_name}.local"),
        format!(".env.{env_name}"),
        ".env.local".to_string(),
        ".env".to_string(),
        "../../.env".to_string(),
    ];
    for file in &candidates {
        if let Ok(()) = load_dotenv_from_file(file) {
            tracing::debug!(file = %file, "dotenv loaded");
            break;
        }
    }
}

fn load_dotenv_from_file(path: &str) -> std::result::Result<(), String> {
    use std::fs;

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    for (i, raw) in content.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(2, '=');
        let key = parts
            .next()
            .ok_or_else(|| format!("invalid dotenv line {}", i + 1))?
            .trim();
        let mut val = parts
            .next()
            .ok_or_else(|| format!("invalid dotenv line {}", i + 1))?
            .trim()
            .to_string();

        // Strip surrounding single or double quotes if present (preserve inner $)
        if val.len() >= 2 {
            let bytes = val.as_bytes();
            if (bytes[0] == b'"' && bytes[val.len() - 1] == b'"')
                || (bytes[0] == b'\'' && bytes[val.len() - 1] == b'\'')
            {
                val = val[1..val.len() - 1].to_string();
            }
        }

        std::env::set_var(key, val);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidates_include_cwd_and_explicit() {
        // SAFETY: test-only env mutation; other discovery tests are skipped to avoid races.
        let prev = std::env::var("MAILIFY_CONFIG").ok();
        unsafe {
            std::env::set_var("MAILIFY_CONFIG", "/custom/path.toml");
        }
        let c = config_candidates();
        assert_eq!(
            c.first().map(PathBuf::as_path),
            Some(std::path::Path::new("/custom/path.toml"))
        );
        assert!(c.iter().any(|p| p == std::path::Path::new("Mailify.toml")));

        match prev {
            Some(v) => unsafe { std::env::set_var("MAILIFY_CONFIG", v) },
            None => unsafe { std::env::remove_var("MAILIFY_CONFIG") },
        }
    }

    #[test]
    fn user_config_dir_prefers_xdg() {
        let prev = std::env::var("XDG_CONFIG_HOME").ok();
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", "/xdg/override");
        }
        assert_eq!(user_config_dir(), Some(PathBuf::from("/xdg/override")));
        match prev {
            Some(v) => unsafe { std::env::set_var("XDG_CONFIG_HOME", v) },
            None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
        }
    }
}
