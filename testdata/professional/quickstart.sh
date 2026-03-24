#!/bin/bash
#
# Quick start script for professional test dataset
# 

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "============================================"
echo "Woof Professional Test Dataset - Quick Start"
echo "============================================"
echo ""

# Check prerequisites
echo "Checking prerequisites..."

if ! command -v git &> /dev/null; then
    echo "❌ git not found. Please install git."
    exit 1
fi
echo "✅ git found"

if ! command -v yq &> /dev/null; then
    echo "⚠️  yq not found. Installing..."
    echo "   go install github.com/mikefarah/yq/v4@latest"
    exit 1
fi
echo "✅ yq found"

if ! command -v jq &> /dev/null; then
    echo "⚠️  jq not found. Please install jq."
    exit 1
fi
echo "✅ jq found"

echo ""

# Show dataset info
echo "Dataset Information:"
echo "--------------------"
./scripts/download.sh info
echo ""

# Download options
echo "Download Options:"
echo "-----------------"
echo "1) Download small project (consul, ~500MB) - Quick test"
echo "2) Download medium projects (consul, vault, prometheus) - Standard test"
echo "3) Download all projects (~10GB) - Full test suite"
echo "4) Skip download (projects already downloaded)"
echo ""

read -p "Select option (1-4): " choice

case $choice in
    1)
        echo ""
        echo "Downloading consul..."
        ./scripts/download.sh download consul
        ;;
    2)
        echo ""
        echo "Downloading medium projects..."
        ./scripts/download.sh download consul
        ./scripts/download.sh download vault
        ./scripts/download.sh download prometheus
        ;;
    3)
        echo ""
        echo "Downloading all projects (this will take a while)..."
        ./scripts/download.sh download
        ;;
    4)
        echo "Skipping download..."
        ;;
    *)
        echo "Invalid option"
        exit 1
        ;;
esac

echo ""
echo "============================================"
echo "Running Analysis"
echo "============================================"
echo ""

# Check woof binary
WOOF_BIN="../../target/release/woofmt"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="../../target/debug/woofmt"
fi

if [ ! -f "$WOOF_BIN" ]; then
    echo "❌ woof binary not found. Please build first:"
    echo "   cd ../.. && cargo build --release"
    exit 1
fi

echo "✅ woof binary found: $WOOF_BIN"
echo ""

# Analysis options
echo "Analysis Options:"
echo "-----------------"
echo "1) Analyze consul only - Quick test"
echo "2) Run cold_start scenario - Standard benchmark"
echo "3) Run all scenarios - Full benchmark"
echo "4) Skip analysis"
echo ""

read -p "Select option (1-4): " analysis_choice

case $analysis_choice in
    1)
        echo ""
        echo "Analyzing consul..."
        ./scripts/analyze.sh project consul
        ;;
    2)
        echo ""
        echo "Running cold_start scenario..."
        ./scripts/analyze.sh scenario cold_start
        ;;
    3)
        echo ""
        echo "Running all scenarios (this will take a while)..."
        ./scripts/analyze.sh all
        ;;
    4)
        echo "Skipping analysis..."
        ;;
esac

echo ""
echo "============================================"
echo "Results"
echo "============================================"
echo ""

if [ -d "results" ]; then
    echo "Results directory: ./results/"
    echo ""
    
    # Show project results
    for result_dir in results/*/; do
        if [ -f "$result_dir/metrics.json" ]; then
            project=$(basename "$result_dir")
            duration=$(jq -r '.duration_ms // "N/A"' "$result_dir/metrics.json")
            memory=$(jq -r '.memory_delta_mb // "N/A"' "$result_dir/metrics.json")
            issues=$(jq -r '.issues.total // "N/A"' "$result_dir/metrics.json")
            
            echo "Project: $project"
            echo "  Duration: ${duration}ms"
            echo "  Memory: ${memory}MB"
            echo "  Issues: $issues"
            echo ""
        fi
    done
    
    # Generate report
    echo "Generate detailed report?"
    read -p "[y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ./scripts/analyze.sh report
        echo ""
        echo "Report generated: results/report.md"
    fi
else
    echo "No results yet. Run analysis first."
fi

echo ""
echo "============================================"
echo "Quick Start Complete!"
echo "============================================"
echo ""
echo "Next steps:"
echo "  - View detailed results: ls -la results/"
echo "  - Run specific project: ./scripts/analyze.sh project <name>"
echo "  - Run specific scenario: ./scripts/analyze.sh scenario <name>"
echo "  - View dataset info: ./scripts/download.sh info"
echo ""
echo "For more information, see README.md"
