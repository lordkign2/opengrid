use crate::crypto::{Signer, Ed25519Signer};
use crate::storage::{AppendLog, MemoryLog};
use crate::error::Result;

/// Represents a participating node in the OpenGrid mesh.
/// 
/// A Node holds identity (keys) and a local storage log.
pub struct Node {
    pub id: Vec<u8>,
    _signer: Box<dyn Signer>,
    log: Box<dyn AppendLog>,
}

impl Node {
    /// Create a new ephemeral node with a random identity and in-memory storage.
    pub fn new_ephemeral() -> Self {
        let signer = Ed25519Signer::new_random();
        let id = signer.public_key();
        
        Self {
            id,
            _signer: Box::new(signer),
            log: Box::new(MemoryLog::new()),
        }
    }

    /// Submit a generic event to this node's local log.
    /// In a real implementation, this would sign the event and wrap it in a protocol frame.
    pub fn submit_event(&mut self, payload: &[u8]) -> Result<u64> {
        // TODO: Sign payload properly
        self.log.append(payload)
    }

    /// Retrieve the current storage tip.
    pub fn current_version(&self) -> u64 {
        self.log.len()
    }
}