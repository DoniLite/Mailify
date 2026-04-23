//! Persistent priority mail queue backed by Postgres via apalis.
//!
//! Jobs survive process restarts: apalis persists them in Postgres. On boot the worker picks up
//! every non-completed job and resumes processing. Priority is expressed via `Priority::weight()`
//! (lower = earlier). Workers are spawned with a configurable concurrency.

pub mod job;
pub mod worker;

pub use job::{MailJob, MailJobKind};
pub use worker::{JobSnapshot, QueueHandle, QueueRuntime};
