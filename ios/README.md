# OpenGrid iOS Client

This is the iOS client for the OpenGrid system, built with SwiftUI and integrated with the Rust core engine via UniFFI.

## Architecture

The iOS app follows a strict architectural pattern to maintain separation of concerns:

```
SwiftUI View
   ↓
ObservableObject (OpenGridObserver)
   ↓
Rust UniFFI bindings (opengridFFI.swift)
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
- ❌ Background networking
- ❌ Business logic duplication

## Setup Instructions

1. Make sure Rust and UniFFI are installed
2. Generate the UniFFI Swift bindings
3. Build the project

## Project Structure

- `OpenGridIOS/ContentView.swift` - Main SwiftUI view
- `OpenGridIOS/OpenGridObserver.swift` - ObservableObject that interfaces with Rust
- `OpenGridIOS/OpenGridIOSApp.swift` - App entry point
- `OpenGridIOS/opengridFFI.swift` - Auto-generated UniFFI bindings