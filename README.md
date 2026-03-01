# Spruce 🌲

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0--experimental-orange.svg)](#current-status)
[![Build](https://img.shields.io/badge/build-compiles-green.svg)](#architecture)

**Spruce** is an experimental research project exploring next-generation mobile development through the combination of Vue 3.6 Vapor Mode with Pure Rust UI rendering. While the full vision is still under development, Spruce currently provides excellent Vue 3.6 development tooling and project scaffolding.

[Quick Start](#quick-start) • [What Works Today](#what-works-today) • [Architecture Vision](#architecture-vision) • [Development Environment](#development-environment) • [Research Goals](#research-goals) • [Community](#community)

---

## What Works Today ✅

### **Vue 3.6 Development Tooling**
Spruce provides a professional CLI and development environment for Vue 3.6 applications:

- **🎯 Vue 3.6.0-beta.7 Project Scaffolding** - Generate modern Vue apps with latest beta features
- **⚡ Professional Templates** - Mobile-optimized Vue applications with TypeScript
- **🛠️ Complete Dev Environment** - Vite + Vue + TypeScript + Hot Reload
- **📱 Mobile-First Design** - Responsive templates ready for mobile deployment
- **🏗️ Clean Architecture** - Well-structured project templates following Vue best practices

### **Working CLI Commands**
```bash
# ✅ These commands work today:
spruce create MyApp           # Creates Vue 3.6 project with TypeScript
spruce create --template shopping-app
spruce create --template vue-mobile

# 🚧 These exist but are placeholder implementations:
spruce dev --platform android
spruce build --release
spruce deploy
```

### **Functional Demo Application**
- **Live Vue 3.6 demo** at `/examples/demo-app/` 
- **Working development server** with `npm run dev`
- **Full TypeScript support** with proper Vue SFC types
- **Modern Vue patterns** - Composition API, reactive refs, computed properties

## Quick Start

### **Creating a Vue 3.6 Project**
```bash
# Clone and build Spruce CLI
git clone https://github.com/spruce-platform/spruce
cd spruce
cargo build --release

# Create a new Vue 3.6 project
./target/release/spruce create MyApp --template vue-mobile
cd MyApp

# Install dependencies and start development
npm install --force  # --force needed for Vue 3.6 beta
npm run dev          # Opens Vue app at http://localhost:3000
```

### **Exploring the Demo**
```bash
# Run the functional Vue 3.6 demo
cd examples/demo-app
npm install --force
npm run dev          # Opens demo at http://localhost:3000
```

## Architecture Vision 🏗️

Spruce is researching a revolutionary approach to mobile development by combining:

### **Core Innovation Concepts**
- **Vue 3.6 Vapor Mode**: Compile-time optimizations eliminating Virtual DOM overhead
- **Pure Rust UI Rendering**: Direct GPU rendering bypassing JavaScript-to-native bridges
- **SpruceVM Engine**: Custom JavaScript runtime optimized for mobile performance
- **Zero-Bridge Architecture**: Eliminate performance bottlenecks from platform communication

### **Target Performance Goals** (Research Targets)
```
Category           Current (Web)    Research Target
─────────────────  ─────────────    ───────────────
Startup Time       ~1000ms         <20ms (Cold)
UI Responsiveness   16.67ms/frame   <10ms latency
Memory Usage        ~50MB baseline   <25MB baseline
Bundle Size         ~2MB           <5MB compressed
Touch-to-Paint      ~100ms         <16ms (60fps)
```

*Note: These are research targets, not current measurements*

### **Designed Architecture** (Not Yet Implemented)

```rust
┌─────────────────────────────────────────────────┐
│              🌲 SPRUCE VISION                   │
├─────────────────────────────────────────────────┤
│                                                 │
│  Vue 3.6 App  ──→  SpruceVM  ──→  Rust UI      │
│  (Developer    │   (Custom   │    (Direct      │
│   Writes)      │    Runtime) │     Rendering)  │
│                                                 │
│             ┌─────────────────┐                │
│             │  Pure Rust UI   │                │
│             │  • GPU Direct   │                │
│             │  • SIMD Ops     │                │
│             │  • Zero Bridge  │                │
│             └─────────────────┘                │
│                       │                        │
│             ┌─────────▼─────────┐              │
│             │  Native Platform  │              │
│             │ • Android/iOS     │              │
│             │ • Direct Surface  │              │
│             │ • 60+ FPS Target  │              │
│             └───────────────────┘              │
└─────────────────────────────────────────────────┘
```

## Current Status (v0.1.0-experimental)

### ✅ **Implemented and Working**
- **Vue 3.6 Development Environment**: Full-featured Vue development with latest beta
- **CLI Project Scaffolding**: Professional Vue project generation with TypeScript
- **Build System Architecture**: Comprehensive Cargo workspace with clean compilation
- **Template System**: High-quality Vue application templates
- **Demo Application**: Functional Vue 3.6 app demonstrating modern patterns
- **TypeScript Integration**: Complete Vue SFC support with proper type definitions

### 🚧 **Designed but Not Implemented**
- **SpruceVM JavaScript Engine**: Architecture defined, runtime not implemented
- **Pure Rust UI Renderer**: Type systems complete, GPU rendering not functional
- **Android Platform Integration**: JNI bridge designed, ANativeWindow integration incomplete
- **iOS Platform Support**: Architecture planned, implementation not started
- **Hot Reload for Native**: Command structure exists, functionality not implemented
- **Mobile App Building**: CLI commands structured, actual APK/IPA generation missing

### 📋 **Research Areas**
- **Performance Benchmarking**: Framework for testing, actual measurements needed
- **Cross-Platform Rendering**: Unified API designed, platform implementations pending
- **Developer Experience**: Vue-only workflow designed, native integration incomplete

## Development Environment

### **Vue 3.6 Development** (Works Today)
```bash
# Create new project
spruce create MyApp

# Development server with hot reload
cd MyApp
npm install --force
npm run dev              # ✅ Working Vue 3.6 + Vite

# TypeScript checking
npx tsc --noEmit        # ✅ Full Vue SFC type support

# Production build  
npm run build           # ✅ Optimized web build
```

### **Generated Project Structure**
```
MyApp/
├── src/
│   ├── App.vue          # Vue 3.6 Composition API
│   ├── main.ts          # TypeScript entry point
│   └── components/      # Vue components
├── package.json         # Vue 3.6.0-beta.7 dependencies
├── tsconfig.json        # Vue + TypeScript config
├── vite.config.ts       # Modern build setup
└── spruce.config.ts     # Platform configuration (future use)
```

## Research Goals 🎯

### **Performance Innovation**
Spruce explores whether combining Vue's developer experience with Rust's performance can achieve:
- **Native-level performance** without requiring native development skills
- **60+ FPS consistently** through direct GPU rendering
- **Sub-millisecond reactivity** via zero-allocation signal updates
- **Instant startup** through pre-compiled bytecode

### **Developer Experience**
- **Vue-only development**: Write only Vue/TypeScript, no platform-specific code
- **Universal deployment**: Single codebase for iOS, Android, Web, Desktop
- **Hot reload everywhere**: Instant updates across all platforms during development
- **Zero configuration**: Working mobile apps without complex native setup

### **Technical Innovation**
- **Vapor Mode Compilation**: Vue templates → optimized bytecode → direct rendering
- **Signal-based Reactivity**: Alien signals for zero-allocation state updates
- **Custom JavaScript VM**: SpruceVM optimized specifically for UI workloads
- **Direct GPU Access**: Bypass platform UI layers for maximum performance

## Platform Support Roadmap

### **Current Platform Status**

| Platform | CLI Support | Templates | Dev Server | Native Build | Status |
|----------|-------------|-----------|------------|--------------|---------|
| **Web**     | ✅ Working | ✅ Working | ✅ Working | ✅ Working  | **Production Ready** |
| **Android** | 🚧 Designed | 🚧 Designed | 📋 Planned | 📋 Planned | **Architecture Phase** |
| **iOS**     | 🚧 Designed | 🚧 Designed | 📋 Planned | 📋 Planned | **Architecture Phase** |
| **Desktop** | 🚧 Designed | 📋 Planned | 📋 Planned | 📋 Planned | **Planning Phase** |

### **Implementation Priorities**
1. **Phase 1**: Complete Vue 3.6 tooling (✅ Done)
2. **Phase 2**: Basic Android compilation (🚧 In Progress)
3. **Phase 3**: SpruceVM runtime implementation (📋 Planned)
4. **Phase 4**: Pure Rust UI renderer (📋 Planned)
5. **Phase 5**: iOS platform support (📋 Planned)

## Examples and Templates

### **Available Templates**
```bash
# Mobile-first Vue application
spruce create --template vue-mobile

# E-commerce application
spruce create --template shopping-app  

# Minimal starting point
spruce create --template blank
```

### **Template Features**
- **Vue 3.6.0-beta.7**: Latest Vue with Vapor Mode preparation
- **TypeScript**: Full type safety and IDE support
- **Vite**: Modern development server with hot reload
- **Responsive Design**: Mobile-optimized CSS and layouts
- **Modern Patterns**: Composition API, reactive refs, computed properties
- **Performance Ready**: Optimized bundle sizes and loading

## Community & Contributing

### **Current Status**
Spruce is an **experimental research project** exploring innovative mobile development approaches. While the full vision is under development, the Vue 3.6 development tools are functional and useful today.

### **How to Contribute**

#### **Immediate Opportunities**
- **Vue Template Enhancement**: Improve the CLI-generated Vue applications
- **Documentation**: Help clarify current vs planned features
- **Testing**: Try the Vue development workflow and report issues
- **Examples**: Create sample applications using Spruce templates

#### **Research Contributions**
- **Performance Analysis**: Benchmark current Vue vs native performance
- **Architecture Design**: Contribute to SpruceVM and Rust UI design
- **Platform Integration**: Help implement Android/iOS compilation
- **Developer Experience**: Improve the Vue → Native workflow

### **Getting Involved**
- **Discord**: [Join our community](https://discord.gg/T2rDj6rW) for discussions and support
- **GitHub Issues**: Report bugs, request features, or ask questions
- **Documentation**: Help improve guides and examples
- **Research**: Contribute to performance and architecture research

### **Project Philosophy**
Spruce believes that mobile development should be:
- **Accessible**: Vue developers shouldn't need to learn platform-specific languages
- **Performant**: Apps should achieve native-level performance automatically
- **Modern**: Development should use the latest web technologies and patterns
- **Universal**: One codebase should work everywhere without compromise

## Current Limitations ⚠️

### **What Doesn't Work Yet**
- **Native mobile app building**: CLI commands exist but don't generate APKs/IPAs
- **Platform deployment**: Deploy commands are placeholder implementations
- **Performance claims**: Benchmarks are not yet implemented
- **SpruceVM runtime**: JavaScript execution still uses standard engines
- **Rust UI rendering**: UI still renders through standard DOM/WebView

### **Vue 3.6 Beta Considerations**
- **Dependency conflicts**: Requires `npm install --force` for compatibility
- **Beta stability**: Some Vue 3.6 features may change before final release
- **Limited ecosystem**: Not all Vue plugins support 3.6 beta yet

## Future Vision 🚀

When complete, Spruce aims to enable:

```typescript
// Developer writes only Vue 3.6 + TypeScript
<template>
  <div class="mobile-app">
    <TouchableView @tap="handleTap">
      <Text>Hello Mobile!</Text>
    </TouchableView>
  </div>
</template>

<script setup lang="ts">
// Vue Composition API
const count = ref(0)
const handleTap = () => count.value++
</script>

// Compiles to:
// ✅ Android APK with native performance
// ✅ iOS IPA with native performance  
// ✅ Web PWA with standard performance
// ✅ Desktop app with native performance
```

All from a single Vue codebase, with native performance on every platform.

---

## License

Spruce is released under the **MIT License**. 

**Spruce** - *Exploring the future of mobile development through Vue + Rust innovation.*

---

### Acknowledgments

This project explores innovative combinations of:
- **Vue 3.6 Vapor Mode** - Compile-time optimizations from the Vue team
- **Rust Performance** - Memory safety and speed for mobile applications
- **Modern Development** - TypeScript, Vite, and contemporary web tooling

Built with curiosity about the future of cross-platform mobile development. 🌲