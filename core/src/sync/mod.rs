//! Synchronization protocol definitions
//!
//! This module defines the message types and protocols used for
//! synchronizing state between nodes in the mesh network.

pub mod protocol;

// Re-export key types
pub use protocol::{SyncMessage, SyncRequest, SyncResponse};

/// Trait for sync transport implementations
///
/// This trait abstracts the underlying network transport mechanism.
/// Implementations could use various protocols (Bluetooth, WiFi Direct,
/// mesh routing protocols, etc.) while maintaining the same sync interface.
pub trait SyncTransport {
    /// Send a sync message to a remote node
    ///
    /// # Arguments
    /// * `message` - The message to send
    /// * `destination` - Identifier of the destination node
    ///
    /// # Returns
    /// Result indicating success or failure
    fn send_message(
        &self,
        message: &SyncMessage,
        destination: &crate::node::NodeId,
    ) -> Result<(), crate::error::OpenGridError>;

    /// Receive incoming sync messages
    ///
    /// # Returns
    /// Vector of received messages
    fn receive_messages(&self) -> Result<Vec<SyncMessage>, crate::error::OpenGridError>;

    /// Get connected peers
    ///
    /// # Returns
    /// List of currently connected peer node IDs
    fn connected_peers(&self) -> Result<Vec<crate::node::NodeId>, crate::error::OpenGridError>;
}

/// Sync session manager
///
/// Manages synchronization sessions between nodes, handling
/// message sequencing, acknowledgments, and conflict resolution.
pub struct SyncSession {
    /// Local node ID
    local_node_id: crate::node::NodeId,
    
    /// Remote node ID
    remote_node_id: crate::node::NodeId,
    
    /// Session state
    state: SyncSessionState,
}

/// State of a synchronization session
#[derive(Debug, Clone, PartialEq)]
pub enum SyncSessionState {
    /// Initial state - no sync in progress
    Idle,
    
    /// Sending sync request
    SendingRequest,
    
    /// Waiting for response
    WaitingResponse,
    
    /// Active synchronization in progress
    Syncing,
    
    /// Sync completed successfully
    Completed,
    
    /// Sync failed
    Failed(crate::error::OpenGridError),
}

impl SyncSession {
    /// Create a new sync session
    ///
    /// # Arguments
    /// * `local_node_id` - ID of the local node
    /// * `remote_node_id` - ID of the remote node
    pub fn new(local_node_id: crate::node::NodeId, remote_node_id: crate::node::NodeId) -> Self {
        Self {
            local_node_id,
            remote_node_id,
            state: SyncSessionState::Idle,
        }
    }

    /// Start synchronization with remote node
    ///
    /// # Arguments
    /// * `transport` - Transport to use for communication
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn start_sync<T: SyncTransport>(
        &mut self,
        transport: &T,
    ) -> Result<(), crate::error::OpenGridError> {
        // In a real implementation, this would:
        // 1. Create and send sync request
        // 2. Transition to appropriate state
        // 3. Handle response
        self.state = SyncSessionState::SendingRequest;
        Ok(())
    }

    /// Process incoming sync message
    ///
    /// # Arguments
    /// * `message` - Incoming message to process
    ///
    /// # Returns
    /// Result indicating success or failure
    pub fn process_message(
        &mut self,
        message: &SyncMessage,
    ) -> Result<(), crate::error::OpenGridError> {
        // In a real implementation, this would:
        // 1. Validate message authenticity
        // 2. Process according to message type
        // 3. Update session state
        // 4. Send appropriate response
        Ok(())
    }

    /// Get current session state
    pub fn state(&self) -> &SyncSessionState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_session_creation() {
        let local_id = crate::node::NodeId::new();
        let remote_id = crate::node::NodeId::new();
        
        let session = SyncSession::new(local_id, remote_id);
        
        assert_eq!(session.local_node_id, local_id);
        assert_eq!(session.remote_node_id, remote_id);
        assert_eq!(*session.state(), SyncSessionState::Idle);
    }

    #[test]
    fn test_sync_session_state_transitions() {
        let local_id = crate::node::NodeId::new();
        let remote_id = crate::node::NodeId::new();
        
        let mut session = SyncSession::new(local_id, remote_id);
        assert_eq!(*session.state(), SyncSessionState::Idle);
        
        // Mock transport for testing
        struct MockTransport;
        
        impl SyncTransport for MockTransport {
            fn send_message(
                &self,
                _message: &SyncMessage,
                _destination: &crate::node::NodeId,
            ) -> Result<(), crate::error::OpenGridError> {
                Ok(())
            }
            
            fn receive_messages(&self) -> Result<Vec<SyncMessage>, crate::error::OpenGridError> {
                Ok(vec![])
            }
            
            fn connected_peers(&self) -> Result<Vec<crate::node::NodeId>, crate::error::OpenGridError> {
                Ok(vec![])
            }
        }
        
        let transport = MockTransport;
        let result = session.start_sync(&transport);
        assert!(result.is_ok());
        // Note: State would transition in real implementation
    }
}