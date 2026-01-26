//! Sync message definitions and protocol structures
//!
//! This module defines the wire format for synchronization messages
//! exchanged between nodes in the mesh network.

use serde::{Deserialize, Serialize};

/// Main synchronization message envelope
///
/// All sync communications are wrapped in this envelope which
/// contains routing information and the actual message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMessage {
    /// Unique message identifier
    pub message_id: uuid::Uuid,

    /// Sender node identifier
    pub sender_id: crate::node::NodeId,

    /// Recipient node identifier
    pub recipient_id: crate::node::NodeId,

    /// Message timestamp (logical, not wall-clock)
    pub timestamp: u64,

    /// The actual message content
    pub content: SyncContent,
}

/// Content variants for synchronization messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncContent {
    /// Request to initiate synchronization
    SyncRequest(SyncRequest),

    /// Response to a sync request
    SyncResponse(SyncResponse),

    /// State delta for incremental sync
    StateDelta(StateDelta),

    /// Acknowledgment of received message
    Acknowledgment(Acknowledgment),

    /// Error notification
    SyncError(SyncError),
}

/// Request to initiate synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    /// Requested sync protocol version
    pub protocol_version: u32,

    /// Capabilities supported by requesting node
    pub capabilities: Vec<String>,

    /// Current state version/vector clock
    pub current_version: Vec<u8>,

    /// Requested sync scope (full, delta, specific ledgers)
    pub scope: SyncScope,
}

/// Response to a synchronization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    /// Accepted protocol version
    pub protocol_version: u32,

    /// Status of the sync response
    pub status: SyncResponseStatus,

    /// Reason for rejection (if applicable)
    pub rejection_reason: Option<String>,

    /// Remote node's current version information
    pub remote_version: Vec<u8>,
}

/// Status of a sync response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncResponseStatus {
    /// Sync accepted and will proceed
    Accepted,

    /// Sync rejected (incompatible versions, etc.)
    Rejected,

    /// Busy - try again later
    Busy,
}

/// Scope of synchronization requested
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncScope {
    /// Full state synchronization
    Full,

    /// Delta synchronization since last known version
    Delta,

    /// Specific ledger synchronization
    SpecificLedger(String),
}

/// State delta for incremental synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDelta {
    /// Ledger identifier this delta applies to
    pub ledger_id: String,

    /// Operations to apply
    pub operations: Vec<DeltaOperation>,

    /// Context information for causality tracking
    pub context: Vec<u8>,
}

/// Individual operation in a state delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaOperation {
    /// Unique operation identifier
    pub op_id: uuid::Uuid,

    /// Logical timestamp
    pub timestamp: u64,

    /// Operation data
    pub data: Vec<u8>,
}

/// Message acknowledgment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Acknowledgment {
    /// ID of acknowledged message
    pub acknowledged_message_id: uuid::Uuid,

    /// Status of processing
    pub status: AckStatus,
}

/// Acknowledgment status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AckStatus {
    /// Message processed successfully
    Success,

    /// Message rejected
    Rejected,

    /// Processing failed
    Failed,
}

/// Synchronization error notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncError {
    /// Error code
    pub error_code: SyncErrorCode,

    /// Human-readable error description
    pub description: String,

    /// Optional additional error data
    pub data: Option<Vec<u8>>,
}

/// Synchronization error codes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncErrorCode {
    /// Protocol version mismatch
    VersionMismatch,

    /// Authentication failure
    AuthenticationFailed,

    /// Invalid message format
    InvalidFormat,

    /// Internal processing error
    InternalError,

    /// Network timeout
    Timeout,
}

impl SyncMessage {
    /// Create a new sync message
    ///
    /// # Arguments
    /// * `sender_id` - ID of sending node
    /// * `recipient_id` - ID of receiving node
    /// * `timestamp` - Logical timestamp
    /// * `content` - Message content
    pub fn new(
        sender_id: crate::node::NodeId,
        recipient_id: crate::node::NodeId,
        timestamp: u64,
        content: SyncContent,
    ) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4(),
            sender_id,
            recipient_id,
            timestamp,
            content,
        }
    }

    /// Create a sync request message
    ///
    /// # Arguments
    /// * `sender_id` - ID of requesting node
    /// * `recipient_id` - ID of target node
    /// * `timestamp` - Logical timestamp
    /// * `current_version` - Current state version
    pub fn sync_request(
        sender_id: crate::node::NodeId,
        recipient_id: crate::node::NodeId,
        timestamp: u64,
        current_version: Vec<u8>,
    ) -> Self {
        let request = SyncRequest {
            protocol_version: 1,
            capabilities: vec!["basic_sync".to_string()],
            current_version,
            scope: SyncScope::Delta,
        };

        Self::new(
            sender_id,
            recipient_id,
            timestamp,
            SyncContent::SyncRequest(request),
        )
    }

    /// Create a sync response message
    ///
    /// # Arguments
    /// * `sender_id` - ID of responding node
    /// * `recipient_id` - ID of requesting node
    /// * `timestamp` - Logical timestamp
    /// * `status` - Response status
    pub fn sync_response(
        sender_id: crate::node::NodeId,
        recipient_id: crate::node::NodeId,
        timestamp: u64,
        status: SyncResponseStatus,
    ) -> Self {
        let response = SyncResponse {
            protocol_version: 1,
            status,
            rejection_reason: None,
            remote_version: vec![],
        };

        Self::new(
            sender_id,
            recipient_id,
            timestamp,
            SyncContent::SyncResponse(response),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_message_creation() {
        let sender = crate::node::NodeId::new();
        let recipient = crate::node::NodeId::new();
        let timestamp = 12345;

        let message = SyncMessage::sync_request(
            sender,
            recipient,
            timestamp,
            vec![1, 2, 3],
        );

        assert_eq!(message.sender_id, sender);
        assert_eq!(message.recipient_id, recipient);
        assert_eq!(message.timestamp, timestamp);
        matches!(message.content, SyncContent::SyncRequest(_));
    }

    #[test]
    fn test_sync_response_creation() {
        let sender = crate::node::NodeId::new();
        let recipient = crate::node::NodeId::new();
        let timestamp = 12345;

        let message = SyncMessage::sync_response(
            sender,
            recipient,
            timestamp,
            SyncResponseStatus::Accepted,
        );

        assert_eq!(message.sender_id, sender);
        assert_eq!(message.recipient_id, recipient);
        assert_eq!(message.timestamp, timestamp);
        matches!(message.content, SyncContent::SyncResponse(_));
    }

    #[test]
    fn test_serialization() {
        let sender = crate::node::NodeId::new();
        let recipient = crate::node::NodeId::new();
        
        let message = SyncMessage::sync_request(
            sender,
            recipient,
            12345,
            vec![1, 2, 3],
        );

        let serialized = serde_json::to_vec(&message).unwrap();
        let deserialized: SyncMessage = serde_json::from_slice(&serialized).unwrap();

        assert_eq!(message.message_id, deserialized.message_id);
        assert_eq!(message.sender_id, deserialized.sender_id);
        assert_eq!(message.recipient_id, deserialized.recipient_id);
    }
}