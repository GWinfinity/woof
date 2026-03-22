#!/bin/bash
#
# Cold start benchmark for woof
# 测量woof冷启动时间

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

WOOF_BIN="${PROJECT_ROOT}/target/release/woof"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="${PROJECT_ROOT}/target/debug/woof"
fi

TEST_FILE="${PROJECT_ROOT}/testdata/sample.go"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Simple timing function
measure_time() {
    local name="$1"
    shift
    local cmd="$@"
    
    # Run 5 times and calculate average
    local total=0
    local runs=5
    
    for i in $(seq 1 $runs); do
        # Clear cache between runs (Linux)
        if [ -f /proc/sys/vm/drop_caches ] && [ $i -gt 1 ]; then
            sync
            echo 3 | sudo tee /proc/sys/vm/drop_caches > /dev/null 2>&1 || true
        fi
        
        local start=$(date +%s%N)
        eval "$cmd" > /dev/null 2>&1 || true
        local end=$(date +%s%N)
        
        local elapsed=$(( (end - start) / 1000000 ))
        total=$((total + elapsed))
    done
    
    local avg=$((total / runs))
    echo "$avg"
}

# Measure with /usr/bin/time for detailed stats
measure_detailed() {
    local name="$1"
    shift
    
    log_info "Measuring: $name"
    
    # Use time command for detailed metrics
    /usr/bin/time -v "$@" 2>&1 | grep -E "(Elapsed|Maximum resident|User time|System time)" || true
}

# Main benchmark
run_benchmark() {
    echo "============================================"
    echo "Woof Cold Start Benchmark"
    echo "============================================"
    echo ""
    
    if [ ! -f "$WOOF_BIN" ]; then
        echo "Error: woof binary not found at $WOOF_BIN"
        echo "Please build first: cargo build --release"
        exit 1
    fi
    
    log_info "Binary: $WOOF_BIN"
    log_info "Test file: $TEST_FILE"
    echo ""
    
    # 1. Binary startup time (--version)
    echo "1. Binary Startup Time (--version)"
    echo "-----------------------------------"
    local version_time=$(measure_time "version" "$WOOF_BIN --version")
    echo "  Average: ${version_time}ms"
    echo "  Target: <10ms"
    if [ "$version_time" -lt 10 ]; then
        log_success "PASS"
    else
        echo "  ⚠️  Consider optimizing"
    fi
    echo ""
    
    # 2. Help display time
    echo "2. Help Display Time (--help)"
    echo "-----------------------------"
    local help_time=$(measure_time "help" "$WOOF_BIN --help")
    echo "  Average: ${help_time}ms"
    echo "  Target: <15ms"
    if [ "$help_time" -lt 15 ]; then
        log_success "PASS"
    else
        echo "  ⚠️  Consider optimizing"
    fi
    echo ""
    
    # 3. First lint (cold start)
    if [ -f "$TEST_FILE" ]; then
        echo "3. First Lint (Cold Start)"
        echo "--------------------------"
        local lint_time=$(measure_time "lint" "$WOOF_BIN check $TEST_FILE")
        echo "  Average: ${lint_time}ms"
        echo "  Target: <100ms"
        if [ "$lint_time" -lt 100 ]; then
            log_success "PASS"
        else
            echo "  ⚠️  Consider optimizing"
        fi
        echo ""
    fi
    
    # 4. Rules list (loading all rules)
    echo "4. Rules List Loading"
    echo "---------------------"
    local rules_time=$(measure_time "rules" "$WOOF_BIN rules")
    echo "  Average: ${rules_time}ms"
    echo "  Target: <50ms"
    if [ "$rules_time" -lt 50 ]; then
        log_success "PASS"
    else
        echo "  ⚠️  Consider optimizing"
    fi
    echo ""
    
    # 5. Detailed measurement for lint
    if [ -f "$TEST_FILE" ]; then
        echo "5. Detailed Lint Metrics"
        echo "------------------------"
        measure_detailed "lint" "$WOOF_BIN" "check" "$TEST_FILE"
        echo ""
    fi
    
    # 6. Binary size
    echo "6. Binary Size"
    echo "--------------"
    local size=$(stat -c%s "$WOOF_BIN" 2>/dev/null || stat -f%z "$WOOF_BIN")
    local size_mb=$((size / 1024 / 1024))
    echo "  Size: ${size_mb}MB (${size} bytes)"
    echo "  Target: <5MB"
    if [ "$size_mb" -lt 5 ]; then
        log_success "PASS"
    else
        echo "  ⚠️  Consider stripping or optimizing"
    fi
    echo ""
    
    # Summary
    echo "============================================"
    echo "Summary"
    echo "============================================"
    echo "Version: ${version_time}ms"
    echo "Help: ${help_time}ms"
    [ -f "$TEST_FILE" ] && echo "Lint: ${lint_time}ms"
    echo "Rules: ${rules_time}ms"
    echo "Binary: ${size_mb}MB"
    echo ""
}

# Optimization suggestions
show_optimizations() {
    echo "============================================"
    echo "Cold Start Optimization Suggestions"
    echo "============================================"
    echo ""
    
    cat << 'EOF'
Current Potential Issues:
-------------------------
1. Parser initialization on first use
2. Rule registry loading
3. Configuration parsing
4. Dynamic library loading (if any)

Optimization Strategies:
------------------------

1. Lazy Initialization
   - Delay parser pool creation until first lint
   - Lazy load rule definitions
   
2. Static Compilation
   - Use LTO (Link Time Optimization) ✅
   - Strip debug symbols ✅
   - Enable codegen-units=1 ✅
   
3. Pre-initialization
   - Initialize parser pool at startup
   - Pre-compile regex patterns
   - Cache rule registry

4. Reduce Dependencies
   - Minimize startup dependencies
   - Use lazy_static for global state
   
5. Binary Compression
   - Use upx for further compression (optional)

Implementation:
---------------

// src/main.rs - Pre-initialize
fn main() {
    // Pre-initialize parser pool in background
    std::thread::spawn(|| {
        let _ = PARSER_POOL.acquire();
    });
    
    // Parse CLI
    let cli = Cli::parse();
    ...
}

// Cargo.toml optimizations
codegen-units = 1      # ✅ Already set
lto = true             # ✅ Already set
strip = true           # ✅ Already set
panic = "abort"        # ✅ Already set
EOF
}

# Run with strace to analyze syscalls
analyze_syscalls() {
    if ! command -v strace &> /dev/null; then
        log_info "strace not available, skipping syscall analysis"
        return
    fi
    
    echo "============================================"
    echo "Syscall Analysis (first 100ms)"
    echo "============================================"
    echo ""
    
    timeout 0.1 strace -c "$WOOF_BIN" --version 2>&1 | tail -20 || true
}

# Main
main() {
    case "${1:-}" in
        --optimize)
            show_optimizations
            ;;
        --strace)
            analyze_syscalls
            ;;
        *)
            run_benchmark
            ;;
    esac
}

main "$@"
