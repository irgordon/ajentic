//! Phase 8 Rust-owned audit record derivation from ledger facts only.

pub mod contract;
pub mod record;

pub use record::{build_audit_record, AuditFinalStatus, AuditRecord};
