<template>
  <View :style="containerStyle">
    <!-- Header -->
    <View :style="headerStyle">
      <Text :style="titleStyle">Spruce Demo 🚀</Text>
      <Text :style="subtitleStyle">Multi-threaded Vue 3 + Rust + SpruceVM</Text>
    </View>

    <!-- Counter Section -->
    <View :style="sectionStyle">
      <Text :style="sectionTitleStyle">Reactive Counter</Text>
      <Text :style="counterTextStyle">Count: {{ count }}</Text>
      
      <View :style="buttonRowStyle">
        <Button :style="buttonStyle" @press="decrement">
          <Text :style="buttonTextStyle">-</Text>
        </Button>
        <Button :style="buttonStyle" @press="increment">
          <Text :style="buttonTextStyle">+</Text>
        </Button>
        <Button :style="resetButtonStyle" @press="reset">
          <Text :style="buttonTextStyle">Reset</Text>
        </Button>
      </View>
    </View>

    <!-- Performance Demo -->
    <View :style="sectionStyle">
      <Text :style="sectionTitleStyle">Performance Test</Text>
      <Text :style="textStyle">Background thread tasks: {{ backgroundTasks }}</Text>
      
      <Button :style="performanceButtonStyle" @press="runPerformanceTest">
        <Text :style="buttonTextStyle">Run Performance Test</Text>
      </Button>
    </View>

    <!-- Features List -->
    <View :style="sectionStyle">
      <Text :style="sectionTitleStyle">Framework Features</Text>
      
      <ScrollView :style="scrollStyle">
        <View v-for="feature in features" :key="feature.id" :style="featureItemStyle">
          <Text :style="featureIconStyle">{{ feature.icon }}</Text>
          <Text :style="featureTextStyle">{{ feature.text }}</Text>
        </View>
      </ScrollView>
    </View>

    <!-- Input Demo -->
    <View :style="sectionStyle">
      <Text :style="sectionTitleStyle">Native Input</Text>
      
      <TextInput 
        :style="inputStyle"
        :value="inputText"
        @change="handleInputChange"
        placeholder="Type something..."
      />
      
      <Text :style="textStyle">You typed: {{ inputText }}</Text>
    </View>

    <!-- Native API Demo -->
    <View :style="sectionStyle">
      <Text :style="sectionTitleStyle">Native APIs</Text>
      
      <Button :style="apiButtonStyle" @press="vibrate">
        <Text :style="buttonTextStyle">📳 Vibrate</Text>
      </Button>
      
      <Button :style="apiButtonStyle" @press="showAlert">
        <Text :style="buttonTextStyle">🚨 Show Alert</Text>
      </Button>
    </View>
  </View>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { spruceRuntime } from '../../runtime/vue-renderer'

// Reactive state
const count = ref(0)
const inputText = ref('')
const backgroundTasks = ref(0)

const features = ref([
  { id: 1, icon: '🦀', text: 'Rust-powered native rendering' },
  { id: 2, icon: '⚡', text: 'SpruceVM ultra-fast JavaScript engine' },
  { id: 3, icon: '🟢', text: 'Vue 3 composition API' },
  { id: 4, icon: '🧵', text: 'Multi-threaded architecture' },
  { id: 5, icon: '🔄', text: 'Zero-copy bridge communication' },
  { id: 6, icon: '📱', text: 'True native performance' },
  { id: 7, icon: '🎯', text: 'Direct native API access' },
  { id: 8, icon: '🚀', text: 'Instant first-frame rendering' },
])

// Event handlers
function increment() {
  count.value++
}

function decrement() {
  count.value--
}

function reset() {
  count.value = 0
}

function handleInputChange(event: any) {
  inputText.value = event.target.value
}

async function runPerformanceTest() {
  console.log('Running performance test...')
  
  // Simulate background tasks
  for (let i = 0; i < 10; i++) {
    backgroundTasks.value++
    
    // Call native function for heavy computation
    await spruceRuntime.callNativeFunction('compute.fibonacci', JSON.stringify([30]))
    
    // Small delay to show progress
    await new Promise(resolve => setTimeout(resolve, 100))
  }
  
  console.log('Performance test completed!')
}

async function vibrate() {
  try {
    await spruceRuntime.callNativeFunction('device.vibrate', JSON.stringify([500]))
    console.log('Device vibrated!')
  } catch (error) {
    console.error('Vibration failed:', error)
  }
}

async function showAlert() {
  try {
    await spruceRuntime.callNativeFunction('ui.showAlert', 
      JSON.stringify(['Spruce', 'Hello from Vue + Rust UI!']))
    console.log('Alert shown!')
  } catch (error) {
    console.error('Alert failed:', error)
  }
}

// Lifecycle
onMounted(() => {
  console.log('✅ Spruce Demo App mounted!')
})

// Styles
const containerStyle = computed(() => ({
  flex: 1,
  backgroundColor: '#f8fafc',
  padding: 16,
}))

const headerStyle = {
  alignItems: 'center',
  padding: 20,
  backgroundColor: 'white',
  borderRadius: 12,
  marginBottom: 16,
  shadowOpacity: 0.1,
  shadowRadius: 8,
}

const titleStyle = {
  fontSize: 28,
  fontWeight: 'bold',
  color: '#1e293b',
  textAlign: 'center',
}

const subtitleStyle = {
  fontSize: 16,
  color: '#64748b',
  textAlign: 'center',
  marginTop: 8,
}

const sectionStyle = {
  backgroundColor: 'white',
  padding: 16,
  borderRadius: 12,
  marginBottom: 16,
  shadowOpacity: 0.1,
  shadowRadius: 4,
}

const sectionTitleStyle = {
  fontSize: 18,
  fontWeight: 'bold',
  color: '#374151',
  marginBottom: 12,
}

const counterTextStyle = {
  fontSize: 24,
  fontWeight: 'bold',
  color: '#059669',
  textAlign: 'center',
  marginBottom: 16,
}

const buttonRowStyle = {
  flexDirection: 'row',
  justifyContent: 'space-around',
  alignItems: 'center',
}

const buttonStyle = {
  backgroundColor: '#3b82f6',
  padding: 12,
  borderRadius: 8,
  minWidth: 50,
  alignItems: 'center',
}

const resetButtonStyle = {
  backgroundColor: '#ef4444',
  padding: 12,
  borderRadius: 8,
  minWidth: 80,
  alignItems: 'center',
}

const performanceButtonStyle = {
  backgroundColor: '#8b5cf6',
  padding: 12,
  borderRadius: 8,
  marginTop: 8,
  alignItems: 'center',
}

const apiButtonStyle = {
  backgroundColor: '#f59e0b',
  padding: 12,
  borderRadius: 8,
  marginBottom: 8,
  alignItems: 'center',
}

const buttonTextStyle = {
  color: 'white',
  fontWeight: 'bold',
  fontSize: 16,
}

const textStyle = {
  fontSize: 14,
  color: '#4b5563',
  marginTop: 8,
}

const scrollStyle = {
  maxHeight: 200,
}

const featureItemStyle = {
  flexDirection: 'row',
  alignItems: 'center',
  padding: 8,
  backgroundColor: '#f1f5f9',
  borderRadius: 8,
  marginBottom: 8,
}

const featureIconStyle = {
  fontSize: 20,
  marginRight: 12,
}

const featureTextStyle = {
  fontSize: 14,
  color: '#475569',
  flex: 1,
}

const inputStyle = {
  borderWidth: 1,
  borderColor: '#d1d5db',
  borderRadius: 8,
  padding: 12,
  fontSize: 16,
  backgroundColor: 'white',
  marginBottom: 8,
}
</script>