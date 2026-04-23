use mailify_core::smtp_override::{SmtpOverride, TlsMode};

#[test]
fn tls_mode_display() {
    assert_eq!(TlsMode::None.to_string(), "none");
    assert_eq!(TlsMode::StartTls.to_string(), "starttls");
    assert_eq!(TlsMode::Tls.to_string(), "tls");
}

#[test]
fn tls_mode_serialization_lowercase() {
    assert_eq!(serde_json::to_string(&TlsMode::StartTls).unwrap(), "\"starttls\"");
    let back: TlsMode = serde_json::from_str("\"tls\"").unwrap();
    assert_eq!(back, TlsMode::Tls);
}

#[test]
fn smtp_override_default_tls_is_starttls_when_field_missing() {
    let s: SmtpOverride =
        serde_json::from_str(r#"{"host":"smtp.example.com","port":587}"#).expect("deserialize");
    assert_eq!(s.tls, TlsMode::StartTls);
}

#[test]
fn smtp_override_roundtrip() {
    let s = SmtpOverride {
        host: "smtp.example.com".into(),
        port: 465,
        username: Some("user".into()),
        password: Some("pass".into()),
        tls: TlsMode::Tls,
        timeout_secs: Some(30),
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: SmtpOverride = serde_json::from_str(&json).unwrap();
    assert_eq!(back.host, s.host);
    assert_eq!(back.port, s.port);
    assert_eq!(back.tls, s.tls);
}
