//! CRDT module - Conflict-free Replicated Data Types
//!
//! This module defines the core traits and contracts for CRDT implementations.
//! Specific CRDT types will be implemented in submodules.

pub mod traits;

// Re-export key traits
pub use traits::{Convergent, Commutative, Idempotent};