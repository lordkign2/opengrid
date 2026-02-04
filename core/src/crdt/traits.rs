/// A trait for data structures that can be merged deterministically.
///
/// This is the foundation for all state in the mesh.
/// Implementations must satisfy:
/// - Associativity: (a + b) + c == a + (b + c)
/// - Commutativity: a + b == b + a
/// - Idempotency: a + a == a
pub trait CrdtMerge {
    /// Merge another state into this one.
    /// The result represents the union of information from both states.
    fn merge(&mut self, other: &Self);
}

/// A marker trait for types that behave as full CRDTs within the system.
/// Usually implies serialization and state inspection capabilities.
pub trait Crdt: CrdtMerge {
    // Future: Add methods for state delta calculation or version vectors here.
}