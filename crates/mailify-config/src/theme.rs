use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Theme tokens passed to templates as `{{ theme.* }}` variables. Match Tailwind-style keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub brand_name: String,
    pub brand_logo_url: Option<String>,
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub radius: String,
    pub footer_text: Option<String>,
    pub social_links: HashMap<String, String>,
    /// Free-form extra tokens forwarded to templates as `{{ theme.extra.<key> }}`.
    #[serde(default)]
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    pub primary: String,
    pub primary_foreground: String,
    pub secondary: String,
    pub secondary_foreground: String,
    pub background: String,
    pub foreground: String,
    pub muted: String,
    pub border: String,
    pub danger: String,
    pub success: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFonts {
    pub body: String,
    pub heading: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            brand_name: "Mailify".into(),
            brand_logo_url: None,
            colors: ThemeColors {
                primary: "#0f172a".into(),
                primary_foreground: "#f8fafc".into(),
                secondary: "#e2e8f0".into(),
                secondary_foreground: "#0f172a".into(),
                background: "#ffffff".into(),
                foreground: "#0f172a".into(),
                muted: "#64748b".into(),
                border: "#e2e8f0".into(),
                danger: "#dc2626".into(),
                success: "#16a34a".into(),
            },
            fonts: ThemeFonts {
                body: "ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, sans-serif".into(),
                heading: "ui-sans-serif, system-ui, -apple-system, Segoe UI, Roboto, sans-serif".into(),
            },
            radius: "8px".into(),
            footer_text: None,
            social_links: Default::default(),
            extra: Default::default(),
        }
    }
}
