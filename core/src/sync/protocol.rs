use serde::{Serialize, Deserialize};

/// Represents the knowledge of a node about the state of the mesh.
/// Maps NodeID -> MaxVersion
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VersionVector {
    pub versions: Vec<(Vec<u8>, u64)>, 
}

impl VersionVector {
    pub fn set(&mut self, node_id: Vec<u8>, version: u64) {
        if let Some(entry) = self.versions.iter_mut().find(|(id, _)| id == &node_id) {
            if version > entry.1 {
                entry.1 = version;
            }
        } else {
            self.versions.push((node_id, version));
        }
    }

    pub fn get(&self, node_id: &[u8]) -> u64 {
        self.versions.iter()
            .find(|(id, _)| id == node_id)
            .map(|(_, v)| *v)
            .unwrap_or(0)
    }

    pub fn merge(&mut self, other: &VersionVector) {
        for (id, v) in &other.versions {
            self.set(id.clone(), *v);
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_vector_merge() {
        let mut v1 = VersionVector::default();
        let node_a = b"node_a".to_vec();
        let node_b = b"node_b".to_vec();

        v1.set(node_a.clone(), 10);
        v1.set(node_b.clone(), 5);

        let mut v2 = VersionVector::default();
        v2.set(node_a.clone(), 8);
        v2.set(node_b.clone(), 12);
        let node_c = b"node_c".to_vec();
        v2.set(node_c.clone(), 3);

        v1.merge(&v2);

        assert_eq!(v1.get(&node_a), 10);
        assert_eq!(v1.get(&node_b), 12);
        assert_eq!(v1.get(&node_c), 3);
    }
}