use mailify_core::{email::EmailAddress, priority::Priority};
use mailify_queue::job::{MailJob, MailJobKind};
use serde_json::json;

fn addr(e: &str) -> EmailAddress {
    EmailAddress {
        email: e.into(),
        name: None,
    }
}

#[test]
fn registered_job_roundtrips_through_json() {
    let job = MailJob {
        id: uuid::Uuid::nil(),
        priority: Priority::High,
        kind: MailJobKind::Registered {
            template_id: "welcome".into(),
        },
        from: addr("from@example.com"),
        to: vec![addr("to@example.com")],
        cc: vec![],
        bcc: vec![],
        reply_to: None,
        attachments: vec![],
        headers: Default::default(),
        locale: "fr".into(),
        vars: json!({ "name": "Alice" }),
        smtp_override: None,
        subject_override: Some("Custom".into()),
    };

    let s = serde_json::to_string(&job).unwrap();
    let back: MailJob = serde_json::from_str(&s).unwrap();
    assert_eq!(back.id, job.id);
    assert_eq!(back.priority, Priority::High);
    assert_eq!(back.locale, "fr");
    match back.kind {
        MailJobKind::Registered { template_id } => assert_eq!(template_id, "welcome"),
        _ => panic!("expected Registered"),
    }
}

#[test]
fn custom_job_serialization_preserves_kind() {
    let job = MailJob {
        id: uuid::Uuid::nil(),
        priority: Priority::Bulk,
        kind: MailJobKind::Custom {
            html: "<p>hi</p>".into(),
            subject: "subject".into(),
            text: Some("hi".into()),
        },
        from: addr("x@x"),
        to: vec![addr("y@y")],
        cc: vec![],
        bcc: vec![],
        reply_to: None,
        attachments: vec![],
        headers: Default::default(),
        locale: "en".into(),
        vars: serde_json::Value::Null,
        smtp_override: None,
        subject_override: None,
    };
    let s = serde_json::to_string(&job).unwrap();
    assert!(s.contains("\"type\":\"custom\""));
    let back: MailJob = serde_json::from_str(&s).unwrap();
    match back.kind {
        MailJobKind::Custom { html, .. } => assert_eq!(html, "<p>hi</p>"),
        _ => panic!("expected Custom"),
    }
}

#[test]
fn smtp_override_omitted_from_json_when_none() {
    let job = MailJob::new_registered("welcome", addr("f@f"), vec![addr("t@t")], "en");
    let s = serde_json::to_string(&job).unwrap();
    assert!(!s.contains("smtp_override"));
}
