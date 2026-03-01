<template>
  <div class="container">
    <!-- Header -->
    <header class="header">
      <h1 class="title">Spruce Demo 🌲</h1>
      <p class="subtitle">Vue 3.6 Vapor Mode Architecture Preview</p>
    </header>

    <!-- Counter Section -->
    <section class="section">
      <h2 class="section-title">Vue 3.6 Reactivity</h2>
      <p class="counter-display">Count: {{ count }}</p>
      
      <div class="button-row">
        <button class="btn btn-primary" @click="decrement">-</button>
        <button class="btn btn-primary" @click="increment">+</button>
        <button class="btn btn-danger" @click="reset">Reset</button>
      </div>
    </section>

    <!-- Architecture Demo -->
    <section class="section">
      <h2 class="section-title">Architecture Preview</h2>
      <p class="text">Rendering with: {{ renderingMode }}</p>
      <p class="text">Vue version: {{ vueVersion }}</p>
      
      <button class="btn btn-secondary" @click="toggleRenderMode">
        Switch to {{ nextRenderMode }}
      </button>
    </section>

    <!-- Features List -->
    <section class="section">
      <h2 class="section-title">Planned Architecture</h2>
      
      <div class="features-grid">
        <div v-for="feature in features" :key="feature.id" class="feature-item">
          <span class="feature-icon">{{ feature.icon }}</span>
          <span class="feature-text">{{ feature.text }}</span>
          <span class="feature-status">{{ feature.status }}</span>
        </div>
      </div>
    </section>

    <!-- Input Demo -->
    <section class="section">
      <h2 class="section-title">Vue Input Example</h2>
      
      <input 
        class="input"
        v-model="inputText"
        placeholder="Type something (Vue 3.6 reactive)..."
      />
      
      <p class="text">You typed: <strong>{{ inputText }}</strong></p>
      <p class="text">Input length: {{ inputText.length }} characters</p>
    </section>

    <!-- Performance Info -->
    <section class="section">
      <h2 class="section-title">Performance Targets</h2>
      
      <div class="performance-grid">
        <div class="perf-item">
          <span class="perf-label">Current:</span>
          <span class="perf-value">Vue 3.6 Beta</span>
        </div>
        <div class="perf-item">
          <span class="perf-label">Target:</span>
          <span class="perf-value">Pure Rust UI</span>
        </div>
        <div class="perf-item">
          <span class="perf-label">Goal:</span>
          <span class="perf-value">60+ FPS</span>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { version } from 'vue'

// Reactive state
const count = ref(0)
const inputText = ref('')
const renderingMode = ref('Vue 3.6 DOM')

// Vue version info
const vueVersion = version

// Computed properties
const nextRenderMode = computed(() => 
  renderingMode.value === 'Vue 3.6 DOM' ? 'Pure Rust UI (Planned)' : 'Vue 3.6 DOM'
)

const features = ref([
  { id: 1, icon: '🦀', text: 'Pure Rust UI renderer', status: '🚧 Designing' },
  { id: 2, icon: '⚡', text: 'SpruceVM JavaScript engine', status: '🚧 Prototyping' },
  { id: 3, icon: '🟢', text: 'Vue 3.6 Vapor Mode', status: '✅ Ready' },
  { id: 4, icon: '🧵', text: 'Multi-threaded architecture', status: '🚧 Planning' },
  { id: 5, icon: '🔄', text: 'Zero-copy communication', status: '📋 Planned' },
  { id: 6, icon: '📱', text: 'Native mobile performance', status: '🎯 Goal' },
  { id: 7, icon: '🎯', text: 'Direct GPU rendering', status: '📋 Planned' },
  { id: 8, icon: '🚀', text: 'Hot reload system', status: '🚧 Designing' },
])

// Event handlers
function increment() {
  count.value++
}

function decrement() {
  count.value > 0 && count.value--
}

function reset() {
  count.value = 0
}

function toggleRenderMode() {
  // This demonstrates the architecture concept
  // In the future, this would switch between Vue DOM and Pure Rust UI
  renderingMode.value = renderingMode.value === 'Vue 3.6 DOM' 
    ? 'Pure Rust UI (Not yet implemented)' 
    : 'Vue 3.6 DOM'
    
  console.log(`Switched to: ${renderingMode.value}`)
}

// Lifecycle
onMounted(() => {
  console.log('✅ Spruce Architecture Preview mounted!')
  console.log(`Running Vue ${vueVersion}`)
})

</script>

<style scoped>
.container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  padding: 20px;
}

.header {
  text-align: center;
  padding: 40px 20px;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
  border-radius: 16px;
  margin-bottom: 24px;
}

.title {
  font-size: 2.5rem;
  font-weight: 700;
  margin: 0 0 12px 0;
  text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
}

.subtitle {
  font-size: 1.1rem;
  opacity: 0.9;
  margin: 0;
  font-weight: 300;
}

.section {
  background: rgba(255, 255, 255, 0.15);
  backdrop-filter: blur(10px);
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.section-title {
  font-size: 1.4rem;
  font-weight: 600;
  margin: 0 0 16px 0;
  color: rgba(255, 255, 255, 0.95);
}

.counter-display {
  font-size: 2rem;
  font-weight: 700;
  text-align: center;
  margin: 16px 0;
  color: #FFE082;
}

.button-row {
  display: flex;
  gap: 12px;
  justify-content: center;
  flex-wrap: wrap;
}

.btn {
  padding: 12px 20px;
  border: none;
  border-radius: 12px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  min-width: 48px;
  min-height: 48px;
}

.btn-primary {
  background: rgba(59, 130, 246, 0.8);
  color: white;
}

.btn-secondary {
  background: rgba(139, 92, 246, 0.8);
  color: white;
}

.btn-danger {
  background: rgba(239, 68, 68, 0.8);
  color: white;
}

.btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.text {
  font-size: 1rem;
  opacity: 0.9;
  margin: 8px 0;
}

.features-grid {
  display: grid;
  gap: 12px;
}

.feature-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
}

.feature-icon {
  font-size: 1.5rem;
}

.feature-text {
  flex: 1;
  font-weight: 500;
}

.feature-status {
  font-size: 0.9rem;
  opacity: 0.8;
  font-weight: 600;
}

.input {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 1rem;
  backdrop-filter: blur(10px);
  margin-bottom: 12px;
}

.input::placeholder {
  color: rgba(255, 255, 255, 0.7);
}

.input:focus {
  outline: none;
  border-color: rgba(255, 224, 130, 0.6);
  box-shadow: 0 0 0 3px rgba(255, 224, 130, 0.2);
}

.performance-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
}

.perf-item {
  text-align: center;
  padding: 16px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
}

.perf-label {
  display: block;
  font-size: 0.9rem;
  opacity: 0.8;
  margin-bottom: 4px;
}

.perf-value {
  display: block;
  font-size: 1.1rem;
  font-weight: 600;
  color: #FFE082;
}

@media (max-width: 768px) {
  .title {
    font-size: 2rem;
  }
  
  .container {
    padding: 16px;
  }
  
  .section {
    padding: 20px;
  }
  
  .button-row {
    flex-direction: column;
    align-items: center;
  }
}
</style>