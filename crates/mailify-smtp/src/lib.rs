use base64::Engine;
use lettre::{
    message::{header::ContentType, Attachment as LettreAttachment, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use mailify_config::SmtpConfig;
use mailify_core::{
    email::{Attachment, EmailAddress, RenderedEmail},
    smtp_override::{SmtpOverride, TlsMode},
};
use tracing::debug;

#[derive(Debug, thiserror::Error)]
pub enum SmtpError {
    #[error("lettre build error: {0}")]
    Build(String),
    #[error("lettre transport error: {0}")]
    Transport(String),
    #[error("invalid address: {0}")]
    Address(String),
    #[error("base64 decode failed: {0}")]
    Base64(String),
}

/// Recipients for a send call. Kept separate from the rendered body so the queue can carry
/// routing info alongside the rendered content.
#[derive(Debug, Clone)]
pub struct Envelope {
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Vec<EmailAddress>,
    pub bcc: Vec<EmailAddress>,
    pub reply_to: Option<EmailAddress>,
    pub headers: std::collections::HashMap<String, String>,
    pub attachments: Vec<Attachment>,
}

pub struct SmtpSender {
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpSender {
    pub fn from_config(cfg: &SmtpConfig) -> Result<Self, SmtpError> {
        let override_ = SmtpOverride {
            host: cfg.host.clone(),
            port: cfg.port,
            username: cfg.username.clone(),
            password: cfg.password.clone(),
            tls: cfg.tls,
            timeout_secs: Some(cfg.timeout_secs),
        };
        Self::from_override(&override_)
    }

    pub fn from_override(ov: &SmtpOverride) -> Result<Self, SmtpError> {
        let builder = match ov.tls {
            TlsMode::None => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&ov.host),
            TlsMode::StartTls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&ov.host)
                .map_err(|e| SmtpError::Build(e.to_string()))?,
            TlsMode::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&ov.host)
                .map_err(|e| SmtpError::Build(e.to_string()))?,
        };

        let mut builder = builder.port(ov.port);

        if let Some(timeout) = ov.timeout_secs {
            builder = builder.timeout(Some(std::time::Duration::from_secs(timeout)));
        }

        if let (Some(u), Some(p)) = (ov.username.as_ref(), ov.password.as_ref()) {
            builder = builder.credentials(Credentials::new(u.clone(), p.clone()));
        }

        Ok(Self {
            transport: builder.build(),
        })
    }

    pub async fn send(
        &self,
        envelope: &Envelope,
        rendered: &RenderedEmail,
    ) -> Result<(), SmtpError> {
        let msg = build_message(envelope, rendered)?;
        debug!(to = ?envelope.to.iter().map(|a| &a.email).collect::<Vec<_>>(), subject = %rendered.subject, "dispatching email");
        self.transport
            .send(msg)
            .await
            .map_err(|e| SmtpError::Transport(e.to_string()))?;
        Ok(())
    }
}

fn build_message(envelope: &Envelope, rendered: &RenderedEmail) -> Result<Message, SmtpError> {
    let mut builder = Message::builder().subject(&rendered.subject);

    builder = builder.from(parse_mailbox(&envelope.from)?);
    for t in &envelope.to {
        builder = builder.to(parse_mailbox(t)?);
    }
    for c in &envelope.cc {
        builder = builder.cc(parse_mailbox(c)?);
    }
    for b in &envelope.bcc {
        builder = builder.bcc(parse_mailbox(b)?);
    }
    if let Some(r) = &envelope.reply_to {
        builder = builder.reply_to(parse_mailbox(r)?);
    }

    let mut html_alt = MultiPart::alternative().singlepart(
        SinglePart::builder().header(ContentType::TEXT_PLAIN).body(
            rendered
                .text
                .clone()
                .unwrap_or_else(|| fallback_text(&rendered.html)),
        ),
    );
    html_alt = html_alt.singlepart(
        SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(rendered.html.clone()),
    );

    let body = if envelope.attachments.is_empty() {
        MultiPart::mixed().multipart(html_alt)
    } else {
        let mut mixed = MultiPart::mixed().multipart(html_alt);
        for att in &envelope.attachments {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(&att.content_base64)
                .map_err(|e| SmtpError::Base64(e.to_string()))?;
            let ct: ContentType = att.content_type.parse().map_err(|_| {
                SmtpError::Build(format!("invalid content-type: {}", att.content_type))
            })?;
            let part = match &att.inline_cid {
                Some(cid) => LettreAttachment::new_inline(cid.clone()).body(decoded, ct),
                None => LettreAttachment::new(att.filename.clone()).body(decoded, ct),
            };
            mixed = mixed.singlepart(part);
        }
        mixed
    };

    let mut message = builder
        .multipart(body)
        .map_err(|e| SmtpError::Build(e.to_string()))?;

    // Custom raw headers applied post-build.
    for (k, v) in &envelope.headers {
        let name = lettre::message::header::HeaderName::new_from_ascii(k.clone())
            .map_err(|e| SmtpError::Build(e.to_string()))?;
        let raw = lettre::message::header::HeaderValue::new(name, v.clone());
        message.headers_mut().insert_raw(raw);
    }

    Ok(message)
}

fn parse_mailbox(addr: &EmailAddress) -> Result<lettre::message::Mailbox, SmtpError> {
    match &addr.name {
        Some(name) => format!("{} <{}>", name, addr.email)
            .parse()
            .map_err(|e: lettre::address::AddressError| SmtpError::Address(e.to_string())),
        None => addr
            .email
            .parse()
            .map_err(|e: lettre::address::AddressError| SmtpError::Address(e.to_string())),
    }
}

/// Naive HTML → text fallback (strips tags). Good enough for deliverability; not a full converter.
fn fallback_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use mailify_core::email::EmailAddress;

    #[test]
    fn fallback_text_strips_tags_and_collapses_whitespace() {
        let html = "<p>Hello <b>world</b></p>\n<p>   Second </p>";
        assert_eq!(fallback_text(html), "Hello world Second");
    }

    #[test]
    fn fallback_text_handles_empty_input() {
        assert_eq!(fallback_text(""), "");
        assert_eq!(fallback_text("<p></p>"), "");
    }

    #[test]
    fn parse_mailbox_plain_address() {
        let a = EmailAddress {
            email: "alice@example.com".into(),
            name: None,
        };
        let mb = parse_mailbox(&a).expect("parse");
        assert_eq!(mb.email.to_string(), "alice@example.com");
    }

    #[test]
    fn parse_mailbox_with_display_name() {
        let a = EmailAddress {
            email: "bob@example.com".into(),
            name: Some("Bob Smith".into()),
        };
        let mb = parse_mailbox(&a).expect("parse");
        let formatted = mb.to_string();
        assert!(formatted.contains("Bob Smith"));
        assert!(formatted.contains("bob@example.com"));
    }

    #[test]
    fn parse_mailbox_rejects_invalid() {
        let a = EmailAddress {
            email: "not-an-email".into(),
            name: None,
        };
        assert!(parse_mailbox(&a).is_err());
    }

    #[test]
    fn build_message_sets_subject_and_recipients() {
        use mailify_core::email::RenderedEmail;

        let envelope = Envelope {
            from: EmailAddress {
                email: "from@example.com".into(),
                name: None,
            },
            to: vec![EmailAddress {
                email: "to@example.com".into(),
                name: None,
            }],
            cc: vec![],
            bcc: vec![],
            reply_to: None,
            headers: Default::default(),
            attachments: vec![],
        };
        let rendered = RenderedEmail {
            subject: "Test Subject".into(),
            html: "<p>hi</p>".into(),
            text: Some("hi".into()),
        };
        let msg = build_message(&envelope, &rendered).expect("build");
        let raw = String::from_utf8_lossy(&msg.formatted()).to_string();
        assert!(raw.contains("Subject: Test Subject"));
        assert!(raw.contains("to@example.com"));
        assert!(raw.contains("from@example.com"));
    }

    #[test]
    fn build_message_includes_custom_headers() {
        use mailify_core::email::RenderedEmail;

        let mut headers = std::collections::HashMap::new();
        headers.insert("X-Mailify-Tag".to_string(), "welcome".to_string());

        let envelope = Envelope {
            from: EmailAddress {
                email: "from@example.com".into(),
                name: None,
            },
            to: vec![EmailAddress {
                email: "to@example.com".into(),
                name: None,
            }],
            cc: vec![],
            bcc: vec![],
            reply_to: None,
            headers,
            attachments: vec![],
        };
        let rendered = RenderedEmail {
            subject: "s".into(),
            html: "<p>x</p>".into(),
            text: None,
        };
        let msg = build_message(&envelope, &rendered).expect("build");
        let raw = String::from_utf8_lossy(&msg.formatted()).to_string();
        assert!(raw.contains("X-Mailify-Tag: welcome"));
    }
}
