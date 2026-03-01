/// SpruceVM Runtime - Vue 3 component execution and lifecycle management
/// 
/// Manages Vue component instances, reactive state, and lifecycle hooks
/// with ultra-fast performance optimizations.

use anyhow::Result;
use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use crate::sprucevm::{bytecode::*, engine::BytecodeEngine, memory::MemoryManager};

/// Vue component instance running in SpruceVM
#[derive(Debug, Clone)]
pub struct ComponentInstance {
    /// Unique component ID
    pub id: u32,
    
    /// Component data (reactive refs, computed, etc.)
    pub data: HashMap<String, ReactiveValue>,
    
    /// Component methods
    pub methods: HashMap<String, u32>, // method_name -> bytecode_offset
    
    /// Computed properties with dependency tracking
    pub computed: HashMap<String, ComputedProperty>,
    
    /// Component lifecycle state
    pub lifecycle_state: LifecycleState,
    
    /// Virtual DOM representation
    pub vdom: Option<VirtualDOMNode>,
}

/// Reactive value with dependency tracking
#[derive(Debug, Clone)]
pub struct ReactiveValue {
    pub value: serde_json::Value,
    pub dependencies: Vec<String>,
    pub version: u64, // For change tracking
}

/// Computed property with cached result
#[derive(Debug, Clone)]
pub struct ComputedProperty {
    pub bytecode_offset: u32,
    pub cached_value: Option<serde_json::Value>,
    pub dependencies: Vec<String>,
    pub dirty: bool,
}

/// Component lifecycle states
#[derive(Debug, PartialEq, Clone)]
pub enum LifecycleState {
    Created,
    Mounted,
    Updated,
    BeforeUnmount,
    Unmounted,
}

/// Virtual DOM node for fast diffing
#[derive(Debug, Clone)]
pub struct VirtualDOMNode {
    pub tag: String,
    pub props: HashMap<String, serde_json::Value>,
    pub children: Vec<VirtualDOMNode>,
    pub text: Option<String>,
}

/// SpruceVM Runtime - executes Vue components with maximum performance
pub struct VueRuntime {
    /// Bytecode execution engine
    engine: Arc<RwLock<BytecodeEngine>>,
    
    /// Memory manager for zero-allocation operations
    memory: MemoryManager,
    
    /// Active component instances
    components: HashMap<u32, ComponentInstance>,
    
    /// Next component ID
    next_component_id: u32,
    
    /// Global reactive state
    global_state: HashMap<String, ReactiveValue>,
}

impl VueRuntime {
    /// Create new SpruceVM runtime
    pub fn new(engine: Arc<RwLock<BytecodeEngine>>, memory: MemoryManager) -> Result<Self> {
        Ok(Self {
            engine,
            memory,
            components: HashMap::new(),
            next_component_id: 1,
            global_state: HashMap::new(),
        })
    }

    /// Execute compiled Vue component
    pub fn execute_component(&mut self, compiled: &CompiledComponent) -> Result<ComponentInstance> {
        let component_id = self.next_component_id;
        self.next_component_id += 1;

        // Initialize component data from setup() function
        let mut data = HashMap::new();
        
        // Execute setup bytecode to get initial reactive state
        if let Some(setup_offset) = compiled.setup_bytecode_offset {
            let setup_result = self.engine.write().execute_bytecode_range(
                &compiled.bytecode.instructions, 
                setup_offset as usize, 
                compiled.bytecode.instructions.len()
            )?;
            
            // Parse setup result to extract reactive refs, computed properties, etc.
            if let Some(setup_data) = setup_result.as_object() {
                for (key, value) in setup_data {
                    data.insert(key.clone(), ReactiveValue {
                        value: value.clone(),
                        dependencies: vec![],
                        version: 0,
                    });
                }
            }
        }

        // Initialize computed properties
        let mut computed = HashMap::new();
        for (name, offset) in &compiled.computed_properties {
            computed.insert(name.clone(), ComputedProperty {
                bytecode_offset: *offset,
                cached_value: None,
                dependencies: vec![],
                dirty: true,
            });
        }

        // Create component instance
        let mut instance = ComponentInstance {
            id: component_id,
            data,
            methods: compiled.methods.clone(),
            computed,
            lifecycle_state: LifecycleState::Created,
            vdom: None,
        };

        // Execute template to generate initial VDOM
        if let Some(template_offset) = compiled.template_bytecode_offset {
            let vdom_result = self.engine.write().execute_bytecode_range(
                &compiled.bytecode.instructions,
                template_offset as usize,
                compiled.bytecode.instructions.len()
            )?;
            instance.vdom = self.parse_vdom_from_result(&vdom_result)?;
        }

        // Store component
        self.components.insert(component_id, instance.clone());

        // Trigger created() lifecycle hook
        self.trigger_lifecycle_hook(&mut instance, "created")?;

        Ok(instance)
    }

    /// Update reactive data and trigger re-renders
    pub fn update_reactive_data(&mut self, component_id: u32, key: &str, new_value: serde_json::Value) -> Result<()> {
        if let Some(component) = self.components.get_mut(&component_id) {
            if let Some(reactive_val) = component.data.get_mut(key) {
                reactive_val.value = new_value;
                reactive_val.version += 1;

                // Mark computed properties that depend on this key as dirty
                for computed in component.computed.values_mut() {
                    if computed.dependencies.contains(&key.to_string()) {
                        computed.dirty = true;
                        computed.cached_value = None;
                    }
                }

                // Trigger reactivity update (re-render)
                self.trigger_update(component_id)?;
            }
        }
        Ok(())
    }

    /// Trigger component update and re-render
    fn trigger_update(&mut self, component_id: u32) -> Result<()> {
        // Extract what we need before the mutable borrow
        let lifecycle_hook_offset = self.components
            .get_mut(&component_id)
            .and_then(|component| {
                component.lifecycle_state = LifecycleState::Updated;
                component.methods.get("updated").copied()
            });

        // Execute lifecycle hook if it exists
        if let Some(method_offset) = lifecycle_hook_offset {
            self.engine.write().execute_bytecode(method_offset, &[])?;
        }

        Ok(())
    }

    /// Execute Vue component lifecycle hooks
    fn trigger_lifecycle_hook(&mut self, component: &mut ComponentInstance, hook_name: &str) -> Result<()> {
        if let Some(&method_offset) = component.methods.get(hook_name) {
            // Execute the lifecycle hook bytecode
            self.engine.write().execute_bytecode(method_offset, &[])?;
        }
        Ok(())
    }

    /// Parse VDOM from bytecode execution result
    fn parse_vdom_from_result(&self, result: &serde_json::Value) -> Result<Option<VirtualDOMNode>> {
        // Parse the JSON result into VDOM structure
        // This is a simplified implementation
        if let Some(obj) = result.as_object() {
            if let Some(tag) = obj.get("tag").and_then(|t| t.as_str()) {
                let mut props = HashMap::new();
                if let Some(props_obj) = obj.get("props").and_then(|p| p.as_object()) {
                    for (key, value) in props_obj {
                        props.insert(key.clone(), value.clone());
                    }
                }

                let children = if let Some(children_array) = obj.get("children").and_then(|c| c.as_array()) {
                    children_array.iter()
                        .filter_map(|child| self.parse_vdom_from_result(child).ok().flatten())
                        .collect()
                } else {
                    vec![]
                };

                return Ok(Some(VirtualDOMNode {
                    tag: tag.to_string(),
                    props,
                    children,
                    text: obj.get("text").and_then(|t| t.as_str()).map(|s| s.to_string()),
                }));
            }
        }
        
        Ok(None)
    }

    /// Get component by ID
    pub fn get_component(&self, component_id: u32) -> Option<&ComponentInstance> {
        self.components.get(&component_id)
    }

    /// Destroy component and cleanup resources
    pub fn destroy_component(&mut self, component_id: u32) -> Result<()> {
        if let Some(mut component) = self.components.remove(&component_id) {
            // Trigger beforeUnmount hook
            component.lifecycle_state = LifecycleState::BeforeUnmount;
            self.trigger_lifecycle_hook(&mut component, "beforeUnmount")?;
            
            // Cleanup and trigger unmounted hook
            component.lifecycle_state = LifecycleState::Unmounted;
            self.trigger_lifecycle_hook(&mut component, "unmounted")?;
        }
        Ok(())
    }

    /// Get runtime performance statistics
    pub fn get_stats(&self) -> RuntimeStats {
        RuntimeStats {
            active_components: self.components.len(),
            total_reactive_values: self.components.values()
                .map(|c| c.data.len())
                .sum(),
            memory_usage: self.memory.get_stats().heap_size,
        }
    }
}

/// Runtime performance statistics
#[derive(Debug, Clone)]
pub struct RuntimeStats {
    pub active_components: usize,
    pub total_reactive_values: usize,
    pub memory_usage: usize,
}

impl Default for ComponentInstance {
    fn default() -> Self {
        Self {
            id: 0,
            data: HashMap::new(),
            methods: HashMap::new(),
            computed: HashMap::new(),
            lifecycle_state: LifecycleState::Created,
            vdom: None,
        }
    }
}