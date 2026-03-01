/// Comprehensive tests for Android implementation
/// 
/// Tests the complete Android integration including:
/// - Surface creation and management
/// - Input event processing
/// - JNI bridge functionality
/// - Lifecycle management
/// - Pure Rust UI rendering

#[cfg(test)]
mod tests {
    use super::*;
    use crate::android::*;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_android_app_creation() {
        let app = AndroidApplication::new().unwrap();
        
        // Test initial state
        assert!(!app.is_rendering.load(std::sync::atomic::Ordering::SeqCst));
        assert_eq!(app.frame_count.load(std::sync::atomic::Ordering::SeqCst), 0);
        
        // Test device info
        let device_info = app.get_device_info();
        assert!(!device_info.manufacturer.is_empty());
        assert!(!device_info.model.is_empty());
    }

    #[test]
    fn test_surface_creation_and_operations() {
        let mock_window = 0x12345678 as *mut std::ffi::c_void;
        let surface = surface::AndroidSurface::new(mock_window, 1920, 1080).unwrap();
        
        assert_eq!(surface.width, 1920);
        assert_eq!(surface.height, 1080);
        
        // Test color conversion
        let white = surface.rgb_to_pixel(255, 255, 255, 255);
        let red = surface.rgb_to_pixel(255, 0, 0, 255);
        
        assert_eq!(white, 0xFFFFFFFF);
        assert_eq!(red, 0xFF0000FF);
        
        // Test surface lock
        let lock = surface.lock().unwrap();
        drop(lock); // Should unlock automatically
        
        // Test surface resize
        let mut surface_mut = surface;
        surface_mut.resize(2560, 1440).unwrap();
        assert_eq!(surface_mut.width, 2560);
        assert_eq!(surface_mut.height, 1440);
    }

    #[test]
    fn test_input_event_processing() {
        let handler = input::AndroidInputHandler::new();
        let vapor_runtime = crate::sprucevm::vue36_vapor::VaporRuntime::new();
        
        // Test touch events
        let touch_down = input::AndroidInputEvent::Touch {
            action: input::TouchAction::Down,
            x: 100.0,
            y: 200.0,
            pointer_id: 0,
            pressure: 1.0,
            timestamp: 123456789,
        };
        
        handler.process_event(touch_down, &vapor_runtime).unwrap();
        
        // Verify touch is tracked
        let touches = handler.active_touches.lock().unwrap();
        assert!(touches.contains_key(&0));
        assert_eq!(touches[&0].initial_x, 100.0);
        assert_eq!(touches[&0].initial_y, 200.0);
        
        // Test keyboard events
        let key_event = input::AndroidInputEvent::Key {
            action: input::KeyAction::Down,
            key_code: 66, // Enter key
            unicode: Some('\n'),
            modifiers: input::KeyModifiers::default(),
            timestamp: 123456790,
        };
        
        handler.process_event(key_event, &vapor_runtime).unwrap();
        
        // Test metrics
        let metrics = handler.get_metrics();
        assert!(metrics.events_processed >= 2);
    }

    #[test]
    fn test_gesture_recognition() {
        let mut detector = input::GestureDetector::new();
        
        // Simulate tap gesture
        detector.on_touch_down(100.0, 100.0, 0);
        detector.on_touch_up(105.0, 105.0, 0, Duration::from_millis(150));
        
        let touch_state = input::TouchState {
            initial_x: 100.0,
            initial_y: 100.0,
            current_x: 105.0,
            current_y: 105.0,
            pressure: 1.0,
            start_time: std::time::Instant::now() - Duration::from_millis(150),
            last_update: std::time::Instant::now(),
        };
        
        let gesture = detector.detect_final_gesture(&touch_state);
        assert!(matches!(gesture, Some(input::Gesture::Tap { tap_count: 1, .. })));
        
        // Test long press
        let long_touch_state = input::TouchState {
            initial_x: 100.0,
            initial_y: 100.0,
            current_x: 100.0,
            current_y: 100.0,
            pressure: 1.0,
            start_time: std::time::Instant::now() - Duration::from_millis(600),
            last_update: std::time::Instant::now(),
        };
        
        let long_gesture = detector.detect_final_gesture(&long_touch_state);
        assert!(matches!(long_gesture, Some(input::Gesture::LongPress { .. })));
    }

    #[test]
    fn test_lifecycle_management() {
        let lifecycle = lifecycle::AndroidLifecycle::new();
        
        // Test initial state
        assert_eq!(lifecycle.get_state(), lifecycle::LifecycleState::Created);
        assert!(!lifecycle.is_foreground());
        assert!(!lifecycle.can_render());
        
        // Test state transitions
        lifecycle.set_state(lifecycle::LifecycleState::Started);
        assert_eq!(lifecycle.get_state(), lifecycle::LifecycleState::Started);
        assert!(lifecycle.is_foreground());
        
        lifecycle.set_state(lifecycle::LifecycleState::Resumed);
        assert_eq!(lifecycle.get_state(), lifecycle::LifecycleState::Resumed);
        assert!(lifecycle.can_render());
        
        lifecycle.set_state(lifecycle::LifecycleState::Paused);
        assert_eq!(lifecycle.get_state(), lifecycle::LifecycleState::Paused);
        assert!(!lifecycle.can_render());
        
        // Test transition validation
        assert!(lifecycle.is_valid_transition(
            lifecycle::LifecycleState::Created,
            lifecycle::LifecycleState::Started
        ));
        assert!(!lifecycle.is_valid_transition(
            lifecycle::LifecycleState::Created,
            lifecycle::LifecycleState::Resumed
        ));
        
        // Test statistics
        let stats = lifecycle.get_statistics();
        assert!(stats.transition_count > 0);
        assert_eq!(stats.current_state, lifecycle::LifecycleState::Paused);
    }

    #[test]
    fn test_memory_pressure_handling() {
        let lifecycle = lifecycle::AndroidLifecycle::new();
        
        // Test memory pressure levels
        assert_eq!(lifecycle.get_memory_pressure(), lifecycle::MemoryPressure::None);
        
        lifecycle.set_memory_pressure(lifecycle::MemoryPressure::Critical);
        assert_eq!(lifecycle.get_memory_pressure(), lifecycle::MemoryPressure::Critical);
        
        // Test health score impact
        let health_score = lifecycle.get_health_score();
        assert!(health_score < 100); // Should be penalized
        
        // Test recommendations
        let recommendations = lifecycle.get_memory_recommendations();
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("critical")));
    }

    #[test] 
    fn test_jni_bridge() {
        let bridge = jni_bridge::AndroidJNIBridge::new();
        
        // Test initial state
        assert!(bridge.jvm.is_none());
        
        // Test system property fallback (without actual JVM)
        // In real test environment, these would fail gracefully
        assert!(bridge.get_device_manufacturer().is_err());
        assert!(bridge.get_api_level().is_err());
    }

    #[test]
    fn test_android_ui_renderer() {
        let mut renderer = renderer::AndroidUIRenderer::new().unwrap();
        
        // Test surface initialization
        renderer.init_android_surface(1920, 1080).unwrap();
        
        // Test component mounting
        let component = crate::rust_ui::RustComponent {
            id: 1,
            component_type: crate::rust_ui::ComponentType::VaporView {
                vapor_id: "test_view".to_string(),
                template_hash: 12345,
            },
            props: crate::rust_ui::RustProps::default(),
            children: vec![],
            reactive_bindings: vec![],
        };
        
        renderer.mount_component(component).unwrap();
        
        // Test metrics
        let metrics = renderer.get_android_metrics();
        assert_eq!(metrics.screen_density, 3.0); // Default HDPI
    }

    #[test]
    fn test_density_conversions() {
        let converter = renderer::DensityConverter::new(3.0, 3.0); // HDPI
        
        // Test DP to PX conversion
        assert_eq!(converter.dp_to_px(16.0), 48.0);
        assert_eq!(converter.sp_to_px(14.0), 42.0);
        assert_eq!(converter.px_to_dp(48.0), 16.0);
        
        // Test MDPI (1.0 density)
        let mdpi_converter = renderer::DensityConverter::new(1.0, 1.0);
        assert_eq!(mdpi_converter.dp_to_px(16.0), 16.0);
        
        // Test XHDPI (2.0 density)
        let xhdpi_converter = renderer::DensityConverter::new(2.0, 2.0);
        assert_eq!(xhdpi_converter.dp_to_px(16.0), 32.0);
    }

    #[test]
    fn test_performance_metrics() {
        let metrics = AndroidPerformanceMetrics {
            fps: 60.0,
            frame_time_us: 15000.0,
            cpu_usage: 25.0,
            temperature: 40.0,
            gpu_memory_mb: 45.0,
            battery_drain: 300.0,
        };
        
        assert!(metrics.is_performance_optimal());
        assert_eq!(metrics.get_performance_grade(), 'A');
        
        // Test poor performance
        let poor_metrics = AndroidPerformanceMetrics {
            fps: 20.0,
            frame_time_us: 50000.0,
            cpu_usage: 80.0,
            temperature: 60.0,
            gpu_memory_mb: 200.0,
            battery_drain: 1500.0,
        };
        
        assert!(!poor_metrics.is_performance_optimal());
        assert_eq!(poor_metrics.get_performance_grade(), 'F');
    }

    #[test]
    fn test_render_performance() {
        let mut profiler = renderer::AndroidRenderProfiler::new();
        
        // Simulate frame rendering
        profiler.begin_frame();
        profiler.record_draw_call();
        profiler.record_draw_call();
        profiler.end_frame(16000.0); // 16ms frame time
        
        assert_eq!(profiler.draw_calls, 2);
        
        // Add more frames for average calculation
        for _ in 0..59 {
            profiler.begin_frame();
            profiler.record_draw_call();
            profiler.end_frame(16000.0);
        }
        
        let avg_fps = profiler.get_avg_fps();
        assert!(avg_fps > 55.0 && avg_fps < 65.0); // Should be around 60 FPS
    }

    #[test]
    fn test_texture_atlas() {
        let mut atlas = renderer::AndroidTextureAtlas::new().unwrap();
        
        // Test initial state
        assert_eq!(atlas.width, 0);
        assert_eq!(atlas.height, 0);
        
        // Test resize
        atlas.resize(1024, 1024).unwrap();
        assert_eq!(atlas.width, 1024);
        assert_eq!(atlas.height, 1024);
        assert_eq!(atlas.free_regions.len(), 1);
        assert_eq!(atlas.free_regions[0].width, 1024);
        assert_eq!(atlas.free_regions[0].height, 1024);
    }

    #[test]
    fn test_complete_android_integration() {
        // This test simulates a complete Vue component lifecycle on Android
        
        // 1. Create Android application
        let app = AndroidApplication::new().unwrap();
        
        // 2. Initialize surface
        let mock_window = 0x12345678 as *mut std::ffi::c_void;
        app.init_surface(mock_window, 1920, 1080).unwrap();
        
        // 3. Create Vue 3.6 Vapor template
        let vapor_template = crate::sprucevm::vue36_vapor::VaporTemplate {
            mount_fn: "function mount() { return element; }".to_string(),
            update_fns: std::collections::HashMap::new(),
            hoisted_elements: vec![],
            signal_deps: vec![1, 2, 3],
            memory_footprint: 1024,
        };
        
        // 4. Mount Vapor app
        app.mount_vapor_app(vapor_template, "root").unwrap();
        
        // 5. Simulate lifecycle events
        app.on_create().unwrap();
        app.on_start().unwrap();
        app.on_resume().unwrap();
        
        // 6. Simulate input events
        let touch_event = input::AndroidInputEvent::Touch {
            action: input::TouchAction::Down,
            x: 500.0,
            y: 600.0,
            pointer_id: 0,
            pressure: 1.0,
            timestamp: 123456789,
        };
        
        app.handle_input_event(touch_event).unwrap();
        
        // 7. Simulate lifecycle pause/resume
        app.on_pause().unwrap();
        app.on_resume().unwrap();
        
        // 8. Cleanup
        app.on_pause().unwrap();
        app.on_stop().unwrap();
        app.on_destroy().unwrap();
        
        // Verify final state
        assert!(!app.is_rendering.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test] 
    fn test_vue_vapor_android_integration() {
        // Test Vue 3.6 Vapor mode integration with Android
        let vapor_runtime = crate::sprucevm::vue36_vapor::VaporRuntime::new();
        
        // Create reactive signal
        let counter_signal = vapor_runtime.create_signal(0i32);
        
        // Create effect that responds to signal changes
        let effect_count = Arc::new(std::sync::atomic::AtomicU32::new(0));
        let effect_count_clone = effect_count.clone();
        let counter_signal_clone = counter_signal.clone();
        
        vapor_runtime.scheduler.create_effect(move || {
            let _value = counter_signal_clone.get();
            effect_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        
        // Verify initial effect run
        assert_eq!(effect_count.load(std::sync::atomic::Ordering::SeqCst), 1);
        
        // Update signal and flush effects
        counter_signal.set(5, &vapor_runtime.scheduler);
        vapor_runtime.scheduler.flush_effects();
        
        // Verify effect re-ran
        assert_eq!(effect_count.load(std::sync::atomic::Ordering::SeqCst), 2);
    }

    #[test]
    fn test_android_performance_optimization() {
        // Test mobile-specific performance optimizations
        
        // 1. Touch target size enforcement
        let mut component = crate::rust_ui::RustComponent {
            id: 1,
            component_type: crate::rust_ui::ComponentType::Button {
                label: "Small Button".to_string(),
                vapor_onclick: None,
            },
            props: crate::rust_ui::RustProps {
                width: Some(30.0), // Too small for mobile
                height: Some(30.0), // Too small for mobile
                ..Default::default()
            },
            children: vec![],
            reactive_bindings: vec![],
        };
        
        let renderer = renderer::AndroidUIRenderer::new().unwrap();
        let optimized = renderer.optimize_for_mobile(component).unwrap();
        
        // Should be enforced to minimum touch size
        let min_size = renderer.mobile_layout.dp_converter.dp_to_px(48.0);
        assert!(optimized.props.width.unwrap() >= min_size);
        assert!(optimized.props.height.unwrap() >= min_size);
        
        // 2. Accessibility enhancement
        assert!(optimized.props.custom.contains_key("contentDescription"));
        assert!(optimized.props.custom.contains_key("focusable"));
    }
}

/// Benchmarks for Android performance testing
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn bench_input_processing() {
        let handler = input::AndroidInputHandler::new();
        let vapor_runtime = crate::sprucevm::vue36_vapor::VaporRuntime::new();
        
        let start = Instant::now();
        
        // Process 1000 touch events
        for i in 0..1000 {
            let event = input::AndroidInputEvent::Touch {
                action: input::TouchAction::Move,
                x: (i % 1920) as f32,
                y: (i % 1080) as f32,
                pointer_id: 0,
                pressure: 1.0,
                timestamp: i as u64,
            };
            
            handler.process_event(event, &vapor_runtime).unwrap();
        }
        
        let elapsed = start.elapsed();
        let events_per_ms = 1000.0 / elapsed.as_millis() as f32;
        
        println!("📱 Input processing: {:.2} events/ms", events_per_ms);
        
        // Should handle at least 100 events per millisecond
        assert!(events_per_ms > 100.0);
    }

    #[test] 
    fn bench_rendering_pipeline() {
        let mut renderer = renderer::AndroidUIRenderer::new().unwrap();
        renderer.init_android_surface(1920, 1080).unwrap();
        
        // Create complex component tree
        let mut root_component = crate::rust_ui::RustComponent {
            id: 1,
            component_type: crate::rust_ui::ComponentType::FlexContainer {
                direction: crate::rust_ui::FlexDirection::Column,
                justify: crate::rust_ui::JustifyContent::Center,
                align: crate::rust_ui::AlignItems::Center,
            },
            props: crate::rust_ui::RustProps::default(),
            children: vec![],
            reactive_bindings: vec![],
        };
        
        // Add 100 child components
        for i in 0..100 {
            let child = Arc::new(crate::rust_ui::RustComponent {
                id: i + 2,
                component_type: crate::rust_ui::ComponentType::VaporText {
                    content: format!("Item {}", i),
                    is_reactive: true,
                },
                props: crate::rust_ui::RustProps::default(),
                children: vec![],
                reactive_bindings: vec![],
            });
            root_component.children.push(child);
        }
        
        let start = Instant::now();
        
        // Mount and render 60 frames
        renderer.mount_component(root_component).unwrap();
        
        for _ in 0..60 {
            let mock_surface = surface::AndroidSurface::new(
                0x12345678 as *mut std::ffi::c_void,
                1920,
                1080
            ).unwrap();
            
            renderer.render_android_frame(&mock_surface).unwrap();
        }
        
        let elapsed = start.elapsed();
        let fps = 60.0 / elapsed.as_secs_f32();
        
        println!("📱 Rendering performance: {:.2} FPS", fps);
        
        // Should maintain at least 30 FPS with 100 components
        assert!(fps > 30.0);
    }

    #[test]
    fn bench_memory_allocation() {
        let start = Instant::now();
        let mut surfaces = Vec::new();
        
        // Create and destroy 1000 surfaces
        for _ in 0..1000 {
            let surface = surface::AndroidSurface::new(
                0x12345678 as *mut std::ffi::c_void,
                800,
                600
            ).unwrap();
            surfaces.push(surface);
        }
        
        // Clear all at once
        surfaces.clear();
        
        let elapsed = start.elapsed();
        let allocations_per_ms = 1000.0 / elapsed.as_millis() as f32;
        
        println!("📱 Memory allocation: {:.2} allocations/ms", allocations_per_ms);
        
        // Should handle at least 10 allocations per millisecond
        assert!(allocations_per_ms > 10.0);
    }
}

/// Integration test helper functions
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    pub fn create_test_vapor_template() -> crate::sprucevm::vue36_vapor::VaporTemplate {
        crate::sprucevm::vue36_vapor::VaporTemplate {
            mount_fn: r#"
                function mount(container) {
                    const element = document.createElement('div');
                    element.className = 'test-component';
                    element.textContent = 'Hello Android!';
                    container.appendChild(element);
                    return element;
                }
            "#.to_string(),
            update_fns: {
                let mut map = std::collections::HashMap::new();
                map.insert(1, "function updateText() { element.textContent = signals[1].value; }".to_string());
                map
            },
            hoisted_elements: vec!["<span>Static content</span>".to_string()],
            signal_deps: vec![1],
            memory_footprint: 512,
        }
    }
    
    pub fn simulate_android_lifecycle(app: &AndroidApplication) -> Result<(), anyhow::Error> {
        app.on_create()?;
        app.on_start()?;
        app.on_resume()?;
        
        // Simulate some activity
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        app.on_pause()?;
        app.on_stop()?;
        app.on_destroy()?;
        
        Ok(())
    }
    
    pub fn create_test_component_tree(depth: usize, width: usize) -> crate::rust_ui::RustComponent {
        if depth == 0 {
            return crate::rust_ui::RustComponent {
                id: 1,
                component_type: crate::rust_ui::ComponentType::VaporText {
                    content: "Leaf node".to_string(),
                    is_reactive: false,
                },
                props: crate::rust_ui::RustProps::default(),
                children: vec![],
                reactive_bindings: vec![],
            };
        }
        
        let mut children = Vec::new();
        for i in 0..width {
            let child = create_test_component_tree(depth - 1, width);
            children.push(Arc::new(child));
        }
        
        crate::rust_ui::RustComponent {
            id: depth as u32 * 1000 + width as u32,
            component_type: crate::rust_ui::ComponentType::FlexContainer {
                direction: crate::rust_ui::FlexDirection::Column,
                justify: crate::rust_ui::JustifyContent::FlexStart,
                align: crate::rust_ui::AlignItems::Stretch,
            },
            props: crate::rust_ui::RustProps::default(),
            children,
            reactive_bindings: vec![],
        }
    }
}
}