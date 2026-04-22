use std::sync::Arc;

use mailify_auth::JwtIssuer;
use mailify_config::AppConfig;
use mailify_queue::QueueHandle;
use mailify_templates::TemplateRegistry;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<AppConfig>,
    pub registry: Arc<TemplateRegistry>,
    pub queue: QueueHandle,
    pub jwt: Arc<JwtIssuer>,
}
