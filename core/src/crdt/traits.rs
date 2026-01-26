//! Core CRDT mathematical properties and contracts
//!
//! These traits define the mathematical properties that all CRDTs must satisfy:
//!
//! - Convergence: All replicas eventually reach the same state
//! - Commutativity: Operation order doesn't affect the final result
//! - Idempotence: Applying the same operation multiple times has the same effect

use crate::error::OpenGridError;

/// Convergence property: all replicas eventually reach the same state
///
/// When two replicas have exchanged all operations, they must converge
/// to identical states regardless of network partitioning or message ordering.
pub trait Convergent {
    /// Merge state from another replica
    ///
    /// # Arguments
    /// * `other` - Reference to another replica's state
    ///
    /// # Returns
    /// Result indicating success or failure
    fn merge(&mut self, other: &Self) -> Result<(), OpenGridError>;

    /// Check if this replica has the same state as another
    ///
    /// # Arguments
    /// * `other` - Reference to another replica's state
    ///
    /// # Returns
    /// True if states are equivalent, false otherwise
    fn equivalent(&self, other: &Self) -> bool;
}

/// Commutativity property: operation order doesn't matter
///
/// Operations can be applied in any order and still produce equivalent results.
/// This enables concurrent updates without coordination.
pub trait Commutative {
    /// The type of operations this CRDT supports
    type Operation;

    /// Apply an operation to this replica
    ///
    /// # Arguments
    /// * `operation` - The operation to apply
    ///
    /// # Returns
    /// Result indicating success or failure
    fn apply(&mut self, operation: Self::Operation) -> Result<(), OpenGridError>;
}

/// Idempotence property: repeated operations have no additional effect
///
/// Applying the same operation multiple times produces the same result
/// as applying it once.
pub trait Idempotent {
    /// The type of operations this CRDT supports
    type Operation;

    /// Check if an operation is idempotent when applied to current state
    ///
    /// # Arguments
    /// * `operation` - The operation to check
    ///
    /// # Returns
    /// True if the operation is idempotent, false otherwise
    fn is_idempotent(&self, operation: &Self::Operation) -> bool;
}

/// Core CRDT trait combining all required properties
///
/// This trait combines convergence, commutativity, and idempotence
/// to define a complete CRDT implementation contract.
pub trait Crdt: Convergent + Commutative + Idempotent {
    /// The type of values this CRDT represents
    type Value;

    /// The type of operations this CRDT supports
    type Operation;

    /// Create a new CRDT instance
    ///
    /// # Arguments
    /// * `initial_value` - Initial value for the CRDT
    ///
    /// # Returns
    /// A new CRDT instance
    fn new(initial_value: Self::Value) -> Self;

    /// Read the current converged value
    ///
    /// # Returns
    /// The current value according to the CRDT's convergence semantics
    fn read(&self) -> Self::Value;

    /// Reset the CRDT to its initial state
    ///
    /// # Returns
    /// Result indicating success or failure
    fn reset(&mut self) -> Result<(), OpenGridError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test struct to demonstrate trait usage
    struct TestCrdt {
        value: u64,
    }

    impl Convergent for TestCrdt {
        fn merge(&mut self, other: &Self) -> Result<(), OpenGridError> {
            self.value = self.value.max(other.value);
            Ok(())
        }

        fn equivalent(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }

    impl Commutative for TestCrdt {
        type Operation = u64;

        fn apply(&mut self, operation: Self::Operation) -> Result<(), OpenGridError> {
            self.value += operation;
            Ok(())
        }
    }

    impl Idempotent for TestCrdt {
        type Operation = u64;

        fn is_idempotent(&self, _operation: &Self::Operation) -> bool {
            // In a real implementation, this would check idempotence
            true
        }
    }

    impl Crdt for TestCrdt {
        type Value = u64;
        type Operation = u64;

        fn new(initial_value: Self::Value) -> Self {
            Self { value: initial_value }
        }

        fn read(&self) -> Self::Value {
            self.value
        }

        fn reset(&mut self) -> Result<(), OpenGridError> {
            self.value = 0;
            Ok(())
        }
    }

    #[test]
    fn test_crdt_traits() {
        let mut crdt1 = TestCrdt::new(0);
        let mut crdt2 = TestCrdt::new(0);

        // Apply operations
        crdt1.apply(5).unwrap();
        crdt2.apply(3).unwrap();

        // Merge states
        crdt1.merge(&crdt2).unwrap();
        crdt2.merge(&crdt1).unwrap();

        // Check convergence
        assert_eq!(crdt1.read(), 8);
        assert_eq!(crdt2.read(), 8);
        assert!(crdt1.equivalent(&crdt2));
    }
}