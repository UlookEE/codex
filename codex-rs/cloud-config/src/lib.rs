//! Cloud-hosted configuration data for Codex.
//!
//! This crate owns transport behavior for cloud-delivered config data. Bundles
//! are always fetched fresh from the backend (no on-disk cache). Parsing and
//! composition remain in `codex-config`.

mod backend;
mod bundle_loader;
mod metrics;
mod service;
mod validation;

pub use bundle_loader::cloud_config_bundle_loader;
pub use bundle_loader::cloud_config_bundle_loader_for_storage;
