use mailify_core::{
    email::{Attachment, EmailAddress},
    priority::Priority,
    smtp_override::SmtpOverride,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Dispatched unit-of-work persisted in Postgres via apalis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailJob {
    pub id: Uuid,
    pub priority: Priority,
    pub kind: MailJobKind,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    #[serde(default)]
    pub cc: Vec<EmailAddress>,
    #[serde(default)]
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    pub locale: String,
    /// Vars fed to the template renderer.
    #[serde(default)]
    pub vars: Value,
    /// Per-request SMTP config. **Never logged. Memory-only for job lifetime.**
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_override: Option<SmtpOverride>,
    /// Optional subject override (takes precedence over template's subject asset).
    pub subject_override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MailJobKind {
    /// Render a registered template by id.
    Registered { template_id: String },
    /// Render caller-supplied raw HTML (one-shot, not stored in registry).
    Custom {
        html: String,
        subject: String,
        text: Option<String>,
    },
}

impl MailJob {
    pub fn new_registered(
        template_id: impl Into<String>,
        from: EmailAddress,
        to: Vec<EmailAddress>,
        locale: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            priority: Priority::Normal,
            kind: MailJobKind::Registered { template_id: template_id.into() },
            from,
            to,
            cc: vec![],
            bcc: vec![],
            reply_to: None,
            attachments: vec![],
            headers: Default::default(),
            locale: locale.into(),
            vars: Value::Null,
            smtp_override: None,
            subject_override: None,
        }
    }
}
