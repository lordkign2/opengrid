# OpenGrid Core

A minimal, offline-first mesh resource exchange protocol engine written in Rust.

## Architecture

OpenGrid is designed as a distributed system that runs on edge devices (mobile phones, embedded nodes). It prioritizes partition tolerance and availability (AP) over immediate consistency.

### Core Components

- **Append-Only Logs**: All state transitions are recorded in an immutable, append-only log. This ensures that history is preserved and synchronization is a matter of verifying log prefixes and exchanging missing suffixes.
- **CRDTs (Conflict-free Replicated Data Types)**: To handle state convergence without centralized consensus, all high-level data models (Ledgers) are built on top of CRDTs. This allows nodes to merge state changes from any other node deterministically.
- **Offline-First**: The system assumes no reliable network. Operations are local-first and synced when connectivity is available.
- **Node Identity**: Each participant is identified by a cryptographic key pair (currently placeholder ED25519).

### FFI & Mobile Support

The `core` is written in pure Rust with no IO or UI dependencies.
The `ffi` crate exposes a C-ABI compliant interface using **Diplomat**, which generates bindings for:
- Android (Kotlin/JNI)
- iOS (Swift)
- C/C++

This separation ensures that the core logic remains portable and testable in isolation.

## Usage

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```