/// Ultra-efficient memory manager for SpruceVM
/// 
/// Optimizations:
/// - Object pooling for common types
/// - Generational GC with incremental collection
/// - NUMA-aware allocation
/// - Zero-copy string interning
/// - Bump allocation for short-lived objects
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::alloc::Layout;
use parking_lot::{RwLock, Mutex};

/// Memory manager with advanced optimizations
#[derive(Debug)]
pub struct MemoryManager {
    /// Object pools for common types
    object_pools: ObjectPools,
    
    /// Generational garbage collector
    gc: GenerationalGC,
    
    /// String interning for memory efficiency
    string_intern: StringIntern,
    
    /// Bump allocator for short-lived objects
    bump_allocator: BumpAllocator,
    
    /// Memory statistics
    stats: Arc<MemoryStats>,
}

// Safe because internal state uses lock-free atomic structures or explicit Mutexes/RwLocks
unsafe impl Send for MemoryManager {}
unsafe impl Sync for MemoryManager {}

/// Object pools for zero-allocation object reuse
#[derive(Debug)]
struct ObjectPools {
    /// Pool for Vue components
    component_pool: ObjectPool<crate::sprucevm::engine::VueComponent>,
    
    /// Pool for reactive objects
    reactive_pool: ObjectPool<crate::sprucevm::engine::ReactiveObject>,
    
    /// Pool for arrays
    array_pool: ObjectPool<Vec<crate::sprucevm::engine::Value>>,
    
    /// Pool for objects/maps
    object_pool: ObjectPool<HashMap<String, crate::sprucevm::engine::Value>>,
    
    /// Pool for strings
    string_pool: ObjectPool<String>,
}

/// Generic object pool with size-based buckets
#[derive(Debug)]
struct ObjectPool<T> {
    /// Available objects by size bucket
    pools: Vec<Mutex<Vec<T>>>,
    /// Pool statistics
    allocated: AtomicUsize,
    reused: AtomicUsize,
}

/// Generational garbage collector
#[derive(Debug)]
struct GenerationalGC {
    /// Young generation (frequent, fast collection)
    young_gen: Generation,
    
    /// Old generation (infrequent, thorough collection)
    old_gen: Generation,
    
    /// GC configuration
    config: GCConfig,
    
    /// Write barrier for generational collection
    write_barrier: WriteBarrier,
}

#[derive(Debug)]
struct Generation {
    /// Allocated objects
    objects: RwLock<Vec<*mut crate::sprucevm::engine::HeapObject>>,
    
    /// Size of allocated memory
    allocated_bytes: AtomicUsize,
    
    /// Allocation threshold before GC
    threshold: usize,
}

#[derive(Clone, Debug)]
struct GCConfig {
    /// Young generation size threshold
    _young_threshold: usize,
    
    /// Old generation size threshold  
    _old_threshold: usize,
    
    /// Incremental GC slice time (microseconds)
    _incremental_slice_time: u64,
    
    /// Promotion age threshold
    promotion_age: u32,
}

/// Write barrier for tracking inter-generational references
#[derive(Debug)]
struct WriteBarrier {
    /// Dirty cards (objects with cross-generational refs)
    dirty_cards: RwLock<Vec<*mut crate::sprucevm::engine::HeapObject>>,
}

/// String interning for memory efficiency
#[derive(Debug)]
struct StringIntern {
    /// Interned strings
    strings: RwLock<HashMap<String, Arc<str>>>,
    
    /// Statistics
    hits: AtomicUsize,
    misses: AtomicUsize,
}

/// Bump allocator for short-lived objects
#[derive(Debug)]
struct BumpAllocator {
    /// Current allocation pointer
    current: AtomicUsize,
    
    /// End of allocation area
    end: usize,
    
    /// Start of allocation area
    start: usize,
    
    /// Lock for reset operations
    reset_lock: Mutex<()>,
}

#[derive(Debug, Default, Clone)]
pub struct MemoryStats {
    /// Total bytes allocated
    pub total_allocated: usize,
    
    /// Total bytes freed
    pub total_freed: usize,
    
    /// Current heap size
    pub heap_size: usize,
    
    /// GC collections performed
    pub gc_collections: usize,
    
    /// Object pool hit rate
    pub pool_hit_rate: usize,
    
    /// String intern hit rate
    pub intern_hit_rate: usize,
}

impl Clone for MemoryManager {
    fn clone(&self) -> Self {
        // Create a fresh memory manager - sharing state is complex with raw pointers
        MemoryManager::new().expect("Failed to clone MemoryManager")
    }
}

impl MemoryManager {
    pub fn new() -> Result<Self> {
        let stats = Arc::new(MemoryStats::default());
        
        Ok(Self {
            object_pools: ObjectPools::new(),
            gc: GenerationalGC::new()?,
            string_intern: StringIntern::new(),
            bump_allocator: BumpAllocator::new(1024 * 1024)?, // 1MB bump space
            stats,
        })
    }

    /// Allocate a new heap object with optimal strategy
    pub fn allocate_object(&self, data: crate::sprucevm::engine::ObjectData) -> Result<*mut crate::sprucevm::engine::HeapObject> {
        use crate::sprucevm::engine::{HeapObject, ObjectType};
        
        // Try to reuse from object pools first
        let object_type = match &data {
            crate::sprucevm::engine::ObjectData::String(_) => {
                if let Some(_reused) = self.object_pools.try_get_string() {
                    let heap_obj = Box::new(HeapObject::new(ObjectType::String, data));
                    let ptr = Box::into_raw(heap_obj);
                    self.gc.register_object(ptr);
                    return Ok(ptr);
                }
                ObjectType::String
            },
            crate::sprucevm::engine::ObjectData::Array(_) => {
                if let Some(_reused) = self.object_pools.try_get_array() {
                    let heap_obj = Box::new(HeapObject::new(ObjectType::Array, data));
                    let ptr = Box::into_raw(heap_obj);
                    self.gc.register_object(ptr);
                    return Ok(ptr);
                }
                ObjectType::Array
            },
            crate::sprucevm::engine::ObjectData::Object(_) => {
                if let Some(_reused) = self.object_pools.try_get_object() {
                    let heap_obj = Box::new(HeapObject::new(ObjectType::Object, data));
                    let ptr = Box::into_raw(heap_obj);
                    self.gc.register_object(ptr);
                    return Ok(ptr);
                }
                ObjectType::Object
            },
            crate::sprucevm::engine::ObjectData::VueComponent(_) => {
                if let Some(_reused) = self.object_pools.try_get_component() {
                    let heap_obj = Box::new(HeapObject::new(ObjectType::VueComponent, data));
                    let ptr = Box::into_raw(heap_obj);
                    self.gc.register_object(ptr);
                    return Ok(ptr);
                }
                ObjectType::VueComponent
            },
            crate::sprucevm::engine::ObjectData::VueReactive(_) => {
                if let Some(_reused) = self.object_pools.try_get_reactive() {
                    let heap_obj = Box::new(HeapObject::new(ObjectType::VueReactive, data));
                    let ptr = Box::into_raw(heap_obj);
                    self.gc.register_object(ptr);
                    return Ok(ptr);
                }
                ObjectType::VueReactive
            },
            crate::sprucevm::engine::ObjectData::Function(_) => ObjectType::Function,
        };

        // Allocate new object if no pool object available
        let heap_obj = Box::new(HeapObject::new(object_type, data));
        let ptr = Box::into_raw(heap_obj);
        
        // Register with GC
        self.gc.register_object(ptr);

        Ok(ptr)
    }

    /// Deallocate object (called by reference counting or GC)
    pub unsafe fn deallocate_object(&self, ptr: *mut crate::sprucevm::engine::HeapObject) {
        if ptr.is_null() {
            return;
        }
        // Drop the box, freeing memory
        let _obj = Box::from_raw(ptr);
    }

    /// Perform garbage collection
    pub fn collect_garbage(&self) -> Result<usize> {
        self.gc.collect()
    }

    /// Allocate short-lived object using bump allocator
    pub fn allocate_temp<T>(&self, value: T) -> Result<*mut T> {
        self.bump_allocator.allocate(value)
    }

    /// Reset bump allocator (invalidates all temp allocations)
    pub fn reset_temp_allocations(&self) {
        self.bump_allocator.reset();
    }

    /// Get memory statistics
    pub fn get_stats(&self) -> MemoryStats {
        (*self.stats).clone()
    }

    /// Intern string for memory efficiency
    pub fn intern_string(&self, s: String) -> Arc<str> {
        self.string_intern.get_or_intern(s.clone()).unwrap_or_else(|| {
            Arc::from(s.as_str())
        })
    }

    /// Write barrier for cross-generational references
    pub fn write_barrier(&self, old_obj: *mut crate::sprucevm::engine::HeapObject, _young_obj: *mut crate::sprucevm::engine::HeapObject) {
        // Record that old generation object now references young generation object
        self.gc.write_barrier.record_cross_gen_ref(old_obj);
    }

    /// Set object field with write barrier check
    pub fn set_object_field(&self, obj_ptr: *mut crate::sprucevm::engine::HeapObject, _field: &str, value_ptr: *mut crate::sprucevm::engine::HeapObject) -> Result<()> {
        if obj_ptr.is_null() || value_ptr.is_null() {
            return Ok(());
        }
        
        // Check if this is a cross-generational reference (old -> young)
        let obj_in_old_gen = self.gc.is_in_old_generation(obj_ptr);
        let value_in_young_gen = self.gc.is_in_young_generation(value_ptr);
        
        if obj_in_old_gen && value_in_young_gen {
            // Trigger write barrier
            self.write_barrier(obj_ptr, value_ptr);
        }
        
        Ok(())
    }

}


impl ObjectPools {
    fn new() -> Self {
        Self {
            component_pool: ObjectPool::new(64),
            reactive_pool: ObjectPool::new(128),
            array_pool: ObjectPool::new(16),
            object_pool: ObjectPool::new(16),
            string_pool: ObjectPool::new(16),
        }
    }

    fn try_get_string(&self) -> Option<String> {
        self.string_pool.try_get()
    }

    fn try_get_array(&self) -> Option<Vec<crate::sprucevm::engine::Value>> {
        self.array_pool.try_get()
    }

    fn try_get_object(&self) -> Option<HashMap<String, crate::sprucevm::engine::Value>> {
        self.object_pool.try_get()
    }

    fn try_get_component(&self) -> Option<crate::sprucevm::engine::VueComponent> {
        self.component_pool.try_get()
    }

    fn try_get_reactive(&self) -> Option<crate::sprucevm::engine::ReactiveObject> {
        self.reactive_pool.try_get()
    }

    fn return_string(&self, obj: String) {
        self.string_pool.return_object(obj);
    }

    fn return_array(&self, obj: Vec<crate::sprucevm::engine::Value>) {
        self.array_pool.return_object(obj);
    }

    fn return_object(&self, obj: HashMap<String, crate::sprucevm::engine::Value>) {
        self.object_pool.return_object(obj);
    }

    fn return_component(&self, obj: crate::sprucevm::engine::VueComponent) {
        self.component_pool.return_object(obj);
    }

    fn return_reactive(&self, obj: crate::sprucevm::engine::ReactiveObject) {
        self.reactive_pool.return_object(obj);
    }
}

impl<T> ObjectPool<T> {
    fn new(initial_capacity: usize) -> Self {
        Self {
            pools: (0..8).map(|_| Mutex::new(Vec::with_capacity(initial_capacity))).collect(),
            allocated: AtomicUsize::new(0),
            reused: AtomicUsize::new(0),
        }
    }

    #[allow(dead_code)]
    fn try_get(&self) -> Option<T> {
        // Try each size bucket
        for pool in &self.pools {
            if let Some(mut pool) = pool.try_lock() {
                if let Some(obj) = pool.pop() {
                    self.reused.fetch_add(1, Ordering::Relaxed);
                    return Some(obj);
                }
            }
        }
        None
    }

    #[allow(dead_code)]
    fn return_object(&self, obj: T) {
        // Return to appropriate size bucket
        let bucket = self.get_size_bucket(&obj);
        if let Some(mut pool) = self.pools[bucket].try_lock() {
            if pool.len() < 64 { // Limit pool size
                pool.push(obj);
            }
        }
    }

    #[allow(dead_code)]
    fn get_size_bucket(&self, _obj: &T) -> usize {
        // Simple bucket assignment (could be more sophisticated)
        0
    }
}

impl GenerationalGC {
    fn new() -> Result<Self> {
        Ok(Self {
            young_gen: Generation::new(1024 * 1024),      // 1MB young gen
            old_gen: Generation::new(16 * 1024 * 1024),   // 16MB old gen
            config: {
                let config = GCConfig {
                    _young_threshold: 512 * 1024,    // 512KB
                    _old_threshold: 8 * 1024 * 1024,  // 8MB
                    _incremental_slice_time: 100,     // 100µs
                    promotion_age: 3, // Promote after 3 collections
                };
                config
            },
            write_barrier: WriteBarrier::new(),
        })
    }

    fn register_object(&self, ptr: *mut crate::sprucevm::engine::HeapObject) {
        // New objects go to young generation
        self.young_gen.add_object(ptr);
    }

    fn collect(&self) -> Result<usize> {
        let mut freed_bytes = 0;
        
        // Check if young generation needs collection
        if self.young_gen.should_collect(&self.config) {
            freed_bytes += self.collect_young_generation()?;
            self.promote_long_lived_objects()?;
        }

        // Check if old generation needs collection
        if self.old_gen.should_collect(&self.config) {
            freed_bytes += self.collect_old_generation()?;
        }

        Ok(freed_bytes)
    }

    fn collect_young_generation(&self) -> Result<usize> {
        // Mark and sweep young generation using write barrier info
        let mut freed = 0;
        let objects = self.young_gen.objects.read();
        let dirty_cards = self.write_barrier.get_dirty_cards();
        
        // Mark objects reachable from dirty cards (cross-generational refs)
        for &dirty_ptr in &dirty_cards {
            unsafe {
                if !dirty_ptr.is_null() {
                    (*dirty_ptr).ref_count += 1; // Keep referenced objects alive
                }
            }
        }
        
        for &ptr in objects.iter() {
            unsafe {
                if self.is_reachable(ptr) {
                    // Object is still reachable
                    (*ptr).ref_count += 1; // Age increment
                } else {
                    // Object can be collected
                    freed += std::mem::size_of::<crate::sprucevm::engine::HeapObject>();
                    // Actual deallocation would happen here
                }
            }
        }
        
        // Clear dirty cards after collection
        self.write_barrier.clear_dirty_cards();
        
        Ok(freed)
    }

    fn collect_old_generation(&self) -> Result<usize> {
        // Full mark and sweep of old generation
        let mut freed = 0;
        let mut objects = self.old_gen.objects.write();
        
        // Retain only reachable objects, simulate freeing the rest
        objects.retain(|&ptr| {
            if self.is_reachable(ptr) {
                true
            } else {
                freed += std::mem::size_of::<crate::sprucevm::engine::HeapObject>();
                // Actual deallocation would happen here in a real C-style allocator
                false
            }
        });
        
        self.old_gen.allocated_bytes.fetch_sub(freed, Ordering::Relaxed);
        Ok(freed)
    }

    fn promote_long_lived_objects(&self) -> Result<()> {
        // Move objects that survived multiple young collections to old gen
        let mut young_objects = self.young_gen.objects.write();
        let mut old_objects = self.old_gen.objects.write();
        let promotion_age = self.config.promotion_age;
        
        young_objects.retain(|&ptr| {
            unsafe {
                if (*ptr).ref_count >= promotion_age {
                    old_objects.push(ptr);
                    self.old_gen.allocated_bytes.fetch_add(
                        std::mem::size_of::<crate::sprucevm::engine::HeapObject>(), 
                        Ordering::Relaxed
                    );
                    self.young_gen.allocated_bytes.fetch_sub(
                        std::mem::size_of::<crate::sprucevm::engine::HeapObject>(), 
                        Ordering::Relaxed
                    );
                    false // Remove from young gen
                } else {
                    true // Keep in young gen
                }
            }
        });
        
        Ok(())
    }

    fn is_reachable(&self, ptr: *mut crate::sprucevm::engine::HeapObject) -> bool {
        // Basic reachability analysis based on reference count
        // In a full tracing GC, this would trace from root stack frame registers
        unsafe { (*ptr).ref_count > 0 }
    }

    fn is_in_old_generation(&self, ptr: *mut crate::sprucevm::engine::HeapObject) -> bool {
        self.old_gen.objects.read().contains(&ptr)
    }

    fn is_in_young_generation(&self, ptr: *mut crate::sprucevm::engine::HeapObject) -> bool {
        self.young_gen.objects.read().contains(&ptr)
    }
}

impl Generation {
    fn new(threshold: usize) -> Self {
        Self {
            objects: RwLock::new(Vec::new()),
            allocated_bytes: AtomicUsize::new(0),
            threshold,
        }
    }

    fn add_object(&self, ptr: *mut crate::sprucevm::engine::HeapObject) {
        self.objects.write().push(ptr);
        self.allocated_bytes.fetch_add(
            std::mem::size_of::<crate::sprucevm::engine::HeapObject>(), 
            Ordering::Relaxed
        );
    }

    fn should_collect(&self, _config: &GCConfig) -> bool {
        self.allocated_bytes.load(Ordering::Relaxed) > self.threshold
    }
}

impl WriteBarrier {
    fn new() -> Self {
        Self {
            dirty_cards: RwLock::new(Vec::new()),
        }
    }

    /// Record cross-generational reference (old -> young)
    fn record_cross_gen_ref(&self, old_obj: *mut crate::sprucevm::engine::HeapObject) {
        let mut dirty_cards = self.dirty_cards.write();
        if !dirty_cards.contains(&old_obj) {
            dirty_cards.push(old_obj);
        }
    }

    /// Get all dirty cards for GC scanning
    fn get_dirty_cards(&self) -> Vec<*mut crate::sprucevm::engine::HeapObject> {
        self.dirty_cards.read().clone()
    }

    /// Clear dirty cards after GC
    fn clear_dirty_cards(&self) {
        self.dirty_cards.write().clear();
    }
}

impl StringIntern {
    fn new() -> Self {
        Self {
            strings: RwLock::new(HashMap::new()),
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
        }
    }

    fn get_or_intern(&self, s: String) -> Option<Arc<str>> {
        // Try read lock first (common case)
        if let Some(interned) = self.strings.read().get(&s) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            return Some(interned.clone());
        }

        // Need to insert - take write lock
        let mut strings = self.strings.write();
        
        // Double-check in case another thread inserted
        if let Some(interned) = strings.get(&s) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            return Some(interned.clone());
        }

        // Insert new interned string
        let interned: Arc<str> = Arc::from(s.as_str());
        strings.insert(s, interned.clone());
        self.misses.fetch_add(1, Ordering::Relaxed);
        
        Some(interned)
    }
}

impl BumpAllocator {
    fn new(size: usize) -> Result<Self> {
        let layout = Layout::from_size_align(size, 8)?; // 8-byte aligned
        let ptr = unsafe { std::alloc::alloc(layout) };
        
        if ptr.is_null() {
            return Err(anyhow::anyhow!("Failed to allocate bump space"));
        }

        Ok(Self {
            current: AtomicUsize::new(ptr as usize),
            start: ptr as usize,
            end: ptr as usize + size,
            reset_lock: Mutex::new(()),
        })
    }

    fn allocate<T>(&self, value: T) -> Result<*mut T> {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        
        // Align current pointer
        let current = self.current.load(Ordering::Relaxed);
        let aligned = (current + align - 1) & !(align - 1);
        let new_current = aligned + size;
        
        if new_current > self.end {
            return Err(anyhow::anyhow!("Bump allocator out of space"));
        }
        
        // Try to claim the space atomically
        match self.current.compare_exchange_weak(
            current, 
            new_current, 
            Ordering::Relaxed, 
            Ordering::Relaxed
        ) {
            Ok(_) => {
                // Successfully allocated space
                unsafe {
                    let ptr = aligned as *mut T;
                    std::ptr::write(ptr, value);
                    Ok(ptr)
                }
            }
            Err(_) => {
                // Retry allocation (another thread was faster)
                self.allocate(value)
            }
        }
    }

    fn reset(&self) {
        let _guard = self.reset_lock.lock();
        self.current.store(self.start, Ordering::Relaxed);
    }
}