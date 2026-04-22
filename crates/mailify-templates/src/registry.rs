use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use mailify_config::I18nConfig;
use tracing::{debug, warn};

#[derive(Debug, thiserror::Error)]
pub enum TemplateRegistryError {
    #[error("io error reading templates dir {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("template not found: id={id} locale={locale}")]
    NotFound { id: String, locale: String },
    #[error("no default locale available for template {0}")]
    MissingDefaultLocale(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateKey {
    pub id: String,
    pub locale: String,
}

#[derive(Debug, Clone)]
pub struct TemplateAssets {
    pub html: String,
    pub subject: Option<String>,
    pub text: Option<String>,
}

/// In-memory registry of compiled templates. Populated at boot.
#[derive(Debug)]
pub struct TemplateRegistry {
    inner: HashMap<TemplateKey, TemplateAssets>,
    i18n: I18nConfig,
}

impl TemplateRegistry {
    pub fn empty(i18n: I18nConfig) -> Self {
        Self { inner: HashMap::new(), i18n }
    }

    /// Scan filesystem, load every `<id>/<locale>.html`. Subject/text optional siblings.
    pub fn load_from_dir(
        root: &Path,
        i18n: I18nConfig,
        strict: bool,
    ) -> Result<Self, TemplateRegistryError> {
        let mut registry = Self::empty(i18n.clone());

        let entries = fs::read_dir(root).map_err(|source| TemplateRegistryError::Io {
            path: root.to_path_buf(),
            source,
        })?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let Some(id) = path.file_name().and_then(|s| s.to_str()).map(str::to_owned) else {
                continue;
            };

            let sub_entries = fs::read_dir(&path).map_err(|source| TemplateRegistryError::Io {
                path: path.clone(),
                source,
            })?;

            for sub in sub_entries.flatten() {
                let sub_path = sub.path();
                let Some(fname) = sub_path.file_name().and_then(|s| s.to_str()) else {
                    continue;
                };
                let Some(locale) = fname.strip_suffix(".html") else {
                    continue;
                };

                let html = fs::read_to_string(&sub_path).map_err(|source| {
                    TemplateRegistryError::Io { path: sub_path.clone(), source }
                })?;

                let subject_path = path.join(format!("subject.{locale}.txt"));
                let subject = fs::read_to_string(&subject_path).ok().map(|s| s.trim().to_owned());

                let text_path = path.join(format!("text.{locale}.txt"));
                let text = fs::read_to_string(&text_path).ok();

                debug!(template = %id, locale = %locale, "loaded template");
                registry.inner.insert(
                    TemplateKey { id: id.clone(), locale: locale.to_owned() },
                    TemplateAssets { html, subject, text },
                );
            }

            if strict {
                let default = &i18n.default_locale;
                let has_default = registry
                    .inner
                    .contains_key(&TemplateKey { id: id.clone(), locale: default.clone() });
                if !has_default {
                    return Err(TemplateRegistryError::MissingDefaultLocale(id));
                }
            }
        }

        Ok(registry)
    }

    /// Resolve the best-matching asset for a requested locale using the configured fallback chain.
    pub fn get(&self, id: &str, requested_locale: &str) -> Result<&TemplateAssets, TemplateRegistryError> {
        let chain = self.fallback_chain(requested_locale);
        for locale in &chain {
            if let Some(assets) = self.inner.get(&TemplateKey {
                id: id.to_owned(),
                locale: locale.clone(),
            }) {
                return Ok(assets);
            }
        }
        warn!(template = %id, requested = %requested_locale, ?chain, "template not found");
        Err(TemplateRegistryError::NotFound {
            id: id.to_owned(),
            locale: requested_locale.to_owned(),
        })
    }

    pub fn list_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self.inner.keys().map(|k| k.id.clone()).collect();
        ids.sort();
        ids.dedup();
        ids
    }

    pub fn list(&self) -> Vec<TemplateKey> {
        self.inner.keys().cloned().collect()
    }

    fn fallback_chain(&self, requested: &str) -> Vec<String> {
        let mut chain = vec![requested.to_owned()];
        if let Some((base, _)) = requested.split_once('-') {
            chain.push(base.to_owned());
        }
        for loc in &self.i18n.fallback_chain {
            if !chain.contains(loc) {
                chain.push(loc.clone());
            }
        }
        if !chain.contains(&self.i18n.default_locale) {
            chain.push(self.i18n.default_locale.clone());
        }
        chain
    }
}
