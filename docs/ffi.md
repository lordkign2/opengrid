# OpenGrid FFI Layer (Diplomat)

> Crate: `opengrid-ffi` | Path: `ffi/`

---

## Overview

The FFI (Foreign Function Interface) layer bridges the Rust core engine with mobile platform languages — Kotlin on Android and Swift on iOS — using the [**Diplomat**](https://github.com/rust-diplomat/diplomat) framework.

Diplomat generates **C-ABI compatible** bindings from a set of annotated Rust structs and impl blocks. Platform-specific wrappers (Kotlin, Swift, C++) are generated automatically from the same definitions.

---

## Why Diplomat?

| Alternative | Problem |
|---|---|
| Raw C bindings | Unsafe, error-prone, no lifecycle management |
| wasm-bindgen | Targets WebAssembly only |
| cbindgen | Generates headers only; no language runtimes |
| UniFFI | Requires UDL files or proc-macros; more complex setup |
| **Diplomat** | Generates idiomatic Kotlin + Swift + C++ from a single Rust definition |

Diplomat enforces a strict **opaque handle** model: callers never see raw Rust structs. All state is owned by Rust and accessed via opaque pointer handles.

---

## Project Structure

```
ffi/
├── Cargo.toml              # FFI crate manifest (cdylib + rlib)
├── diplomat.toml           # Diplomat output configuration
├── src/
│   ├── lib.rs              # Bridge definitions (diplomat::bridge)
│   ├── test_lib.rs         # FFI-layer tests
│   └── debug_udl.rs        # UDL debugging helper
└── generated/
    └── kotlin/             # Auto-generated Kotlin bindings
```

---

## Cargo Configuration

The `opengrid-ffi` crate is compiled as both a `cdylib` (for dynamic linking in Android `.so` / iOS `.dylib`) and an `rlib` (for Rust-to-Rust testing):

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
opengrid-core = { path = "../core" }
diplomat = "0.14"
diplomat-runtime = "0.14"

[build-dependencies]
diplomat-tool = "0.14"
```

---

## Diplomat Configuration (`diplomat.toml`)

```toml
[kotlin]
package = "com.opengrid.generated"
library = "opengrid_ffi"
```

This tells Diplomat to generate Kotlin bindings into the `com.opengrid.generated` package, and to load the native library named `opengrid_ffi` (which maps to `libopengrid_ffi.so` on Android and `libopengrid_ffi.dylib` on iOS).

---

## Bridge Definitions (`src/lib.rs`)

All types exposed across the FFI boundary are defined inside a `#[diplomat::bridge]` module:

```rust
#[diplomat::bridge]
mod ffi {
    use opengrid_core::node::Node;

    #[diplomat::opaque]
    pub struct OpenGridEngine;

    impl OpenGridEngine {
        pub fn new() -> Box<OpenGridEngine> { ... }
        pub fn create_node(&self, _name: &str) -> Box<NodeHandle> { ... }
    }

    #[diplomat::opaque]
    pub struct NodeHandle(pub Box<Node>);

    impl NodeHandle {
        pub fn new_ephemeral() -> Box<NodeHandle> { ... }
        pub fn submit_event(&mut self, payload: &[u8]) -> u64 { ... }
        pub fn current_version(&self) -> u64 { ... }
    }
}
```

### `#[diplomat::opaque]`

Marks a struct as an **opaque handle**. Diplomat will:
1. Allocate the struct on the Rust heap via `Box`
2. Pass a raw pointer across the boundary (as `*mut OpenGridEngine` in C)
3. Generate lifecycle wrappers (create/destroy) in the target language

Platform code **never** sees the struct's fields — it interacts only through the generated methods.

---

## Exposed API

### `OpenGridEngine`

| Method | Signature | Description |
|---|---|---|
| `new` | `() -> Box<OpenGridEngine>` | Create a new engine instance |
| `create_node` | `(&self, name: &str) -> Box<NodeHandle>` | Create a named node handle |

### `NodeHandle`

| Method | Signature | Description |
|---|---|---|
| `new_ephemeral` | `() -> Box<NodeHandle>` | Create a node with random identity & memory storage |
| `submit_event` | `(&mut self, payload: &[u8]) -> u64` | Submit an event to the node's log; returns log index (0 on error) |
| `current_version` | `(&self) -> u64` | Return the current number of entries in the log |

---

## FFI Design Rules

These rules are strictly enforced at the FFI boundary:

### 1. No Rust Types Exposed
Platform code must never see a raw Rust struct, enum, or reference. Everything is an opaque handle.

### 2. No Shared Memory
Data passed across the boundary is **always copied**. There are no shared references between Rust and the host language.

### 3. No Async Across the Boundary
Rust async/futures do not translate to other languages. Callers must dispatch FFI calls to a background thread themselves (which all platform-side code does).

### 4. All Errors Are Values
Return types use `u64` where `0` signals an error. Future versions will use proper error types exposed as Diplomat-compatible enums.

### 5. Ownership Is Explicit
Each `Box<T>` returned from Rust transfers ownership to the caller. The generated platform wrappers destroy the box when the object goes out of scope.

---

## Generated Bindings (`generated/kotlin/`)

Diplomat auto-generates Kotlin bindings from the bridge definitions. These bindings are typically committed to the repository so the Android project doesn't need to rerun the generator on every build.

The generated Kotlin code looks like:

```kotlin
// Auto-generated — DO NOT EDIT
package com.opengrid.generated

class OpenGridEngine internal constructor(internal val handle: Long) {
    companion object {
        @JvmStatic external fun create(): Long
        @JvmStatic external fun destroy(handle: Long)
    }
    
    fun createNode(name: String): NodeHandle {
        return NodeHandle(createNodeNative(handle, name))
    }
}
```

---

## C Header Files

Diplomat also generates C headers that describe the low-level ABI:

| File | Description |
|---|---|
| `OpenGridEngine.h` | C API for `OpenGridEngine` |
| `OpenGridEngine.d.h` | Destructor declarations |
| `NodeHandle.h` | C API for `NodeHandle` |
| `NodeHandle.d.h` | Destructor declarations |
| `diplomat_runtime.h` | Diplomat runtime types (slices, strings, etc.) |

These headers are used by:
- The Swift/C++ wrapper generator
- Any direct C consumer of the library
- iOS bridging headers if using C interop directly

---

## Regenerating Bindings

To regenerate all platform bindings from the bridge definition:

```bash
# 1. Install diplomat-tool
cargo install diplomat-tool@0.14

# 2. Build the FFI crate to ensure it compiles
cargo build -p opengrid-ffi

# 3. Generate Kotlin bindings
diplomat-tool kotlin ffi/src/lib.rs -o ffi/generated/kotlin --library opengrid_ffi

# 4. Generate Swift bindings
diplomat-tool swift ffi/src/lib.rs -o ios/Sources/OpenGridIOS/Generated

# 5. Generate C headers
diplomat-tool c ffi/src/lib.rs -o ffi/
```

---

## Manual JNI Bridge (Android)

Because the Diplomat Kotlin generator may not always produce exactly the right JNI signatures for a given NDK version, the Android app uses a **manual JNI bridge layer** (`DiplomatBridge.kt`) in addition to or instead of auto-generated code.

```kotlin
object DiplomatBridge {
    init { System.loadLibrary("opengrid_ffi") }

    @JvmStatic external fun OpenGridEngine_new(): Long
    @JvmStatic external fun OpenGridEngine_create_node(enginePtr: Long, name: String): Long
    @JvmStatic external fun OpenGridEngine_destroy(enginePtr: Long)

    @JvmStatic external fun NodeHandle_new_ephemeral(): Long
    @JvmStatic external fun NodeHandle_submit_event(nodePtr: Long, payload: ByteArray): Long
    @JvmStatic external fun NodeHandle_current_version(nodePtr: Long): Long
    @JvmStatic external fun NodeHandle_destroy(nodePtr: Long)
}
```

This bridge:
1. Loads the `libopengrid_ffi.so` native library from `jniLibs/`
2. Maps each Kotlin `external fun` declaration to a corresponding exported Rust function
3. Passes raw pointer values as `Long` — a 64-bit integer large enough for any pointer on any Android ABI

Higher-level Kotlin wrapper classes (`OpenGridEngine`, `NodeHandle`) wrap these raw calls to provide a more idiomatic API.

---

## Cross-Compilation

### Android ABI Targets

| ABI | Rust Target | Usage |
|---|---|---|
| `arm64-v8a` | `aarch64-linux-android` | Modern ARM devices (primary) |
| `armeabi-v7a` | `armv7-linux-androideabi` | Legacy ARM devices |
| `x86_64` | `x86_64-linux-android` | Android emulator (x86) |
| `x86` | `i686-linux-android` | Android emulator (x86, legacy) |

```bash
# Install all targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Build for each target
export ANDROID_NDK_HOME=/path/to/ndk
cargo build --target aarch64-linux-android --release -p opengrid-ffi
cargo build --target x86_64-linux-android --release -p opengrid-ffi
```

### iOS Targets

| Arch | Rust Target | Usage |
|---|---|---|
| ARM64 (device) | `aarch64-apple-ios` | Physical iPhone/iPad |
| ARM64 (simulator) | `aarch64-apple-ios-sim` | Simulator on Apple Silicon |
| x86_64 (simulator) | `x86_64-apple-ios` | Simulator on Intel Mac |

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

cargo build --target aarch64-apple-ios --release -p opengrid-ffi
lipo -create \
  target/aarch64-apple-ios-sim/release/libopengrid_ffi.a \
  target/x86_64-apple-ios/release/libopengrid_ffi.a \
  -output target/universal-sim/libopengrid_ffi.a
```

---

## Future API Additions

Planned FFI methods for upcoming features:

```rust
// Planned additions to the bridge:
impl OpenGridEngine {
    pub fn sync_with_peer(&self, peer_data: &[u8]) -> FFIResult<Vec<u8>>;
    pub fn get_state_snapshot(&self) -> Box<StateSnapshot>;
    pub fn import_remote_events(&self, events: &[u8]) -> FFIResult<u64>;
}

impl NodeHandle {
    pub fn get_ledger_balance(&self, ledger_type: u8) -> i64;
    pub fn export_log_slice(&self, from: u64, to: u64) -> Box<EventSlice>;
}
```

---

## Troubleshooting

### `UnsatisfiedLinkError` on Android
- Verify `libopengrid_ffi.so` is placed in the correct `jniLibs/` subdirectory for the target ABI.
- Check the NDK version matches the `ANDROID_NDK_HOME` used during compilation.
- Run `objdump -T libopengrid_ffi.so | grep NodeHandle` to verify symbols are exported.

### `diplomat-tool` not found
```bash
cargo install diplomat-tool --version 0.14
```
Make sure `~/.cargo/bin` is in your `PATH`.

### Diplomat parse errors
- Ensure only `pub` types inside `#[diplomat::bridge]` use Diplomat attributes.
- Verify the Diplomat version in `Cargo.toml` matches the installed `diplomat-tool` version.
- Check that all trait bounds are satisfied (Diplomat cannot bridge arbitrary generic types).
