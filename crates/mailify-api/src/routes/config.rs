use std::sync::Arc;

use axum::{extract::State, Json};
use mailify_config::{AppConfig, LogFormat};
use mailify_core::smtp_override::TlsMode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::state::AppState;

/// Sanitized view of the loaded AppConfig. **Never** returns secrets:
/// - `smtp.password` → redacted (only indicates presence)
/// - `auth.jwt_secret` → redacted (only indicates length)
/// - `auth.api_keys` → key IDs only; argon2 hashes never exposed.
#[derive(Debug, Serialize, ToSchema)]
pub struct SanitizedConfig {
    pub server: ServerView,
    pub database: DatabaseView,
    pub smtp: SmtpView,
    pub auth: AuthView,
    pub queue: QueueView,
    pub templates: TemplatesView,
    /// Tailwind/brand tokens — free-form JSON so the docs don't drift when the theme struct changes.
    #[schema(value_type = Object)]
    pub theme: serde_json::Value,
    pub i18n: I18nView,
    pub observability: ObservabilityView,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerView {
    pub host: String,
    pub port: u16,
    pub request_timeout_secs: u64,
    pub body_limit_bytes: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DatabaseView {
    pub url_redacted: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SmtpView {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password_set: bool,
    pub tls: TlsMode,
    pub default_from_email: String,
    pub default_from_name: Option<String>,
    pub timeout_secs: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthView {
    pub jwt_issuer: String,
    pub jwt_ttl_secs: u64,
    pub jwt_secret_length: usize,
    pub api_key_ids: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct QueueView {
    pub worker_concurrency: usize,
    pub max_retries: usize,
    pub retry_backoff_secs: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplatesView {
    pub path: String,
    pub strict: bool,
    pub loaded_count: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct I18nView {
    pub default_locale: String,
    pub fallback_chain: Vec<String>,
    pub supported_locales: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ObservabilityView {
    pub log_level: String,
    #[schema(value_type = String)]
    pub log_format: LogFormat,
}

/// Inspect loaded runtime configuration. Secrets are redacted:
/// passwords/JWT secret → presence/length only; API keys → IDs only; DB URL → credentials masked.
#[utoipa::path(
    get,
    path = "/config",
    tag = "system",
    security(("bearer_jwt" = [])),
    responses((status = 200, description = "Sanitized config", body = SanitizedConfig))
)]
pub async fn get_config(State(state): State<Arc<AppState>>) -> Json<SanitizedConfig> {
    Json(sanitize(&state.cfg, state.registry.list_ids().len()))
}

fn sanitize(cfg: &AppConfig, loaded_templates: usize) -> SanitizedConfig {
    SanitizedConfig {
        server: ServerView {
            host: cfg.server.host.clone(),
            port: cfg.server.port,
            request_timeout_secs: cfg.server.request_timeout_secs,
            body_limit_bytes: cfg.server.body_limit_bytes,
        },
        database: DatabaseView {
            url_redacted: redact_db_url(&cfg.database.url),
            max_connections: cfg.database.max_connections,
            min_connections: cfg.database.min_connections,
        },
        smtp: SmtpView {
            host: cfg.smtp.host.clone(),
            port: cfg.smtp.port,
            username: cfg.smtp.username.clone(),
            password_set: cfg.smtp.password.as_ref().is_some_and(|s| !s.is_empty()),
            tls: cfg.smtp.tls,
            default_from_email: cfg.smtp.default_from_email.clone(),
            default_from_name: cfg.smtp.default_from_name.clone(),
            timeout_secs: cfg.smtp.timeout_secs,
        },
        auth: AuthView {
            jwt_issuer: cfg.auth.jwt_issuer.clone(),
            jwt_ttl_secs: cfg.auth.jwt_ttl_secs,
            jwt_secret_length: cfg.auth.jwt_secret.len(),
            api_key_ids: cfg.auth.api_keys.keys().cloned().collect(),
        },
        queue: QueueView {
            worker_concurrency: cfg.queue.worker_concurrency,
            max_retries: cfg.queue.max_retries,
            retry_backoff_secs: cfg.queue.retry_backoff_secs,
        },
        templates: TemplatesView {
            path: cfg.templates.path.display().to_string(),
            strict: cfg.templates.strict,
            loaded_count: loaded_templates,
        },
        theme: serde_json::to_value(&cfg.theme).unwrap_or(serde_json::Value::Null),
        i18n: I18nView {
            default_locale: cfg.i18n.default_locale.clone(),
            fallback_chain: cfg.i18n.fallback_chain.clone(),
            supported_locales: cfg.i18n.supported_locales.clone(),
        },
        observability: ObservabilityView {
            log_level: cfg.observability.log_level.clone(),
            log_format: cfg.observability.log_format,
        },
    }
}

/// Strip credentials from a `postgres://user:pass@host:port/db` URL.
fn redact_db_url(url: &str) -> String {
    match url::Url::parse(url) {
        Ok(mut u) => {
            if !u.username().is_empty() {
                let _ = u.set_username("***");
            }
            if u.password().is_some() {
                let _ = u.set_password(Some("***"));
            }
            u.to_string()
        }
        Err(_) => "***".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_db_url_credentials() {
        let out = redact_db_url("postgres://user:secret@db.internal:5432/mailify");
        assert!(out.contains("***:***@"));
        assert!(!out.contains("secret"));
        assert!(!out.contains("user"));
    }

    #[test]
    fn redacts_unparseable_url_fully() {
        assert_eq!(redact_db_url("not-a-url"), "***");
    }
}
