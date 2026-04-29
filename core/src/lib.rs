//! AJENTIC core library.
//!
//! Ledger, audit emission, replay, real provider adapters, and UI behavior
//! are reserved for later phases.

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

pub mod validation;
