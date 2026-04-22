use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::{CoreError, Result};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmailAddress {
    #[validate(email)]
    pub email: String,
    pub name: Option<String>,
}

impl EmailAddress {
    pub fn new(email: impl Into<String>) -> Result<Self> {
        let e = Self {
            email: email.into(),
            name: None,
        };
        e.validate()
            .map_err(|err| CoreError::InvalidEmailAddress(err.to_string()))?;
        Ok(e)
    }

    pub fn with_name(email: impl Into<String>, name: impl Into<String>) -> Result<Self> {
        let e = Self {
            email: email.into(),
            name: Some(name.into()),
        };
        e.validate()
            .map_err(|err| CoreError::InvalidEmailAddress(err.to_string()))?;
        Ok(e)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    /// Base64-encoded payload.
    pub content_base64: String,
    pub inline_cid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailMessage {
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    #[serde(default)]
    pub cc: Vec<EmailAddress>,
    #[serde(default)]
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct RenderedEmail {
    pub subject: String,
    pub html: String,
    pub text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_address_accepts_well_formed_email() {
        let a = EmailAddress::new("user@example.com").unwrap();
        assert_eq!(a.email, "user@example.com");
    }

    #[test]
    fn invalid_address_rejected() {
        let err = EmailAddress::new("not-an-email").unwrap_err();
        assert!(matches!(err, CoreError::InvalidEmailAddress(_)));
    }
}
