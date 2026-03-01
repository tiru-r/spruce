/// JNI Interface for Android MainActivity Integration
/// 
/// This module provides the C-compatible JNI functions that the Android
/// MainActivity calls to interact with the Rust Spruce implementation.

use crate::android::{get_android_app, AndroidApplication};
use anyhow::Result;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jint, jobject, jstring};
use jni::JNIEnv;
use std::sync::Arc;
use tracing;

/// Initialize the native Spruce application
/// Called from MainActivity.initializeNativeApp()
#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_initializeNativeApp(
    env: JNIEnv,
    _class: JClass,
) -> jboolean {
    // Initialize Android logging
    if let Err(e) = init_logging() {
        eprintln!("Failed to initialize logging: {}", e);
        return 0; // false
    }
    
    tracing::info!("🚀 Initializing Spruce Android application");
    
    // Initialize the global Android application
    let app = get_android_app();
    
    // Store JNI environment for future use
    if let Err(e) = initialize_jni_bridge(&env) {
        tracing::error!("❌ Failed to initialize JNI bridge: {}", e);
        return 0; // false
    }
    
    tracing::info!("✅ Spruce Android application initialized successfully");
    1 // true
}

/// Cleanup the native Spruce application
/// Called from MainActivity.cleanupNativeApp()
#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_cleanupNativeApp(
    _env: JNIEnv,
    _class: JClass,
) {
    tracing::info!("🧹 Cleaning up Spruce Android application");
    // Cleanup is handled by Drop implementations
}

// Lifecycle JNI methods

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnCreate(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_create() {
        tracing::error!("❌ onCreate failed: {}", e);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnStart(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_start() {
        tracing::error!("❌ onStart failed: {}", e);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnResume(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_resume() {
        tracing::error!("❌ onResume failed: {}", e);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnPause(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_pause() {
        tracing::error!("❌ onPause failed: {}", e);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnStop(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_stop() {
        tracing::error!("❌ onStop failed: {}", e);
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeOnDestroy(
    _env: JNIEnv,
    _class: JClass,
) {
    let app = get_android_app();
    if let Err(e) = app.on_destroy() {
        tracing::error!("❌ onDestroy failed: {}", e);
    }
}

// Surface JNI methods

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeInitSurface(
    _env: JNIEnv,
    _class: JClass,
    surface: jobject,
    width: jint,
    height: jint,
) {
    let app = get_android_app();
    
    tracing::info!("🎨 Initializing native surface: {}x{}", width, height);
    
    // Convert Android Surface to native window pointer
    let native_window = surface as *mut std::ffi::c_void;
    
    if let Err(e) = app.init_surface(native_window, width as u32, height as u32) {
        tracing::error!("❌ Failed to initialize surface: {}", e);
    } else {
        tracing::info!("✅ Surface initialized successfully");
    }
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeDestroySurface(
    _env: JNIEnv,
    _class: JClass,
) {
    tracing::info!("🧹 Destroying native surface");
    // Surface cleanup is handled automatically by the AndroidApplication
}

#[no_mangle]
pub extern "C" fn Java_com_spruce_app_MainActivity_nativeGetDeviceInfo<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    let app = get_android_app();
    let device_info = app.get_device_info();
    
    let info_json = serde_json::to_string(&device_info).unwrap_or_else(|_| {
        r#"{"error": "Failed to serialize device info"}"#.to_string()
    });
    
    env.new_string(info_json).unwrap_or_else(|e| {
        tracing::error!("Failed to create JString: {}", e);
        env.new_string("{}").unwrap()
    })
}

// Utility functions

fn init_logging() -> Result<()> {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("SpruceNative")
    );
    Ok(())
}

fn initialize_jni_bridge(env: &JNIEnv) -> Result<()> {
    // Initialize JNI bridge with the current JavaVM
    let jvm = env.get_java_vm()?;
    let bridge = crate::android::jni_bridge::get_jni_bridge();
    bridge.init_with_jvm(Arc::new(jvm))?;
    Ok(())
}

// Export C-compatible initialization function for older Android versions
#[no_mangle]
pub extern "C" fn JNI_OnLoad(vm: *mut jni::sys::JavaVM, _reserved: *mut std::ffi::c_void) -> jni::sys::jint {
    tracing::info!("🔧 JNI_OnLoad called");
    
    // Return JNI version
    jni::sys::JNI_VERSION_1_6
}

#[no_mangle]
pub extern "C" fn JNI_OnUnload(_vm: *mut jni::sys::JavaVM, _reserved: *mut std::ffi::c_void) {
    tracing::info!("🧹 JNI_OnUnload called");
}