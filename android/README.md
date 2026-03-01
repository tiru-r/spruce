# Spruce Android Development Setup

## Prerequisites

1. **Android Studio** (latest version)
2. **Android SDK** (API 26+)  
3. **Android NDK** (version 25+)
4. **Rust** with Android targets

## Environment Setup

### 1. Install Rust Android Targets

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

### 2. Configure NDK Toolchain

Set up your environment variables. Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# Android SDK/NDK paths (adjust to your installation)
export ANDROID_HOME="$HOME/Android/Sdk"
export ANDROID_NDK_ROOT="$ANDROID_HOME/ndk/25.2.9519653"  # or latest version
export PATH="$ANDROID_HOME/cmdline-tools/latest/bin:$PATH"
export PATH="$ANDROID_HOME/platform-tools:$PATH"

# NDK toolchain
export CC_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang"
export CXX_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android21-clang++"
export AR_aarch64_linux_android="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

export CC_armv7_linux_androideabi="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
export CXX_armv7_linux_androideabi="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang++"
export AR_armv7_linux_androideabi="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
```

### 3. Cargo Configuration

Create `~/.cargo/config.toml` with Android linker settings:

```toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android21-clang"

[target.armv7-linux-androideabi] 
linker = "armv7a-linux-androideabi21-clang"
```

## Build Instructions

### Option 1: Automated Build Script

```bash
cd /path/to/spruce/android
./build_native.sh
```

### Option 2: Manual Build

1. **Build Rust Libraries**:
```bash
cd /path/to/spruce
cargo build --target aarch64-linux-android --release --package spruce-core
cargo build --target armv7-linux-androideabi --release --package spruce-core
```

2. **Copy Libraries**:
```bash
cp target/aarch64-linux-android/release/libspruce_core.so android/app/src/main/jniLibs/arm64-v8a/
cp target/armv7-linux-androideabi/release/libspruce_core.so android/app/src/main/jniLibs/armeabi-v7a/
```

3. **Build APK**:
```bash
cd android
./gradlew assembleDebug
```

## Android Studio Setup

1. Open the `android/` folder in Android Studio
2. Sync project with Gradle files
3. Build and run on device or emulator

## Testing

### Logcat Output
Monitor native logs with:
```bash
adb logcat -s SpruceNative
```

Expected output:
```
I/SpruceNative: 🚀 Initializing Spruce Android application
I/SpruceNative: ✅ Spruce Android application initialized successfully
I/SpruceNative: 🎨 Initializing native surface: 1080x1920
I/SpruceNative: ✅ Surface initialized successfully
```

## Troubleshooting

### NDK Not Found
- Install Android NDK through Android Studio SDK Manager
- Verify `ANDROID_NDK_ROOT` path is correct
- Ensure NDK toolchain binaries exist

### Linker Errors
- Check NDK version compatibility
- Verify environment variables are set correctly
- Try rebuilding with `cargo clean` first

### JNI Errors
- Confirm library is loaded in MainActivity
- Check JNI method signatures match Rust exports
- Verify library is copied to correct architecture folder

## Architecture

```
┌─────────────────────────────────────┐
│          Android Activity          │
│         (MainActivity.kt)           │
└─────────────┬───────────────────────┘
              │ JNI calls
┌─────────────▼───────────────────────┐
│       Native Library (.so)         │
│      (libspruce_core.so)            │
└─────────────┬───────────────────────┘
              │
┌─────────────▼───────────────────────┐
│       Spruce Rust Runtime          │
│   • AndroidApplication              │
│   • Surface Rendering               │
│   • Vue 3.6 Vapor Mode             │
│   • Pure Rust UI                   │
└─────────────────────────────────────┘
```

## Current Status

✅ **Rust compilation** - Core library compiles for Android targets
✅ **JNI interface** - Native methods defined and exported  
✅ **Android project** - Complete APK project structure
✅ **Build pipeline** - Automated build script
🚧 **Testing** - Needs Android device/emulator testing
🚧 **Surface rendering** - Basic implementation ready
🚧 **Vue integration** - Template compilation in progress

This is experimental research code for exploring native Vue.js applications on Android.