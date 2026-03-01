/// Android Input Event Handling for Pure Rust UI
/// 
/// Processes touch, keyboard, and other input events from Android
/// and translates them to Vue 3.6 Vapor reactive updates.

use anyhow::Result;
use std::sync::{Arc, Mutex, atomic::{AtomicU32, Ordering}};
use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::sprucevm::vue36_vapor::{VaporRuntime, VaporScheduler};

/// Android input event types
#[derive(Debug, Clone)]
pub enum AndroidInputEvent {
    /// Touch events (finger down, move, up)
    Touch {
        action: TouchAction,
        x: f32,
        y: f32,
        pointer_id: u32,
        pressure: f32,
        timestamp: u64,
    },
    /// Keyboard events
    Key {
        action: KeyAction,
        key_code: u32,
        unicode: Option<char>,
        modifiers: KeyModifiers,
        timestamp: u64,
    },
    /// Motion events (scroll, fling)
    Motion {
        action: MotionAction,
        x: f32,
        y: f32,
        velocity_x: f32,
        velocity_y: f32,
        timestamp: u64,
    },
    /// Focus events
    Focus {
        has_focus: bool,
    },
    /// Generic events for custom handling
    Generic {
        event_type: u32,
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum TouchAction {
    Down = 0,
    Up = 1,
    Move = 2,
    Cancel = 3,
    PointerDown = 5,
    PointerUp = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyAction {
    Down = 0,
    Up = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum MotionAction {
    Scroll = 8,
    Hover = 7,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool,
}

/// Input event processor with gesture recognition
pub struct AndroidInputHandler {
    /// Current active touches
    active_touches: Arc<Mutex<HashMap<u32, TouchState>>>,
    /// Gesture detector
    gesture_detector: Arc<Mutex<GestureDetector>>,
    /// Event sequence counter
    event_counter: AtomicU32,
    /// Input processing metrics
    metrics: Arc<Mutex<InputMetrics>>,
}

/// State of an active touch
#[derive(Debug, Clone)]
struct TouchState {
    initial_x: f32,
    initial_y: f32,
    current_x: f32,
    current_y: f32,
    pressure: f32,
    start_time: Instant,
    last_update: Instant,
}

/// Gesture detection and recognition
struct GestureDetector {
    /// Tap detection
    tap_detector: TapDetector,
    /// Pan/drag detection
    pan_detector: PanDetector,
    /// Pinch/zoom detection
    pinch_detector: PinchDetector,
    /// Fling detection
    fling_detector: FlingDetector,
}

#[derive(Default)]
struct TapDetector {
    last_tap_time: Option<Instant>,
    last_tap_x: f32,
    last_tap_y: f32,
    tap_count: u32,
}

#[derive(Default)]
struct PanDetector {
    is_panning: bool,
    start_x: f32,
    start_y: f32,
    current_x: f32,
    current_y: f32,
}

#[derive(Default)]
struct PinchDetector {
    is_pinching: bool,
    initial_distance: f32,
    current_scale: f32,
    focal_x: f32,
    focal_y: f32,
}

#[derive(Default)]
struct FlingDetector {
    velocities: Vec<(f32, f32, Instant)>, // (vx, vy, time)
}

/// Input processing performance metrics
#[derive(Debug, Default)]
struct InputMetrics {
    events_processed: u64,
    average_latency_us: f32,
    gesture_recognition_time_us: f32,
    last_reset: Instant,
}

/// Recognized gesture types
#[derive(Debug, Clone)]
pub enum Gesture {
    Tap {
        x: f32,
        y: f32,
        tap_count: u32,
    },
    DoubleTap {
        x: f32,
        y: f32,
    },
    LongPress {
        x: f32,
        y: f32,
        duration: Duration,
    },
    Pan {
        start_x: f32,
        start_y: f32,
        current_x: f32,
        current_y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    Pinch {
        center_x: f32,
        center_y: f32,
        scale: f32,
        rotation: f32,
    },
    Fling {
        start_x: f32,
        start_y: f32,
        velocity_x: f32,
        velocity_y: f32,
    },
}

impl AndroidInputHandler {
    pub fn new() -> Self {
        Self {
            active_touches: Arc::new(Mutex::new(HashMap::new())),
            gesture_detector: Arc::new(Mutex::new(GestureDetector::new())),
            event_counter: AtomicU32::new(0),
            metrics: Arc::new(Mutex::new(InputMetrics::default())),
        }
    }

    /// Process incoming Android input event
    pub fn process_event(&self, event: AndroidInputEvent, vapor_runtime: &VaporRuntime) -> Result<()> {
        let start_time = Instant::now();
        let event_id = self.event_counter.fetch_add(1, Ordering::SeqCst);

        tracing::trace!("📱 Processing input event #{}: {:?}", event_id, event);

        match event {
            AndroidInputEvent::Touch { action, x, y, pointer_id, pressure, timestamp } => {
                self.handle_touch_event(action, x, y, pointer_id, pressure, timestamp, vapor_runtime)?;
            }
            AndroidInputEvent::Key { action, key_code, unicode, modifiers, timestamp } => {
                self.handle_key_event(action, key_code, unicode, modifiers, timestamp, vapor_runtime)?;
            }
            AndroidInputEvent::Motion { action, x, y, velocity_x, velocity_y, timestamp } => {
                self.handle_motion_event(action, x, y, velocity_x, velocity_y, timestamp, vapor_runtime)?;
            }
            AndroidInputEvent::Focus { has_focus } => {
                self.handle_focus_event(has_focus, vapor_runtime)?;
            }
            AndroidInputEvent::Generic { event_type, data } => {
                self.handle_generic_event(event_type, data, vapor_runtime)?;
            }
        }

        // Update metrics
        let processing_time = start_time.elapsed();
        let mut metrics = self.metrics.lock().unwrap();
        metrics.events_processed += 1;
        metrics.average_latency_us = (metrics.average_latency_us * 0.9) + (processing_time.as_micros() as f32 * 0.1);

        Ok(())
    }

    /// Handle touch events with gesture recognition
    fn handle_touch_event(
        &self,
        action: TouchAction,
        x: f32,
        y: f32,
        pointer_id: u32,
        pressure: f32,
        _timestamp: u64,
        vapor_runtime: &VaporRuntime,
    ) -> Result<()> {
        let mut touches = self.active_touches.lock().unwrap();
        let mut gesture_detector = self.gesture_detector.lock().unwrap();

        match action {
            TouchAction::Down | TouchAction::PointerDown => {
                let touch_state = TouchState {
                    initial_x: x,
                    initial_y: y,
                    current_x: x,
                    current_y: y,
                    pressure,
                    start_time: Instant::now(),
                    last_update: Instant::now(),
                };

                touches.insert(pointer_id, touch_state);
                tracing::trace!("👆 Touch down: {} at ({}, {})", pointer_id, x, y);

                // Start gesture detection
                gesture_detector.on_touch_down(x, y, pointer_id);
            }
            TouchAction::Move => {
                if let Some(touch_state) = touches.get_mut(&pointer_id) {
                    touch_state.current_x = x;
                    touch_state.current_y = y;
                    touch_state.pressure = pressure;
                    touch_state.last_update = Instant::now();

                    // Update gesture detection
                    gesture_detector.on_touch_move(x, y, pointer_id, &touches);

                    // Check for gestures
                    if let Some(gesture) = gesture_detector.detect_gesture(&touches) {
                        self.handle_gesture(gesture, vapor_runtime)?;
                    }
                }
            }
            TouchAction::Up | TouchAction::PointerUp => {
                if let Some(touch_state) = touches.remove(&pointer_id) {
                    let duration = touch_state.start_time.elapsed();
                    tracing::trace!("👆 Touch up: {} after {:?}", pointer_id, duration);

                    // Finalize gesture detection
                    gesture_detector.on_touch_up(x, y, pointer_id, duration);

                    // Check for final gestures (tap, long press, etc.)
                    if let Some(gesture) = gesture_detector.detect_final_gesture(&touch_state) {
                        self.handle_gesture(gesture, vapor_runtime)?;
                    }
                }
            }
            TouchAction::Cancel => {
                touches.remove(&pointer_id);
                gesture_detector.on_touch_cancel(pointer_id);
                tracing::trace!("👆 Touch cancelled: {}", pointer_id);
            }
        }

        Ok(())
    }

    /// Handle keyboard events
    fn handle_key_event(
        &self,
        action: KeyAction,
        key_code: u32,
        unicode: Option<char>,
        modifiers: KeyModifiers,
        _timestamp: u64,
        vapor_runtime: &VaporRuntime,
    ) -> Result<()> {
        tracing::trace!("⌨️ Key event: {:?} code={} char={:?}", action, key_code, unicode);

        // Create keyboard event for Vue components
        let key_event = VueKeyboardEvent {
            action,
            key_code,
            char: unicode,
            ctrl_key: modifiers.ctrl,
            alt_key: modifiers.alt,
            shift_key: modifiers.shift,
            meta_key: modifiers.meta,
        };

        // Trigger Vue keyboard event handlers
        self.trigger_vue_event("keyevent", serde_json::to_value(key_event)?, vapor_runtime)?;

        Ok(())
    }

    /// Handle motion events (scroll, hover)
    fn handle_motion_event(
        &self,
        action: MotionAction,
        x: f32,
        y: f32,
        velocity_x: f32,
        velocity_y: f32,
        _timestamp: u64,
        vapor_runtime: &VaporRuntime,
    ) -> Result<()> {
        tracing::trace!("🖱️ Motion event: {:?} at ({}, {}) vel=({}, {})", action, x, y, velocity_x, velocity_y);

        match action {
            MotionAction::Scroll => {
                let scroll_event = VueScrollEvent {
                    x,
                    y,
                    delta_x: velocity_x,
                    delta_y: velocity_y,
                };

                self.trigger_vue_event("scroll", serde_json::to_value(scroll_event)?, vapor_runtime)?;
            }
            MotionAction::Hover => {
                let hover_event = VueMouseEvent {
                    x,
                    y,
                    event_type: "mousemove".to_string(),
                };

                self.trigger_vue_event("mousemove", serde_json::to_value(hover_event)?, vapor_runtime)?;
            }
        }

        Ok(())
    }

    /// Handle focus events
    fn handle_focus_event(&self, has_focus: bool, vapor_runtime: &VaporRuntime) -> Result<()> {
        tracing::debug!("👁️ Focus event: {}", has_focus);

        let focus_event = VueFocusEvent { has_focus };
        self.trigger_vue_event("focus", serde_json::to_value(focus_event)?, vapor_runtime)?;

        Ok(())
    }

    /// Handle generic/custom events
    fn handle_generic_event(&self, event_type: u32, data: Vec<u8>, vapor_runtime: &VaporRuntime) -> Result<()> {
        tracing::trace!("🔧 Generic event: type={}, data={} bytes", event_type, data.len());

        let generic_event = VueGenericEvent {
            event_type,
            data: base64::encode(&data),
        };

        self.trigger_vue_event("generic", serde_json::to_value(generic_event)?, vapor_runtime)?;

        Ok(())
    }

    /// Handle recognized gestures
    fn handle_gesture(&self, gesture: Gesture, vapor_runtime: &VaporRuntime) -> Result<()> {
        tracing::debug!("✋ Gesture detected: {:?}", gesture);

        match gesture {
            Gesture::Tap { x, y, tap_count } => {
                let event_name = if tap_count == 1 { "tap" } else { "doubletap" };
                let tap_event = VueTapEvent { x, y, tap_count };
                self.trigger_vue_event(event_name, serde_json::to_value(tap_event)?, vapor_runtime)?;
            }
            Gesture::LongPress { x, y, duration } => {
                let longpress_event = VueLongPressEvent {
                    x,
                    y,
                    duration_ms: duration.as_millis() as u32,
                };
                self.trigger_vue_event("longpress", serde_json::to_value(longpress_event)?, vapor_runtime)?;
            }
            Gesture::Pan { start_x, start_y, current_x, current_y, delta_x, delta_y } => {
                let pan_event = VuePanEvent {
                    start_x,
                    start_y,
                    current_x,
                    current_y,
                    delta_x,
                    delta_y,
                };
                self.trigger_vue_event("pan", serde_json::to_value(pan_event)?, vapor_runtime)?;
            }
            Gesture::Pinch { center_x, center_y, scale, rotation } => {
                let pinch_event = VuePinchEvent {
                    center_x,
                    center_y,
                    scale,
                    rotation,
                };
                self.trigger_vue_event("pinch", serde_json::to_value(pinch_event)?, vapor_runtime)?;
            }
            Gesture::Fling { start_x, start_y, velocity_x, velocity_y } => {
                let fling_event = VueFlingEvent {
                    start_x,
                    start_y,
                    velocity_x,
                    velocity_y,
                };
                self.trigger_vue_event("fling", serde_json::to_value(fling_event)?, vapor_runtime)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Trigger Vue event in the Vapor runtime
    fn trigger_vue_event(
        &self,
        event_name: &str,
        event_data: serde_json::Value,
        vapor_runtime: &VaporRuntime,
    ) -> Result<()> {
        // Create a reactive signal for the event
        let event_signal = vapor_runtime.create_signal(event_data);

        // Trigger effects that depend on this event type
        vapor_runtime.scheduler.create_effect(move || {
            let _event_value = event_signal.get();
            // This would trigger Vue event handlers
            tracing::trace!("🎯 Vue event triggered: {}", event_name);
        });

        vapor_runtime.scheduler.flush_effects();

        Ok(())
    }

    /// Get input processing metrics
    pub fn get_metrics(&self) -> InputMetrics {
        self.metrics.lock().unwrap().clone()
    }

    /// Reset metrics (for benchmarking)
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        *metrics = InputMetrics::default();
    }
}

impl GestureDetector {
    fn new() -> Self {
        Self {
            tap_detector: TapDetector::default(),
            pan_detector: PanDetector::default(),
            pinch_detector: PinchDetector::default(),
            fling_detector: FlingDetector::default(),
        }
    }

    fn on_touch_down(&mut self, x: f32, y: f32, _pointer_id: u32) {
        // Reset detectors
        self.pan_detector = PanDetector {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            is_panning: false,
        };
    }

    fn on_touch_move(&mut self, x: f32, y: f32, _pointer_id: u32, touches: &HashMap<u32, TouchState>) {
        // Update pan detector
        self.pan_detector.current_x = x;
        self.pan_detector.current_y = y;

        let distance = ((x - self.pan_detector.start_x).powi(2) + (y - self.pan_detector.start_y).powi(2)).sqrt();
        if distance > 10.0 {
            self.pan_detector.is_panning = true;
        }

        // Update pinch detector for multi-touch
        if touches.len() >= 2 {
            self.update_pinch_detector(touches);
        }
    }

    fn on_touch_up(&mut self, x: f32, y: f32, _pointer_id: u32, duration: Duration) {
        // Update tap detector
        let now = Instant::now();
        let is_tap = duration < Duration::from_millis(200) && 
                     ((x - self.pan_detector.start_x).abs() < 20.0 && (y - self.pan_detector.start_y).abs() < 20.0);

        if is_tap {
            if let Some(last_tap) = self.tap_detector.last_tap_time {
                if now.duration_since(last_tap) < Duration::from_millis(300) &&
                   (x - self.tap_detector.last_tap_x).abs() < 50.0 &&
                   (y - self.tap_detector.last_tap_y).abs() < 50.0 {
                    self.tap_detector.tap_count += 1;
                } else {
                    self.tap_detector.tap_count = 1;
                }
            } else {
                self.tap_detector.tap_count = 1;
            }

            self.tap_detector.last_tap_time = Some(now);
            self.tap_detector.last_tap_x = x;
            self.tap_detector.last_tap_y = y;
        }

        // Add to fling detector history
        let velocity_x = (x - self.pan_detector.current_x) / duration.as_secs_f32();
        let velocity_y = (y - self.pan_detector.current_y) / duration.as_secs_f32();
        self.fling_detector.velocities.push((velocity_x, velocity_y, now));

        // Keep only recent velocities (last 100ms)
        self.fling_detector.velocities.retain(|(_, _, time)| {
            now.duration_since(*time) < Duration::from_millis(100)
        });
    }

    fn on_touch_cancel(&mut self, _pointer_id: u32) {
        // Reset all detectors
        self.pan_detector = PanDetector::default();
        self.pinch_detector = PinchDetector::default();
        self.fling_detector = FlingDetector::default();
    }

    fn detect_gesture(&self, _touches: &HashMap<u32, TouchState>) -> Option<Gesture> {
        // Detect ongoing gestures (pan, pinch)
        if self.pan_detector.is_panning {
            return Some(Gesture::Pan {
                start_x: self.pan_detector.start_x,
                start_y: self.pan_detector.start_y,
                current_x: self.pan_detector.current_x,
                current_y: self.pan_detector.current_y,
                delta_x: self.pan_detector.current_x - self.pan_detector.start_x,
                delta_y: self.pan_detector.current_y - self.pan_detector.start_y,
            });
        }

        if self.pinch_detector.is_pinching {
            return Some(Gesture::Pinch {
                center_x: self.pinch_detector.focal_x,
                center_y: self.pinch_detector.focal_y,
                scale: self.pinch_detector.current_scale,
                rotation: 0.0, // TODO: Calculate rotation
            });
        }

        None
    }

    fn detect_final_gesture(&self, touch_state: &TouchState) -> Option<Gesture> {
        let duration = touch_state.start_time.elapsed();

        // Long press detection
        if duration > Duration::from_millis(500) {
            return Some(Gesture::LongPress {
                x: touch_state.current_x,
                y: touch_state.current_y,
                duration,
            });
        }

        // Tap detection
        if self.tap_detector.tap_count > 0 {
            return Some(Gesture::Tap {
                x: touch_state.current_x,
                y: touch_state.current_y,
                tap_count: self.tap_detector.tap_count,
            });
        }

        // Fling detection
        if !self.fling_detector.velocities.is_empty() {
            let avg_velocity = self.fling_detector.velocities.iter().fold((0.0, 0.0), |acc, (vx, vy, _)| {
                (acc.0 + vx, acc.1 + vy)
            });
            let count = self.fling_detector.velocities.len() as f32;
            let velocity_x = avg_velocity.0 / count;
            let velocity_y = avg_velocity.1 / count;

            if velocity_x.abs() > 100.0 || velocity_y.abs() > 100.0 {
                return Some(Gesture::Fling {
                    start_x: touch_state.initial_x,
                    start_y: touch_state.initial_y,
                    velocity_x,
                    velocity_y,
                });
            }
        }

        None
    }

    fn update_pinch_detector(&mut self, touches: &HashMap<u32, TouchState>) {
        if touches.len() < 2 {
            return;
        }

        let touch_points: Vec<_> = touches.values().collect();
        let touch1 = touch_points[0];
        let touch2 = touch_points[1];

        let distance = ((touch1.current_x - touch2.current_x).powi(2) + 
                       (touch1.current_y - touch2.current_y).powi(2)).sqrt();

        let focal_x = (touch1.current_x + touch2.current_x) / 2.0;
        let focal_y = (touch1.current_y + touch2.current_y) / 2.0;

        if !self.pinch_detector.is_pinching {
            self.pinch_detector.initial_distance = distance;
            self.pinch_detector.is_pinching = true;
        }

        self.pinch_detector.current_scale = distance / self.pinch_detector.initial_distance;
        self.pinch_detector.focal_x = focal_x;
        self.pinch_detector.focal_y = focal_y;
    }
}

// Vue event data structures
#[derive(Debug, serde::Serialize)]
struct VueKeyboardEvent {
    action: KeyAction,
    key_code: u32,
    #[serde(rename = "char")]
    char: Option<char>,
    ctrl_key: bool,
    alt_key: bool,
    shift_key: bool,
    meta_key: bool,
}

#[derive(Debug, serde::Serialize)]
struct VueScrollEvent {
    x: f32,
    y: f32,
    delta_x: f32,
    delta_y: f32,
}

#[derive(Debug, serde::Serialize)]
struct VueMouseEvent {
    x: f32,
    y: f32,
    event_type: String,
}

#[derive(Debug, serde::Serialize)]
struct VueFocusEvent {
    has_focus: bool,
}

#[derive(Debug, serde::Serialize)]
struct VueGenericEvent {
    event_type: u32,
    data: String, // base64 encoded
}

#[derive(Debug, serde::Serialize)]
struct VueTapEvent {
    x: f32,
    y: f32,
    tap_count: u32,
}

#[derive(Debug, serde::Serialize)]
struct VueLongPressEvent {
    x: f32,
    y: f32,
    duration_ms: u32,
}

#[derive(Debug, serde::Serialize)]
struct VuePanEvent {
    start_x: f32,
    start_y: f32,
    current_x: f32,
    current_y: f32,
    delta_x: f32,
    delta_y: f32,
}

#[derive(Debug, serde::Serialize)]
struct VuePinchEvent {
    center_x: f32,
    center_y: f32,
    scale: f32,
    rotation: f32,
}

#[derive(Debug, serde::Serialize)]
struct VueFlingEvent {
    start_x: f32,
    start_y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl Clone for InputMetrics {
    fn clone(&self) -> Self {
        Self {
            events_processed: self.events_processed,
            average_latency_us: self.average_latency_us,
            gesture_recognition_time_us: self.gesture_recognition_time_us,
            last_reset: self.last_reset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch_event_processing() {
        let handler = AndroidInputHandler::new();
        let vapor_runtime = VaporRuntime::new();

        let touch_event = AndroidInputEvent::Touch {
            action: TouchAction::Down,
            x: 100.0,
            y: 200.0,
            pointer_id: 0,
            pressure: 1.0,
            timestamp: 123456789,
        };

        handler.process_event(touch_event, &vapor_runtime).unwrap();

        let touches = handler.active_touches.lock().unwrap();
        assert!(touches.contains_key(&0));
        assert_eq!(touches[&0].initial_x, 100.0);
        assert_eq!(touches[&0].initial_y, 200.0);
    }

    #[test]
    fn test_gesture_detection() {
        let mut detector = GestureDetector::new();
        let mut touches = HashMap::new();

        // Simulate tap
        detector.on_touch_down(100.0, 100.0, 0);
        detector.on_touch_up(105.0, 105.0, 0, Duration::from_millis(150));

        let touch_state = TouchState {
            initial_x: 100.0,
            initial_y: 100.0,
            current_x: 105.0,
            current_y: 105.0,
            pressure: 1.0,
            start_time: Instant::now() - Duration::from_millis(150),
            last_update: Instant::now(),
        };

        let gesture = detector.detect_final_gesture(&touch_state);
        assert!(matches!(gesture, Some(Gesture::Tap { tap_count: 1, .. })));
    }
}