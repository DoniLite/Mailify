use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Per-request SMTP override. **Never persisted.** Held in memory for a single job lifetime.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SmtpOverride {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_tls")]
    pub tls: TlsMode,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum TlsMode {
    None,
    StartTls,
    Tls,
}

fn default_tls() -> TlsMode {
    TlsMode::StartTls
}

impl std::fmt::Display for TlsMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TlsMode::None => f.write_str("none"),
            TlsMode::StartTls => f.write_str("starttls"),
            TlsMode::Tls => f.write_str("tls"),
        }
    }
}
