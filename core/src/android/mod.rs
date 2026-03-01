/// Complete Android Implementation for Spruce
/// 
/// Features:
/// - Pure Rust UI rendering with native Android surface integration
/// - Vue 3.6 Vapor mode compatibility
/// - JNI bindings for Android platform APIs
/// - Direct GPU rendering via ANativeWindow
/// - Touch input handling
/// - Android lifecycle management
/// 
/// Performance target: 60 FPS with 16.67ms frame budget

use anyhow::Result;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, AtomicU32, Ordering}};
use std::collections::HashMap;

pub mod surface;
pub mod input;
pub mod jni_bridge;
pub mod lifecycle;
pub mod renderer;

#[cfg(test)]
mod tests;

use crate::rust_ui::{RustUIRenderer, RustComponent};
use crate::sprucevm::vue36_vapor::{VaporRuntime, VaporTemplate};

/// Main Android application context
pub struct AndroidApplication {
    /// Native window surface for rendering
    surface: Arc<Mutex<Option<surface::AndroidSurface>>>,
    /// Pure Rust UI renderer
    ui_renderer: Arc<Mutex<RustUIRenderer>>,
    /// Vue 3.6 Vapor runtime
    vapor_runtime: Arc<VaporRuntime>,
    /// Input event processor
    input_handler: Arc<input::AndroidInputHandler>,
    /// JNI bridge for Android APIs
    jni_bridge: Arc<jni_bridge::AndroidJNIBridge>,
    /// Application lifecycle state
    lifecycle_state: Arc<lifecycle::AndroidLifecycle>,
    /// Frame rendering state
    is_rendering: AtomicBool,
    /// Frame counter for performance monitoring
    frame_count: AtomicU32,
}

impl AndroidApplication {
    /// Create new Android application with pure Rust UI
    pub fn new() -> Result<Self> {
        tracing::info!("🚀 Initializing Android application with pure Rust UI");

        let surface = Arc::new(Mutex::new(None));
        let ui_renderer = Arc::new(Mutex::new(RustUIRenderer::new()?));
        let vapor_runtime = Arc::new(VaporRuntime::new());
        let input_handler = Arc::new(input::AndroidInputHandler::new());
        let jni_bridge = Arc::new(jni_bridge::AndroidJNIBridge::new());
        let lifecycle_state = Arc::new(lifecycle::AndroidLifecycle::new());

        Ok(Self {
            surface,
            ui_renderer,
            vapor_runtime,
            input_handler,
            jni_bridge,
            lifecycle_state,
            is_rendering: AtomicBool::new(false),
            frame_count: AtomicU32::new(0),
        })
    }

    /// Initialize Android surface for pure Rust rendering
    pub fn init_surface(&self, native_window: *mut std::ffi::c_void, width: u32, height: u32) -> Result<()> {
        tracing::info!("🎨 Initializing Android surface {}x{}", width, height);

        let android_surface = surface::AndroidSurface::new(native_window, width, height)?;
        *self.surface.lock().unwrap() = Some(android_surface);

        // Initialize UI renderer with Android surface
        let mut renderer = self.ui_renderer.lock().unwrap();
        renderer.init_android_surface(width, height)?;

        tracing::info!("✅ Android surface initialized successfully");
        Ok(())
    }

    /// Mount Vue 3.6 Vapor component on Android
    pub fn mount_vapor_app(&self, template: VaporTemplate, container_id: &str) -> Result<()> {
        tracing::info!("🔧 Mounting Vue 3.6 Vapor app: {}", container_id);

        // Mount the Vapor template in the runtime
        self.vapor_runtime.mount_component(template.clone(), container_id)?;

        // Convert Vapor component to pure Rust UI
        let mut renderer = self.ui_renderer.lock().unwrap();
        let rust_component = self.convert_vapor_to_android_ui(&template)?;
        renderer.mount_component(rust_component)?;

        tracing::info!("✅ Vapor app mounted successfully");
        Ok(())
    }

    /// Start the main Android rendering loop
    pub fn start_render_loop(&self) -> Result<()> {
        if self.is_rendering.swap(true, Ordering::SeqCst) {
            return Ok(()); // Already rendering
        }

        tracing::info!("🎬 Starting Android render loop");

        std::thread::spawn({
            let surface = self.surface.clone();
            let ui_renderer = self.ui_renderer.clone();
            let vapor_runtime = self.vapor_runtime.clone();
            let frame_count = Arc::new(AtomicU32::new(0));
            let is_rendering = Arc::new(AtomicBool::new(true));

            move || {
                let mut last_frame_time = std::time::Instant::now();

                while is_rendering.load(Ordering::SeqCst) {
                    let frame_start = std::time::Instant::now();

                    // Process Vapor reactivity updates
                    vapor_runtime.scheduler.flush_effects();

                    // Render frame to Android surface
                    if let Some(ref android_surface) = *surface.lock().unwrap() {
                        if let Ok(mut renderer) = ui_renderer.lock() {
                            if let Err(e) = renderer.render_android_frame(android_surface) {
                                tracing::error!("Frame render error: {}", e);
                            }
                        }
                    }

                    frame_count.fetch_add(1, Ordering::SeqCst);

                    // Maintain 60 FPS (16.67ms per frame)
                    let frame_time = frame_start.elapsed();
                    if frame_time < std::time::Duration::from_millis(16) {
                        std::thread::sleep(std::time::Duration::from_millis(16) - frame_time);
                    }

                    // Log performance metrics every second
                    if last_frame_time.elapsed() >= std::time::Duration::from_secs(1) {
                        let fps = frame_count.swap(0, Ordering::SeqCst);
                        tracing::debug!("🎯 Android FPS: {}, Frame time: {:?}", fps, frame_time);
                        last_frame_time = std::time::Instant::now();
                    }
                }

                tracing::info!("🛑 Android render loop stopped");
            }
        });

        Ok(())
    }

    /// Stop the rendering loop
    pub fn stop_render_loop(&self) {
        self.is_rendering.store(false, Ordering::SeqCst);
    }

    /// Handle Android input events
    pub fn handle_input_event(&self, event: input::AndroidInputEvent) -> Result<()> {
        self.input_handler.process_event(event, &self.vapor_runtime)
    }

    /// Handle Android lifecycle events
    pub fn on_create(&self) -> Result<()> {
        tracing::info!("📱 Android onCreate");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Created);
        Ok(())
    }

    pub fn on_start(&self) -> Result<()> {
        tracing::info!("▶️ Android onStart");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Started);
        Ok(())
    }

    pub fn on_resume(&self) -> Result<()> {
        tracing::info!("⏯️ Android onResume");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Resumed);
        self.start_render_loop()?;
        Ok(())
    }

    pub fn on_pause(&self) -> Result<()> {
        tracing::info!("⏸️ Android onPause");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Paused);
        self.stop_render_loop();
        Ok(())
    }

    pub fn on_stop(&self) -> Result<()> {
        tracing::info!("⏹️ Android onStop");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Stopped);
        Ok(())
    }

    pub fn on_destroy(&self) -> Result<()> {
        tracing::info!("💥 Android onDestroy");
        self.lifecycle_state.set_state(lifecycle::LifecycleState::Destroyed);
        self.stop_render_loop();
        
        // Cleanup resources
        *self.surface.lock().unwrap() = None;
        
        Ok(())
    }

    /// Convert Vapor template to Android-optimized Rust UI
    fn convert_vapor_to_android_ui(&self, template: &VaporTemplate) -> Result<RustComponent> {
        use crate::rust_ui::{ComponentType, RustProps, Color, Padding};

        // Create Android-optimized UI component tree
        let android_component = RustComponent {
            id: 1,
            component_type: ComponentType::VaporView {
                vapor_id: "android_root".to_string(),
                template_hash: template.memory_footprint as u64,
            },
            props: RustProps {
                width: None, // Fill screen
                height: None, // Fill screen
                background_color: Some(Color { r: 255, g: 255, b: 255, a: 255 }),
                text_color: Some(Color { r: 0, g: 0, b: 0, a: 255 }),
                font_size: Some(16.0),
                padding: Some(Padding { 
                    top: 16.0, 
                    right: 16.0, 
                    bottom: 16.0, 
                    left: 16.0 
                }),
                margin: None,
                border: None,
                custom: HashMap::new(),
            },
            children: vec![],
            reactive_bindings: template.signal_deps.iter().map(|&signal_id| {
                crate::rust_ui::ReactiveBinding {
                    property: "vapor_data".to_string(),
                    vapor_signal_id: signal_id,
                    update_fn: format!("updateAndroidComponent_{}", signal_id),
                }
            }).collect(),
        };

        Ok(android_component)
    }

    /// Get Android platform information
    pub fn get_device_info(&self) -> AndroidDeviceInfo {
        AndroidDeviceInfo {
            manufacturer: self.jni_bridge.get_device_manufacturer().unwrap_or_else(|_| "Unknown".to_string()),
            model: self.jni_bridge.get_device_model().unwrap_or_else(|_| "Unknown".to_string()),
            android_version: self.jni_bridge.get_android_version().unwrap_or_else(|_| "Unknown".to_string()),
            api_level: self.jni_bridge.get_api_level().unwrap_or_else(|_| 0),
            screen_density: self.jni_bridge.get_screen_density().unwrap_or_else(|_| 1.0),
            gpu_renderer: "Pure Rust".to_string(),
        }
    }
}

/// Android device information
#[derive(Debug, Clone)]
pub struct AndroidDeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub android_version: String,
    pub api_level: i32,
    pub screen_density: f32,
    pub gpu_renderer: String,
}

/// Android-specific performance metrics
#[derive(Debug, Default)]
pub struct AndroidPerformanceMetrics {
    /// Average FPS over last second
    pub fps: f32,
    /// Frame render time in microseconds
    pub frame_time_us: f32,
    /// GPU memory usage in MB
    pub gpu_memory_mb: f32,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Battery drain rate (mA/h)
    pub battery_drain: f32,
    /// Temperature in Celsius
    pub temperature: f32,
}

impl AndroidPerformanceMetrics {
    /// Check if performance is optimal for mobile
    pub fn is_performance_optimal(&self) -> bool {
        self.fps >= 55.0 && 
        self.frame_time_us < 16670.0 && // Under 16.67ms
        self.cpu_usage < 30.0 &&
        self.temperature < 45.0
    }

    /// Get performance grade (A-F)
    pub fn get_performance_grade(&self) -> char {
        if self.fps >= 58.0 && self.frame_time_us < 16000.0 { 'A' }
        else if self.fps >= 50.0 && self.frame_time_us < 20000.0 { 'B' }
        else if self.fps >= 40.0 && self.frame_time_us < 25000.0 { 'C' }
        else if self.fps >= 30.0 && self.frame_time_us < 33000.0 { 'D' }
        else { 'F' }
    }
}

/// Global Android application instance
static mut ANDROID_APP: Option<Arc<AndroidApplication>> = None;
static ANDROID_APP_INIT: std::sync::Once = std::sync::Once::new();

/// Get the global Android application instance
pub fn get_android_app() -> Arc<AndroidApplication> {
    ANDROID_APP_INIT.call_once(|| {
        unsafe {
            ANDROID_APP = Some(Arc::new(AndroidApplication::new().expect("Failed to create Android app")));
        }
    });
    
    unsafe { ANDROID_APP.as_ref().unwrap().clone() }
}

/// Initialize Android application (called from JNI)
#[no_mangle]
pub extern "C" fn spruce_android_init() -> bool {
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Debug));
    
    tracing::info!("🤖 Spruce Android initialized with pure Rust UI");
    let _app = get_android_app();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_android_app_creation() {
        let app = AndroidApplication::new().unwrap();
        assert!(!app.is_rendering.load(Ordering::SeqCst));
        assert_eq!(app.frame_count.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = AndroidPerformanceMetrics {
            fps: 60.0,
            frame_time_us: 15000.0,
            cpu_usage: 25.0,
            temperature: 40.0,
            ..Default::default()
        };
        
        assert!(metrics.is_performance_optimal());
        assert_eq!(metrics.get_performance_grade(), 'A');
    }
}