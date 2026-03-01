/// Android Lifecycle Management for Rust UI
/// 
/// Manages Android activity lifecycle states and coordinates
/// with the Rust UI rendering system.

use anyhow::Result;
use std::sync::{Arc, RwLock, atomic::{AtomicU32, AtomicBool, Ordering}};
use std::time::{Duration, Instant};

/// Android activity lifecycle states
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LifecycleState {
    Created,
    Started,
    Resumed,
    Paused,
    Stopped,
    Destroyed,
}

/// Lifecycle event callbacks
pub trait LifecycleObserver: Send + Sync {
    fn on_create(&self) -> Result<()>;
    fn on_start(&self) -> Result<()>;
    fn on_resume(&self) -> Result<()>;
    fn on_pause(&self) -> Result<()>;
    fn on_stop(&self) -> Result<()>;
    fn on_destroy(&self) -> Result<()>;
    fn on_low_memory(&self) -> Result<()>;
    fn on_configuration_changed(&self) -> Result<()>;
}

/// Android lifecycle manager
pub struct AndroidLifecycle {
    /// Current lifecycle state
    current_state: Arc<RwLock<LifecycleState>>,
    /// Lifecycle observers
    observers: Arc<RwLock<Vec<Arc<dyn LifecycleObserver>>>>,
    /// State transition counter
    transition_count: AtomicU32,
    /// App is in foreground
    is_foreground: AtomicBool,
    /// Time spent in each state
    state_timings: Arc<RwLock<StateTimings>>,
    /// Memory pressure level
    memory_pressure: Arc<RwLock<MemoryPressure>>,
}

/// Timing information for lifecycle states
#[derive(Debug, Default)]
pub struct StateTimings {
    /// Time when entered current state
    current_state_start: Option<Instant>,
    /// Total time spent in each state
    time_in_created: Duration,
    time_in_started: Duration,
    time_in_resumed: Duration,
    time_in_paused: Duration,
    time_in_stopped: Duration,
    /// Number of times each state was entered
    created_count: u32,
    started_count: u32,
    resumed_count: u32,
    paused_count: u32,
    stopped_count: u32,
    destroyed_count: u32,
}

/// Memory pressure levels
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryPressure {
    None,
    Low,
    Moderate,
    Critical,
}

impl AndroidLifecycle {
    pub fn new() -> Self {
        Self {
            current_state: Arc::new(RwLock::new(LifecycleState::Created)),
            observers: Arc::new(RwLock::new(Vec::new())),
            transition_count: AtomicU32::new(0),
            is_foreground: AtomicBool::new(false),
            state_timings: Arc::new(RwLock::new(StateTimings::default())),
            memory_pressure: Arc::new(RwLock::new(MemoryPressure::None)),
        }
    }

    /// Add lifecycle observer
    pub fn add_observer(&self, observer: Arc<dyn LifecycleObserver>) {
        let mut observers = self.observers.write().unwrap();
        observers.push(observer);
        tracing::debug!("📝 Added lifecycle observer (total: {})", observers.len());
    }

    /// Remove lifecycle observer
    pub fn remove_observer(&self, observer: Arc<dyn LifecycleObserver>) {
        let mut observers = self.observers.write().unwrap();
        observers.retain(|o| !Arc::ptr_eq(o, &observer));
        tracing::debug!("📝 Removed lifecycle observer (total: {})", observers.len());
    }

    /// Set current lifecycle state
    pub fn set_state(&self, new_state: LifecycleState) {
        let old_state = {
            let mut state = self.current_state.write().unwrap();
            let old = *state;
            *state = new_state;
            old
        };

        if old_state != new_state {
            self.transition_count.fetch_add(1, Ordering::SeqCst);
            self.update_timing_info(old_state, new_state);
            self.update_foreground_status(new_state);
            self.notify_observers(new_state);

            tracing::info!("📱 Lifecycle transition: {:?} -> {:?}", old_state, new_state);
        }
    }

    /// Get current lifecycle state
    pub fn get_state(&self) -> LifecycleState {
        *self.current_state.read().unwrap()
    }

    /// Check if app is in foreground
    pub fn is_foreground(&self) -> bool {
        self.is_foreground.load(Ordering::SeqCst)
    }

    /// Check if app can render (resumed state)
    pub fn can_render(&self) -> bool {
        matches!(self.get_state(), LifecycleState::Resumed)
    }

    /// Set memory pressure level
    pub fn set_memory_pressure(&self, pressure: MemoryPressure) {
        let old_pressure = {
            let mut mp = self.memory_pressure.write().unwrap();
            let old = *mp;
            *mp = pressure;
            old
        };

        if old_pressure != pressure {
            tracing::warn!("💾 Memory pressure: {:?} -> {:?}", old_pressure, pressure);
            
            if pressure != MemoryPressure::None {
                self.notify_low_memory();
            }
        }
    }

    /// Get current memory pressure
    pub fn get_memory_pressure(&self) -> MemoryPressure {
        *self.memory_pressure.read().unwrap()
    }

    /// Handle configuration changes (rotation, locale, etc.)
    pub fn on_configuration_changed(&self) {
        tracing::info!("🔄 Configuration changed");
        
        let observers = self.observers.read().unwrap();
        for observer in observers.iter() {
            if let Err(e) = observer.on_configuration_changed() {
                tracing::error!("Configuration change observer failed: {}", e);
            }
        }
    }

    /// Get lifecycle statistics
    pub fn get_statistics(&self) -> LifecycleStatistics {
        let timings = self.state_timings.read().unwrap();
        
        LifecycleStatistics {
            current_state: self.get_state(),
            transition_count: self.transition_count.load(Ordering::SeqCst),
            is_foreground: self.is_foreground(),
            memory_pressure: self.get_memory_pressure(),
            time_in_created: timings.time_in_created,
            time_in_resumed: timings.time_in_resumed,
            time_in_paused: timings.time_in_paused,
            created_count: timings.created_count,
            resumed_count: timings.resumed_count,
            paused_count: timings.paused_count,
            destroyed_count: timings.destroyed_count,
        }
    }

    /// Update timing information
    fn update_timing_info(&self, old_state: LifecycleState, new_state: LifecycleState) {
        let mut timings = self.state_timings.write().unwrap();
        let now = Instant::now();

        // Update time spent in previous state
        if let Some(start_time) = timings.current_state_start {
            let duration = now.duration_since(start_time);
            
            match old_state {
                LifecycleState::Created => timings.time_in_created += duration,
                LifecycleState::Started => timings.time_in_started += duration,
                LifecycleState::Resumed => timings.time_in_resumed += duration,
                LifecycleState::Paused => timings.time_in_paused += duration,
                LifecycleState::Stopped => timings.time_in_stopped += duration,
                LifecycleState::Destroyed => {}, // Terminal state
            }
        }

        // Update counters for new state
        match new_state {
            LifecycleState::Created => timings.created_count += 1,
            LifecycleState::Started => timings.started_count += 1,
            LifecycleState::Resumed => timings.resumed_count += 1,
            LifecycleState::Paused => timings.paused_count += 1,
            LifecycleState::Stopped => timings.stopped_count += 1,
            LifecycleState::Destroyed => timings.destroyed_count += 1,
        }

        timings.current_state_start = Some(now);
    }

    /// Update foreground status based on state
    fn update_foreground_status(&self, state: LifecycleState) {
        let is_foreground = matches!(state, LifecycleState::Resumed | LifecycleState::Started);
        self.is_foreground.store(is_foreground, Ordering::SeqCst);
    }

    /// Notify all observers of state change
    fn notify_observers(&self, new_state: LifecycleState) {
        let observers = self.observers.read().unwrap();
        
        for observer in observers.iter() {
            let result = match new_state {
                LifecycleState::Created => observer.on_create(),
                LifecycleState::Started => observer.on_start(),
                LifecycleState::Resumed => observer.on_resume(),
                LifecycleState::Paused => observer.on_pause(),
                LifecycleState::Stopped => observer.on_stop(),
                LifecycleState::Destroyed => observer.on_destroy(),
            };

            if let Err(e) = result {
                tracing::error!("Lifecycle observer failed in {:?}: {}", new_state, e);
            }
        }
    }

    /// Notify observers of low memory
    fn notify_low_memory(&self) {
        let observers = self.observers.read().unwrap();
        
        for observer in observers.iter() {
            if let Err(e) = observer.on_low_memory() {
                tracing::error!("Low memory observer failed: {}", e);
            }
        }
    }

    /// Check if transition is valid
    pub fn is_valid_transition(&self, from: LifecycleState, to: LifecycleState) -> bool {
        use LifecycleState::*;
        
        match (from, to) {
            // From Created
            (Created, Started) => true,
            (Created, Destroyed) => true,
            
            // From Started
            (Started, Resumed) => true,
            (Started, Stopped) => true,
            
            // From Resumed
            (Resumed, Paused) => true,
            
            // From Paused
            (Paused, Resumed) => true,
            (Paused, Stopped) => true,
            
            // From Stopped
            (Stopped, Started) => true,
            (Stopped, Destroyed) => true,
            
            // From Destroyed (terminal state)
            (Destroyed, _) => false,
            
            // Same state
            (state1, state2) if state1 == state2 => true,
            
            // Invalid transitions
            _ => false,
        }
    }

    /// Force state transition with validation
    pub fn transition_to(&self, target_state: LifecycleState) -> Result<()> {
        let current = self.get_state();
        
        if !self.is_valid_transition(current, target_state) {
            return Err(anyhow::anyhow!(
                "Invalid lifecycle transition: {:?} -> {:?}",
                current,
                target_state
            ));
        }

        self.set_state(target_state);
        Ok(())
    }

    /// Get lifecycle health score (0-100)
    pub fn get_health_score(&self) -> u8 {
        let stats = self.get_statistics();
        let mut score = 100u8;

        // Penalize for memory pressure
        score = match stats.memory_pressure {
            MemoryPressure::None => score,
            MemoryPressure::Low => score.saturating_sub(10),
            MemoryPressure::Moderate => score.saturating_sub(25),
            MemoryPressure::Critical => score.saturating_sub(50),
        };

        // Penalize for excessive transitions
        if stats.transition_count > 100 {
            score = score.saturating_sub(20);
        }

        // Penalize if destroyed multiple times
        if stats.destroyed_count > 1 {
            score = score.saturating_sub(30);
        }

        score
    }
}

/// Lifecycle statistics for monitoring
#[derive(Debug, Clone)]
pub struct LifecycleStatistics {
    pub current_state: LifecycleState,
    pub transition_count: u32,
    pub is_foreground: bool,
    pub memory_pressure: MemoryPressure,
    pub time_in_created: Duration,
    pub time_in_resumed: Duration,
    pub time_in_paused: Duration,
    pub created_count: u32,
    pub resumed_count: u32,
    pub paused_count: u32,
    pub destroyed_count: u32,
}

/// Default lifecycle observer for Rust UI components
pub struct RustUILifecycleObserver {
    ui_renderer: Arc<std::sync::Mutex<crate::rust_ui::RustUIRenderer>>,
}

impl RustUILifecycleObserver {
    pub fn new(ui_renderer: Arc<std::sync::Mutex<crate::rust_ui::RustUIRenderer>>) -> Self {
        Self { ui_renderer }
    }
}

impl LifecycleObserver for RustUILifecycleObserver {
    fn on_create(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onCreate");
        Ok(())
    }

    fn on_start(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onStart");
        Ok(())
    }

    fn on_resume(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onResume - starting rendering");
        // Resume UI rendering
        Ok(())
    }

    fn on_pause(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onPause - pausing rendering");
        // Pause UI rendering to save battery
        Ok(())
    }

    fn on_stop(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onStop");
        Ok(())
    }

    fn on_destroy(&self) -> Result<()> {
        tracing::debug!("🎨 Rust UI: onDestroy - cleanup");
        // Cleanup UI resources
        Ok(())
    }

    fn on_low_memory(&self) -> Result<()> {
        tracing::warn!("🎨 Rust UI: low memory - reducing cache sizes");
        // Reduce UI caches, release unused resources
        Ok(())
    }

    fn on_configuration_changed(&self) -> Result<()> {
        tracing::info!("🎨 Rust UI: configuration changed - adapting layout");
        // Handle screen rotation, theme changes, etc.
        Ok(())
    }
}

/// Memory management utilities
impl AndroidLifecycle {
    /// Trigger garbage collection if under memory pressure
    pub fn request_gc_if_needed(&self) {
        match self.get_memory_pressure() {
            MemoryPressure::Moderate | MemoryPressure::Critical => {
                tracing::info!("🗑️ Requesting garbage collection due to memory pressure");
                // In real implementation, might call System.gc() via JNI
            }
            _ => {}
        }
    }

    /// Get memory recommendations
    pub fn get_memory_recommendations(&self) -> Vec<String> {
        let pressure = self.get_memory_pressure();
        let mut recommendations = Vec::new();

        match pressure {
            MemoryPressure::None => {
                recommendations.push("Memory usage is optimal".to_string());
            }
            MemoryPressure::Low => {
                recommendations.push("Consider reducing cache sizes".to_string());
                recommendations.push("Release unused bitmap resources".to_string());
            }
            MemoryPressure::Moderate => {
                recommendations.push("Reduce UI texture cache".to_string());
                recommendations.push("Minimize background processing".to_string());
                recommendations.push("Consider pausing non-essential features".to_string());
            }
            MemoryPressure::Critical => {
                recommendations.push("Immediately release all non-essential resources".to_string());
                recommendations.push("Pause rendering if possible".to_string());
                recommendations.push("Force garbage collection".to_string());
                recommendations.push("Consider finishing activity".to_string());
            }
        }

        recommendations
    }
}

unsafe impl Send for AndroidLifecycle {}
unsafe impl Sync for AndroidLifecycle {}

impl std::fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LifecycleState::Created => write!(f, "Created"),
            LifecycleState::Started => write!(f, "Started"),
            LifecycleState::Resumed => write!(f, "Resumed"),
            LifecycleState::Paused => write!(f, "Paused"),
            LifecycleState::Stopped => write!(f, "Stopped"),
            LifecycleState::Destroyed => write!(f, "Destroyed"),
        }
    }
}

impl std::fmt::Display for MemoryPressure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryPressure::None => write!(f, "None"),
            MemoryPressure::Low => write!(f, "Low"),
            MemoryPressure::Moderate => write!(f, "Moderate"),
            MemoryPressure::Critical => write!(f, "Critical"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct TestObserver {
        call_count: Arc<Mutex<u32>>,
    }

    impl TestObserver {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn get_call_count(&self) -> u32 {
            *self.call_count.lock().unwrap()
        }
    }

    impl LifecycleObserver for TestObserver {
        fn on_create(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_start(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_resume(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_pause(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_stop(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_destroy(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_low_memory(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }

        fn on_configuration_changed(&self) -> Result<()> {
            *self.call_count.lock().unwrap() += 1;
            Ok(())
        }
    }

    #[test]
    fn test_lifecycle_state_transitions() {
        let lifecycle = AndroidLifecycle::new();
        
        assert_eq!(lifecycle.get_state(), LifecycleState::Created);
        
        lifecycle.set_state(LifecycleState::Started);
        assert_eq!(lifecycle.get_state(), LifecycleState::Started);
        assert!(lifecycle.is_foreground());
        
        lifecycle.set_state(LifecycleState::Resumed);
        assert_eq!(lifecycle.get_state(), LifecycleState::Resumed);
        assert!(lifecycle.can_render());
    }

    #[test]
    fn test_lifecycle_observers() {
        let lifecycle = AndroidLifecycle::new();
        let observer = Arc::new(TestObserver::new());
        
        lifecycle.add_observer(observer.clone());
        lifecycle.set_state(LifecycleState::Started);
        lifecycle.set_state(LifecycleState::Resumed);
        lifecycle.set_state(LifecycleState::Paused);
        
        assert_eq!(observer.get_call_count(), 3);
    }

    #[test]
    fn test_valid_transitions() {
        let lifecycle = AndroidLifecycle::new();
        
        assert!(lifecycle.is_valid_transition(LifecycleState::Created, LifecycleState::Started));
        assert!(lifecycle.is_valid_transition(LifecycleState::Resumed, LifecycleState::Paused));
        assert!(!lifecycle.is_valid_transition(LifecycleState::Created, LifecycleState::Resumed));
        assert!(!lifecycle.is_valid_transition(LifecycleState::Destroyed, LifecycleState::Created));
    }

    #[test]
    fn test_memory_pressure() {
        let lifecycle = AndroidLifecycle::new();
        
        assert_eq!(lifecycle.get_memory_pressure(), MemoryPressure::None);
        
        lifecycle.set_memory_pressure(MemoryPressure::Critical);
        assert_eq!(lifecycle.get_memory_pressure(), MemoryPressure::Critical);
        
        let health = lifecycle.get_health_score();
        assert!(health < 100); // Should be penalized for critical memory pressure
    }

    #[test]
    fn test_lifecycle_statistics() {
        let lifecycle = AndroidLifecycle::new();
        
        lifecycle.set_state(LifecycleState::Started);
        lifecycle.set_state(LifecycleState::Resumed);
        lifecycle.set_state(LifecycleState::Paused);
        
        let stats = lifecycle.get_statistics();
        assert_eq!(stats.current_state, LifecycleState::Paused);
        assert!(stats.transition_count > 0);
        assert_eq!(stats.resumed_count, 1);
        assert_eq!(stats.paused_count, 1);
    }
}