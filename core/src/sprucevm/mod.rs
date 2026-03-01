/// SpruceVM - Ultra-fast Vue 3.6 Vapor Mode Engine (NO Virtual DOM)
/// 
/// Performance targets (Vapor-only):
/// - 70%+ faster than React Native (no Virtual DOM overhead)
/// - Sub-5ms Vue component compilation (direct DOM codegen)
/// - Zero-allocation signal updates (direct memory access)
/// - SIMD-optimized DOM operations
/// - 60% smaller bundle size (no VDOM runtime)
/// - 80% less memory usage (no virtual tree)
/// 
/// Key Vapor Mode advantages:
/// - Direct DOM manipulation (no Virtual DOM layer)
/// - Signal-based reactivity (no proxy overhead)
/// - Compile-time optimizations (no runtime diffing)
/// - Ultra-fine-grained updates (property-level tracking)
pub mod engine;
pub mod bytecode;
pub mod runtime;
pub mod compiler;
pub mod memory;
pub mod memory_safe;
pub mod register_allocator;
pub mod vue_parser;
pub mod property_access;
pub mod bridge_optimization;
pub mod vue36_vapor;
pub mod vapor_only;
pub mod vue36_complete;
pub mod optimizations;


/// SpruceVM - Complete Vue 3.6 Vapor Mode Engine with Alien Signals
/// 
/// Full Vue 3.6.0-beta.7 implementation with:
/// - Alien Signals reactivity system (major @vue/reactivity refactor)
/// - Pure Vapor Mode compilation (NO Virtual DOM)
/// - Performance targets: 100k components in 100ms
/// - Bundle optimization: <10KB base size
/// - Memory optimization: 50% reduction
/// - Direct DOM manipulation with signal bindings
/// - Compile-time optimizations
/// - Zero runtime overhead
pub type SpruceVM = vue36_complete::Vue36SpruceVM;

// All SpruceVM functionality is now implemented in vue36_complete::Vue36SpruceVM
// This provides the complete Vue 3.6.0-beta.7 feature set with Alien Signals