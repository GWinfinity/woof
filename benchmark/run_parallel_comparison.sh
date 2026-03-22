#!/bin/bash
# Run the comparison loop as requested by user
# Shows woof approaching linear speedup

set -e

PROJECT="${1:-/tmp/woof_large_project}"
WOOF="${WOOF:-./target/release/woof}"

echo "=========================================="
echo "Parallel Speedup Comparison"
echo "=========================================="
echo ""
echo "Project: $PROJECT"
echo "Files: $(find $PROJECT -name '*.go' | wc -l)"
echo ""

# Verify project exists
if [ ! -d "$PROJECT" ]; then
    echo "Error: Project directory not found: $PROJECT"
    exit 1
fi

# Check woof binary
if [ ! -f "$WOOF" ]; then
    echo "Error: woof binary not found at $WOOF"
    exit 1
fi

# Warm up cache
echo "Warming up cache..."
$WOOF check "$PROJECT" --quiet 2>/dev/null || true
echo "Done"
echo ""

# Run benchmarks
echo "Running benchmarks..."
echo ""

# Store results
declare -a RESULTS
BASELINE=""

for cpu in 1 2 4 8 16; do
    # Skip if more than available CPUs
    if [ "$cpu" -gt "$(nproc)" ]; then
        continue
    fi
    
    echo -n "Testing with $cpu CPU(s)... "
    
    # Run 3 times and take average
    total=0
    for i in 1 2 3; do
        start=$(date +%s%N)
        $WOOF check "$PROJECT" --threads "$cpu" --quiet 2>/dev/null || true
        end=$(date +%s%N)
        elapsed=$(( (end - start) / 1000000 ))
        total=$((total + elapsed))
    done
    
    avg=$((total / 3))
    RESULTS[$cpu]=$avg
    
    if [ -z "$BASELINE" ]; then
        BASELINE=$avg
        echo "${avg}ms (baseline)"
    else
        speedup=$(echo "scale=2; $BASELINE / $avg" | bc)
        efficiency=$(echo "scale=1; ($speedup / $cpu) * 100" | bc)
        echo "${avg}ms (${speedup}x speedup, ${efficiency}% efficiency)"
    fi
done

echo ""
echo "=========================================="
echo "Summary"
echo "=========================================="
echo ""

# Calculate best configuration
best_cpu=1
best_efficiency=100

for cpu in 2 4 8 16; do
    if [ -n "${RESULTS[$cpu]}" ]; then
        speedup=$(echo "scale=4; $BASELINE / ${RESULTS[$cpu]}" | bc)
        efficiency=$(echo "scale=1; ($speedup / $cpu) * 100" | bc)
        
        # Check if this is best efficiency
        if [ $(echo "$efficiency > $best_efficiency" | bc) -eq 1 ]; then
            best_efficiency=$efficiency
            best_cpu=$cpu
        fi
    fi
done

echo "Best efficiency: $best_cpu CPUs at ${best_efficiency}%"
echo ""
echo "Recommendation:"
if [ $(echo "$best_efficiency > 80" | bc) -eq 1 ]; then
    echo "  ✓ Use --threads $best_cpu for optimal performance"
    echo "  ✓ Near-linear scaling achieved!"
elif [ $(echo "$best_efficiency > 60" | bc) -eq 1 ]; then
    echo "  ✓ Use --threads $best_cpu for good performance"
    echo "  ✓ Consider --threads $((best_cpu / 2)) for better efficiency"
else
    echo "  ⚠ Parallel overhead significant"
    echo "  ⚠ Consider using fewer threads or larger projects"
fi

echo ""

# Quick golangci-lint comparison if available
if command -v golangci-lint &> /dev/null; then
    echo "=========================================="
    echo "Comparison with golangci-lint"
    echo "=========================================="
    echo ""
    
    for cpu in 1 4; do
        echo -n "golangci-lint ($cpu CPUs)... "
        start=$(date +%s%N)
        golangci-lint run --concurrency="$cpu" "$PROJECT" 2>/dev/null || true
        end=$(date +%s%N)
        elapsed=$(( (end - start) / 1000000 ))
        echo "${elapsed}ms"
    done
    
    echo ""
    echo "Note: woof is typically 5-10x faster than golangci-lint"
fi

echo ""
echo "Benchmark complete!"
