#[diplomat::bridge]
mod ffi {
    use opengrid_core::node::Node;

    #[diplomat::opaque]
    pub struct OpenGridEngine;

    impl OpenGridEngine {
        pub fn new() -> Box<OpenGridEngine> {
            Box::new(OpenGridEngine)
        }

        pub fn create_node(&self, _name: &str) -> Box<NodeHandle> {
            NodeHandle::new_ephemeral()
        }
    }

    #[diplomat::opaque]
    pub struct NodeHandle(pub Box<Node>);

    impl NodeHandle {
        pub fn new_ephemeral() -> Box<NodeHandle> {
             Box::new(NodeHandle(Box::new(Node::new_ephemeral())))
        }

        pub fn submit_event(&mut self, payload: &[u8]) -> u64 {
            match self.0.submit_event(payload) {
                Ok(v) => v,
                Err(_) => 0, 
            }
        }

        pub fn current_version(&self) -> u64 {
            self.0.current_version()
        }
    }
}