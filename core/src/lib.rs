//! AJENTIC core library.
//!
//! This crate defines the authoritative Rust surfaces for lifecycle,
//! governance, replay, normalization, validation, and verification.
//!
//! Some capabilities within these modules may be reserved for later
//! phases. Module presence does not imply full feature maturity or
//! production readiness.

pub mod audit;
pub mod candidate;
pub mod constraints;
pub mod domain;
pub mod errors;
pub mod evaluation;
pub mod execution;
pub mod governance;
pub mod ledger;
pub mod normalization;
pub mod objective;
pub mod policy;
pub mod replay;
pub mod reuse;

pub mod validation;
pub mod verification;
