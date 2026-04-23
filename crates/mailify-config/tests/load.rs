//! AppConfig loader behavior: defaults, env override, nested keys.
//!
//! Mutating `std::env` is process-global — these tests run serially within this file via shared
//! state, but should not be split across threads with conflicting env mutations. Each test uses
//! a distinct env prefix to avoid cross-contamination.

use std::sync::Mutex;

use mailify_config::{AppConfig, LogFormat};
use mailify_core::smtp_override::TlsMode;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn clear_mailify_vars() {
    for (k, _) in std::env::vars() {
        if k.starts_with("MAILIFY_") {
            unsafe { std::env::remove_var(&k) };
        }
    }
    unsafe { std::env::set_var("MAILIFY_DOTENV", "false") };
}

#[test]
fn defaults_load_when_no_env() {
    let _g = ENV_LOCK.lock().unwrap();
    clear_mailify_vars();

    let cfg = AppConfig::load().expect("load defaults");
    assert_eq!(cfg.server.port, 8080);
    assert_eq!(cfg.i18n.default_locale, "en");
    assert_eq!(cfg.observability.log_format, LogFormat::Pretty);
    assert_eq!(cfg.smtp.tls, TlsMode::None);
}

#[test]
fn env_override_top_level_and_nested() {
    let _g = ENV_LOCK.lock().unwrap();
    clear_mailify_vars();
    unsafe {
        std::env::set_var("MAILIFY_SERVER__PORT", "9090");
        std::env::set_var("MAILIFY_SMTP__HOST", "smtp.example.com");
        std::env::set_var("MAILIFY_SMTP__TLS", "tls");
        std::env::set_var("MAILIFY_THEME__BRAND_NAME", "Acme");
        std::env::set_var("MAILIFY_OBSERVABILITY__LOG_FORMAT", "json");
    }

    let cfg = AppConfig::load().expect("load with env");
    assert_eq!(cfg.server.port, 9090);
    assert_eq!(cfg.smtp.host, "smtp.example.com");
    assert_eq!(cfg.smtp.tls, TlsMode::Tls);
    assert_eq!(cfg.theme.brand_name, "Acme");
    assert_eq!(cfg.observability.log_format, LogFormat::Json);
}

#[test]
fn api_keys_map_loaded_from_env() {
    let _g = ENV_LOCK.lock().unwrap();
    clear_mailify_vars();
    unsafe {
        std::env::set_var("MAILIFY_AUTH__API_KEYS__WEB", "$argon2id$fakehash1");
        std::env::set_var("MAILIFY_AUTH__API_KEYS__CLI", "$argon2id$fakehash2");
    }

    let cfg = AppConfig::load().expect("load");
    let keys: std::collections::HashSet<&str> =
        cfg.auth.api_keys.keys().map(String::as_str).collect();
    assert!(keys.contains("web"));
    assert!(keys.contains("cli"));
    assert_eq!(cfg.auth.api_keys["web"], "$argon2id$fakehash1");
}

#[test]
fn invalid_port_returns_error() {
    let _g = ENV_LOCK.lock().unwrap();
    clear_mailify_vars();
    unsafe { std::env::set_var("MAILIFY_SERVER__PORT", "not-a-number") };

    let err = AppConfig::load().expect_err("port must fail to parse");
    let s = err.to_string();
    assert!(s.contains("port") || s.to_lowercase().contains("invalid"));
}
