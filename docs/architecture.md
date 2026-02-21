# OpenGrid System Architecture

> Last updated: 2026-02-21

---

## Overview

OpenGrid is an **offline-first, partition-tolerant resource exchange engine** designed to operate in environments with unreliable or entirely absent centralized infrastructure.

The system enables hyper-local peer-to-peer resource coordination — energy sharing, data relaying, water credit tracking — using:

- **Local mesh networking** (Bluetooth, Wi-Fi Direct, QUIC)
- **CRDT-based ledgers** for deterministic convergence
- **Append-only event logs** for immutable history and replay

OpenGrid is a **distributed systems engine**, not an app. UIs are replaceable. Networks are unreliable. The core must always converge.

---

## Design Principles

| Principle | What It Means |
|---|---|
| **Offline-first** | All operations are local. Sync is opportunistic. |
| **No central authority** | No server must be reachable for the system to function. |
| **Append-only data model** | Events are immutable once written. No deletes. |
| **Deterministic convergence** | Two nodes with the same events always reach the same state. |
| **Explicit trust boundaries** | The FFI is a hard contract; no logic leaks across it. |
| **UI is never the source of truth** | The Rust engine owns all state. UIs render snapshots. |

---

## High-Level System Diagram

```
┌─────────────────────────┐   ┌─────────────────────────┐
│      Android Node       │   │        iOS Node          │
│   (Jetpack Compose)     │   │       (SwiftUI)          │
└────────────┬────────────┘   └────────────┬────────────┘
             │ ViewModel/LiveData           │ ObservableObject/@Published
             ▼                             ▼
      DiplomatBridge.kt           OpenGridObserver.swift
             │                             │
             └──────────────┬──────────────┘
                            │  FFI (Diplomat / C-ABI)
                            ▼
             ┌──────────────────────────────┐
             │     opengrid-ffi (Rust)      │
             │  Opaque handles, C types     │
             └──────────────┬───────────────┘
                            │
                            ▼
             ┌──────────────────────────────┐
             │     opengrid-core (Rust)     │
             ├──────────────────────────────┤
             │ Node  │ Ledger │ CRDT        │
             │ Sync  │ Crypto │ Storage     │
             └──────────────────────────────┘
                            │
            ┌───────────────┼───────────────┐
            ▼               ▼               ▼
      (Future)         (Future)         (Future)
  SQLite/RocksDB     BLE Transport   ED25519 Signing
```

---

## Component Overview

### 1. Rust Core Engine (`core/`)

The central library. Platform-agnostic. Never touches IO directly.

**Sub-modules:**

| Module | Responsibility |
|---|---|
| `node` | Identity (key pair) + local event log |
| `ledger` | Domain-specific CRDT-backed state |
| `crdt` | Mathematical merge contracts (associativity, commutativity, idempotency) |
| `sync` | Anti-entropy message protocol (VersionVector, SyncMessage) |
| `crypto` | Signing + verification trait abstractions |
| `storage` | Append-only log trait + in-memory implementation |
| `error` | Unified error enum (`CoreError`) |

**→ [Core Engine Documentation](core-engine.md)**

---

### 2. FFI Layer (`ffi/`)

Exposes the Rust core to mobile platforms through a C-compatible ABI using [Diplomat](https://github.com/rust-diplomat/diplomat).

**Key design constraints:**

- Only **opaque handles** are exposed — no raw Rust structs
- **No shared memory** — all data is copied across the boundary
- **No async** across the boundary — callers must dispatch to threads
- All errors returned as value types (not Rust panics)

**Exposed types:**
- `OpenGridEngine` — factory for creating nodes
- `NodeHandle` — opaque reference to a mesh node

**→ [FFI Documentation](ffi.md)**

---

### 3. Android Client (`android/`)

A Kotlin + Jetpack Compose app wired to the FFI layer via a manual JNI bridge.

**Stack:**
```
Compose UI → ViewModel (Coroutines) → DiplomatBridge.kt (JNI) → libopengrid_ffi.so
```

**→ [Android Documentation](android.md)**

---

### 4. iOS Client (`ios/`)

A Swift + SwiftUI app wired to the FFI layer via Diplomat-generated Swift bindings.

**Stack:**
```
SwiftUI → ObservableObject (GCD) → Swift wrappers → libopengrid_ffi.a
```

**→ [iOS Documentation](ios.md)**

---

## Data Flow

### Event Submission (Local)

```
User action in UI
    │
    ▼
ViewModel / ObservableObject
    │ dispatches to background thread
    ▼
FFI: NodeHandle.submit_event(payload: &[u8]) -> u64
    │
    ▼
core::Node::submit_event
    │ signs payload (TODO)
    ▼
core::storage::AppendLog::append
    │
    ▼
Returns log index (version)
    │
    ▼
UI updates status via LiveData / @Published
```

### Sync (Gossip Anti-Entropy)

```
Node A                                Node B
  │                                     │
  │   SyncMessage::Hello { my_vector }  │
  │ ────────────────────────────────►   │
  │                                     │ computes diff
  │   SyncMessage::Updates { events }   │
  │ ◄────────────────────────────────   │
  │                                     │
  │ applies events to local log         │
  │                                     │
  │   SyncMessage::Hello { new_vector } │
  │ ────────────────────────────────►   │ (round 2)
```

The sync process is **transport-agnostic**. The same messages can flow over:
- BLE (Bluetooth Low Energy) — short proximity bursts
- Wi-Fi Direct — larger batch transfers
- QUIC / TCP — server-to-server or backbone relay

---

## CRDT and Ledger Model

### Why CRDTs?

In a system where:
- Nodes go offline for hours or days
- Messages can arrive out of order or not at all
- There is no central coordinator

...traditional locking, transactions, and consensus algorithms fail. CRDTs (Conflict-free Replicated Data Types) solve this by making **merge a mathematical operation** that:

- Is commutative (order doesn't matter)
- Is associative (grouping doesn't matter)
- Is idempotent (applying the same thing twice doesn't change the result)

This guarantees that any set of nodes, sharing any subset of events, will always reach the same final state when they eventually sync.

### Ledger vs. CRDT

```
┌────────────────────────────────────┐
│              Ledger                │
│  (domain model: Resource Exchange) │
│                                    │
│  Built ON TOP of a CRDT           │
│  Reconstructed by replaying log   │
│  Never stored as primary state    │
└────────────────────────────────────┘
           │ implements
           ▼
┌────────────────────────────────────┐
│              CRDT                  │
│  (mathematical guarantees)         │
│                                    │
│  CrdtMerge + Crdt traits          │
│  Associativity, Commutativity,    │
│  Idempotency                      │
└────────────────────────────────────┘
```

---

## Storage Model

All state is stored as an **append-only log of signed, raw byte events**.

```
Log index:  0          1          2          3
            │          │          │          │
            ▼          ▼          ▼          ▼
         event_0    event_1    event_2    event_3
         (bytes)    (bytes)    (bytes)    (bytes)
```

Properties:
- **Immutable** — entries are never modified after writing
- **Ordered** — index is monotonically increasing
- **Replayed** — ledger state is rebuilt by replaying the log from index 0
- **Signed** — each event is signed by the originating node (future implementation)
- **Transport-safe** — events are raw bytes; format is application-defined

This model enables:
- **Crash recovery** — replay the log to restore state
- **Partial sync** — exchange only the log slice the peer is missing
- **Audit trail** — full history is always preserved

---

## Cryptographic Identity Model

Each node has a stable identity anchored to a **cryptographic key pair**:

```
┌─────────────────────────────────────┐
│              Node                   │
│                                     │
│  id = public_key_bytes (32 bytes)   │
│  _signer = Box<dyn Signer>          │
│                                     │
│  Events signed with private key     │
│  Identity verified by public key    │
└─────────────────────────────────────┘
```

**Properties:**
- Two nodes with the same public key are the same identity
- The private key never leaves the node
- On mobile, keys should be stored in the platform keystore (Android Keystore / iOS Secure Enclave)
- There is no PKI, no CA, and no certificate chain — identity is self-sovereign

---

## Failure Model

The system explicitly assumes and tolerates:

| Failure Mode | How OpenGrid Handles It |
|---|---|
| Node goes offline mid-operation | Local log is consistent; sync resumes when back online |
| Message arrives out of order | Version vectors track what each node has seen |
| Message is duplicated | CRDT idempotency absorbs duplicates without side effects |
| Clocks are unreliable / skewed | Clock time is never used for ordering; log index is sufficient |
| Storage is interrupted | Append-only log survives partial writes at restart |
| Remote node is dishonest | Cryptographic signatures allow rejection of unsigned events |

---

## Non-Goals

OpenGrid **deliberately does not** implement:

| Non-Goal | Reason |
|---|---|
| Global consensus | CAP theorem: consistency sacrificed for AP |
| Blockchain | Too heavyweight; no mining or token system |
| Real-time guarantees | Network is unreliable; best-effort delivery only |
| Centralized identity / user accounts | Self-sovereign identity only |
| Mandatory cloud dependency | The system must work with zero internet |
| Global ordering of events | Local ordering per node; cross-node ordering via vector clocks only |

---

## Long-Term Evolution

OpenGrid is designed to grow to:

| Deployment Target | Notes |
|---|---|
| Mobile phones | Primary target — Android + iOS |
| Embedded mesh routers | RISC-V / ARM embedded; `no_std` Rust |
| Satellite uplink nodes | High-latency; large batch sync |
| Disaster response kits | Air-gapped; BLE-only mesh |
| NGO / rural deployments | Minimal infrastructure; high reliability |

The Rust core engine is intended to **outlive any specific UI or transport implementation**.

---

## Cross-Cutting Concerns

### Logging

Currently: `println!` / stderr only.
Planned: trait-injected logger compatible with `android_logger` and `os_log`.

### Observability

Planned: a `Metrics` trait allowing the engine to emit counters (events written, merges performed, bytes synced) to any observer.

### Versioning

The sync protocol includes `VersionVector` for peer state knowledge. Breaking changes to the event format will require a version field in the event envelope.

---

## Summary

```
Principle               Implementation
─────────────────────   ─────────────────────────────────────────────
Offline-first           All ops local; sync is opportunistic add-on
No central authority    Gossip sync; no required server
Append-only             MemoryLog → future: SQLite, RocksDB
Deterministic merges    CRDT traits enforce mathematical properties
Explicit boundaries     FFI layer: opaque handles, no shared memory
UI = snapshot renderer  All state in Rust; UI polls snapshots
```

**OpenGrid is correct first, fast second, and networked third.**