# OpenGrid Core Engine

> Crate: `opengrid-core` | Path: `core/`

---

## Overview

The core engine is the heart of the OpenGrid system. It is a pure Rust library that implements all business logic for a mesh resource exchange node. It has:

- **No network I/O** — transport is always abstracted behind a trait
- **No disk I/O** — storage is always abstracted behind a trait
- **No UI dependencies** — zero coupling to any platform UI framework
- **No `unsafe` code** — except where unavoidable at FFI boundaries

This makes the core engine fully portable across Android, iOS, servers, and embedded hardware.

---

## Module Map

```
core/src/
├── lib.rs          # Public API surface and module declarations
├── error.rs        # Unified error type (CoreError)
├── node.rs         # Mesh node: identity + event log
├── ledger.rs       # Ledger trait for CRDT-backed domain state
├── crdt/
│   ├── mod.rs      # Module re-exports
│   └── traits.rs   # CrdtMerge + Crdt traits (mathematical contracts)
├── sync/
│   ├── mod.rs      # Module re-exports
│   └── protocol.rs # SyncMessage enum + VersionVector
├── crypto/
│   └── mod.rs      # Signer + Verifier traits + Ed25519Signer stub
└── storage/
    └── mod.rs      # AppendLog trait + MemoryLog implementation
```

---

## Error Model (`error.rs`)

All fallible operations in the core engine return `core::Result<T>`, a type alias for `std::result::Result<T, CoreError>`.

```rust
pub enum CoreError {
    IdentityError(String),   // Key generation or ID resolution failures
    StorageError(String),    // Append or read failures from the log
    SyncError(String),       // Invalid or unexpected sync messages
    CrdtError(String),       // Merge conflicts or invalid CRDT state
    CryptoError,             // Signature verification failure
    NotImplemented(String),  // Stub placeholder for future features
}
```

**Design rule:** Error variants are always strings or unit — no nested error types. This keeps FFI conversion simple.

---

## Node (`node.rs`)

The `Node` struct represents a single participant in the OpenGrid mesh.

```rust
pub struct Node {
    pub id: Vec<u8>,         // Public key bytes (node identity)
    _signer: Box<dyn Signer>,
    log: Box<dyn AppendLog>,
}
```

### Key Methods

| Method | Description |
|---|---|
| `Node::new_ephemeral()` | Creates a node with a random identity and in-memory storage. Used for testing and initial prototyping. |
| `node.submit_event(&[u8]) -> Result<u64>` | Appends a raw event payload to the local log. Returns the log index. |
| `node.current_version() -> u64` | Returns the current number of events in the log (the "tip"). |

### Design Notes

- `id` is the serialized public key. Two nodes with the same key are considered the same participant.
- The `_signer` is boxed as a trait object so implementations (HSM, software, platform keychain) are pluggable.
- The `log` is boxed as a trait object so storage backends (memory, SQLite, RocksDB) are swappable at runtime.
- In the future, `submit_event` will sign the payload before writing it.

### Example

```rust
use opengrid_core::node::Node;

let mut node = Node::new_ephemeral();
println!("Node ID (hex): {:?}", node.id);

let version = node.submit_event(b"{ \"type\": \"resource_offer\", \"amount\": 100 }").unwrap();
println!("Event stored at version: {}", version);
println!("Current tip: {}", node.current_version());
```

---

## CRDT Layer (`crdt/`)

The CRDT module defines the mathematical contracts that all shared state must satisfy.

### `CrdtMerge` Trait

```rust
pub trait CrdtMerge {
    fn merge(&mut self, other: &Self);
}
```

Any type implementing `CrdtMerge` must guarantee:

| Property | Meaning |
|---|---|
| **Associativity** | `(a.merge(b)).merge(c)` == `a.merge(b.merge(c))` |
| **Commutativity** | `a.merge(b)` == `b.merge(a)` |
| **Idempotency** | `a.merge(a)` == `a` |

These three properties guarantee that any set of nodes, given the same events in any order, with any partial replication, will eventually reach the same state.

### `Crdt` Trait

```rust
pub trait Crdt: CrdtMerge {
    // Marker trait — future methods for delta-state and version tracking
}
```

`Crdt` is a marker that extends `CrdtMerge`. Types that implement `Crdt` are suitable for use in `Ledger` implementations.

### Planned CRDT Implementations

| Type | Description | Use Case |
|---|---|---|
| `GCounter` | Grow-only counter | Counting resource transfers |
| `PNCounter` | Increment/decrement counter | Net balance tracking |
| `MVReg` | Multi-value register | Last-writer-wins values |
| `LWWElementSet` | Last-write-wins set | Peer tables, capability sets |
| `ORSet` | Observed-remove set | Offer/claim sets |

---

## Ledger (`ledger.rs`)

A `Ledger` represents a domain of shared state, such as "Resource Exchange" or "Trust Metrics".

```rust
pub trait Ledger: Crdt {
    type Operation;
    fn apply(&mut self, op: &Self::Operation) -> Result<()>;
}
```

Ledgers are built on top of CRDTs and reconstructed by replaying the event log from the beginning. They are always derivable — never stored as primary state.

### Design Rules

- **Balances are derived, never stored.** The ledger is always computed from the log.
- **Operations are idempotent.** Applying the same operation twice yields the same result.
- **No deletes.** Past operations are immutable. Corrections are made via new operations.

---

## Sync Protocol (`sync/`)

The sync layer defines the message format for gossip-based anti-entropy synchronization between nodes.

### `VersionVector`

Tracks what each node knows about the mesh:

```rust
pub struct VersionVector {
    pub versions: Vec<(Vec<u8>, u64)>,  // (NodeID, MaxVersion)
}
```

| Method | Description |
|---|---|
| `set(node_id, version)` | Update knowledge of a node's version |
| `get(node_id) -> u64` | Query known version for a node |
| `merge(&other)` | Merge two version vectors (takes the max of each) |

### `SyncMessage`

```rust
pub enum SyncMessage {
    Hello { my_vector: VersionVector },
    Updates { events: Vec<Vec<u8>> },
}
```

**Synchronization protocol (simplified):**

```
Node A                            Node B
  │                                 │
  │─── Hello { my_vector } ────────>│ "I have these versions"
  │                                 │
  │<─── Updates { missing_events } ─│ "Here's what you're missing"
  │                                 │
  │ (apply events to local log)     │
```

### Transport Abstraction

The sync module intentionally does not define a transport. Transport will be implemented separately as:
- **BLE** (Bluetooth Low Energy) for Android/iOS proximity sync
- **Wi-Fi Direct** for Android high-bandwidth sync
- **QUIC** / **TCP** for server nodes

---

## Cryptography (`crypto/`)

All events in the mesh are signed by their originating node. Signatures allow any peer to verify event authenticity without a trusted third party.

### Traits

```rust
pub trait Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>>;
    fn public_key(&self) -> Vec<u8>;
}

pub trait Verifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool>;
}
```

### Current Implementation

`Ed25519Signer` is a placeholder that returns zero-filled keys and signatures. It exists to make the compilation unit complete while real cryptography is added.

```rust
pub struct Ed25519Signer { /* placeholder */ }
```

### Planned Implementations

| Platform | Implementation |
|---|---|
| Android | Android Keystore API (hardware-backed on devices with TEE) |
| iOS | Secure Enclave via CryptoKit |
| Server | `ring` or `ed25519-dalek` |
| Embedded | Platform-native secure element |

### Security Notes

- Keys should be generated and stored in secure enclaves where available.
- The public key is the node's identity — it should never change for the lifetime of a node's participation.
- Event signatures must be verified before events are accepted into the local log.

---

## Storage Layer (`storage/`)

Storage is modeled as an append-only log of raw byte entries.

### `AppendLog` Trait

```rust
pub trait AppendLog {
    fn append(&mut self, entry: &[u8]) -> Result<u64>;
    fn read(&self, index: u64) -> Result<Option<Vec<u8>>>;
    fn len(&self) -> u64;
    fn is_empty(&self) -> bool;
}
```

### `MemoryLog`

An in-memory implementation backed by a `Vec<Vec<u8>>`. Used for:
- Unit testing
- Ephemeral nodes (no persistence needed)
- Initial prototyping

```rust
let mut log = MemoryLog::new();
let idx = log.append(b"my event bytes").unwrap();
let entry = log.read(idx).unwrap(); // Some(b"my event bytes")
```

### Planned Storage Backends

| Backend | Use Case |
|---|---|
| `SqliteLog` | Mobile devices — uses Android Room or iOS SQLite |
| `RocksDBLog` | Server nodes — high throughput persistent log |
| `FileLog` | Embedded / simple deployment |

---

## Public API (`lib.rs`)

The `lib.rs` exposes all sub-modules and provides a single `init()` function for future global setup:

```rust
pub mod crdt;
pub mod crypto;
pub mod error;
pub mod ledger;
pub mod node;
pub mod storage;
pub mod sync;

pub use error::{CoreError, Result};

pub fn init() {
    // No-op for now; reserved for future global initialization
}
```

---

## Testing

The core engine has embedded unit tests in each module.

```bash
# Run all core tests
cargo test -p opengrid-core

# Run with stdout output
cargo test -p opengrid-core -- --nocapture

# Run a specific test
cargo test -p opengrid-core test_node_creation_and_event
```

**Key test cases:**

| Test | Location | Validates |
|---|---|---|
| `test_node_creation_and_event` | `lib.rs` | Node creation + event submission lifecycle |
| `test_memory_log_append_and_read` | `storage/mod.rs` | Append-only log semantics |
| `test_version_vector_merge` | `sync/protocol.rs` | Version vector merge correctness |

---

## Extension Guide

### Adding a New CRDT Implementation

1. Create a new file (e.g., `core/src/crdt/gcounter.rs`).
2. Implement `CrdtMerge` with the three mathematical properties.
3. Implement `Crdt` as a marker.
4. Add the module to `crdt/mod.rs`.
5. Write property-based tests to verify associativity, commutativity, and idempotency.

### Adding a New Storage Backend

1. Create a new file (e.g., `core/src/storage/sqlite.rs`).
2. Implement the `AppendLog` trait.
3. Add the module to `storage/mod.rs` behind a feature flag if it has external dependencies.

### Adding a New Ledger Domain

1. Create a new file (e.g., `core/src/ledger/resource.rs`).
2. Define an `Operation` enum for your domain.
3. Implement `Crdt` + `Ledger` for your domain struct.
4. Wire it into the `Node` so it is rebuilt from the log on startup.

---

## Non-Goals

The core engine **will never contain**:

- Direct network calls (no `tokio`, no `reqwest`, etc.)
- Platform-specific code (no `android_logger`, no iOS frameworks)
- File system access (storage is always via trait injection)
- UI logic of any kind
- Global mutable state
