use mailify_config::{I18nConfig, Theme};
use mailify_templates::{RenderContext, TemplateRenderer};
use serde_json::json;

fn ctx(vars: serde_json::Value) -> RenderContext {
    RenderContext {
        theme: Theme::default(),
        locale: "en".into(),
        vars,
    }
}

fn empty_registry() -> mailify_templates::TemplateRegistry {
    mailify_templates::TemplateRegistry::empty(I18nConfig {
        default_locale: "en".into(),
        fallback_chain: vec!["en".into()],
        supported_locales: vec!["en".into()],
    })
}

#[test]
fn renders_raw_html_with_vars() {
    let reg = empty_registry();
    let renderer = TemplateRenderer::new(&reg);
    let out = renderer
        .render_raw(
            "<p>Hi {{ vars.name }}</p>",
            "Hello {{ vars.name }}",
            Some("Hi {{ vars.name }}"),
            &ctx(json!({ "name": "Alice" })),
        )
        .unwrap();
    assert_eq!(out.subject, "Hello Alice");
    assert!(out.html.contains("Hi Alice"));
    assert_eq!(out.text.as_deref(), Some("Hi Alice"));
}

#[test]
fn renders_theme_tokens() {
    let reg = empty_registry();
    let renderer = TemplateRenderer::new(&reg);
    let out = renderer
        .render_raw(
            "<p>color: {{ theme.colors.primary }}</p>",
            "s",
            None,
            &ctx(json!({})),
        )
        .unwrap();
    assert!(out.html.contains("color: #0f172a"));
}
