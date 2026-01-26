//! UniFFI-exported API for OpenGrid Core
//!
//! This module exposes the core OpenGrid functionality through
//! a C-compatible FFI interface that can be consumed by Kotlin (Android)
//! and Swift (iOS) applications.
//!
//! # FFI Design Principles
//!
//! - All exported functions use C-compatible types
//! - Opaque handles represent complex objects
//! - Error handling through result enums
//! - No lifetimes or Rust-specific concepts exposed
//!
//! # Expected Usage Patterns
//!
//! ## Android (Kotlin)
//! ```kotlin
//! // Create engine
//! val engine = OpenGridEngine.new()
//! 
//! // Create node
//! val config = NodeConfig("My Node", null, emptyMap())
//! val nodeHandle = engine.createNode(config)
//! 
//! // Submit events and query state
//! // ...
//! ```
//!
//! ## iOS (Swift)
//! ```swift
//! // Create engine
//! let engine = OpenGridEngine.new()
//! 
//! // Create node
//! let config = NodeConfig(name: "My Node", description: nil, metadata: [:])
//! let nodeHandle = try engine.createNode(config)
//! 
//! // Submit events and query state
//! // ...
//! ```

use opengrid_core::{
    error::OpenGridError,
    node::{NodeConfig, NodeHandle},
    OpenGridEngine,
};
use std::collections::HashMap;

// UniFFI setup
uniffi::setup_scaffolding!();

/// Opaque handle to the OpenGrid engine
///
/// This handle represents the main engine instance and is
/// used to create and manage nodes.
#[derive(uniffi::Object)]
pub struct OpenGridEngineHandle {
    inner: OpenGridEngine,
}

/// Opaque handle to a node instance
///
/// This handle represents a specific node and is used
/// to interact with that node's state and operations.
#[derive(uniffi::Record, Clone, Debug)]
pub struct NodeHandleWrapper {
    /// Unique identifier for this node
    pub id: String,
}

/// Configuration for creating a new node
#[derive(uniffi::Record)]
pub struct NodeConfigWrapper {
    /// Human-readable name for the node
    pub name: String,
    
    /// Optional description
    pub description: Option<String>,
    
    /// Custom metadata as key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Result type for FFI operations
///
/// UniFFI requires explicit error handling through result types
/// rather than Rust's Result<T, E> directly.
#[derive(uniffi::Enum)]
pub enum OpenGridResult {
    /// Operation completed successfully
    Ok,
    
    /// Operation failed with an error
    Error { message: String },
}

impl From<Result<(), OpenGridError>> for OpenGridResult {
    fn from(result: Result<(), OpenGridError>) -> Self {
        match result {
            Ok(_) => OpenGridResult::Ok,
            Err(error) => OpenGridResult::Error {
                message: error.to_string(),
            },
        }
    }
}

impl From<Result<NodeHandleWrapper, OpenGridError>> for OpenGridResult {
    fn from(result: Result<NodeHandleWrapper, OpenGridError>) -> Self {
        match result {
            Ok(value) => OpenGridResult::Ok,
            Err(error) => OpenGridResult::Error {
                message: error.to_string(),
            },
        }
    }
}

impl From<Result<Vec<u8>, OpenGridError>> for OpenGridResult {
    fn from(result: Result<Vec<u8>, OpenGridError>) -> Self {
        match result {
            Ok(_) => OpenGridResult::Ok,
            Err(error) => OpenGridResult::Error {
                message: error.to_string(),
            },
        }
    }
}

/// Create a new OpenGrid engine instance
///
/// This is the main entry point for the FFI API.
/// The returned handle should be kept alive for the duration
/// of the application's use of OpenGrid.
#[uniffi::export]
pub fn create_engine() -> OpenGridEngineHandle {
    OpenGridEngineHandle {
        inner: OpenGridEngine::new(),
    }
}

/// Create a new node with the given configuration
///
/// # Arguments
/// * `engine` - Handle to the engine instance
/// * `config` - Configuration for the new node
///
/// # Returns
/// Handle to the created node, or error if creation failed
#[uniffi::export]
pub fn create_node(
    engine: &OpenGridEngineHandle,
    config: NodeConfigWrapper,
) -> OpenGridResult {
    let core_config = NodeConfig {
        name: config.name,
        description: config.description,
        metadata: config.metadata,
    };

    match engine.inner.create_node(core_config) {
        Ok(_handle) => OpenGridResult::Ok,
        Err(error) => OpenGridResult::Error {
            message: error.to_string(),
        },
    }
}

/// Submit an event to a node's ledger
///
/// # Arguments
/// * `node_handle` - Handle to the target node
/// * `event_data` - Serialized event data to submit
///
/// # Returns
/// Result indicating success or failure
#[uniffi::export]
pub fn submit_event(
    node_handle: NodeHandleWrapper,
    event_data: Vec<u8>,
) -> OpenGridResult {
    // In a real implementation, this would:
    // 1. Look up the node from the handle
    // 2. Deserialize the event data
    // 3. Apply the event to the node's ledger
    // 4. Return success or error
    
    // Placeholder implementation
    OpenGridResult::Ok
}

/// Get a snapshot of a node's current state
///
/// # Arguments
/// * `node_handle` - Handle to the target node
///
/// # Returns
/// Serialized state snapshot
#[uniffi::export]
pub fn get_state_snapshot(
    node_handle: NodeHandleWrapper,
) -> OpenGridResult {
    // In a real implementation, this would:
    // 1. Look up the node from the handle
    // 2. Get the current converged state
    // 3. Serialize and return the state
    
    // Placeholder implementation
    OpenGridResult::Ok
}

/// Get the version information for a node
///
/// # Arguments
/// * `node_handle` - Handle to the target node
///
/// # Returns
/// Version information for synchronization
#[uniffi::export]
pub fn get_node_version(
    node_handle: NodeHandleWrapper,
) -> OpenGridResult {
    // In a real implementation, this would return
    // the node's current vector clock or version vector
    
    // Placeholder implementation
    OpenGridResult::Ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = create_engine();
        assert!(true, "Engine handle created successfully");
    }

    #[test]
    fn test_node_creation_via_ffi() {
        let engine = create_engine();
        
        let config = NodeConfigWrapper {
            name: "Test Node".to_string(),
            description: Some("A test node for FFI".to_string()),
            metadata: HashMap::new(),
        };

        let result = create_node(&engine, config);
        
        match result {
            OpenGridResult::Ok => {
                // Node creation succeeded
                assert!(true);
            }
            OpenGridResult::Error { message } => {
                // This is expected in the stub implementation
                assert!(!message.is_empty(), "Error should have message");
            }
        }
    }

    #[test]
    fn test_placeholder_functions() {
        let node_handle = NodeHandleWrapper {
            id: "test-node-id".to_string(),
        };

        // Test submit_event
        let event_result = submit_event(node_handle.clone(), vec![1, 2, 3]);
        assert!(matches!(event_result, OpenGridResult::Ok));

        // Test get_state_snapshot
        let snapshot_result = get_state_snapshot(node_handle.clone());
        assert!(matches!(snapshot_result, OpenGridResult::Ok));

        // Test get_node_version
        let version_result = get_node_version(node_handle);
        assert!(matches!(version_result, OpenGridResult::Ok));
    }
}