use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::routes;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_jwt",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some(
                        "Paste the `access_token` returned by `POST /auth/token` here.",
                    ))
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Mailify",
        description = "Rust mail server with React Email templates, priority queue, and per-request SMTP overrides.",
        version = "0.1.0",
        license(name = "MIT")
    ),
    paths(
        routes::health::health,
        routes::auth::issue_token,
        routes::templates::list,
        routes::templates::preview_get,
        routes::templates::preview_post,
        routes::config::get_config,
        routes::mail::send_registered,
        routes::mail::send_custom,
        routes::mail::get_job_state,
    ),
    components(schemas(
        // Domain types
        mailify_core::email::EmailAddress,
        mailify_core::email::Attachment,
        mailify_core::priority::Priority,
        mailify_core::smtp_override::SmtpOverride,
        mailify_core::smtp_override::TlsMode,
        // Route request/response
        routes::health::HealthResponse,
        routes::auth::IssueTokenRequest,
        routes::auth::IssueTokenResponse,
        routes::templates::TemplateInfo,
        routes::templates::PreviewBody,
        routes::templates::PreviewResponse,
        routes::config::SanitizedConfig,
        routes::config::ServerView,
        routes::config::DatabaseView,
        routes::config::SmtpView,
        routes::config::AuthView,
        routes::config::QueueView,
        routes::config::TemplatesView,
        routes::config::I18nView,
        routes::config::ObservabilityView,
        routes::mail::SendRegisteredRequest,
        routes::mail::SendCustomRequest,
        routes::mail::EnqueuedResponse,
        routes::mail::JobStateResponse,
    )),
    tags(
        (name = "auth", description = "JWT issuance"),
        (name = "mail", description = "Queue emails for delivery"),
        (name = "templates", description = "Template catalog + preview"),
        (name = "system", description = "Health + config introspection"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
