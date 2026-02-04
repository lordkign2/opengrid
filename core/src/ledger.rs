use crate::crdt::Crdt;
use crate::error::Result;

/// A Ledger represents a specific domain of state (e.g., Resource Exchange, Trust Metrics).
/// It is backed by a CRDT and constructed from the event log.
pub trait Ledger: Crdt {
    /// The type of transaction/operation this ledger accepts.
    type Operation;

    /// Apply an operation to the ledger state.
    /// This should be deterministic.
    fn apply(&mut self, op: &Self::Operation) -> Result<()>;
}