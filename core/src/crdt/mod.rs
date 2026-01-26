//! CRDT module - Conflict-free Replicated Data Types
//!
//! This module defines the core traits and contracts for CRDT implementations.
//! Specific CRDT types will be implemented in submodules.

pub mod traits;

// Re-export key traits
pub use traits::{Crdt, Convergent, Commutative, Idempotent};

/// Marker trait for all CRDT implementations
///
/// This trait serves as a bound for generic CRDT operations
/// and ensures all implementations follow the required semantics.
pub trait Crdt: Convergent + Commutative + Idempotent {
    /// The type of values this CRDT represents
    type Value;

    /// The type of operations that can be applied to this CRDT
    type Operation;

    /// Create a new CRDT instance with initial value
    fn new(initial_value: Self::Value) -> Self;

    /// Apply an operation to this CRDT
    fn apply(&mut self, operation: Self::Operation) -> Result<(), crate::error::OpenGridError>;

    /// Read the current converged value
    fn read(&self) -> Self::Value;

    /// Get the current version/dot context
    fn version(&self) -> Vec<u8>;
}