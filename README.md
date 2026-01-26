# OpenGrid Core

Disaster-resilient, offline-first mesh resource exchange protocol - Core Infrastructure

## Overview

OpenGrid Core is the foundational infrastructure for a distributed mesh network protocol built on Conflict-free Replicated Data Types (CRDTs) and append-only event logs. This crate provides the base architecture that enables:

- Offline-first operation with eventual consistency
- Peer-to-peer mesh synchronization
- Disaster-resilient state management
- Mobile and embedded device compatibility
- FFI-safe APIs for native mobile integration

## Architecture

### Core Components

#### 1. Node Management (`node.rs`)
Nodes are the fundamental units of the mesh network. Each node:
- Maintains its own CRDT-based ledger
- Has a unique identifier for mesh discovery
- Manages its lifecycle and configuration
- Exposes FFI-safe handles for external control

#### 2. CRDT Foundation (`crdt/`)
The mathematical backbone ensuring convergence:
- **Convergent**: All replicas reach identical state
- **Commutative**: Operation order independence
- **Idempotent**: Safe repeated operations
- Trait-based design for extensible CRDT types

#### 3. Append-Only Storage (`storage/`)
Immutable event logging ensuring:
- Audit trail of all state changes
- Replayable state reconstruction
- Efficient snapshotting for performance
- Transactional consistency guarantees

#### 4. Mesh Synchronization (`sync/`)
Protocol for peer-to-peer state exchange:
- Message-based communication
- Version vector tracking
- Incremental delta synchronization
- Transport-agnostic design

#### 5. Cryptographic Layer (`crypto/`)
Security primitives for trust:
- Digital signatures for authenticity
- Hash functions for integrity
- Key management for identity
- Random generation for nonces

### Design Principles

#### Append-Only Mindset
All state changes are recorded as immutable events. This ensures:
- Perfect auditability
- Easy rollback and recovery
- Simplified conflict resolution
- No in-place mutations

#### Offline-First by Default
The system assumes intermittent connectivity:
- Local state always available
- Synchronization is opportunistic
- No wall-clock time dependencies
- No global coordination assumptions

#### Explicit Boundaries
Clear separation between core logic and interfaces:
- Core owns all internal state
- External callers use opaque handles
- FFI-safe type system
- Minimal public API surface

## FFI Integration

The `ffi/` crate exposes a C-compatible API for mobile platforms:

### Android (Kotlin)
```kotlin
val engine = OpenGridEngine.new()
val config = NodeConfig("My Node", null, mapOf())
val nodeHandle = engine.createNode(config)
```

### iOS (Swift)
```swift
let engine = OpenGridEngine.new()
let config = NodeConfig(name: "My Node", description: nil, metadata: [:])
let nodeHandle = try engine.createNode(config)
```

### Key FFI Concepts
- **Opaque Handles**: Represent internal objects safely
- **Result Types**: Explicit success/failure handling
- **C-Compatible Types**: No Rust lifetimes or generics
- **UniFFI**: Automatic binding generation

## Future Expansion Points

This foundation enables several advanced capabilities:

### Mobile Applications
- Native Android/iOS SDKs via FFI
- Background sync services
- Battery-efficient operations
- Platform-native UI integration

### Mesh Networking
- Bluetooth/WiFi Direct transports
- Mesh routing protocols
- NAT traversal mechanisms
- Bandwidth optimization

### Embedded Systems
- Resource-constrained deployments
- Real-time operation requirements
- Hardware security module integration
- Deterministic performance characteristics

## Building and Testing

### Prerequisites
- Rust 1.75 or later
- Cargo package manager

### Build Commands
```bash
# Build core library
cargo build --release

# Run tests
cargo test

# Build FFI library
cargo build --release -p opengrid-ffi
```

### Test Coverage
The current implementation includes:
- Unit tests for all major components
- Compilation verification
- FFI interface testing
- Trait contract verification

## Error Handling

Unified error model using `OpenGridError` enum:
- Node lifecycle errors
- CRDT convergence issues
- Storage failures
- Synchronization problems
- Cryptographic validation failures
- Invalid inputs
- Internal system errors

All public APIs return `Result<T, OpenGridError>` for consistent error handling.

## Current Status

This is a **base infrastructure** implementation:
- ✅ Module structure and boundaries established
- ✅ Core traits and interfaces defined
- ✅ FFI scaffolding in place
- ✅ Basic compilation and testing working
- ❌ CRDT implementations pending
- ❌ Network transport implementations pending
- ❌ Storage backend implementations pending
- ❌ Cryptographic implementations pending

## Anti-Goals

This implementation explicitly avoids:
- Blockchain-specific concepts
- Consensus algorithms (PoW, PoS, etc.)
- REST/HTTP assumptions
- UI/application logic
- Cloud service dependencies

## License

MIT OR Apache-2.0

## Contributing

This is infrastructure software designed for long-term evolution. Contributions should focus on:
- Correctness and safety
- Clear architectural boundaries
- Extensibility for future features
- Performance characteristics suitable for mobile/embedded use