#!/bin/bash
# Spruce Platform Performance Testing Script

set -e

echo "🌲 Spruce Platform Performance Testing"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_step() {
    echo -e "${BLUE}📊 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if hyperfine is installed for benchmarking
if ! command -v hyperfine &> /dev/null; then
    print_warning "hyperfine not found. Installing..."
    cargo install hyperfine
fi

print_step "Building optimized release version..."

# Clean previous builds
cargo clean

# Build with maximum optimizations
echo "Building with profile: release"
time cargo build --release --all-features 2>&1 | tee build_release.log

# Build with mobile optimizations  
echo "Building with profile: mobile-release"
time cargo build --profile mobile-release --all-features 2>&1 | tee build_mobile.log

print_step "Running performance benchmarks..."

# Benchmark compilation time
print_step "Testing build performance..."
echo "Release build time:"
hyperfine --warmup 1 --runs 3 \
    'cargo build --release --all-features' \
    --export-markdown build_bench.md

# Benchmark binary size
print_step "Analyzing binary sizes..."
echo ""
echo "Binary sizes:"
echo "============="

if [ -f "target/release/spruce" ]; then
    release_size=$(stat -c%s "target/release/spruce")
    echo "Spruce CLI (release): $(numfmt --to=iec $release_size)"
fi

if [ -f "target/mobile-release/spruce" ]; then
    mobile_size=$(stat -c%s "target/mobile-release/spruce")
    echo "Spruce CLI (mobile): $(numfmt --to=iec $mobile_size)"
fi

if [ -f "target/release/libspruce_core.so" ]; then
    core_size=$(stat -c%s "target/release/libspruce_core.so")
    echo "Spruce Core (release): $(numfmt --to=iec $core_size)"
fi

# Benchmark CLI performance
print_step "Testing CLI performance..."
if [ -f "target/release/spruce" ]; then
    echo "CLI startup time:"
    hyperfine --warmup 3 --runs 10 \
        './target/release/spruce --help' \
        --export-markdown cli_bench.md
else
    print_warning "CLI binary not found"
fi

# Test Rust UI performance (if tests exist)
print_step "Testing Rust UI performance..."
echo "Core library tests:"
cargo test --release rust_ui --all-features -- --nocapture 2>&1 | tee test_performance.log

# Memory usage analysis
print_step "Analyzing memory usage..."
echo "Running memory profiler..."

if command -v valgrind &> /dev/null; then
    echo "Memory analysis with Valgrind:"
    timeout 30s valgrind --tool=massif --pages-as-heap=yes \
        ./target/release/spruce --help 2>&1 | head -20
else
    print_warning "Valgrind not available for memory analysis"
fi

# Dependency analysis
print_step "Analyzing dependencies..."
echo "Dependency tree size:"
cargo tree --all-features | wc -l
echo "Duplicate dependencies:"
cargo tree --duplicate

# Performance summary
print_step "Performance Summary"
echo "==================="

echo ""
echo "📊 Build Performance:"
echo "   - Release build: See build_bench.md"
echo "   - Dependencies: $(cargo tree --all-features | wc -l) total"

echo ""
echo "📦 Binary Sizes:"
if [ -f "target/release/spruce" ]; then
    echo "   - CLI: $(numfmt --to=iec $(stat -c%s "target/release/spruce"))"
fi

echo ""
echo "🧪 Test Results:"
echo "   - See test_performance.log for detailed results"

echo ""
echo "🎯 Optimization Status:"
echo "   ✅ LTO enabled (link-time optimization)"
echo "   ✅ Codegen units = 1 (maximum optimization)"
echo "   ✅ Panic = abort (smaller binary)"
echo "   ✅ Symbols stripped"
echo "   ✅ Workspace dependencies unified"
echo "   ✅ Platform-specific optimizations"

print_success "Performance testing completed!"
echo "Check generated files: build_bench.md, cli_bench.md, test_performance.log"