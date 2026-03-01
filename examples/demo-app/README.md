# Spruce Demo App

This demo application showcases Vue 3.6 running in a web browser, demonstrating the target developer experience for the Spruce platform.

## What This Demonstrates

### ✅ **Current Capabilities**
- **Vue 3.6.0-beta.7**: Uses the exact version that will be integrated with Spruce
- **Vue 3 Reactivity**: Shows Vue's reactive system that will be accelerated by SpruceVM
- **Component Architecture**: Demonstrates the Vue component structure that will compile to Rust UI
- **Developer Experience**: Shows how Vue developers will write code for Spruce apps

### 🚧 **Architecture Preview**
- **Planned Features**: Shows the status of various Spruce components
- **Performance Targets**: Displays the goals for the Rust UI renderer
- **Rendering Mode**: Demonstrates the concept of switching between Vue DOM and Pure Rust UI

## Running the Demo

Since the Spruce runtime is still in development, this demo runs as a standard Vue 3.6 web application:

```bash
# Install dependencies (Vue 3.6.0-beta.7)
npm install --force  # --force needed for Vue 3.6 beta compatibility

# Run in development mode (Vite + Vue 3.6)
npm run dev

# Build for production
npm run build

# Preview production build  
npm run preview

# Future: Spruce development (when runtime is complete)
npm run spruce:dev
npm run spruce:build
```

The demo will be available at http://localhost:3000 and includes hot-reload for development.

## Technical Details

### Vue 3.6 Features
- **Composition API**: Modern Vue development patterns
- **Reactive Refs**: Efficient state management
- **Computed Properties**: Automatic dependency tracking
- **Scoped CSS**: Component-isolated styles

### Spruce Integration Points
- **Component Structure**: Matches planned Rust UI component hierarchy
- **Event Handling**: Demonstrates touch/click patterns for mobile
- **State Management**: Shows reactive patterns that will be accelerated
- **Styling**: Uses patterns compatible with planned Rust UI styling

## Future Integration

When the Spruce platform is complete, this same Vue code will:

1. **Compile to Rust**: Vue components will be translated to native Rust UI components
2. **Run on SpruceVM**: JavaScript will execute on the custom Spruce JavaScript engine
3. **Render with GPU**: UI will be rendered directly to GPU surfaces
4. **Hot Reload**: Changes will update instantly on connected devices

## Architecture Mapping

| Current (Web) | Future (Spruce) |
|---------------|-----------------|
| Browser DOM | Pure Rust UI |
| V8 JavaScript | SpruceVM Engine |
| CSS Rendering | GPU Shaders |
| DevTools | Spruce Debugger |
| `npm run dev` | `spruce dev` |
| Web Bundle | Native APK/IPA |

This demo provides a preview of the developer experience while the underlying Rust implementation is being built.