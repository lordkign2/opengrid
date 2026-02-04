pub mod crdt;
pub mod crypto;
pub mod error;
pub mod ledger;
pub mod node;
pub mod storage;
pub mod sync;

pub use error::{CoreError, Result};

/// Initialize the core library.
/// This is a placeholder for any global setup if needed in the future.
pub fn init() {
    // No-op for now
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    #[test]
    fn test_node_creation_and_event() {
        let mut node = Node::new_ephemeral();
        assert!(!node.id.is_empty(), "Node should have an ID");
        
        let payload = b"Hello OpenGrid";
        let res = node.submit_event(payload);
        
        assert!(res.is_ok(), "Should be able to submit event");
        assert_eq!(node.current_version(), 1, "Log length should be 1");
    }
}