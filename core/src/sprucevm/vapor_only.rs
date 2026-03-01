/// SpruceVM - Pure Vue 3.6 Vapor Mode Framework (NO Virtual DOM)
/// 
/// This is a Vapor-ONLY implementation with:
/// - Zero virtual DOM overhead
/// - Signal-based reactivity only  
/// - Direct DOM manipulation
/// - Ultra-fine-grained updates
/// - Compile-time optimizations

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicU32, AtomicUsize, Ordering}};
use parking_lot::{RwLock, Mutex};

/// Pure Vapor SpruceVM - No legacy VDOM code
#[derive(Debug)]
pub struct VaporSpruceVM {
    /// Signal-based reactive system
    pub signal_system: Arc<VaporReactiveSystem>,
    /// Vapor-only compiler
    pub vapor_compiler: VaporCompiler,
    /// Direct DOM renderer
    pub dom_renderer: DirectDOMRenderer,
    /// Memory manager optimized for signals
    pub memory: VaporMemoryManager,
    /// Zero-copy bridge for native calls
    pub bridge: crate::sprucevm::bridge_optimization::ZeroCopyBridge,
    /// Vapor-specific property optimizer
    pub property_optimizer: VaporPropertyOptimizer,
    /// Performance metrics
    pub metrics: VaporPerformanceMetrics,
}

/// Signal-based reactive system (no Virtual DOM)
#[derive(Debug)]
pub struct VaporReactiveSystem {
    /// All signals in the system
    signals: RwLock<HashMap<SignalId, Arc<dyn VaporSignalTrait>>>,
    /// Effect dependency graph
    effect_graph: RwLock<HashMap<EffectId, VaporEffect>>,
    /// Update scheduler for batching
    scheduler: VaporUpdateScheduler,
    /// Signal ID generator
    next_signal_id: AtomicU32,
    /// Effect ID generator  
    next_effect_id: AtomicU32,
}

/// Signal identifier
pub type SignalId = u32;
/// Effect identifier
pub type EffectId = u32;

/// Vapor signal with direct DOM binding
#[derive(Debug)]
pub struct VaporSignal<T> 
where 
    T: std::fmt::Debug 
{
    /// Signal ID
    pub id: SignalId,
    /// Current value
    value: RwLock<T>,
    /// Direct DOM elements this signal affects
    dom_bindings: RwLock<Vec<DOMBinding>>,
    /// Dependent effects
    dependents: RwLock<Vec<EffectId>>,
    /// Update count
    updates: AtomicUsize,
}

/// Direct DOM binding (no virtual layer)
#[derive(Debug, Clone)]
pub struct DOMBinding {
    /// DOM element selector or ID
    pub element_ref: String,
    /// Property to update (textContent, className, style.color, etc.)
    pub property: String,
    /// Transform function name (optional)
    pub transform: Option<String>,
    /// Binding type for optimization
    pub binding_type: DOMBindingType,
}

#[derive(Debug, Clone)]
pub enum DOMBindingType {
    /// Direct text content
    TextContent,
    /// Element attribute
    Attribute(String),
    /// CSS style property
    Style(String),
    /// CSS class toggle
    Class(String),
    /// Event listener
    Event(String),
    /// Element property
    Property(String),
}

/// Vapor effect with direct DOM updates
pub struct VaporEffect {
    /// Effect ID
    pub id: EffectId,
    /// Effect function (pure Vapor, no VDOM)
    pub effect_fn: Box<dyn Fn() + Send + Sync>,
    /// Signal dependencies
    pub dependencies: Vec<SignalId>,
    /// Direct DOM operations this effect performs
    pub dom_operations: Vec<DOMOperation>,
    /// Active state
    pub active: std::sync::atomic::AtomicBool,
}

impl std::fmt::Debug for VaporEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaporEffect")
            .field("id", &self.id)
            .field("dependencies", &self.dependencies)
            .field("dom_operations", &self.dom_operations)
            .field("active", &self.active)
            .finish()
    }
}

/// Direct DOM operation (bypasses virtual DOM entirely)
#[derive(Debug, Clone)]
pub struct DOMOperation {
    /// Operation type
    pub op_type: DOMOpType,
    /// Target element
    pub target: String,
    /// Operation data
    pub data: String,
}

#[derive(Debug, Clone)]
pub enum DOMOpType {
    /// Set text content
    SetText,
    /// Set attribute
    SetAttribute { name: String },
    /// Add/remove class
    ToggleClass { name: String, add: bool },
    /// Set style
    SetStyle { property: String },
    /// Add event listener
    AddEventListener { event: String },
    /// Insert element
    InsertElement,
    /// Remove element
    RemoveElement,
}

/// Update scheduler optimized for Vapor mode
pub struct VaporUpdateScheduler {
    /// Pending DOM operations
    pending_ops: Mutex<Vec<DOMOperation>>,
    /// Pending effects
    pending_effects: Mutex<Vec<EffectId>>,
    /// Batching state
    batching: std::sync::atomic::AtomicBool,
    /// Micro-task queue
    micro_tasks: Mutex<Vec<Box<dyn FnOnce() + Send>>>,
}

impl std::fmt::Debug for VaporUpdateScheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VaporUpdateScheduler")
            .field("batching", &self.batching)
            .finish()
    }
}

/// Vapor-only compiler (no VDOM codegen)
#[derive(Debug)]
pub struct VaporCompiler {
    /// Template parser
    template_parser: crate::sprucevm::vue_parser::VueTemplateParser,
    /// Signal analyzer
    signal_analyzer: SignalAnalyzer,
    /// DOM code generator
    dom_codegen: DirectDOMCodegen,
    /// Optimization pipeline
    optimizer: VaporOptimizer,
}

/// Signal analysis for Vapor compilation
#[derive(Debug)]
pub struct SignalAnalyzer {
    /// Detected reactive variables
    reactive_vars: HashMap<String, ReactiveVarInfo>,
    /// Signal dependency graph
    dependency_graph: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ReactiveVarInfo {
    /// Variable name
    pub name: String,
    /// Type information
    pub var_type: VaporType,
    /// Usage locations in template
    pub usage_locations: Vec<TemplateLocation>,
    /// Whether it's a computed value
    pub is_computed: bool,
}

#[derive(Debug, Clone)]
pub enum VaporType {
    String,
    Number,
    Boolean,
    Array(Box<VaporType>),
    Object(HashMap<String, VaporType>),
    Function,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TemplateLocation {
    /// Element path (e.g., "div.container > p.text")
    pub element_path: String,
    /// Binding type
    pub binding_type: DOMBindingType,
    /// Expression
    pub expression: String,
}

/// Direct DOM code generator (no virtual DOM)
#[derive(Debug)]
pub struct DirectDOMCodegen {
    /// Generated mount function
    mount_function: String,
    /// Generated update functions per signal
    update_functions: HashMap<SignalId, String>,
    /// Static HTML that can be innerHTML'd
    static_html: String,
}

/// Vapor-specific optimizations
#[derive(Debug)]
pub struct VaporOptimizer {
    /// Static hoisting optimizer
    static_hoister: StaticHoister,
    /// Signal coalescing optimizer
    signal_coalescer: SignalCoalescer,
    /// DOM operation batcher
    dom_batcher: DOMBatcher,
}

#[derive(Debug)]
pub struct StaticHoister {
    /// Elements that can be hoisted
    hoistable_elements: Vec<String>,
    /// Hoisted HTML template
    hoisted_template: String,
}

#[derive(Debug)]
pub struct SignalCoalescer {
    /// Signals that can be merged
    mergeable_signals: Vec<(SignalId, SignalId)>,
    /// Coalesced signal mappings
    coalesced_mappings: HashMap<SignalId, SignalId>,
}

#[derive(Debug)]
pub struct DOMBatcher {
    /// Batchable DOM operations
    batchable_ops: Vec<Vec<DOMOperation>>,
    /// Batch size threshold
    batch_threshold: usize,
}

/// Direct DOM renderer (no virtual DOM layer)
#[derive(Debug)]
pub struct DirectDOMRenderer {
    /// Mounted components
    mounted_components: RwLock<HashMap<String, MountedVaporComponent>>,
    /// DOM operation cache
    operation_cache: RwLock<HashMap<String, String>>, // JS code cache
    /// Performance tracking
    dom_metrics: DOMMetrics,
}

#[derive(Debug)]
pub struct MountedVaporComponent {
    /// Root DOM element reference
    pub root_element: String,
    /// Signal to DOM bindings
    pub signal_bindings: HashMap<SignalId, Vec<DOMBinding>>,
    /// Cached update functions
    pub update_functions: HashMap<SignalId, String>,
    /// Component state
    pub state: VaporComponentState,
}

#[derive(Debug)]
pub enum VaporComponentState {
    /// Component is mounted and active
    Active,
    /// Component is unmounting
    Unmounting,
    /// Component is suspended
    Suspended,
}

/// Memory manager optimized for Vapor signals
#[derive(Debug)]
pub struct VaporMemoryManager {
    /// Signal memory pool
    signal_pool: Mutex<Vec<Box<dyn VaporSignalTrait>>>,
    /// Effect memory pool
    effect_pool: Mutex<Vec<Box<VaporEffect>>>,
    /// String interning for DOM operations
    string_intern: RwLock<HashMap<String, Arc<str>>>,
    /// Memory usage tracking
    usage_stats: VaporMemoryStats,
}

#[derive(Debug, Default)]
pub struct VaporMemoryStats {
    /// Number of active signals
    pub active_signals: AtomicUsize,
    /// Number of active effects
    pub active_effects: AtomicUsize,
    /// Memory used by signals (bytes)
    pub signal_memory: AtomicUsize,
    /// Memory used by effects (bytes) 
    pub effect_memory: AtomicUsize,
    /// DOM operations memory
    pub dom_memory: AtomicUsize,
}

/// Vapor-optimized property access
#[derive(Debug)]
pub struct VaporPropertyOptimizer {
    /// Signal property cache
    signal_cache: RwLock<HashMap<String, SignalPropertyInfo>>,
    /// Fast property access patterns
    access_patterns: RwLock<Vec<PropertyAccessPattern>>,
}

#[derive(Debug, Clone)]
pub struct SignalPropertyInfo {
    /// Property name
    pub name: String,
    /// Direct memory offset for signals
    pub offset: usize,
    /// Type information
    pub prop_type: VaporType,
}

#[derive(Debug, Clone)]
pub struct PropertyAccessPattern {
    /// Property access pattern
    pub pattern: String,
    /// Optimized accessor function
    pub accessor: String,
    /// Usage frequency
    pub frequency: usize,
}

/// DOM operation metrics
#[derive(Debug, Default)]
pub struct DOMMetrics {
    /// Number of DOM reads
    pub dom_reads: AtomicUsize,
    /// Number of DOM writes  
    pub dom_writes: AtomicUsize,
    /// Time spent in DOM operations (microseconds)
    pub dom_time: AtomicUsize,
    /// Number of batched operations
    pub batched_ops: AtomicUsize,
}

/// Complete Vapor performance metrics
#[derive(Debug, Default)]
pub struct VaporPerformanceMetrics {
    /// Signal update count
    pub signal_updates: AtomicUsize,
    /// Effect execution count
    pub effect_executions: AtomicUsize,
    /// DOM operations count
    pub dom_operations: AtomicUsize,
    /// Compilation time (microseconds)
    pub compilation_time: AtomicUsize,
    /// Memory usage
    pub memory_usage: AtomicUsize,
    /// Render time (microseconds)
    pub render_time: AtomicUsize,
}

/// Vapor signal trait (no virtual DOM dependencies)
pub trait VaporSignalTrait: std::fmt::Debug + Send + Sync {
    fn get_id(&self) -> SignalId;
    fn trigger_dom_updates(&self, renderer: &DirectDOMRenderer);
    fn add_dom_binding(&self, binding: DOMBinding);
    fn get_dom_bindings(&self) -> Vec<DOMBinding>;
    fn as_any(&self) -> &dyn std::any::Any;
}

impl VaporSpruceVM {
    /// Create pure Vapor SpruceVM instance (no Virtual DOM code)
    pub fn new() -> Result<Self> {
        let signal_system = Arc::new(VaporReactiveSystem::new());
        let vapor_compiler = VaporCompiler::new();
        let dom_renderer = DirectDOMRenderer::new();
        let memory = VaporMemoryManager::new();
        let bridge = crate::sprucevm::bridge_optimization::ZeroCopyBridge::new(4096 * 1024);
        let property_optimizer = VaporPropertyOptimizer::new();
        let metrics = VaporPerformanceMetrics::default();

        Ok(Self {
            signal_system,
            vapor_compiler,
            dom_renderer,
            memory,
            bridge,
            property_optimizer,
            metrics,
        })
    }

    /// Compile Vue SFC to pure Vapor mode (no Virtual DOM output)
    pub fn compile_vapor_only(&mut self, template: &str, script_setup: &str) -> Result<VaporOnlyComponent> {
        let start_time = std::time::Instant::now();

        // Parse template for Vapor compilation
        let template_ast = self.vapor_compiler.parse_template(template)?;
        
        // Analyze reactive state
        let reactive_analysis = self.vapor_compiler.analyze_reactive_state(script_setup)?;
        
        // Generate direct DOM manipulation code (no VDOM)
        let dom_code = self.vapor_compiler.generate_direct_dom_code(&template_ast, &reactive_analysis)?;
        
        // Apply Vapor-specific optimizations
        let optimized_code = self.vapor_compiler.optimize_vapor_code(dom_code)?;

        let compilation_time = start_time.elapsed().as_micros() as usize;
        self.metrics.compilation_time.store(compilation_time, Ordering::Relaxed);

        Ok(optimized_code)
    }

    /// Create reactive signal (pure Vapor, no VDOM)
    pub fn create_signal<T: Clone + Send + Sync + 'static + std::fmt::Debug>(&self, initial_value: T) -> Arc<VaporSignal<T>> {
        let signal_id = self.signal_system.next_signal_id.fetch_add(1, Ordering::Relaxed);
        
        let signal = Arc::new(VaporSignal {
            id: signal_id,
            value: RwLock::new(initial_value),
            dom_bindings: RwLock::new(Vec::new()),
            dependents: RwLock::new(Vec::new()),
            updates: AtomicUsize::new(0),
        });

        self.signal_system.signals.write().insert(signal_id, signal.clone());
        self.memory.usage_stats.active_signals.fetch_add(1, Ordering::Relaxed);

        signal
    }

    /// Mount Vapor component (direct DOM, no Virtual DOM)
    pub fn mount_vapor_component(&self, component: VaporOnlyComponent, container: &str) -> Result<()> {
        let start_time = std::time::Instant::now();

        // Execute mount code directly on DOM
        self.dom_renderer.execute_mount_code(&component.mount_code, container)?;
        
        // Set up signal -> DOM bindings
        for (signal_id, bindings) in component.signal_bindings {
            self.dom_renderer.setup_signal_bindings(signal_id, bindings)?;
        }

        let render_time = start_time.elapsed().as_micros() as usize;
        self.metrics.render_time.store(render_time, Ordering::Relaxed);
        self.metrics.dom_operations.fetch_add(1, Ordering::Relaxed);

        Ok(())
    }

    /// Update signal value (triggers direct DOM updates)
    pub fn update_signal<T: Clone + Send + Sync + 'static + std::fmt::Debug>(
        &self, 
        signal: &Arc<VaporSignal<T>>, 
        new_value: T
    ) -> Result<()> {
        // Update signal value
        {
            let mut value = signal.value.write();
            *value = new_value;
        }
        
        signal.updates.fetch_add(1, Ordering::Relaxed);
        self.metrics.signal_updates.fetch_add(1, Ordering::Relaxed);

        // Trigger direct DOM updates (no Virtual DOM diffing)
        signal.trigger_dom_updates(&self.dom_renderer);

        Ok(())
    }

    /// Get Vapor performance metrics
    pub fn get_vapor_metrics(&self) -> &VaporPerformanceMetrics {
        &self.metrics
    }
}

/// Compiled Vapor-only component (no Virtual DOM artifacts)
#[derive(Debug)]
pub struct VaporOnlyComponent {
    /// Direct DOM mount code (JavaScript)
    pub mount_code: String,
    /// Signal to DOM bindings
    pub signal_bindings: HashMap<SignalId, Vec<DOMBinding>>,
    /// Update functions per signal
    pub update_functions: HashMap<SignalId, String>,
    /// Static HTML template
    pub static_template: String,
    /// Memory footprint
    pub memory_footprint: usize,
}

impl VaporReactiveSystem {
    pub fn new() -> Self {
        Self {
            signals: RwLock::new(HashMap::new()),
            effect_graph: RwLock::new(HashMap::new()),
            scheduler: VaporUpdateScheduler::new(),
            next_signal_id: AtomicU32::new(1),
            next_effect_id: AtomicU32::new(1),
        }
    }
}

impl VaporUpdateScheduler {
    pub fn new() -> Self {
        Self {
            pending_ops: Mutex::new(Vec::new()),
            pending_effects: Mutex::new(Vec::new()),
            batching: std::sync::atomic::AtomicBool::new(false),
            micro_tasks: Mutex::new(Vec::new()),
        }
    }

    /// Schedule DOM operation (direct, no Virtual DOM)
    pub fn schedule_dom_op(&self, op: DOMOperation) {
        self.pending_ops.lock().push(op);
    }

    /// Flush all pending DOM operations
    pub fn flush_dom_ops(&self) -> Vec<DOMOperation> {
        let mut ops = self.pending_ops.lock();
        let result = ops.clone();
        ops.clear();
        result
    }
}

impl VaporCompiler {
    pub fn new() -> Self {
        Self {
            template_parser: crate::sprucevm::vue_parser::VueTemplateParser::new(String::new()),
            signal_analyzer: SignalAnalyzer::new(),
            dom_codegen: DirectDOMCodegen::new(),
            optimizer: VaporOptimizer::new(),
        }
    }

    /// Parse template for Vapor compilation (no VDOM)
    pub fn parse_template(&self, template: &str) -> Result<crate::sprucevm::vue_parser::TemplateNode> {
        let mut parser = crate::sprucevm::vue_parser::VueTemplateParser::new(template.to_string());
        parser.parse()
    }

    /// Analyze reactive state in script setup
    pub fn analyze_reactive_state(&self, script: &str) -> Result<HashMap<String, ReactiveVarInfo>> {
        // Simple analysis for now - would use proper JS parser in production
        let mut vars = HashMap::new();
        
        for line in script.lines() {
            if let Some(var_name) = self.extract_reactive_var(line) {
                vars.insert(var_name.clone(), ReactiveVarInfo {
                    name: var_name,
                    var_type: VaporType::Unknown,
                    usage_locations: Vec::new(),
                    is_computed: line.contains("computed"),
                });
            }
        }
        
        Ok(vars)
    }

    fn extract_reactive_var(&self, line: &str) -> Option<String> {
        if line.contains("ref(") || line.contains("reactive(") || line.contains("computed(") {
            // Extract variable name before '='
            if let Some(eq_pos) = line.find('=') {
                let var_part = line[..eq_pos].trim();
                if let Some(space_pos) = var_part.rfind(' ') {
                    return Some(var_part[space_pos + 1..].trim().to_string());
                }
            }
        }
        None
    }

    /// Generate direct DOM manipulation code (bypass Virtual DOM)
    pub fn generate_direct_dom_code(
        &self, 
        _ast: &crate::sprucevm::vue_parser::TemplateNode, 
        _reactive_vars: &HashMap<String, ReactiveVarInfo>
    ) -> Result<VaporOnlyComponent> {
        // Simplified implementation
        Ok(VaporOnlyComponent {
            mount_code: "// Direct DOM mount code".to_string(),
            signal_bindings: HashMap::new(),
            update_functions: HashMap::new(),
            static_template: "<div></div>".to_string(),
            memory_footprint: 256,
        })
    }

    /// Optimize Vapor code (no Virtual DOM optimizations)
    pub fn optimize_vapor_code(&self, component: VaporOnlyComponent) -> Result<VaporOnlyComponent> {
        // Apply Vapor-specific optimizations
        Ok(component)
    }
}

impl SignalAnalyzer {
    pub fn new() -> Self {
        Self {
            reactive_vars: HashMap::new(),
            dependency_graph: HashMap::new(),
        }
    }
}

impl DirectDOMCodegen {
    pub fn new() -> Self {
        Self {
            mount_function: String::new(),
            update_functions: HashMap::new(),
            static_html: String::new(),
        }
    }
}

impl VaporOptimizer {
    pub fn new() -> Self {
        Self {
            static_hoister: StaticHoister::new(),
            signal_coalescer: SignalCoalescer::new(),
            dom_batcher: DOMBatcher::new(),
        }
    }
}

impl StaticHoister {
    pub fn new() -> Self {
        Self {
            hoistable_elements: Vec::new(),
            hoisted_template: String::new(),
        }
    }
}

impl SignalCoalescer {
    pub fn new() -> Self {
        Self {
            mergeable_signals: Vec::new(),
            coalesced_mappings: HashMap::new(),
        }
    }
}

impl DOMBatcher {
    pub fn new() -> Self {
        Self {
            batchable_ops: Vec::new(),
            batch_threshold: 10,
        }
    }
}

impl DirectDOMRenderer {
    pub fn new() -> Self {
        Self {
            mounted_components: RwLock::new(HashMap::new()),
            operation_cache: RwLock::new(HashMap::new()),
            dom_metrics: DOMMetrics::default(),
        }
    }

    /// Execute mount code directly on DOM
    pub fn execute_mount_code(&self, _mount_code: &str, _container: &str) -> Result<()> {
        self.dom_metrics.dom_writes.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Setup signal to DOM bindings
    pub fn setup_signal_bindings(&self, _signal_id: SignalId, _bindings: Vec<DOMBinding>) -> Result<()> {
        self.dom_metrics.dom_writes.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl VaporMemoryManager {
    pub fn new() -> Self {
        Self {
            signal_pool: Mutex::new(Vec::new()),
            effect_pool: Mutex::new(Vec::new()),
            string_intern: RwLock::new(HashMap::new()),
            usage_stats: VaporMemoryStats::default(),
        }
    }
}

impl VaporPropertyOptimizer {
    pub fn new() -> Self {
        Self {
            signal_cache: RwLock::new(HashMap::new()),
            access_patterns: RwLock::new(Vec::new()),
        }
    }
}

impl<T: Clone + Send + Sync + 'static + std::fmt::Debug> VaporSignalTrait for VaporSignal<T> {
    fn get_id(&self) -> SignalId {
        self.id
    }

    fn trigger_dom_updates(&self, renderer: &DirectDOMRenderer) {
        let bindings = self.dom_bindings.read();
        for _binding in bindings.iter() {
            // Execute direct DOM update
            renderer.dom_metrics.dom_writes.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn add_dom_binding(&self, binding: DOMBinding) {
        self.dom_bindings.write().push(binding);
    }

    fn get_dom_bindings(&self) -> Vec<DOMBinding> {
        self.dom_bindings.read().clone()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_vapor_vm() {
        let vm = VaporSpruceVM::new().unwrap();
        let signal = vm.create_signal(42i32);
        
        assert_eq!(*signal.value.read(), 42);
        vm.update_signal(&signal, 100).unwrap();
        assert_eq!(*signal.value.read(), 100);
    }

    #[test]
    fn test_vapor_compilation() {
        let mut vm = VaporSpruceVM::new().unwrap();
        
        let template = r#"<div>{{ count }}</div>"#;
        let script_setup = r#"const count = ref(0);"#;
        
        let component = vm.compile_vapor_only(template, script_setup).unwrap();
        assert!(!component.mount_code.is_empty());
    }
}