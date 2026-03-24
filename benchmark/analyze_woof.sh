#!/bin/bash

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCENARIOS_DIR="$SCRIPT_DIR/scenarios"
RESULTS_DIR="$SCRIPT_DIR/results"

mkdir -p "$RESULTS_DIR"

WOOF_BIN="${SCRIPT_DIR}/../target/release/woofmt"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="${SCRIPT_DIR}/../target/debug/woofmt"
fi

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Woof Analysis on 30 Test Scenarios${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

# Expected issues per scenario (based on typical golangci-lint behavior)
declare -A EXPECTED_ISSUES
declare -A EXPECTED_TOOLS

EXPECTED_ISSUES=(
    ["01_unused_import"]=2
    ["02_unused_variable"]=1
    ["03_shadow_variable"]=1
    ["04_unreachable_code"]=1
    ["05_empty_body"]=2
    ["06_unchecked_error"]=2
    ["07_ignored_error"]=1
    ["08_error_not_last_return"]=1
    ["09_nil_error_return"]=1
    ["10_panic_error"]=1
    ["11_line_too_long"]=1
    ["12_mixed_tabs_spaces"]=1
    ["13_trailing_whitespace"]=2
    ["14_snake_case"]=2
    ["15_stuttering"]=2
    ["16_high_cyclomatic"]=1
    ["17_deep_nesting"]=1
    ["18_long_function"]=1
    ["19_many_parameters"]=1
    ["20_long_parameter_list"]=1
    ["21_race_condition"]=1
    ["22_channel_close"]=0
    ["23_goroutine_leak"]=1
    ["24_mutex_copy"]=1
    ["25_defer_in_loop"]=1
    ["26_redundant_type"]=1
    ["27_simplify_slice"]=1
    ["28_naked_return"]=1
    ["29_deprecated_ioutil"]=1
    ["30_exported_missing_doc"]=4
)

EXPECTED_TOOLS=(
    ["01_unused_import"]="gosimple, unused"
    ["02_unused_variable"]="govet, ineffassign"
    ["03_shadow_variable"]="govet (shadow)"
    ["04_unreachable_code"]="govet, staticcheck"
    ["05_empty_body"]="staticcheck (SA9003)"
    ["06_unchecked_error"]="errcheck, gosec"
    ["07_ignored_error"]="errcheck, govet"
    ["08_error_not_last_return"]="stylecheck (ST1008)"
    ["09_nil_error_return"]="nilerr"
    ["10_panic_error"]="forbidigo"
    ["11_line_too_long"]="lll"
    ["12_mixed_tabs_spaces"]="gofmt"
    ["13_trailing_whitespace"]="whitespace"
    ["14_snake_case"]="golint, revive"
    ["15_stuttering"]="golint, revive"
    ["16_high_cyclomatic"]="gocyclo"
    ["17_deep_nesting"]="nesting"
    ["18_long_function"]="funlen"
    ["19_many_parameters"]="gofumpt, gocritic"
    ["20_long_parameter_list"]="gocritic"
    ["21_race_condition"]="go vet -race, staticcheck"
    ["22_channel_close"]="(valid code)"
    ["23_goroutine_leak"]="goleak"
    ["24_mutex_copy"]="govet, copylocks"
    ["25_defer_in_loop"]="gocritic"
    ["26_redundant_type"]="gosimple (S1009)"
    ["27_simplify_slice"]="gosimple (S1011)"
    ["28_naked_return"]="nakedret"
    ["29_deprecated_ioutil"]="staticcheck (SA1019)"
    ["30_exported_missing_doc"]="golint, revive"
)

echo "Testing woof on each scenario..."
echo ""

TOTAL_TIME=0
TOTAL_DETECTED=0
TOTAL_EXPECTED=0

echo "Scenario | Woof Time (ms) | Woof Issues | Expected Issues | Detection Rate | Tools (golangci-lint)"
echo "---------|----------------|-------------|-----------------|----------------|----------------------"

for i in $(seq -w 1 30); do
    scenario="${i}_"
    file=$(ls "$SCENARIOS_DIR"/${scenario}*.go 2>/dev/null | head -1)
    
    if [ ! -f "$file" ]; then
        continue
    fi
    
    basename=$(basename "$file" .go)
    
    # Time the execution
    start=$(date +%s%N)
    $WOOF_BIN check "$file" > "$RESULTS_DIR/${basename}_woof.txt" 2>&1 || true
    end=$(date +%s%N)
    
    elapsed=$(( (end - start) / 1000000 ))
    TOTAL_TIME=$((TOTAL_TIME + elapsed))
    
    # Count issues
    issues=$(grep -cE "warning|error|info" "$RESULTS_DIR/${basename}_woof.txt" 2>/dev/null || echo "0")
    TOTAL_DETECTED=$((TOTAL_DETECTED + issues))
    
    expected=${EXPECTED_ISSUES[$basename]:-0}
    TOTAL_EXPECTED=$((TOTAL_EXPECTED + expected))
    
    # Calculate detection rate
    if [ "$expected" -gt 0 ]; then
        rate=$(( issues * 100 / expected ))
    elif [ "$issues" -eq 0 ]; then
        rate=100
    else
        rate=0
    fi
    
    tools=${EXPECTED_TOOLS[$basename]:-""}
    
    printf "%-8s | %14d | %11d | %15d | %13d%% | %s\n" "$basename" "$elapsed" "$issues" "$expected" "$rate" "$tools"
done

echo ""
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Batch Performance${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

# Batch test
start=$(date +%s%N)
$WOOF_BIN check "$SCENARIOS_DIR" > "$RESULTS_DIR/batch_woof.txt" 2>&1 || true
end=$(date +%s%N)
batch_time=$(( (end - start) / 1000000 ))

batch_issues=$(grep -cE "warning|error|info" "$RESULTS_DIR/batch_woof.txt" 2>/dev/null || echo "0")

echo -e "Individual total time: ${TOTAL_TIME}ms"
echo -e "Batch time: ${batch_time}ms"
echo -e "Issues detected (batch): ${batch_issues}"
echo ""

# Summary
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Summary${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

if [ "$TOTAL_EXPECTED" -gt 0 ]; then
    overall_rate=$(( TOTAL_DETECTED * 100 / TOTAL_EXPECTED ))
    echo -e "Overall detection rate: ${overall_rate}% ($TOTAL_DETECTED / $TOTAL_EXPECTED expected)"
fi

echo ""
echo -e "${YELLOW}Note: golangci-lint runs 20+ tools and typically takes 5-30 seconds${NC}"
echo -e "${YELLOW}      for the same scenarios, while woof takes <100ms.${NC}"
echo ""

# Show sample detections
echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Sample Detections${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

for scenario in 01_unused_import 06_unchecked_error 11_line_too_long 21_race_condition 27_simplify_slice 30_exported_missing_doc; do
    echo -e "${YELLOW}=== $scenario ===${NC}"
    cat "$RESULTS_DIR/${scenario}_woof.txt" 2>/dev/null || echo "(no output)"
    echo ""
done

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}  Full results saved to: $RESULTS_DIR${NC}"
echo -e "${BLUE}============================================${NC}"
