use crate::error::Result;

/// An abstraction for an append-only log storage.
///
/// This is the fundamental storage primitive for the mesh.
/// Entries are immutable once written.
pub trait AppendLog {
    /// Append a new entry to the log.
    /// Returns the index/offset of the new entry.
    fn append(&mut self, entry: &[u8]) -> Result<u64>;

    /// Read an entry at a specific index.
    fn read(&self, index: u64) -> Result<Option<Vec<u8>>>;

    /// Get the current length of the log (number of entries).
    fn len(&self) -> u64;

    /// Check if the log is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A simple in-memory implementation for testing and initial prototyping.
pub struct MemoryLog {
    entries: Vec<Vec<u8>>,
}

impl MemoryLog {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }
}

impl AppendLog for MemoryLog {
    fn append(&mut self, entry: &[u8]) -> Result<u64> {
        let idx = self.entries.len() as u64;
        self.entries.push(entry.to_vec());
        Ok(idx)
    }

    fn read(&self, index: u64) -> Result<Option<Vec<u8>>> {
        if index < self.entries.len() as u64 {
            Ok(Some(self.entries[index as usize].clone()))
        } else {
            Ok(None)
        }
    }

    fn len(&self) -> u64 {
        self.entries.len() as u64
    }
}