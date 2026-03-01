#!/bin/bash
# Spruce Android Native Build Script
# 
# This script builds the Rust native libraries for Android and sets up
# the APK build pipeline.

set -e  # Exit on any error

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ANDROID_DIR="$SCRIPT_DIR"

echo "🚀 Building Spruce Android Application"
echo "📍 Project root: $PROJECT_ROOT"
echo "📍 Android dir: $ANDROID_DIR"

# Check required tools
check_tool() {
    if ! command -v "$1" &> /dev/null; then
        echo "❌ $1 is required but not installed"
        exit 1
    fi
    echo "✅ $1 found"
}

echo "🔍 Checking required tools..."
check_tool "cargo"
check_tool "rustc"

# Check Android targets
check_android_target() {
    if ! rustup target list --installed | grep -q "$1"; then
        echo "📦 Installing Android target: $1"
        rustup target add "$1"
    else
        echo "✅ Android target available: $1"
    fi
}

echo "🎯 Checking Rust Android targets..."
check_android_target "aarch64-linux-android"
check_android_target "armv7-linux-androideabi"

# Build Rust libraries
echo "🔨 Building Rust native libraries..."

cd "$PROJECT_ROOT"

# Build for ARM64 (primary architecture)
echo "🏗️ Building for ARM64 (aarch64-linux-android)..."
cargo build --target aarch64-linux-android --release --package spruce-core

# Build for ARMv7 (compatibility)
echo "🏗️ Building for ARMv7 (armv7-linux-androideabi)..."
cargo build --target armv7-linux-androideabi --release --package spruce-core

# Create Android jniLibs directories if they don't exist
mkdir -p "$ANDROID_DIR/app/src/main/jniLibs/arm64-v8a"
mkdir -p "$ANDROID_DIR/app/src/main/jniLibs/armeabi-v7a"

# Copy libraries to Android project
echo "📋 Copying native libraries to Android project..."

ARM64_LIB="$PROJECT_ROOT/target/aarch64-linux-android/release/libspruce_core.so"
ARM7_LIB="$PROJECT_ROOT/target/armv7-linux-androideabi/release/libspruce_core.so"

if [ -f "$ARM64_LIB" ]; then
    cp "$ARM64_LIB" "$ANDROID_DIR/app/src/main/jniLibs/arm64-v8a/"
    echo "✅ ARM64 library copied"
else
    echo "❌ ARM64 library not found: $ARM64_LIB"
    exit 1
fi

if [ -f "$ARM7_LIB" ]; then
    cp "$ARM7_LIB" "$ANDROID_DIR/app/src/main/jniLibs/armeabi-v7a/"
    echo "✅ ARMv7 library copied"
else
    echo "❌ ARMv7 library not found: $ARM7_LIB"
    exit 1
fi

echo "🎉 Native libraries built and copied successfully!"

# Optional: Build APK if Gradle is available
if command -v gradle &> /dev/null; then
    echo "🏗️ Building Android APK..."
    cd "$ANDROID_DIR"
    
    if [ "$1" == "--release" ]; then
        echo "📦 Building release APK..."
        gradle assembleRelease
    else
        echo "🔧 Building debug APK..."
        gradle assembleDebug
    fi
    
    echo "✅ APK build complete!"
    echo "📱 APK location: $ANDROID_DIR/app/build/outputs/apk/"
else
    echo "ℹ️ Gradle not found. Native libraries are ready for manual Android Studio build."
fi

echo ""
echo "🎯 Next steps:"
echo "1. Open '$ANDROID_DIR' in Android Studio"
echo "2. Build and run the project on a device or emulator"
echo "3. Check logcat for 'SpruceNative' messages"
echo ""
echo "✅ Spruce Android build complete!"