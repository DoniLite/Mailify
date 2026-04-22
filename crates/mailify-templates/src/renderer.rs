use mailify_config::Theme;
use mailify_core::RenderedEmail;
use minijinja::{context, Environment};
use serde::Serialize;
use serde_json::Value;

use crate::registry::{TemplateAssets, TemplateRegistry};

#[derive(Debug, Clone, Serialize)]
pub struct RenderContext {
    pub theme: Theme,
    pub locale: String,
    pub vars: Value,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error(transparent)]
    Registry(#[from] crate::registry::TemplateRegistryError),
    #[error("minijinja render failed: {0}")]
    Render(String),
    #[error("missing subject (neither template asset nor caller-supplied)")]
    MissingSubject,
}

pub struct TemplateRenderer<'a> {
    env: Environment<'a>,
    pub registry: &'a TemplateRegistry,
}

impl<'a> TemplateRenderer<'a> {
    pub fn new(registry: &'a TemplateRegistry) -> Self {
        let mut env = Environment::new();
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);
        Self { env, registry }
    }

    /// Render a registered template by id + locale.
    pub fn render_registered(
        &self,
        id: &str,
        ctx: &RenderContext,
        explicit_subject: Option<&str>,
    ) -> Result<RenderedEmail, RenderError> {
        let assets = self.registry.get(id, &ctx.locale)?;
        self.render_assets(assets, ctx, explicit_subject)
    }

    /// Render caller-supplied raw HTML (one-shot, not persisted).
    pub fn render_raw(
        &self,
        html: &str,
        subject_template: &str,
        text: Option<&str>,
        ctx: &RenderContext,
    ) -> Result<RenderedEmail, RenderError> {
        let html_out = self.render_str(html, ctx)?;
        let subject = self.render_str(subject_template, ctx)?;
        let text_out = text.map(|t| self.render_str(t, ctx)).transpose()?;
        Ok(RenderedEmail {
            subject,
            html: html_out,
            text: text_out,
        })
    }

    fn render_assets(
        &self,
        assets: &TemplateAssets,
        ctx: &RenderContext,
        explicit_subject: Option<&str>,
    ) -> Result<RenderedEmail, RenderError> {
        let html = self.render_str(&assets.html, ctx)?;
        let subject_src = explicit_subject
            .map(str::to_owned)
            .or_else(|| assets.subject.clone())
            .ok_or(RenderError::MissingSubject)?;
        let subject = self.render_str(&subject_src, ctx)?;
        let text = assets
            .text
            .as_ref()
            .map(|t| self.render_str(t, ctx))
            .transpose()?;
        Ok(RenderedEmail {
            subject,
            html,
            text,
        })
    }

    fn render_str(&self, source: &str, ctx: &RenderContext) -> Result<String, RenderError> {
        let tmpl = self
            .env
            .template_from_str(source)
            .map_err(|e| RenderError::Render(e.to_string()))?;
        tmpl.render(context! {
            theme => &ctx.theme,
            locale => &ctx.locale,
            vars => &ctx.vars,
        })
        .map_err(|e| RenderError::Render(e.to_string()))
    }
}
