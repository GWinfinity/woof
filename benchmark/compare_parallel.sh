#!/bin/bash
# Parallel performance comparison: woof vs golangci-lint
# Shows near-linear speedup with increasing CPU cores

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="${1:-.}"
CPU_CORES=(1 2 4 8 16)
ITERATIONS=3
WOOF_BIN="${WOOF_BIN:-./target/release/woofmt}"

# Check available CPUs
AVAILABLE_CPUS=$(nproc)
echo -e "${BLUE}=== Woof Parallel Performance Benchmark ===${NC}"
echo ""
echo "System CPUs: $AVAILABLE_CPUS"
echo "Test iterations per config: $ITERATIONS"
echo "Project: $PROJECT_DIR"
echo ""

# Verify binaries
if [ ! -f "$WOOF_BIN" ]; then
    echo -e "${RED}Error: woof binary not found at $WOOF_BIN${NC}"
    exit 1
fi

# Check for golangci-lint
HAS_GOLANGCI=false
if command -v golangci-lint &> /dev/null; then
    HAS_GOLANGCI=true
    GOLANGCI_VERSION=$(golangci-lint --version | head -1)
    echo "golangci-lint: $GOLANGCI_VERSION"
else
    echo -e "${YELLOW}Warning: golangci-lint not found, skipping comparison${NC}"
fi

echo "woof: $($WOOF_BIN --version)"
echo ""

# Count Go files
GO_FILE_COUNT=$(find "$PROJECT_DIR" -name "*.go" -type f 2>/dev/null | wc -l)
echo "Go files to analyze: $GO_FILE_COUNT"
echo ""

# Results storage
declare -a WOOF_RESULTS
declare -a GOLANGCI_RESULTS

# Benchmark function
benchmark_tool() {
    local tool="$1"
    local cpus="$2"
    local total_time=0
    
    for i in $(seq 1 $ITERATIONS); do
        local start_time end_time elapsed
        
        case "$tool" in
            "woof")
                start_time=$(date +%s%N)
                # Run woof with specified threads, suppress output
                $WOOF_BIN check "$PROJECT_DIR" --threads "$cpus" --quiet 2>/dev/null || true
                end_time=$(date +%s%N)
                ;;
            "golangci-lint")
                start_time=$(date +%s%N)
                # Run golangci-lint with specified concurrency
                golangci-lint run --concurrency="$cpus" "$PROJECT_DIR" 2>/dev/null || true
                end_time=$(date +%s%N)
                ;;
        esac
        
        elapsed=$(( (end_time - start_time) / 1000000 ))  # ms
        total_time=$((total_time + elapsed))
    done
    
    # Return average
    echo $((total_time / ITERATIONS))
}

# Run benchmarks
echo -e "${BLUE}=== Running Benchmarks ===${NC}"
echo ""
printf "%-8s %-12s %-12s %-12s %-12s\n" "CPUs" "woof (ms)" "woof Δ" "golangci" "golangci Δ"
echo "-----------------------------------------------------------------"

# Single thread baseline
WOOF_BASELINE=$(benchmark_tool "woof" 1)
WOOF_RESULTS[1]=$WOOF_BASELINE

if [ "$HAS_GOLANGCI" = true ]; then
    GOLANGCI_BASELINE=$(benchmark_tool "golangci-lint" 1)
    GOLANGCI_RESULTS[1]=$GOLANGCI_BASELINE
else
    GOLANGCI_BASELINE="N/A"
fi

# Print baseline
printf "%-8s %-12s %-12s %-12s %-12s\n" \
    "1" \
    "$WOOF_BASELINE" \
    "1.00x" \
    "$GOLANGCI_BASELINE" \
    "1.00x"

# Test other CPU counts
for cpu in 2 4 8 16; do
    if [ "$cpu" -le "$AVAILABLE_CPUS" ]; then
        echo -n "Testing with $cpu CPUs... "
        
        # Benchmark woof
        WOOF_TIME=$(benchmark_tool "woof" "$cpu")
        WOOF_RESULTS[$cpu]=$WOOF_TIME
        
        # Calculate speedup
        if [ "$WOOF_BASELINE" -gt 0 ]; then
            WOOF_SPEEDUP=$(echo "scale=2; $WOOF_BASELINE / $WOOF_TIME" | bc 2>/dev/null || echo "N/A")
        else
            WOOF_SPEEDUP="N/A"
        fi
        
        # Benchmark golangci-lint
        if [ "$HAS_GOLANGCI" = true ]; then
            GOLANGCI_TIME=$(benchmark_tool "golangci-lint" "$cpu")
            GOLANGCI_RESULTS[$cpu]=$GOLANGCI_TIME
            
            if [ "$GOLANGCI_BASELINE" != "N/A" ] && [ "$GOLANGCI_BASELINE" -gt 0 ]; then
                GOLANGCI_SPEEDUP=$(echo "scale=2; $GOLANGCI_BASELINE / $GOLANGCI_TIME" | bc 2>/dev/null || echo "N/A")
            else
                GOLANGCI_SPEEDUP="N/A"
            fi
        else
            GOLANGCI_TIME="N/A"
            GOLANGCI_SPEEDUP="N/A"
        fi
        
        echo "done"
        
        printf "%-8s %-12s %-12s %-12s %-12s\n" \
            "$cpu" \
            "$WOOF_TIME" \
            "${WOOF_SPEEDUP}x" \
            "$GOLANGCI_TIME" \
            "${GOLANGCI_SPEEDUP}x"
    fi
done

echo ""
echo -e "${BLUE}=== Parallel Efficiency Analysis ===${NC}"
echo ""
echo "Parallel Efficiency = (Speedup / CPU count) * 100%"
echo ""

for cpu in 2 4 8 16; do
    if [ -n "${WOOF_RESULTS[$cpu]}" ] && [ "${WOOF_RESULTS[$cpu]}" -gt 0 ]; then
        WOOF_TIME="${WOOF_RESULTS[$cpu]}"
        SPEEDUP=$(echo "scale=4; $WOOF_BASELINE / $WOOF_TIME" | bc)
        EFFICIENCY=$(echo "scale=1; ($SPEEDUP / $cpu) * 100" | bc)
        
        printf "woof@%2d CPUs: %5.2fx speedup, %5.1f%% efficiency\n" "$cpu" "$SPEEDUP" "$EFFICIENCY"
    fi
done

echo ""

# Summary
echo -e "${BLUE}=== Summary ===${NC}"
echo ""

if [ -n "${WOOF_RESULTS[8]}" ]; then
    SPEEDUP_8=$(echo "scale=2; $WOOF_BASELINE / ${WOOF_RESULTS[8]}" | bc)
    EFFICIENCY_8=$(echo "scale=1; ($SPEEDUP_8 / 8) * 100" | bc)
    
    echo -e "woof at 8 CPUs: ${GREEN}${SPEEDUP_8}x${NC} speedup, ${EFFICIENCY_8}% efficiency"
    
    if [ $(echo "$EFFICIENCY_8 > 80" | bc) -eq 1 ]; then
        echo -e "${GREEN}✓ Excellent parallel efficiency (>80%)${NC}"
    elif [ $(echo "$EFFICIENCY_8 > 60" | bc) -eq 1 ]; then
        echo -e "${YELLOW}✓ Good parallel efficiency (60-80%)${NC}"
    else
        echo -e "${YELLOW}⚠ Moderate parallel efficiency (<60%)${NC}"
    fi
fi

echo ""
echo "Benchmark complete."
