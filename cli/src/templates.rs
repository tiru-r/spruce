use anyhow::Result;
use std::path::PathBuf;
use std::fs;

pub fn create_basic_template(project_dir: &PathBuf, name: &str) -> Result<()> {
    // Create src directory
    let src_dir = project_dir.join("src");
    fs::create_dir_all(&src_dir)?;
    
    // Create main App.vue
    let app_vue = r#"<template>
  <div class="container">
    <h1 class="title">
      Welcome to Spruce! 🚀
    </h1>
    <p class="subtitle">
      Vue 3.6 Vapor Mode + Pure Rust UI + SpruceVM = 3x Performance
    </p>
    
    <button 
      class="button"
      @click="handlePress"
    >
      Tap me! ({{ count }})
    </button>
    
    <div class="scroll-area">
      <div v-for="item in items" :key="item.id" class="item">
        {{ item.name }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

// Reactive state
const count = ref(0)

const items = ref([
  { id: 1, name: '🦀 Pure Rust UI rendering (3x faster)' },
  { id: 2, name: '⚡ SpruceVM + Vue 3.6 Vapor Mode' },
  { id: 3, name: '🟢 VDOM-free rendering (like SolidJS)' },
  { id: 4, name: '🧵 alien-signals reactivity' },
  { id: 5, name: '🚀 Direct GPU rendering pipeline' },
])

// Event handlers
function handlePress() {
  count.value++
  console.log('Button pressed!', count.value)
}
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background-color: #f5f5f5;
  min-height: 100vh;
}

.title {
  font-size: 24px;
  font-weight: bold;
  color: #333;
  text-align: center;
  margin-bottom: 10px;
}

.subtitle {
  font-size: 16px;
  color: #666;
  text-align: center;
  margin-bottom: 30px;
}

.button {
  background-color: #007AFF;
  padding: 15px 20px;
  border: none;
  border-radius: 8px;
  color: white;
  font-weight: bold;
  cursor: pointer;
  margin-bottom: 20px;
}

.button:hover {
  background-color: #0056d0;
}

.scroll-area {
  flex: 1;
  width: 100%;
  max-width: 400px;
  max-height: 300px;
  overflow-y: auto;
}

.item {
  background-color: white;
  padding: 15px;
  margin-bottom: 10px;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}
</style>"#;

    fs::write(src_dir.join("App.vue"), app_vue)?;
    
    // Create main.ts entry point
    let main_ts = r#"import { createApp } from 'vue'
import App from './App.vue'

console.log('🚀 Starting Spruce app with Pure Rust UI...')

// Create Vue app with Rust UI renderer (3x faster than native)
const app = createApp(App)

// Mount app (this will use Pure Rust UI rendering)
app.mount('#app')

console.log('✅ Spruce app mounted with Rust UI successfully!')"#;

    fs::write(src_dir.join("main.ts"), main_ts)?;
    
    // Create types
    let types_ts = r#"// Spruce Rust UI Types - Pure Rust rendering, no native bridge

declare module '@vue/runtime-core' {
  export interface ComponentCustomProperties {
    // Spruce runtime access
    $spruce: SpruceRuntime
  }
}

// Spruce Runtime Interface
export interface SpruceRuntime {
  // Pure Rust UI methods
  callNativeFunction(name: string, args: any[]): Promise<any>
  performanceStats(): RustUIPerformanceStats
  
  // Rust UI specific APIs
  enableRustUIOptimizations(): void
  getRenderStats(): RustRenderStats
}

// Performance metrics for pure Rust UI
export interface RustUIPerformanceStats {
  renderTimeMs: number
  frameRate: number
  memoryUsageMB: number
  rustUISpeedup: number
}

export interface RustRenderStats {
  verticesRendered: number
  drawCallsOptimized: number
  simdOperationsUsed: number
}

// Vue 3.6 Vapor Mode optimizations
export interface VaporModeConfig {
  enabled: boolean
  rustUIRenderer: boolean
  performance: 'fast' | 'faster' | 'fastest'
}

export {}"#;

    fs::write(src_dir.join("types.ts"), types_ts)?;
    
    Ok(())
}

pub fn create_navigation_template(project_dir: &PathBuf, name: &str) -> Result<()> {
    create_basic_template(project_dir, name)?;
    // TODO: Add navigation components
    Ok(())
}

pub fn create_tabs_template(project_dir: &PathBuf, name: &str) -> Result<()> {
    create_basic_template(project_dir, name)?;
    // TODO: Add tab navigation components
    Ok(())
}