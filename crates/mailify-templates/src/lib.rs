//! Template registry + renderer.
//!
//! Directory layout expected:
//! ```text
//! <templates.path>/
//!   <template_id>/
//!     <locale>.html      # pre-built React Email output
//!     subject.<locale>.txt  # optional — subject line per locale (minijinja-rendered)
//!     text.<locale>.txt     # optional — plaintext alternative
//! ```

pub mod registry;
pub mod renderer;

pub use registry::{CatalogEntry, TemplateKey, TemplateRegistry, TemplateRegistryError};
pub use renderer::{RenderContext, TemplateRenderer};
