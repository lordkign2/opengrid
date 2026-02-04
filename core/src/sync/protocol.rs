use serde::{Serialize, Deserialize};

/// Represents the knowledge of a node about the state of the mesh.
/// Maps NodeID -> MaxVersion
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionVector {
    // simplified: just a placeholder map
    // In real impl: HashMap<Vec<u8>, u64>
    pub versions: Vec<(Vec<u8>, u64)>, 
}

/// Messages exchanged between nodes to synchronize state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMessage {
    /// "I have these versions, what do you have?"
    Hello {
        my_vector: VersionVector,
    },
    
    /// "Here are the events you are missing."
    Updates {
        events: Vec<Vec<u8>>,
    },
}