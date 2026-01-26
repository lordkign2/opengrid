# OpenGrid Android Client

This is the Android client for the OpenGrid system, built with Jetpack Compose and integrated with the Rust core engine via UniFFI.

## Architecture

The Android app follows a strict architectural pattern to maintain separation of concerns:

```
Compose UI
   ↓
ViewModel (OpenGridViewModel)
   ↓
Rust UniFFI bindings (opengrid_ffi.kt)
```

## Key Principles

1. **Rust owns all state**: The UI never stores authoritative data; it only requests snapshots from the engine
2. **Command-based interaction**: UI sends commands and polls results; no callbacks from Rust into UI
3. **Snapshot-based rendering**: UI renders immutable copies of state; no mutation of snapshot data
4. **Non-blocking UI**: All Rust calls are dispatched off the main thread

## What Must Never Be Added to UI Layer

- ❌ CRDT logic implementation
- ❌ Network logic
- ❌ State storage in UI layer
- ❌ Direct Bluetooth/WiFi usage (belongs in Rust layer)
- ❌ Business logic duplication

## Setup Instructions

1. Make sure Rust and UniFFI are installed
2. Generate the UniFFI Kotlin bindings
3. Build the project

## Project Structure

- `app/src/main/java/com/opengrid/android/ui/MainActivity.kt` - Main activity
- `app/src/main/java/com/opengrid/android/ui/OpenGridViewModel.kt` - ViewModel that interfaces with Rust
- `app/src/main/java/com/opengrid/android/ui/MainActivity.kt` - Compose UI
- `app/src/main/java/com/opengrid/android/ffi/opengrid_ffi.kt` - Auto-generated UniFFI bindings