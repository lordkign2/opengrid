# OpenGrid

> **An offline-first, decentralized mesh resource exchange protocol engine, written in Rust.**

OpenGrid enables hyper-local peer-to-peer resource coordination (energy, data, connectivity, etc.) across mobile devices and edge nodes — with **zero dependency on centralized infrastructure**. It is built for environments where network reliability cannot be assumed.

---

## Table of Contents

- [Overview](#overview)
- [Design Principles](#design-principles)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Components](#components)
  - [Rust Core Engine](#rust-core-engine-core)
  - [FFI Layer (Diplomat)](#ffi-layer-diplomat-ffi)
  - [Android Client](#android-client-android)
  - [iOS Client](#ios-client-ios)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Building the Core Engine](#building-the-core-engine)
  - [Building Android](#building-android)
  - [Building iOS](#building-ios)
- [Testing](#testing)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [License](#license)

---

## Overview

OpenGrid is a **distributed systems engine**, not an app. It is designed as a portable Rust library that any UI platform can bind to via a stable, version-controlled FFI boundary.

Key characteristics:

| Property | Description |
|---|---|
| **Offline-first** | All operations are local by default. Sync happens opportunistically. |
| **AP system** | Favors Availability and Partition Tolerance (CAP theorem). |
| **CRDT-based** | All shared state converges deterministically, regardless of operation order. |
| **Append-only** | No deletes, no in-place mutations. History is always preserved. |
| **Portable** | Runs on Android, iOS, servers, and embedded hardware from a single Rust core. |

---

## Design Principles

1. **Rust owns all state.** The UI is never the source of truth. It only requests snapshots from the engine.
2. **No central authority.** There is no server that must be reachable for the system to function.
3. **Explicit boundaries.** The FFI layer is a hard contract. No Rust types leak into UI code.
4. **Append-only data model.** Events are signed and immutable once written.
5. **Deterministic convergence.** Two nodes with the same set of events always reach the same state.
6. **Correctness over performance.** Mathematical CRDT properties are enforced through traits.

---

## Architecture

```
┌─────────────────────────┐   ┌─────────────────────────┐
│      Android UI         │   │        iOS UI            │
│   (Jetpack Compose)     │   │       (SwiftUI)          │
└────────────┬────────────┘   └────────────┬────────────┘
             │ ViewModel                   │ ObservableObject
             │ (LiveData / Coroutines)     │ (Combine)
             ▼                             ▼
┌─────────────────────────┐   ┌─────────────────────────┐
│    DiplomatBridge.kt    │   │  OpenGridObserver.swift  │
│     (JNI wrappers)      │   │    (Swift wrappers)      │
└────────────┬────────────┘   └────────────┬────────────┘
             │                             │
             └──────────────┬──────────────┘
                            │  FFI (Diplomat / C-ABI)
                            ▼
             ┌──────────────────────────────┐
             │       opengrid-ffi (Rust)    │
             │   Opaque handles & C types   │
             └──────────────┬───────────────┘
                            │
                            ▼
             ┌──────────────────────────────┐
             │    opengrid-core (Rust)      │
             │                              │
             │  ┌──────┐ ┌───────┐ ┌─────┐ │
             │  │ CRDT │ │Storage│ │ Sync│ │
             │  └──────┘ └───────┘ └─────┘ │
             │  ┌──────┐ ┌───────┐         │
             │  │ Node │ │Crypto │         │
             │  └──────┘ └───────┘         │
             └──────────────────────────────┘
```

The Rust core engine is the single source of truth. All platforms interact with it exclusively through the FFI boundary — no logic is duplicated in the UI layer.

---

## Project Structure

```
opengrid/
├── Cargo.toml                  # Rust workspace manifest
├── Cargo.lock                  # Locked dependency tree
├── package.json                # Node.js tooling (Husky git hooks)
├── validate.py                 # Workspace structure validator
│
├── core/                       # Core Rust engine (no IO, no UI)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Public API surface
│       ├── node.rs             # Node identity & lifecycle
│       ├── ledger.rs           # Ledger trait (CRDT-backed domains)
│       ├── error.rs            # Unified error model
│       ├── crdt/
│       │   ├── mod.rs
│       │   └── traits.rs       # CrdtMerge & Crdt traits
│       ├── sync/
│       │   ├── mod.rs
│       │   └── protocol.rs     # SyncMessage, VersionVector
│       ├── crypto/
│       │   └── mod.rs          # Signer, Verifier traits
│       └── storage/
│           └── mod.rs          # AppendLog trait + MemoryLog
│
├── ffi/                        # FFI crate — Diplomat bindings
│   ├── Cargo.toml
│   ├── diplomat.toml           # Diplomat config (Kotlin target)
│   ├── src/
│   │   └── lib.rs              # #[diplomat::bridge] definitions
│   └── generated/
│       └── kotlin/             # Auto-generated Kotlin bindings
│
├── android/                    # Android application (Kotlin + Compose)
│   ├── README.md
│   ├── build.gradle
│   ├── settings.gradle
│   └── app/
│       └── src/main/
│           ├── AndroidManifest.xml
│           └── java/com/opengrid/android/
│               ├── ffi/DiplomatBridge.kt  # JNI wrapper layer
│               ├── ui/
│               │   ├── MainActivity.kt
│               │   └── OpenGridViewModel.kt
│               └── theme/Theme.kt
│
├── ios/                        # iOS application (Swift + SwiftUI)
│   ├── README.md
│   ├── Package.swift
│   └── Sources/OpenGridIOS/
│       ├── ContentView.swift
│       ├── OpenGridObserver.swift
│       └── OpenGridIOSApp.swift
│
├── docs/                       # Project documentation
│   ├── architecture.md
│   ├── core-engine.md
│   ├── ffi.md
│   ├── android.md
│   └── ios.md
│
└── scripts/
    └── verify_all.py           # Integration test runner
```

---

## Components

### Rust Core Engine (`core/`)

The heart of OpenGrid. A pure Rust library with:

- **No IO** — storage is abstracted behind traits
- **No networking** — sync transport is abstracted
- **No UI** — zero UI framework dependencies
- **No `unsafe`** — fully safe Rust (except FFI boundary)

Key modules:

| Module | Purpose |
|---|---|
| `node` | Represents a mesh participant: identity (key pair) + local log |
| `ledger` | Trait-based domain state backed by CRDTs |
| `crdt` | Mathematical CRDT contracts (`CrdtMerge`, `Crdt`) |
| `sync` | Anti-entropy protocol messages and version vectors |
| `storage` | Append-only log trait + in-memory implementation |
| `crypto` | `Signer` / `Verifier` traits and placeholder Ed25519 impl |
| `error` | Unified `CoreError` enum used across all modules |

**[→ Detailed core engine documentation](docs/core-engine.md)**

---

### FFI Layer / Diplomat (`ffi/`)

Exposes the Rust core to mobile platforms via a **C-compatible ABI** using the [Diplomat](https://github.com/rust-diplomat/diplomat) framework.

Key design rules:
- No Rust structs exposed directly — only opaque handles
- No shared memory between Rust and host language
- No async across the FFI boundary
- All data is copied, never borrowed across the boundary

Exposed types:
- `OpenGridEngine` — top-level engine factory
- `NodeHandle` — opaque handle for a mesh node

**[→ Detailed FFI documentation](docs/ffi.md)**

---

### Android Client (`android/`)

A Kotlin + Jetpack Compose Android application that integrates with the Rust engine.

Technology stack:
- **Language**: Kotlin
- **UI**: Jetpack Compose + Material3
- **State**: ViewModel + LiveData + Kotlin Coroutines
- **FFI**: Manual JNI bridge (`DiplomatBridge.kt`) loading `libopengrid_ffi.so`

Minimum SDK: **API 24 (Android 7.0)**
Target SDK: **API 34 (Android 14)**

**[→ Detailed Android documentation](docs/android.md)**

---

### iOS Client (`ios/`)

A Swift + SwiftUI iOS application that integrates with the Rust engine.

Technology stack:
- **Language**: Swift
- **UI**: SwiftUI
- **State**: ObservableObject + Combine (`@Published`)
- **FFI**: Swift wrappers generated via Diplomat (targeting Swift 5.7+)
- **Package Manager**: Swift Package Manager

Minimum deployment target: **iOS 14**

**[→ Detailed iOS documentation](docs/ios.md)**

---

## Getting Started

### Prerequisites

| Tool | Version | Purpose |
|---|---|---|
| [Rust](https://rustup.rs/) | stable (≥ 1.70) | Core engine build |
| [Cargo](https://doc.rust-lang.org/cargo/) | bundled with Rust | Build tool |
| [Android Studio](https://developer.android.com/studio) | Latest | Android development |
| [Android NDK](https://developer.android.com/ndk) | r26+ | Cross-compilation |
| [Xcode](https://developer.apple.com/xcode/) | 15+ | iOS development |
| [diplomat-tool](https://github.com/rust-diplomat/diplomat) | 0.14 | FFI binding generation |
| Python | 3.8+ | Validation scripts |

Install Rust targets for mobile cross-compilation:

```bash
# Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# iOS targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim
```

---

### Building the Core Engine

```bash
# Clone the repository
git clone https://github.com/lordkign2/opengrid.git
cd opengrid

# Build the entire workspace
cargo build

# Build in release mode
cargo build --release

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

---

### Building Android

1. **Compile the Rust engine for Android targets:**

```bash
export ANDROID_NDK_HOME=/path/to/ndk

cargo build --target aarch64-linux-android --release -p opengrid-ffi
cargo build --target x86_64-linux-android --release -p opengrid-ffi  # For emulator
```

2. **Copy the native library to the Android project:**

```bash
cp target/aarch64-linux-android/release/libopengrid_ffi.so android/app/src/main/jniLibs/arm64-v8a/
cp target/x86_64-linux-android/release/libopengrid_ffi.so android/app/src/main/jniLibs/x86_64/
```

3. **Open `android/` in Android Studio and sync the Gradle project.**

4. **Run on an emulator or connected device:**

```bash
# From the android/ directory
./gradlew installDebug
```

---

### Building iOS

1. **Compile the Rust engine for iOS targets:**

```bash
cargo build --target aarch64-apple-ios --release -p opengrid-ffi
cargo build --target aarch64-apple-ios-sim --release -p opengrid-ffi
```

2. **Open `ios/` as a Swift Package in Xcode.**

3. **Link the compiled `.a` library to the Xcode target.**

4. **Build and run** on a simulator or device.

---

## Testing

### Unit Tests (Rust)

```bash
# Run all core engine tests
cargo test -p opengrid-core

# Run all FFI tests
cargo test -p opengrid-ffi

# Run entire workspace
cargo test
```

### Validation Script

```bash
python validate.py
```

### Integration Test Suite

```bash
# Runs all platform tests (requires platform SDKs installed)
npm run test:all
# or
python scripts/verify_all.py
```

### Android Tests

```bash
cd android
./gradlew test                    # Unit tests
./gradlew connectedAndroidTest    # Instrumented tests (device required)
```

---

## Documentation

Full documentation is available in the [`docs/`](docs/) directory:

| Document | Description |
|---|---|
| [architecture.md](docs/architecture.md) | System-wide architecture overview |
| [core-engine.md](docs/core-engine.md) | Rust core engine internals, modules, and extension points |
| [ffi.md](docs/ffi.md) | FFI layer, Diplomat configuration, and binding generation |
| [android.md](docs/android.md) | Android app architecture, JNI setup, and build instructions |
| [ios.md](docs/ios.md) | iOS app architecture, Swift bindings, and build instructions |

---

## Roadmap

### Phase 1 — Foundation ✅ (Current)
- [x] Rust workspace structure
- [x] Core engine modules (CRDT, sync, storage, crypto, node)
- [x] Diplomat FFI bridge
- [x] Android client (Kotlin + Compose)
- [x] iOS client (Swift + SwiftUI)

### Phase 2 — Data Layer
- [ ] Concrete CRDT implementations (G-Counter, MVReg, LWW-Element-Set)
- [ ] Persistent storage backend (SQLite or RocksDB)
- [ ] Event signing with real Ed25519 keys

### Phase 3 — Networking
- [ ] BLE transport (Android + iOS)
- [ ] Wi-Fi Direct transport (Android)
- [ ] QUIC transport (server nodes)
- [ ] Gossip-based anti-entropy protocol

### Phase 4 — Production Hardening
- [ ] Full test coverage (unit + integration + property-based)
- [ ] CI/CD pipelines
- [ ] Performance benchmarks
- [ ] Security audit

---

## License

MIT — see [LICENSE](LICENSE) for details.

---

> **OpenGrid is a distributed systems engine, not an app. UIs are replaceable. Networks are unreliable. The core must always converge.**