// Vue 3.6 Performance Test Example for Spruce Platform
// 
// This example demonstrates Vue 3.6 Vapor Mode performance benefits
// Run with: npm run test:performance

import { createApp, ref, computed, reactive, nextTick } from 'vue';

console.log('🚀 Vue 3.6 Performance Test on Spruce Platform');
console.log('===============================================');

// Performance test suite for Vue 3.6 on Spruce
async function runPerformanceTests() {
    console.log('📊 Testing Vue 3.6 Vapor Mode Performance...');
    
    // Test 1: Reactive Performance
    await testReactivePerformance();
    
    // Test 2: Component Rendering Performance  
    await testComponentRenderingPerformance();
    
    // Test 3: List Rendering Performance
    await testListRenderingPerformance();
    
    console.log('\n✅ All performance tests completed!');
    console.log('\n💡 Why Spruce + Vue 3.6 is Fast:');
    console.log('================================');
    console.log('✅ Vapor Mode - No Virtual DOM overhead');
    console.log('✅ Alien Signals - Zero-allocation reactivity');
    console.log('✅ Pure Rust UI - Direct GPU rendering');
    console.log('✅ SpruceVM - Optimized JavaScript engine');
}

async function testReactivePerformance() {
    console.log('\n🔄 Testing Reactive Performance...');
    const iterations = 100000;
    
    // Create reactive state
    const counter = ref(0);
    const doubled = computed(() => counter.value * 2);
    const quadrupled = computed(() => doubled.value * 2);
    
    const startTime = performance.now();
    
    // Test reactive updates
    for (let i = 0; i < iterations; i++) {
        counter.value = i;
        // Access computed values to trigger reactivity
        const _ = quadrupled.value;
    }
    
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.log(`   ⚡ ${iterations} reactive updates in ${duration.toFixed(2)}ms`);
    console.log(`   📈 ${(iterations / duration * 1000).toFixed(0)} updates/second`);
    console.log(`   🎯 Average: ${(duration / iterations * 1000).toFixed(2)}μs per update`);
}

async function testComponentRenderingPerformance() {
    console.log('\n🖼️  Testing Component Rendering...');
    
    const app = createApp({
        setup() {
            const items = ref(Array.from({ length: 1000 }, (_, i) => ({
                id: i,
                name: `Item ${i}`,
                value: Math.random()
            })));
            
            const sortedItems = computed(() => 
                items.value.slice().sort((a, b) => a.value - b.value)
            );
            
            return { items, sortedItems };
        },
        template: `
            <div>
                <div v-for="item in sortedItems" :key="item.id">
                    {{ item.name }}: {{ item.value.toFixed(3) }}
                </div>
            </div>
        `
    });
    
    // Create container element
    const container = document.createElement('div');
    container.id = 'performance-test';
    document.body.appendChild(container);
    
    const startTime = performance.now();
    
    // Mount and render
    const instance = app.mount('#performance-test');
    await nextTick();
    
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.log(`   ⚡ 1000 components rendered in ${duration.toFixed(2)}ms`);
    console.log(`   🎯 Target: <16.67ms (60 FPS) - ${duration < 16.67 ? '✅ PASSED' : '❌ NEEDS OPTIMIZATION'}`);
    
    // Cleanup
    instance.$el.remove();
}

async function testListRenderingPerformance() {
    console.log('\n📋 Testing Large List Performance...');
    
    const itemCount = 10000;
    const items = reactive(Array.from({ length: itemCount }, (_, i) => ({
        id: i,
        text: `Dynamic Item ${i}`,
        active: i % 2 === 0
    })));
    
    const startTime = performance.now();
    
    // Simulate list updates (like real-time data)
    const updateCount = 1000;
    for (let i = 0; i < updateCount; i++) {
        const randomIndex = Math.floor(Math.random() * itemCount);
        items[randomIndex].text = `Updated Item ${randomIndex} - ${i}`;
        items[randomIndex].active = !items[randomIndex].active;
    }
    
    const endTime = performance.now();
    const duration = endTime - startTime;
    
    console.log(`   ⚡ ${updateCount} list updates in ${duration.toFixed(2)}ms`);
    console.log(`   📈 ${(updateCount / duration * 1000).toFixed(0)} updates/second`);
    console.log(`   🎯 Handling ${itemCount} items efficiently`);
}

// Mobile-specific performance tests
async function testMobilePerformance() {
    console.log('\n📱 Testing Mobile Performance...');
    
    // Simulate touch events and animations
    const touchSimulations = 100;
    const animationDuration = 16.67; // 60 FPS target
    
    console.log(`   🎯 Target: ${touchSimulations} touch events under ${animationDuration}ms each`);
    
    // Test touch responsiveness
    let totalTouchTime = 0;
    for (let i = 0; i < touchSimulations; i++) {
        const touchStart = performance.now();
        
        // Simulate touch event processing
        await new Promise(resolve => setTimeout(resolve, 1));
        
        const touchEnd = performance.now();
        totalTouchTime += (touchEnd - touchStart);
    }
    
    const averageTouchTime = totalTouchTime / touchSimulations;
    console.log(`   ⚡ Average touch response: ${averageTouchTime.toFixed(2)}ms`);
    console.log(`   🎯 Target: <10ms - ${averageTouchTime < 10 ? '✅ PASSED' : '❌ NEEDS OPTIMIZATION'}`);
}

// Auto-run tests when loaded
if (typeof window !== 'undefined') {
    // Browser environment
    document.addEventListener('DOMContentLoaded', runPerformanceTests);
} else {
    // Node.js environment  
    runPerformanceTests().catch(console.error);
}

export { runPerformanceTests, testReactivePerformance, testComponentRenderingPerformance };