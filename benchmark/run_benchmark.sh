#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Woof vs golangci-lint Benchmark${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCENARIOS_DIR="$SCRIPT_DIR/scenarios"
RESULTS_DIR="$SCRIPT_DIR/results"

mkdir -p "$RESULTS_DIR"

# Check if woof is available
WOOF_BIN="${SCRIPT_DIR}/../target/release/woof"
if [ ! -f "$WOOF_BIN" ]; then
    echo -e "${YELLOW}Warning: woof release binary not found, using debug build${NC}"
    WOOF_BIN="${SCRIPT_DIR}/../target/debug/woof"
fi

# Check if golangci-lint is available
if ! command -v golangci-lint &> /dev/null; then
    echo -e "${YELLOW}Warning: golangci-lint not found in PATH${NC}"
    echo -e "${YELLOW}Install with: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest${NC}"
    USE_GOLANGCI=false
else
    USE_GOLANGCI=true
    GOLANGCI_VERSION=$(golangci-lint --version 2>/dev/null | head -1)
    echo -e "golangci-lint: ${GREEN}$GOLANGCI_VERSION${NC}"
fi

echo -e "woof: ${GREEN}$($WOOF_BIN --version)${NC}"
echo ""

# Create go.mod for scenarios
cd "$SCENARIOS_DIR"
if [ ! -f "go.mod" ]; then
    go mod init benchmark/scenarios 2>/dev/null || true
fi

# Function to run with timing
run_with_timing() {
    local name="$1"
    local cmd="$2"
    local output_file="$3"
    
    echo -e "${BLUE}Running: $name${NC}"
    
    # Run 3 times and take average
    local total_time=0
    local runs=3
    
    for i in $(seq 1 $runs); do
        start_time=$(date +%s%N)
        eval "$cmd" > "$output_file" 2>&1 || true
        end_time=$(date +%s%N)
        
        # Calculate time in milliseconds
        elapsed=$(( (end_time - start_time) / 1000000 ))
        total_time=$((total_time + elapsed))
    done
    
    avg_time=$((total_time / runs))
    echo "$avg_time"
}

# Individual scenario test
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Individual Scenario Tests${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

echo "Scenario | Woof Time (ms) | Woof Issues | golangci-lint Time (ms) | golangci-lint Issues"
echo "---------|----------------|-------------|-------------------------|----------------------"

TOTAL_WOOF_TIME=0
TOTAL_GOLANGCI_TIME=0
TOTAL_WOOF_ISSUES=0
TOTAL_GOLANGCI_ISSUES=0

for scenario in "$SCENARIOS_DIR"/*.go; do
    basename=$(basename "$scenario" .go)
    
    # Run woof
    woof_time=$(run_with_timing "woof: $basename" "$WOOF_BIN check '$scenario'" "$RESULTS_DIR/${basename}_woof.txt")
    TOTAL_WOOF_TIME=$((TOTAL_WOOF_TIME + woof_time))
    
    # Count woof issues
    woof_issues=$(grep -c "warning\|error" "$RESULTS_DIR/${basename}_woof.txt" 2>/dev/null || echo "0")
    TOTAL_WOOF_ISSUES=$((TOTAL_WOOF_ISSUES + woof_issues))
    
    if [ "$USE_GOLANGCI" = true ]; then
        # Run golangci-lint
        golangci_time=$(run_with_timing "golangci-lint: $basename" "golangci-lint run --timeout 10s --no-config '$scenario'" "$RESULTS_DIR/${basename}_golangci.txt")
        TOTAL_GOLANGCI_TIME=$((TOTAL_GOLANGCI_TIME + golangci_time))
        
        # Count golangci issues
        golangci_issues=$(wc -l < "$RESULTS_DIR/${basename}_golangci.txt" 2>/dev/null || echo "0")
        golangci_issues=$(echo "$golangci_issues" | tr -d ' ')
        TOTAL_GOLANGCI_ISSUES=$((TOTAL_GOLANGCI_ISSUES + golangci_issues))
        
        printf "%-8s | %14d | %11d | %23d | %20d\n" "$basename" "$woof_time" "$woof_issues" "$golangci_time" "$golangci_issues"
    else
        printf "%-8s | %14d | %11d | %23s | %20s\n" "$basename" "$woof_time" "$woof_issues" "N/A" "N/A"
    fi
done

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Batch Processing Test (All Scenarios)${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

# Test batch processing
echo -e "${YELLOW}Testing batch processing of all scenarios...${NC}"

# Woof batch
woof_batch_time=$(run_with_timing "woof batch" "$WOOF_BIN check '$SCENARIOS_DIR'" "$RESULTS_DIR/batch_woof.txt")
woof_batch_issues=$(grep -c "warning\|error" "$RESULTS_DIR/batch_woof.txt" 2>/dev/null || echo "0")

echo -e "Woof batch: ${woof_batch_time}ms, $woof_batch_issues issues detected"

if [ "$USE_GOLANGCI" = true ]; then
    # golangci-lint batch
    golangci_batch_time=$(run_with_timing "golangci-lint batch" "golangci-lint run --timeout 30s --no-config '$SCENARIOS_DIR'" "$RESULTS_DIR/batch_golangci.txt")
    golangci_batch_issues=$(wc -l < "$RESULTS_DIR/batch_golangci.txt" 2>/dev/null || echo "0")
    golangci_batch_issues=$(echo "$golangci_batch_issues" | tr -d ' ')
    
    echo -e "golangci-lint batch: ${golangci_batch_time}ms, $golangci_batch_issues issues detected"
    
    # Speed comparison
    echo ""
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}  Summary${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo ""
    
    speedup=$(echo "scale=2; $golangci_batch_time / $woof_batch_time" | bc 2>/dev/null || echo "N/A")
    
    echo -e "Total individual scenarios time:"
    echo -e "  Woof: ${TOTAL_WOOF_TIME}ms"
    echo -e "  golangci-lint: ${TOTAL_GOLANGCI_TIME}ms"
    echo ""
    echo -e "Batch processing time:"
    echo -e "  Woof: ${woof_batch_time}ms"
    echo -e "  golangci-lint: ${golangci_batch_time}ms"
    
    if [ "$speedup" != "N/A" ]; then
        echo ""
        echo -e "Woof is ${GREEN}${speedup}x${NC} faster than golangci-lint (batch)"
    fi
    
    echo ""
    echo -e "Issues detected (batch):"
    echo -e "  Woof: ${woof_batch_issues}"
    echo -e "  golangci-lint: ${golangci_batch_issues}"
fi

# Detailed issue comparison for specific scenarios
echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Detailed Issue Comparison${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

for scenario in 01_unused_import 06_unchecked_error 11_line_too_long 21_race_condition 30_exported_missing_doc; do
    echo -e "${YELLOW}=== $scenario ===${NC}"
    
    if [ -f "$RESULTS_DIR/${scenario}_woof.txt" ]; then
        echo -e "${GREEN}Woof:${NC}"
        cat "$RESULTS_DIR/${scenario}_woof.txt" | head -5
    fi
    
    if [ "$USE_GOLANGCI" = true ] && [ -f "$RESULTS_DIR/${scenario}_golangci.txt" ]; then
        echo -e "${GREEN}golangci-lint:${NC}"
        cat "$RESULTS_DIR/${scenario}_golangci.txt" | head -5
    fi
    echo ""
done

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Benchmark Complete!${NC}"
echo -e "${BLUE}  Results saved to: $RESULTS_DIR${NC}"
echo -e "${BLUE}============================================${NC}"
