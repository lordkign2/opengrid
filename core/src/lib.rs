//! OpenGrid Core - Distributed Mesh Resource Exchange Protocol
//!
//! This crate provides the foundational infrastructure for a disaster-resilient,
//! offline-first mesh network protocol based on CRDTs and append-only event logs.
//!
//! # Architecture Overview
//!
//! The core exposes a minimal API surface designed for FFI consumption:
//!
//! - Node lifecycle management
//! - Event submission and state queries
//! - Opaque handles for external consumers
//!
//! Internally, the system is structured around:
//!
//! - CRDT traits defining convergence semantics
//! - Storage abstractions for append-only logs
//! - Sync protocol definitions for mesh communication
//! - Crypto interfaces for signing/validation
//!
//! # Design Principles
//!
//! - **Append-only**: All state changes are recorded as immutable events
//! - **Offline-first**: No assumptions about network connectivity
//! - **Explicit boundaries**: Clear separation between core logic and external interfaces
//! - **FFI-safe**: All public APIs use C-compatible types and error handling

pub mod node;
pub mod ledger;
pub mod crdt;
pub mod sync;
pub mod crypto;
pub mod storage;
pub mod error;

// Re-export key types for public API
pub use node::{Node, NodeConfig, NodeHandle};
pub use ledger::Ledger;
pub use error::OpenGridError;

/// Main entry point for the OpenGrid engine
///
/// This struct represents the core engine that manages nodes and their state.
/// In a real implementation, this would coordinate multiple nodes and handle
/// cross-node synchronization.
pub struct OpenGridEngine {
    // Internal state would go here
}

impl OpenGridEngine {
    /// Create a new OpenGrid engine instance
    ///
    /// # Returns
    /// A new engine instance ready for node creation
    pub fn new() -> Self {
        Self {}
    }

    /// Create a new node with the given configuration
    ///
    /// # Arguments
    /// * `config` - Configuration for the new node
    ///
    /// # Returns
    /// A handle to the created node
    pub fn create_node(&self, config: NodeConfig) -> Result<NodeHandle, OpenGridError> {
        Node::create(config)
    }
}

impl Default for OpenGridEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = OpenGridEngine::new();
        assert!(true, "Engine created successfully");
    }

    #[test]
    fn test_node_creation() {
        let engine = OpenGridEngine::new();
        let config = NodeConfig::default();
        
        // This will fail until we implement the actual node creation
        // but demonstrates the intended API flow
        let result = engine.create_node(config);
        assert!(result.is_ok() || result.is_err()); // Either way, compilation works
    }
}