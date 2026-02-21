# OpenGrid Android Client

> Path: `android/` | Language: Kotlin | UI: Jetpack Compose

---

## Overview

The Android client is a Kotlin application built with Jetpack Compose that integrates with the OpenGrid Rust core engine via a JNI (Java Native Interface) bridge. It demonstrates the full cross-language stack: Swift/Kotlin UI → JNI/FFI → Rust engine.

---

## Technology Stack

| Component | Technology |
|---|---|
| Language | Kotlin |
| UI framework | Jetpack Compose + Material3 |
| State management | ViewModel + LiveData + Kotlin Coroutines |
| FFI bridge | Manual JNI (`DiplomatBridge.kt`) |
| Native library | `libopengrid_ffi.so` (compiled from Rust) |
| Minimum SDK | API 24 (Android 7.0 Nougat) |
| Target SDK | API 34 (Android 14) |
| Compile SDK | API 34 |
| Build system | Gradle (Groovy DSL) |

---

## Project Structure

```
android/
├── build.gradle            # Top-level Gradle build config
├── settings.gradle         # Project module settings
├── gradle.properties       # Gradle + JVM tuning
└── app/
    ├── build.gradle        # App-level dependencies + Android config
    └── src/
        ├── main/
        │   ├── AndroidManifest.xml
        │   ├── jniLibs/
        │   │   └── x86_64/
        │   │       └── libopengrid_ffi.so      # Pre-compiled Rust library
        │   └── java/com/opengrid/android/
        │       ├── ffi/
        │       │   └── DiplomatBridge.kt       # JNI declarations + high-level wrappers
        │       ├── ui/
        │       │   ├── MainActivity.kt          # Activity + Compose entry point
        │       │   └── OpenGridViewModel.kt     # Engine interface + LiveData state
        │       └── theme/
        │           └── Theme.kt                # Material3 theming
        └── test/
            └── (unit test sources)
```

---

## Architecture

The Android app enforces a strict unidirectional data flow:

```
┌─────────────────────────────────────────┐
│              Compose UI                  │
│  (MainActivity, OpenGridScreen,          │
│   OpenGridContent composables)          │
│                                         │
│  • Renders engineStatus LiveData        │
│  • Dispatches user actions via lambdas  │
└───────────────────┬─────────────────────┘
                    │ observeAsState / onClick callbacks
                    ▼
┌─────────────────────────────────────────┐
│           OpenGridViewModel              │
│  (AndroidViewModel + viewModelScope)    │
│                                         │
│  • Holds: engineStatus LiveData         │
│  • Holds: OpenGridEngine? + NodeHandle? │
│  • Dispatches IO to Dispatchers.IO      │
└───────────────────┬─────────────────────┘
                    │ Kotlin suspend / withContext(Dispatchers.IO)
                    ▼
┌─────────────────────────────────────────┐
│           DiplomatBridge.kt             │
│  (Kotlin object + external funs)        │
│                                         │
│  • Loads libopengrid_ffi.so             │
│  • Declares JNI external functions      │
│  • Wraps raw pointers in Kotlin classes │
└───────────────────┬─────────────────────┘
                    │ JNI (Java Native Interface)
                    ▼
┌─────────────────────────────────────────┐
│         libopengrid_ffi.so              │
│         (Rust FFI crate)                │
│                                         │
│  • OpenGridEngine                       │
│  • NodeHandle                           │
│         ↓                               │
│         opengrid-core                   │
└─────────────────────────────────────────┘
```

---

## Key Files

### `DiplomatBridge.kt`

The lowest-level bridge. Declares external JNI functions and wraps them in Kotlin classes.

```kotlin
object DiplomatBridge {
    init {
        System.loadLibrary("opengrid_ffi")
    }

    // Engine lifecycle
    @JvmStatic external fun OpenGridEngine_new(): Long
    @JvmStatic external fun OpenGridEngine_create_node(enginePtr: Long, name: String): Long
    @JvmStatic external fun OpenGridEngine_destroy(enginePtr: Long)

    // Node lifecycle
    @JvmStatic external fun NodeHandle_new_ephemeral(): Long
    @JvmStatic external fun NodeHandle_submit_event(nodePtr: Long, payload: ByteArray): Long
    @JvmStatic external fun NodeHandle_current_version(nodePtr: Long): Long
    @JvmStatic external fun NodeHandle_destroy(nodePtr: Long)
}
```

**How opaque handles work:**
- Rust allocates the struct on the heap and returns a raw pointer as a `u64`
- Kotlin stores this as `Long` — wide enough to hold any pointer on 32-bit and 64-bit architectures
- When done, Kotlin calls the `_destroy` function to free the Rust heap memory
- The `finalize()` implementations call destroy, but relying on the JVM finalizer is not recommended for large native objects — explicit lifecycle management should be added

### `OpenGridViewModel.kt`

Coordinates the engine lifecycle with Android's ViewModel system:

```kotlin
class OpenGridViewModel(application: Application) : AndroidViewModel(application) {

    private val _engineStatus = MutableLiveData<String>("Not initialized")
    val engineStatus: LiveData<String> = _engineStatus

    private var engine: OpenGridEngine? = null
    private var node: NodeHandle? = null

    fun initializeEngine() {
        viewModelScope.launch {
            withContext(Dispatchers.IO) {
                engine = OpenGridEngine.create()
                node = engine?.createNode("Android Node")
            }
            _engineStatus.value = "Engine initialized successfully"
        }
    }

    fun submitDummyEvent() {
        viewModelScope.launch {
            withContext(Dispatchers.IO) {
                val eventData = "dummy_event_${Date().time}".toByteArray()
                node?.submitEvent(eventData)
                    ?: throw Exception("Node not initialized")
            }
            _engineStatus.value = "Event submitted. Version: ${node?.currentVersion()}"
        }
    }
}
```

**Key design decisions:**
- `withContext(Dispatchers.IO)` ensures Rust calls never block the main thread
- `MutableLiveData` is only written from the main dispatcher after `withContext` returns
- The ViewModel survives configuration changes (screen rotation); the Rust engine goes with it

### `MainActivity.kt`

Entry point. Sets up the ViewModel and passes it to Compose:

```kotlin
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val viewModel = OpenGridViewModel(this.application)
        setContent {
            OpenGridAndroidTheme {
                Surface(modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background) {
                    OpenGridScreen(viewModel = viewModel)
                }
            }
        }
    }
}
```

Compose functions:
- `OpenGridScreen` — observes LiveData via `observeAsState`
- `OpenGridContent` — stateless composable; pure display logic

---

## Native Library Setup

The compiled Rust `.so` file must be placed in the correct `jniLibs` directory for each ABI:

```
app/src/main/jniLibs/
├── arm64-v8a/
│   └── libopengrid_ffi.so     # ARM64 devices (most modern Android phones)
├── armeabi-v7a/
│   └── libopengrid_ffi.so     # 32-bit ARM (legacy devices)
├── x86_64/
│   └── libopengrid_ffi.so     # x86_64 emulator
└── x86/
    └── libopengrid_ffi.so     # x86 emulator (legacy)
```

The `System.loadLibrary("opengrid_ffi")` call will automatically pick the correct `.so` for the running ABI.

---

## Build Setup

### Prerequisites

1. **Android Studio** — Giraffe (2022.3.1) or later
2. **Android NDK** — r26+ (for cross-compilation from Rust)
3. **Rust** — stable toolchain with Android targets:

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi  
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

### Step-by-Step Build

**1. Compile Rust for Android:**

```bash
# From repository root

# Set your NDK path
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/26.1.10909125

# Create linker config if not already present
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[target.aarch64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android24-clang"

[target.x86_64-linux-android]
linker = "$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android24-clang"
EOF

# Build
cargo build --target aarch64-linux-android --release -p opengrid-ffi
cargo build --target x86_64-linux-android --release -p opengrid-ffi
```

**2. Copy libraries to jniLibs:**

```bash
cp target/aarch64-linux-android/release/libopengrid_ffi.so \
   android/app/src/main/jniLibs/arm64-v8a/libopengrid_ffi.so

cp target/x86_64-linux-android/release/libopengrid_ffi.so \
   android/app/src/main/jniLibs/x86_64/libopengrid_ffi.so
```

**3. Open in Android Studio and sync Gradle.**

**4. Run on device or emulator:**

```bash
cd android
./gradlew installDebug
adb shell am start -n com.opengrid.android/.ui.MainActivity
```

---

## Gradle Configuration

### `app/build.gradle`

```groovy
android {
    namespace 'com.opengrid.android'
    compileSdk 34

    defaultConfig {
        applicationId "com.opengrid.android"
        minSdk 24
        targetSdk 34
        versionCode 1
        versionName "1.0"
    }

    buildFeatures {
        compose true
    }
    composeOptions {
        kotlinCompilerExtensionVersion '1.4.3'
    }
}

dependencies {
    implementation 'androidx.core:core-ktx:1.10.1'
    implementation 'androidx.lifecycle:lifecycle-runtime-ktx:2.6.1'
    implementation 'androidx.activity:activity-compose:1.7.2'
    implementation platform('androidx.compose:compose-bom:2023.06.01')
    implementation 'androidx.compose.material3:material3'
    implementation 'androidx.lifecycle:lifecycle-viewmodel-compose:2.6.1'
}
```

---

## Testing

### Unit Tests

```bash
cd android
./gradlew test
```

Unit tests run on the JVM — they cannot test JNI code directly. Mock the `OpenGridEngine` and `NodeHandle` classes for ViewModel tests.

### Instrumented Tests

```bash
# Requires a connected device or running emulator
./gradlew connectedAndroidTest
```

### Manual Test Checklist

- [ ] App launches without crashing
- [ ] "Initialize Engine" button sets status to "Engine initialized successfully"
- [ ] "Submit Dummy Event" increments the version counter
- [ ] Error states display meaningful messages (e.g., submit before init)
- [ ] Screen rotation preserves engine state (ViewModel survives rotation)

---

## Architectural Rules

Things that **must never** appear in the Android UI layer:

| ❌ Prohibited | ✅ Correct approach |
|---|---|
| CRDT logic | Handled entirely in Rust core |
| Sync/networking code | Handled entirely in Rust core |
| State stored in Compose | State is a snapshot from Rust via ViewModel |
| Direct Bluetooth/WiFi usage | Transport is abstracted in Rust layer |
| Business logic duplication | Engine is the single source of truth |
| FFI calls on main thread | All FFI dispatched via `Dispatchers.IO` |

---

## Troubleshooting

### `java.lang.UnsatisfiedLinkError: dlopen failed: library "opengrid_ffi" not found`

- Verify `libopengrid_ffi.so` exists in the correct `jniLibs/<ABI>/` directory.
- Run `./gradlew :app:externalNativeBuildDebug` to check if Gradle finds the native libs.
- Check that the ABI of your emulator/device matches the compiled `.so` (x86_64 emulator needs `jniLibs/x86_64/`).

### App crashes on `OpenGridEngine_new()`

- The Rust function is not correctly exported. Run `nm -D libopengrid_ffi.so | grep OpenGridEngine_new` to verify the symbol is present.
- JNI symbol naming must exactly match: `Java_com_opengrid_android_ffi_DiplomatBridge_OpenGridEngine_1new`.

### LiveData not updating the UI

- Verify you are posting to `_engineStatus.value` on the main dispatcher (after `withContext` returns, you are back on the main dispatcher in `viewModelScope`).

### Compose preview crashes

- The preview composable (`DefaultPreview`) uses static data and should not reference the ViewModel. Ensure the stateless `OpenGridContent` composable is used in previews, not `OpenGridScreen`.
