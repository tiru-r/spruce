use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex as AsyncMutex};
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::threading::{InputEvent};

/// Zero-copy bridge between threads inspired by LynxJS architecture
/// Enables synchronous communication like React Native's new JSI
pub struct Bridge {
    // UI Thread -> JS Thread
    ui_to_js_tx: mpsc::UnboundedSender<BridgeMessage>,
    ui_to_js_rx: Arc<AsyncMutex<mpsc::UnboundedReceiver<BridgeMessage>>>,
    
    // JS Thread -> UI Thread  
    js_to_ui_tx: mpsc::UnboundedSender<BridgeMessage>,
    js_to_ui_rx: Arc<AsyncMutex<mpsc::UnboundedReceiver<BridgeMessage>>>,
    
    // Shared memory regions for zero-copy data transfer
    shared_buffers: Arc<RwLock<HashMap<String, Arc<Vec<u8>>>>>,
    
    // Direct function calls registry (like JSI)
    native_functions: Arc<RwLock<HashMap<String, NativeFunction>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeMessage {
    // UI -> JS (Rust UI events)
    InputEvent(InputEvent),
    RustComponentMounted { component_id: u32 },
    RustComponentUnmounted { component_id: u32 },
    
    // JS -> UI (Rust UI updates)
    UpdateRustComponent { component_id: u32, vertex_data: Vec<u8> },
    CreateRustComponent { component_data: Vec<u8> },
    RemoveRustComponent { component_id: u32 },
    
    // Direct function call (no UI bridge overhead)
    CallNativeFunction { 
        name: String, 
        args: Vec<BridgeValue>,
        callback_id: Option<String>,
    },
    
    // Function call result
    NativeFunctionResult {
        callback_id: String,
        result: BridgeValue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<BridgeValue>),
    Object(HashMap<String, BridgeValue>),
    
    // Special types for zero-copy
    SharedBuffer(String), // Reference to shared buffer by key
    RustUIVertexData(Vec<u8>), // Direct GPU vertex buffer data
}

/// Native function that can be called from JS thread
pub type NativeFunction = Arc<dyn Fn(&[BridgeValue]) -> Result<BridgeValue> + Send + Sync>;

impl Bridge {
    pub fn new() -> Result<Self> {
        let (ui_to_js_tx, ui_to_js_rx) = mpsc::unbounded_channel();
        let (js_to_ui_tx, js_to_ui_rx) = mpsc::unbounded_channel();
        
        let bridge = Self {
            ui_to_js_tx,
            ui_to_js_rx: Arc::new(AsyncMutex::new(ui_to_js_rx)),
            js_to_ui_tx,
            js_to_ui_rx: Arc::new(AsyncMutex::new(js_to_ui_rx)),
            shared_buffers: Arc::new(RwLock::new(HashMap::new())),
            native_functions: Arc::new(RwLock::new(HashMap::new())),
        };
        
        bridge.register_default_functions()?;
        Ok(bridge)
    }

    /// Register default native functions
    fn register_default_functions(&self) -> Result<()> {
        // File system operations
        self.register_native_function("fs.readFile", Arc::new(|args| {
            if let Some(BridgeValue::String(path)) = args.get(0) {
                // TODO: Implement file reading
                Ok(BridgeValue::String(format!("File content from {}", path)))
            } else {
                Ok(BridgeValue::Null)
            }
        }))?;

        // Network operations  
        self.register_native_function("fetch", Arc::new(|args| {
            if let Some(BridgeValue::String(_url)) = args.get(0) {
                // TODO: Implement HTTP request
                Ok(BridgeValue::Object([
                    ("status".to_string(), BridgeValue::Number(200.0)),
                    ("data".to_string(), BridgeValue::String("Response data".to_string())),
                ].iter().cloned().collect()))
            } else {
                Ok(BridgeValue::Null)
            }
        }))?;

        // Device APIs
        self.register_native_function("device.vibrate", Arc::new(|args| {
            if let Some(BridgeValue::Number(duration)) = args.get(0) {
                tracing::info!("Vibrating device for {}ms", duration);
                // TODO: Implement device vibration
                Ok(BridgeValue::Bool(true))
            } else {
                Ok(BridgeValue::Bool(false))
            }
        }))?;

        Ok(())
    }

    /// Register a native function that can be called from JS
    pub fn register_native_function(&self, name: &str, func: NativeFunction) -> Result<()> {
        self.native_functions.write().insert(name.to_string(), func);
        tracing::debug!("Registered native function: {}", name);
        Ok(())
    }

    /// Send message from UI thread to JS thread
    pub async fn send_to_js_thread(&self, message: BridgeMessage) {
        if let Err(e) = self.ui_to_js_tx.send(message) {
            tracing::error!("Failed to send message to JS thread: {}", e);
        }
    }

    /// Send message from JS thread to UI thread
    pub async fn send_to_ui_thread(&self, message: BridgeMessage) {
        if let Err(e) = self.js_to_ui_tx.send(message) {
            tracing::error!("Failed to send message to UI thread: {}", e);
        }
    }

    /// Create shared buffer for zero-copy data transfer
    pub fn create_shared_buffer(&self, key: String, data: Vec<u8>) -> String {
        let buffer = Arc::new(data);
        self.shared_buffers.write().insert(key.clone(), buffer);
        key
    }

    /// Get shared buffer by key
    pub fn get_shared_buffer(&self, key: &str) -> Option<Arc<Vec<u8>>> {
        self.shared_buffers.read().get(key).cloned()
    }

    /// Call native function directly (synchronous like JSI)
    pub fn call_native_function(&self, name: &str, args: &[BridgeValue]) -> Result<BridgeValue> {
        let functions = self.native_functions.read();
        if let Some(func) = functions.get(name) {
            func(args)
        } else {
            tracing::warn!("Native function not found: {}", name);
            Ok(BridgeValue::Null)
        }
    }

    /// Start processing bridge messages
    pub async fn start_message_processing(&self) -> Result<()> {
        let ui_to_js_rx = self.ui_to_js_rx.clone();
        let js_to_ui_rx = self.js_to_ui_rx.clone();
        
        // Process UI -> JS messages
        tokio::spawn(async move {
            let mut rx = ui_to_js_rx.lock().await;
            while let Some(message) = rx.recv().await {
                tracing::debug!("Processing UI->JS message: {:?}", message);
                // TODO: Forward to SpruceVM runtime
            }
        });

        // Process JS -> UI messages  
        tokio::spawn(async move {
            let mut rx = js_to_ui_rx.lock().await;
            while let Some(message) = rx.recv().await {
                tracing::debug!("Processing JS->UI message: {:?}", message);
                // TODO: Forward to UI thread
            }
        });

        Ok(())
    }
}

/// Helper trait for converting Rust types to BridgeValues
pub trait ToBridgeValue {
    fn to_bridge_value(self) -> BridgeValue;
}

impl ToBridgeValue for String {
    fn to_bridge_value(self) -> BridgeValue {
        BridgeValue::String(self)
    }
}

impl ToBridgeValue for f64 {
    fn to_bridge_value(self) -> BridgeValue {
        BridgeValue::Number(self)
    }
}

impl ToBridgeValue for bool {
    fn to_bridge_value(self) -> BridgeValue {
        BridgeValue::Bool(self)
    }
}

impl<T: ToBridgeValue> ToBridgeValue for Vec<T> {
    fn to_bridge_value(self) -> BridgeValue {
        BridgeValue::Array(
            self.into_iter().map(|item| item.to_bridge_value()).collect()
        )
    }
}

/// Macro for easy native function registration
#[macro_export]
macro_rules! register_native_fn {
    ($bridge:expr, $name:expr, |$args:ident| $body:expr) => {
        $bridge.register_native_function($name, Arc::new(|$args| {
            Ok($body.to_bridge_value())
        }))?;
    };
}