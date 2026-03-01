/// Zero-copy bridge communication between JavaScript and native code
/// 
/// Eliminates JSON serialization overhead by using:
/// - Shared memory buffers
/// - Direct pointer passing
/// - Type-tagged union values
/// - Efficient message passing

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicU32, AtomicUsize, Ordering}};
use parking_lot::{RwLock, Mutex};

/// Bridge value that can be passed without serialization
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BridgeValue {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// 32-bit integer
    Int32(i32),
    /// 64-bit float
    Float64(f64),
    /// String reference (points to shared buffer)
    StringRef { offset: u32, length: u32 },
    /// Array reference (points to shared buffer)
    ArrayRef { offset: u32, length: u32 },
    /// Object reference (points to shared buffer)  
    ObjectRef { offset: u32 },
    /// Function reference (direct pointer)
    FunctionRef { function_id: u32 },
    /// Native object handle
    NativeRef { handle: u64 },
}

/// Shared memory buffer for zero-copy communication
#[derive(Debug)]
pub struct SharedBuffer {
    /// Buffer data
    data: Vec<u8>,
    /// Current write position
    write_pos: AtomicUsize,
    /// Buffer capacity
    capacity: usize,
    /// Generation counter to detect buffer reuse
    generation: AtomicU32,
}

impl SharedBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0; capacity],
            write_pos: AtomicUsize::new(0),
            capacity,
            generation: AtomicU32::new(0),
        }
    }

    /// Allocate space in buffer and return offset
    pub fn allocate(&self, size: usize) -> Option<u32> {
        let old_pos = self.write_pos.fetch_add(size, Ordering::Relaxed);
        if old_pos + size <= self.capacity {
            Some(old_pos as u32)
        } else {
            None
        }
    }

    /// Write data to buffer at offset
    pub unsafe fn write_at(&mut self, offset: u32, data: &[u8]) {
        let dst = self.data.as_mut_ptr().add(offset as usize);
        std::ptr::copy_nonoverlapping(data.as_ptr(), dst, data.len());
    }

    /// Read data from buffer at offset
    pub unsafe fn read_at(&self, offset: u32, length: u32) -> &[u8] {
        let src = self.data.as_ptr().add(offset as usize);
        std::slice::from_raw_parts(src, length as usize)
    }

    /// Reset buffer for reuse
    pub fn reset(&self) {
        self.write_pos.store(0, Ordering::Relaxed);
        self.generation.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current generation
    pub fn generation(&self) -> u32 {
        self.generation.load(Ordering::Relaxed)
    }
}

/// High-performance bridge message
#[repr(C)]
#[derive(Debug)]
pub struct BridgeMessage {
    /// Message type
    pub message_type: MessageType,
    /// Function ID to call
    pub function_id: u32,
    /// Number of arguments
    pub arg_count: u32,
    /// Arguments array
    pub args: [BridgeValue; 8], // Fixed size for performance
    /// Return value
    pub return_value: BridgeValue,
    /// Message ID for correlation
    pub message_id: u32,
    /// Buffer generation when message was created
    pub buffer_generation: u32,
}

/// Message types for bridge communication
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    /// Call native function from JS
    NativeCall = 0,
    /// Call JS function from native
    JSCall = 1,
    /// Return value from function call
    Return = 2,
    /// Error occurred during execution
    Error = 3,
    /// Request garbage collection
    GCRequest = 4,
}

/// Fast serializer for bridge values
#[derive(Debug)]
pub struct BridgeSerializer {
    /// Shared buffer for data
    buffer: Arc<Mutex<SharedBuffer>>,
    /// String intern table for deduplication
    string_table: RwLock<HashMap<String, u32>>,
    /// Next message ID
    next_message_id: AtomicU32,
}

impl BridgeSerializer {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(SharedBuffer::new(buffer_size))),
            string_table: RwLock::new(HashMap::new()),
            next_message_id: AtomicU32::new(1),
        }
    }

    /// Serialize JavaScript value to bridge value
    pub fn serialize_js_value(&self, value: &crate::sprucevm::engine::Value) -> Result<BridgeValue> {
        match value {
            crate::sprucevm::engine::Value::Null => Ok(BridgeValue::Null),
            crate::sprucevm::engine::Value::Bool(b) => Ok(BridgeValue::Bool(*b)),
            crate::sprucevm::engine::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                        return Ok(BridgeValue::Int32(i as i32));
                    }
                }
                if let Some(f) = n.as_f64() {
                    Ok(BridgeValue::Float64(f))
                } else {
                    Ok(BridgeValue::Null)
                }
            }
            crate::sprucevm::engine::Value::String(s) => {
                self.serialize_string(s)
            }
            crate::sprucevm::engine::Value::Array(arr) => {
                self.serialize_array(arr)
            }
            crate::sprucevm::engine::Value::Object(obj) => {
                self.serialize_object(obj)
            }
        }
    }

    /// Serialize string to shared buffer
    fn serialize_string(&self, s: &str) -> Result<BridgeValue> {
        // Check string intern table first
        {
            let table = self.string_table.read();
            if let Some(&offset) = table.get(s) {
                return Ok(BridgeValue::StringRef {
                    offset,
                    length: s.len() as u32,
                });
            }
        }

        // Allocate in shared buffer
        let mut buffer = self.buffer.lock();
        let string_bytes = s.as_bytes();
        let offset = buffer.allocate(string_bytes.len())
            .ok_or_else(|| anyhow::anyhow!("Buffer overflow"))?;

        unsafe {
            buffer.write_at(offset, string_bytes);
        }

        // Add to intern table
        {
            let mut table = self.string_table.write();
            table.insert(s.to_string(), offset);
        }

        Ok(BridgeValue::StringRef {
            offset,
            length: s.len() as u32,
        })
    }

    /// Serialize array to shared buffer
    fn serialize_array(&self, arr: &[crate::sprucevm::engine::Value]) -> Result<BridgeValue> {
        let mut buffer = self.buffer.lock();
        
        // Calculate required space
        let header_size = std::mem::size_of::<ArrayHeader>();
        let elements_size = arr.len() * std::mem::size_of::<BridgeValue>();
        let total_size = header_size + elements_size;

        let offset = buffer.allocate(total_size)
            .ok_or_else(|| anyhow::anyhow!("Buffer overflow"))?;

        // Write array header
        let header = ArrayHeader {
            length: arr.len() as u32,
            element_type: ElementType::Mixed,
        };

        unsafe {
            buffer.write_at(offset, header.as_bytes());
        }

        // Serialize elements
        let mut element_offset = offset + header_size as u32;
        for element in arr.iter() {
            // Drop the lock temporarily for recursive serialization
            drop(buffer);
            let bridge_value = self.serialize_js_value(element)?;
            buffer = self.buffer.lock();
            
            unsafe {
                let value_bytes = bridge_value.as_bytes();
                buffer.write_at(element_offset, value_bytes);
                element_offset += std::mem::size_of::<BridgeValue>() as u32;
            }
        }

        Ok(BridgeValue::ArrayRef {
            offset,
            length: arr.len() as u32,
        })
    }

    /// Serialize object to shared buffer
    fn serialize_object(&self, obj: &HashMap<String, crate::sprucevm::engine::Value>) -> Result<BridgeValue> {
        let mut buffer = self.buffer.lock();
        
        // Calculate required space
        let header_size = std::mem::size_of::<ObjectHeader>();
        let entries_size = obj.len() * std::mem::size_of::<ObjectEntry>();
        let total_size = header_size + entries_size;

        let offset = buffer.allocate(total_size)
            .ok_or_else(|| anyhow::anyhow!("Buffer overflow"))?;

        // Write object header
        let header = ObjectHeader {
            property_count: obj.len() as u32,
        };

        unsafe {
            buffer.write_at(offset, header.as_bytes());
        }

        // Serialize properties
        let mut entry_offset = offset + header_size as u32;
        for (key, value) in obj {
            // Drop lock for recursive serialization
            drop(buffer);
            let key_ref = self.serialize_string(key)?;
            let value_ref = self.serialize_js_value(value)?;
            buffer = self.buffer.lock();

            let entry = ObjectEntry {
                key: key_ref,
                value: value_ref,
            };

            unsafe {
                buffer.write_at(entry_offset, entry.as_bytes());
                entry_offset += std::mem::size_of::<ObjectEntry>() as u32;
            }
        }

        Ok(BridgeValue::ObjectRef { offset })
    }

    /// Deserialize bridge value back to JavaScript value
    pub fn deserialize_to_js(&self, bridge_value: &BridgeValue) -> Result<crate::sprucevm::engine::Value> {
        match bridge_value {
            BridgeValue::Null => Ok(crate::sprucevm::engine::Value::Null),
            BridgeValue::Bool(b) => Ok(crate::sprucevm::engine::Value::Bool(*b)),
            BridgeValue::Int32(i) => Ok(crate::sprucevm::engine::Value::Number(
                serde_json::Number::from(*i)
            )),
            BridgeValue::Float64(f) => Ok(crate::sprucevm::engine::Value::Number(
                serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0))
            )),
            BridgeValue::StringRef { offset, length } => {
                self.deserialize_string(*offset, *length)
            }
            BridgeValue::ArrayRef { offset, length } => {
                self.deserialize_array(*offset, *length)
            }
            BridgeValue::ObjectRef { offset } => {
                self.deserialize_object(*offset)
            }
            _ => Ok(crate::sprucevm::engine::Value::Null), // Simplified
        }
    }

    /// Deserialize string from shared buffer
    fn deserialize_string(&self, offset: u32, length: u32) -> Result<crate::sprucevm::engine::Value> {
        let buffer = self.buffer.lock();
        unsafe {
            let bytes = buffer.read_at(offset, length);
            let string = String::from_utf8(bytes.to_vec())?;
            Ok(crate::sprucevm::engine::Value::String(string))
        }
    }

    /// Deserialize array from shared buffer
    fn deserialize_array(&self, offset: u32, _length: u32) -> Result<crate::sprucevm::engine::Value> {
        let (_header, bridge_values) = {
            let buffer = self.buffer.lock();
            unsafe {
                // Read array header
                let header_bytes = buffer.read_at(offset, std::mem::size_of::<ArrayHeader>() as u32);
                let header = ArrayHeader::from_bytes(header_bytes);

                let mut bridge_values = Vec::new();
                let mut element_offset = offset + std::mem::size_of::<ArrayHeader>() as u32;

                for _ in 0..header.length {
                    let value_bytes = buffer.read_at(element_offset, std::mem::size_of::<BridgeValue>() as u32);
                    let bridge_value = BridgeValue::from_bytes(value_bytes);
                    bridge_values.push(bridge_value);
                    element_offset += std::mem::size_of::<BridgeValue>() as u32;
                }

                (header, bridge_values)
            }
        };

        let mut elements = Vec::new();
        for bridge_value in bridge_values {
            let js_value = self.deserialize_to_js(&bridge_value)?;
            elements.push(js_value);
        }

        Ok(crate::sprucevm::engine::Value::Array(elements))
    }

    /// Deserialize object from shared buffer
    fn deserialize_object(&self, offset: u32) -> Result<crate::sprucevm::engine::Value> {
        let entries = {
            let buffer = self.buffer.lock();
            unsafe {
                // Read object header
                let header_bytes = buffer.read_at(offset, std::mem::size_of::<ObjectHeader>() as u32);
                let header = ObjectHeader::from_bytes(header_bytes);

                let mut entries = Vec::new();
                let mut entry_offset = offset + std::mem::size_of::<ObjectHeader>() as u32;

                for _ in 0..header.property_count {
                    let entry_bytes = buffer.read_at(entry_offset, std::mem::size_of::<ObjectEntry>() as u32);
                    let entry = ObjectEntry::from_bytes(entry_bytes);
                    entries.push(entry);
                    entry_offset += std::mem::size_of::<ObjectEntry>() as u32;
                }

                entries
            }
        };

        let mut properties = HashMap::new();
        for entry in entries {
            let key_value = self.deserialize_to_js(&entry.key)?;
            let prop_value = self.deserialize_to_js(&entry.value)?;

            if let crate::sprucevm::engine::Value::String(key_str) = key_value {
                properties.insert(key_str, prop_value);
            }
        }

        Ok(crate::sprucevm::engine::Value::Object(properties))
    }

    /// Create new bridge message
    pub fn create_message(&self, message_type: MessageType, function_id: u32) -> BridgeMessage {
        BridgeMessage {
            message_type,
            function_id,
            arg_count: 0,
            args: [BridgeValue::Null; 8],
            return_value: BridgeValue::Null,
            message_id: self.next_message_id.fetch_add(1, Ordering::Relaxed),
            buffer_generation: self.buffer.lock().generation(),
        }
    }
}

/// Array header in shared buffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ArrayHeader {
    length: u32,
    element_type: ElementType,
}

/// Element type hint for optimization
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum ElementType {
    Mixed = 0,
    AllIntegers = 1,
    AllFloats = 2,
    AllStrings = 3,
    AllObjects = 4,
}

/// Object header in shared buffer
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ObjectHeader {
    property_count: u32,
}

/// Object property entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ObjectEntry {
    key: BridgeValue,
    value: BridgeValue,
}

// Helper traits for serialization
trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl AsBytes for ArrayHeader {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl FromBytes for ArrayHeader {
    fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { *(bytes.as_ptr() as *const Self) }
    }
}

impl AsBytes for ObjectHeader {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl FromBytes for ObjectHeader {
    fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { *(bytes.as_ptr() as *const Self) }
    }
}

impl AsBytes for ObjectEntry {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl FromBytes for ObjectEntry {
    fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { *(bytes.as_ptr() as *const Self) }
    }
}

impl AsBytes for BridgeValue {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                std::mem::size_of::<Self>(),
            )
        }
    }
}

impl FromBytes for BridgeValue {
    fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { *(bytes.as_ptr() as *const Self) }
    }
}

/// High-performance bridge for zero-copy communication
#[derive(Debug)]
pub struct ZeroCopyBridge {
    /// Serializer for message conversion
    serializer: Arc<BridgeSerializer>,
    /// Message queue for async communication
    message_queue: Arc<Mutex<Vec<BridgeMessage>>>,
    /// Performance statistics
    stats: BridgeStats,
}

#[derive(Debug, Default)]
pub struct BridgeStats {
    pub messages_sent: AtomicUsize,
    pub messages_received: AtomicUsize,
    pub bytes_transferred: AtomicUsize,
    pub serialization_time: AtomicUsize, // microseconds
    pub zero_copy_operations: AtomicUsize,
}

impl ZeroCopyBridge {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            serializer: Arc::new(BridgeSerializer::new(buffer_size)),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            stats: BridgeStats::default(),
        }
    }

    /// Send message without JSON serialization
    pub fn send_fast(&self, message: BridgeMessage) -> Result<()> {
        self.message_queue.lock().push(message);
        self.stats.messages_sent.fetch_add(1, Ordering::Relaxed);
        self.stats.zero_copy_operations.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Call native function with zero-copy arguments
    pub fn call_native_fast(
        &self, 
        function_id: u32, 
        args: &[crate::sprucevm::engine::Value]
    ) -> Result<crate::sprucevm::engine::Value> {
        let start_time = std::time::Instant::now();
        
        let mut message = self.serializer.create_message(MessageType::NativeCall, function_id);
        message.arg_count = args.len().min(8) as u32;

        // Serialize arguments without JSON
        for (i, arg) in args.iter().enumerate().take(8) {
            message.args[i] = self.serializer.serialize_js_value(arg)?;
        }

        self.send_fast(message)?;

        let serialization_time = start_time.elapsed().as_micros() as usize;
        self.stats.serialization_time.fetch_add(serialization_time, Ordering::Relaxed);

        // In a real implementation, would wait for response or return async future
        Ok(crate::sprucevm::engine::Value::Null)
    }

    /// Get bridge performance statistics
    pub fn get_stats(&self) -> &BridgeStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_buffer() {
        let buffer = SharedBuffer::new(1024);
        let offset1 = buffer.allocate(100).unwrap();
        let offset2 = buffer.allocate(200).unwrap();
        
        assert_eq!(offset1, 0);
        assert_eq!(offset2, 100);
    }

    #[test]
    fn test_bridge_serialization() {
        let serializer = BridgeSerializer::new(4096);
        let value = crate::sprucevm::engine::Value::String("test".to_string());
        
        let bridge_value = serializer.serialize_js_value(&value).unwrap();
        let deserialized = serializer.deserialize_to_js(&bridge_value).unwrap();
        
        match deserialized {
            crate::sprucevm::engine::Value::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_zero_copy_bridge() {
        let bridge = ZeroCopyBridge::new(4096);
        let args = vec![
            crate::sprucevm::engine::Value::String("hello".to_string()),
            crate::sprucevm::engine::Value::Number(serde_json::Number::from(42)),
        ];
        
        let result = bridge.call_native_fast(123, &args);
        assert!(result.is_ok());
    }
}