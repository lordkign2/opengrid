//! Node identity and lifecycle management
//!
//! A Node represents a participant in the OpenGrid mesh network.
//! Each node maintains its own CRDT-based ledger and can synchronize
//! with other nodes in the mesh.

use crate::error::OpenGridError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a node in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(Uuid);

impl NodeId {
    /// Generate a new random node ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create a node ID from bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(Uuid::from_bytes(bytes))
    }

    /// Convert to byte array
    pub fn to_bytes(self) -> [u8; 16] {
        self.0.into_bytes()
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Configuration for creating a new node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Human-readable name for the node
    pub name: String,
    
    /// Optional description
    pub description: Option<String>,
    
    /// Custom metadata as key-value pairs
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            name: "Unnamed Node".to_string(),
            description: None,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// Opaque handle to a node instance
///
/// This handle is safe to pass across FFI boundaries and
/// can be used to interact with the node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeHandle {
    pub(crate) id: NodeId,
}

impl NodeHandle {
    /// Get the node ID from this handle
    pub fn id(&self) -> NodeId {
        self.id
    }
}

/// Represents a node in the OpenGrid network
///
/// Nodes are the fundamental units of the mesh network.
/// Each node maintains its own state and can synchronize
/// with other nodes.
pub struct Node {
    id: NodeId,
    config: NodeConfig,
    // Internal state would go here
}

impl Node {
    /// Create a new node with the given configuration
    ///
    /// # Arguments
    /// * `config` - Configuration for the new node
    ///
    /// # Returns
    /// A handle to the created node
    pub fn create(config: NodeConfig) -> Result<NodeHandle, OpenGridError> {
        let node = Self {
            id: NodeId::new(),
            config,
        };

        // In a real implementation, we would:
        // 1. Initialize storage
        // 2. Set up CRDT replicas
        // 3. Register with the engine
        // 4. Return the handle

        Ok(NodeHandle { id: node.id })
    }

    /// Get the node's unique identifier
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Get the node's configuration
    pub fn config(&self) -> &NodeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_generation() {
        let id1 = NodeId::new();
        let id2 = NodeId::new();
        
        assert_ne!(id1, id2, "Generated IDs should be unique");
    }

    #[test]
    fn test_node_config_default() {
        let config = NodeConfig::default();
        assert_eq!(config.name, "Unnamed Node");
        assert!(config.description.is_none());
        assert!(config.metadata.is_empty());
    }

    #[test]
    fn test_node_creation() {
        let config = NodeConfig {
            name: "Test Node".to_string(),
            description: Some("A test node".to_string()),
            metadata: std::collections::HashMap::new(),
        };

        let result = Node::create(config);
        assert!(result.is_ok());

        let handle = result.unwrap();
        assert_ne!(handle.id(), NodeId::from_bytes([0; 16]));
    }
}