# OpenGrid System Architecture

## Overview

OpenGrid is an offline-first, partition-tolerant resource exchange system designed to operate in environments with unreliable or absent centralized infrastructure.

The system enables hyper-local peer-to-peer resource coordination (energy, data, water, etc.) using:

- local mesh networking
- CRDT-based ledgers
- append-only event logs

OpenGrid prioritizes resilience, correctness, and local autonomy over throughput or central coordination.

## Design Principles

- Offline-first by default
- No central authority
- Append-only data model
- Deterministic convergence
- Explicit trust boundaries
- UI is never the source of truth

## High-Level Architecture

```
+-------------------+
|   Mobile UI       |
| (Compose / Swift) |
+-------------------+
          |
          | FFI (UniFFI)
          v
+-------------------+
|   OpenGrid Core   |
|   (Rust Engine)   |
+-------------------+
     |      |      |
     v      v      v
 CRDT   Storage   Sync
```

The Rust core is the authoritative engine. All platforms interact with it through a stable FFI boundary.

## Core Components

### 1. Rust Core Engine (core/)

The core engine implements:

- node identity
- CRDT-based ledgers
- merge logic
- sync orchestration
- cryptographic verification

It is platform-agnostic and runs on:

- mobile devices
- servers
- embedded hardware

### 2. CRDT Layer

OpenGrid uses operation-based CRDTs to guarantee convergence under:

- offline operation
- message reordering
- partial replication

Key characteristics:

- no deletes
- no in-place mutation
- idempotent merges

Balances are derived, never stored.

### 3. Storage Layer

Storage is modeled as:

- append-only event logs
- immutable records

Responsibilities:

- persistence
- replay
- recovery after crashes

The storage interface is abstract and replaceable.

### 4. Sync Layer

The sync layer:

- exchanges state opportunistically
- uses gossip-style anti-entropy
- assumes no reliable ordering or delivery

Transport is abstracted and implemented by:

- BLE / Wi-Fi Direct (mobile)
- TCP / QUIC (servers)

### 5. Cryptography

All events are:

- signed by their origin node
- verifiable offline

Crypto is treated as a core primitive, not an add-on.

## FFI Boundary (ffi/)

The FFI layer exposes:

- opaque engine handles
- command-style APIs
- snapshot-style read APIs

### FFI Design Rules

- No Rust structs exposed
- No shared memory
- No async across boundary
- All data is copied

Example API categories:

- create_node
- submit_event
- import_remote_events
- get_state_snapshot

## UI Architecture

### Android (Jetpack Compose)

```
Compose UI
   ↓
ViewModel
   ↓
Rust FFI
   ↓
OpenGrid Core
```

UI renders snapshots
ViewModel polls engine state
Platform services feed events into Rust

### iOS (SwiftUI)

```
SwiftUI
   ↓
ObservableObject
   ↓
Rust FFI
   ↓
OpenGrid Core
```

Identical model to Android.

### Server Node (server/)

Servers run the same Rust core with:

- persistent storage
- stable network transport

They act as:

- relay nodes
- long-term storage peers
- inter-mesh bridges

Servers are not authoritative.

## Failure Model

The system assumes:

- nodes disappear without warning
- messages arrive late or duplicated
- clocks are unreliable
- storage may be interrupted

Correctness is maintained via:

- CRDT convergence
- append-only logs
- deterministic merges

## Non-Goals

- Global consensus
- Blockchain
- Real-time guarantees
- Centralized identity
- Mandatory cloud dependency

## Long-Term Evolution

OpenGrid is designed to expand to:

- embedded mesh routers
- satellite uplink nodes
- disaster response kits
- NGO and rural deployments

The Rust core is intended to outlive any specific UI.

## Summary

OpenGrid is a distributed systems engine, not an app. UIs are replaceable. Networks are unreliable. The core must always converge.