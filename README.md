# Spruce 🌲

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0--preview-orange.svg)](#current-status)
[![Build](https://img.shields.io/badge/build-compiles-green.svg)](#architecture)

**Spruce** is an experimental mobile development platform that aims to combine Vue 3.6 Vapor Mode with pure Rust UI rendering. This project represents innovative architecture research for high-performance mobile development using Vue.js.

[Introduction](#why-spruce) • [Architecture](#architecture) • [Current Status](#current-status) • [CLI Framework](#spruce-cli) • [Quick Start](#quick-start) • [Community](#community--contributing)

---

## Why Spruce?

Modern mobile development faces performance bottlenecks from JavaScript bridge overhead, Virtual DOM reconciliation, and native UI limitations. Even standard business applications can suffer from frame drops and sluggish interactions.

Spruce explores a revolutionary approach by designing an architecture that combines **Vue 3.6 Vapor Mode** with **pure Rust UI rendering**. Our research focuses on:

- **Zero Virtual DOM**: Vue 3.6 Vapor Mode compiles templates directly to optimized bytecode
- **Pure Rust UI**: Custom rendering engine designed to bypass native bridge bottlenecks  
- **Alien Signals**: Advanced reactivity system with zero-allocation signal updates
- **60+ FPS Target**: Architecture designed for consistent performance under any load

Spruce isn't just another mobile framework—it's architectural research into the future of mobile development platforms.

## Architecture

### Multi-Threaded Design Concept

Spruce is designed around a high-concurrency, tri-threaded model to keep UI performance optimal:

```rust
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   UI Thread     │◄──►│ SpruceVM Thread  │◄──►│ Background Pool │
│   (Rust UI)     │    │ (Bytecode Engine)│    │   (Async I/O)   │
│                 │    │                  │    │                 │
│ • GPU Vertices  │    │ • Vapor Bytecode │    │ • Network I/O   │
│ • Flex Layout   │    │ • alien-signals  │    │ • File System   │
│ • SIMD Colors   │    │ • Register VM    │    │ • Image Decode  │
│ • Zero Bridge   │    │ • JIT Ready      │    │ • Background Ops│
└─────────────────┘    └──────────────────┘    └─────────────────┘
        │                        │                        │
        └────────────────────────┼────────────────────────┘
                                 │
        ┌─────────────────────────▼─────────────────────────┐
        │        🚀 PURE RUST UI RENDERER 🚀                │
        │   Vue Template → Optimized Bytecode → GPU         │
        │ • Designed for 3x Faster Performance              │
        │ • Zero-Copy Shared Memory Architecture            │
        │ • Direct GPU Vertex Buffer Design                 │
        └──────────────────────────────────────────────────┘
```

## Current Status (v0.1.0-preview)

### ✅ **What's Implemented**

#### **Architecture & Type Systems**
- Complete Rust workspace with professional build configuration
- Comprehensive type system for Vue 3.6 Vapor Mode integration
- Detailed interfaces for Pure Rust UI rendering
- Multi-threaded architecture design with proper concurrency patterns

#### **CLI Framework**
- Working command-line interface with project scaffolding
- Vue project templates with proper Vue 3.6.0-beta.7 dependencies
- Build and deployment command structure
- Professional CLI user experience

#### **Build System**
- Clean compilation with zero errors
- Multiple optimization profiles for different targets
- Workspace-shared dependencies and configurations
- Platform-specific build targeting

#### **Documentation**
- Comprehensive architecture documentation
- Implementation guides and examples
- Professional project structure

### 🚧 **In Active Development**

#### **Core Engine Implementation**
- **SpruceVM JavaScript Engine**: Type definitions complete, execution engine in progress
- **Vue 3.6 Compilation Pipeline**: Interface designed, implementation started
- **Pure Rust UI Rendering**: Architecture complete, GPU integration in progress
- **Android Platform**: JNI bridge structured, ANativeWindow integration in progress

#### **Development Tools**
- **Hot Reload System**: Command structure ready, file watching in progress
- **Performance Profiling**: Benchmarking framework designed
- **Visual Development Tools**: Architecture planned

### 📋 **Planned Features**
- **iOS Platform**: Complete iOS implementation with Metal rendering
- **Desktop Platforms**: Native Windows, macOS, and Linux support
- **Web Platform**: WebAssembly compilation target
- **Visual Development Environment**: Spruce Studio IDE
- **Plugin Ecosystem**: Third-party plugin architecture

## Spruce CLI

The CLI provides a foundation for the complete development experience:

### **Project Management**
```bash
# Create a new Spruce project (templates in development)
spruce create MyApp --template vue-mobile

# Development server framework (implementation in progress)
spruce dev --platform android
```

### **Build System**  
```bash
# Build system architecture (backend implementation in progress)
spruce build --release --platform android

# Deployment framework (implementation planned)
spruce deploy --target play-store
```

### **AI-Powered Development** (Planned)
```bash
# AI assistance framework (implementation planned)
spruce ai generate --feature "user authentication"
spruce ai optimize --component UserList
```

## Quick Start

### Current Capabilities
```bash
# Clone and build the project
git clone https://github.com/sprucedev/spruce
cd spruce

# Build the CLI (compiles successfully)
cargo build --release

# Explore the architecture
cargo doc --open
```

### Vue 3.6 Template Example
```vue
<template>
  <div class="app">
    <header class="app-header">
      <h1 class="app-title">{{ title }}</h1>
      <p class="counter-display">{{ count }} {{ counterLabel }}</p>
    </header>
    <main class="app-main">
      <button @click="increment" class="counter-btn">+</button>
      <button @click="decrement" class="counter-btn">-</button>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

// Vue 3.6 Vapor Mode ready
const title = ref('Welcome to Spruce!')
const count = ref(0)
const counterLabel = computed(() => count.value === 1 ? 'tap' : 'taps')

const increment = () => count.value++
const decrement = () => count.value > 0 && count.value--
</script>
```

## Platform Support

### **Current Architecture Status**

- 🚧 **Android** (Architecture designed, core implementation in progress)
- 📋 **iOS** (Architecture planned, implementation pending)
- 📋 **Desktop** (Design phase)
- 📋 **Web** (WebAssembly target planned)

### **Android Development Progress**
- 🚧 Pure Rust UI rendering architecture with ANativeWindow integration
- 🚧 Touch and gesture recognition system design
- 🚧 Android lifecycle management framework
- 🚧 JNI bridges for Java API access
- 🚧 GPU-accelerated surface rendering design
- 🚧 Multi-threaded architecture implementation

## Performance Goals

Spruce is being designed with ambitious performance targets:

### **Target Metrics (When Implemented)**

| Category | Metric | Target Performance |
| :--- | :--- | :--- |
| **Startup** | First Frame (Cold) | **~20ms** ⚡ |
| **Reactivity** | Update Latency | **Sub-millisecond** |
| **Rendering** | Draw Call Overhead | **Zero (Direct GPU)** |
| **Memory** | Baseline Heap | **<25MB** |
| **Binary** | Runtime Size | **<5MB** (Compressed) |

*Performance claims will be substantiated with benchmarks once core implementation is complete.*

## Technical Innovation

### **Research Areas**
- **Zero-Bridge Architecture**: Eliminating JavaScript-to-native communication overhead
- **Vapor Mode Integration**: Direct compilation of Vue templates to optimized bytecode  
- **SIMD Optimization**: Leveraging modern CPU instructions for UI calculations
- **Lock-Free Concurrency**: Multi-threaded rendering without synchronization overhead

### **Key Technical Challenges Being Solved**
1. **Vue 3.6 to Rust Compilation**: Seamlessly translating Vue components to native UI
2. **Cross-Platform GPU Rendering**: Unified rendering API across mobile platforms
3. **Hot Reload for Hybrid Stack**: Live updates for Vue + Rust development
4. **Zero-Copy Communication**: Efficient data sharing between threads and runtimes

## Community & Contributing

Spruce is an open-source research project exploring the future of mobile development. We welcome contributors interested in:

### **Research Areas**
- **JavaScript Engine Development**: SpruceVM bytecode optimization
- **Graphics Programming**: Direct GPU rendering implementation
- **Vue.js Integration**: Vapor Mode compilation pipeline
- **Mobile Platform Development**: Android/iOS native integration
- **Developer Tools**: CLI and development server implementation

### **How to Contribute**
1. **Explore the Architecture**: Read through the comprehensive type systems and interfaces
2. **Implement Core Features**: Help bring the designed architecture to life
3. **Performance Research**: Contribute to benchmarking and optimization
4. **Documentation**: Improve guides and examples

### **Getting Started**
1. **Explore the Codebase**: Check out the well-structured Rust modules
2. **Join Discussions**: [Discord community](https://discord.gg/T2rDj6rW)
3. **Implementation Work**: Help implement the designed architecture
4. **Research Contributions**: Contribute to performance and architecture research

## Project Vision

Spruce represents a bold vision for the future of mobile development:
- **Developer Experience**: Vue developers should write only Vue code
- **Performance**: Native-level performance without platform complexity
- **Innovation**: Push the boundaries of what's possible with modern tooling

The project combines cutting-edge research in JavaScript engines, graphics programming, and mobile platform development to create something genuinely new in the mobile development space.

## License

Spruce is released under the **MIT License**. Built with ❤️ for research into the future of mobile development.

---

**Spruce** - *Exploring the future of high-performance mobile development.*