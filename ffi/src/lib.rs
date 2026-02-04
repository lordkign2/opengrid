#[diplomat::bridge]
mod ffi {
    use opengrid_core::node::Node;

    #[diplomat::opaque]
    pub struct NodeHandle(pub Box<Node>);

    /*
    #[diplomat::enum]
    pub enum FfiError {
        IdentityError,
        StorageError,
        SyncError,
        CrdtError,
        CryptoError,
        NotImplemented,
        Unknown,
    }
    */

    impl NodeHandle {
        /// Create a new ephemeral node.
        pub fn new_ephemeral() -> Box<NodeHandle> {
             Box::new(NodeHandle(Box::new(Node::new_ephemeral())))
        }

        /// Submit an event payload.
        /// Returns 0 on failure (TODO: Add error handling), version on success.
        pub fn submit_event(&mut self, payload: &[u8]) -> u64 {
            match self.0.submit_event(payload) {
                Ok(v) => v,
                Err(_) => 0, 
            }
        }

        /// Get the current version (log length).
        pub fn current_version(&self) -> u64 {
            self.0.current_version()
        }
    }
}