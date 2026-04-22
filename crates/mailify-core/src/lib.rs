pub mod email;
pub mod error;
pub mod priority;
pub mod smtp_override;

pub use email::{Attachment, EmailAddress, EmailMessage, RenderedEmail};
pub use error::{CoreError, Result};
pub use priority::Priority;
pub use smtp_override::SmtpOverride;
