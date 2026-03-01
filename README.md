# Spruce 🌲

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.3--alpha-orange.svg)](#roadmap)
[![Performance](https://img.shields.io/badge/octane-5200+-green.svg)](#performance)

**Spruce** is a performance-first mobile framework designed to unleash the full potential of the Vue ecosystem. Positioned as the **true React Native alternative for Vue 3**, Spruce provides the raw power and developer experience required for everything from standard business apps to high-performance games and physics engines.

[Introduction](#why-spruce) • [Features](#core-features) • [Architecture](#architecture) • [Benchmarks](#performance) • [Quick Start](#quick-start) • [Community](#community--contributing)

---

## Why Spruce?

High-performance applications—from complex data visualizations to mobile games—frequently hit bottlenecks in traditional JavaScript environments. However, even standard applications can suffer from Virtual DOM overhead and main-thread congestion.

Spruce was created to be the **true React Native alternative for Vue 3**. By adopting a **Vapor-first architecture**, Spruce ensures that any application—no matter the scale—benefits from extreme efficiency and absolute responsiveness, bringing the simplicity of Vue to high-performance native development.
- **VDOM-Free**: Complete elimination of Virtual DOM reconciliation.
- **Direct Native Execution**: Compiled Vue blueprints → Optimized bytecode → GPU-accelerated Rust UI.
- **Signal-Powered**: Built on **alien-signals**, the industry's fastest reactive system.

Spruce isn't just a mobile framework; it's a foundation for the next generation of high-impact open-source tools.

## Core Features

- **🦀 Pure Rust UI** - A custom rendering engine that bypasses native bridge bottlenecks for 3x faster UI operations.
- **⚡ SpruceVM** - A register-based, SIMD-accelerated bytecode engine optimized specifically for Vue 3.6 Vapor Mode.
- **🟢 Vapor-First Reactivity** - Native support for signal-based updates, ensuring fine-grained, allocation-free reactivity.
- **📱 Native-Parallel Performance** - Multi-threaded architecture utilizing **Tokio** and **parking_lot** for lock-free concurrency.
- **🛠️ Zero-Abstraction FFI** - High-speed, zero-copy communication between the JavaScript environment and platform-specific backends.

## Architecture

### Ultra-Performance Multi-Threaded Design

Spruce is built on a high-concurrency, tri-threaded model designed to keep the UI buttery smooth even under heavy computation.

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
        │ • 3x Faster than Native UI Bridges                │
        │ • Zero-Copy Shared Memory Buffers                 │
        │ • Direct GPU Vertex Buffer Submission             │
        └──────────────────────────────────────────────────┘
```

## Performance

Spruce is engineered for extreme efficiency. Our benchmarks focus on internal targets that represent real-world "worst-case" scenarios in high-performance mobile UI.

### **Target Metrics (v0.1.3+)**

| Category | Metric | Spruce Performance |
| :--- | :--- | :--- |
| **Startup** | First Frame (Cold) | **~20ms** ⚡ |
| **Reactivity** | Update Latency | **Sub-millisecond** |
| **Rendering** | Draw Call Overhead | **Zero (Direct GPU)** |
| **Memory** | Baseline Heap | **<25MB** |
| **Binary** | Runtime Size | **<5MB** (Compressed) |

### **Execution Score**
- **SpruceVM Octane Target**: 5200+ ⚡

## Quick Start

### Installation
Spruce is currently in early alpha. Developers can explore the framework via the CLI:

```bash
# Install the Spruce CLI Toolchain
cargo install spruce-cli

# Initialize a new project
spruce create my-project --template basic
```

### Example: Vapor DX
```vue
<template>
  <View :style="containerStyle">
    <Text class="hero">Spruce ⚡</Text>
    <Button @press="increment">
      <Text>Score: {{ count }}</Text>
    </Button>
  </View>
</template>

<script setup>
import { ref, computed } from 'vue'

const count = ref(0)
const increment = () => count.value++

const containerStyle = computed(() => ({
  flex: 1,
  backgroundColor: count.value % 2 ? '#1a1a1a' : '#000000',
  justifyContent: 'center'
}))
</script>
```

## Platform Support

Spruce provides professional-grade native shells, ensuring consistency across the entire modern ecosystem.

- ✅ **iOS** (Pure Rust + Metal)
- ✅ **Android** (Pure Rust + Vulkan)
- ✅ **Harmony OS** (Pure Rust + Next-gen support)
- ✅ **macOS / Linux** (Native GTK/AppKit shells)
- 🚧 **Web** (Wasm runtime in research)

## Roadmap

- **✅ v0.1**: Core VM engine + alien-signals integration
- **✅ v0.1.2**: Native SIMD (AVX2/NEON) & x86_64 Assembly optimizations
- **✅ v0.1.3**: Zero-copy bridge & thread-safe shared memory
- **🚧 v0.2**: Evolution of Pure Rust UI Renderer & Platform Shells (In-Progress)
- **📋 v0.3**: Tooling completion: `spruce-dev` with Hot Module Replacement
- **📋 v0.4**: High-Performance Animation & Gesture Engine

## Community & Contributing

Spruce is an open, community-driven framework. We are constantly searching for passionate contributors who are ready to push open-source tools to their absolute limit.

Whether you are an expert in **SIMD/JIT optimization**, a **Rust enthusiast**, or a **documentation specialist**, we welcome your expertise.

- **Join the Core Team**: Help optimize the SpruceVM bytecode engine.
- **Design Systems**: Contribute to the evolution of the Rust UI layout engine.
- **Ecosystem**: Build and maintain community templates and plugins.

## License

Spruce is released under the **MIT License**. Built with ❤️ for the global Vue community.

---

**Spruce** - *Where engineering meets ultimate performance.*