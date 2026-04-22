use thiserror::Error;

pub type Result<T> = std::result::Result<T, CoreError>;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid email address: {0}")]
    InvalidEmailAddress(String),

    #[error("template not found: id={id} locale={locale}")]
    TemplateNotFound { id: String, locale: String },

    #[error("template render failed: {0}")]
    TemplateRender(String),

    #[error("smtp error: {0}")]
    Smtp(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("auth error: {0}")]
    Auth(String),

    #[error("queue error: {0}")]
    Queue(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("unexpected: {0}")]
    Unexpected(String),
}
