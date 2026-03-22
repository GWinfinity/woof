#!/bin/bash
#
# Demo script for linter performance tracking
# 演示逐linter耗时功能

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

WOOF_BIN="${PROJECT_ROOT}/target/release/woof"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="${PROJECT_ROOT}/target/debug/woof"
fi

echo "============================================"
echo "Woof Linter Performance Demo"
echo "============================================"
echo ""

if [ ! -f "$WOOF_BIN" ]; then
    echo "Error: woof binary not found"
    exit 1
fi

# Test file
TEST_FILE="${PROJECT_ROOT}/testdata/sample.go"

if [ ! -f "$TEST_FILE" ]; then
    echo "Creating test file..."
    cat > "$TEST_FILE" << 'EOF'
package main

import (
	"fmt"
	"os"
)

func main() {
	unused := "this is unused"
	_ = unused
	
	if true {
	}
	
	fmt.Println(os.Getenv("HOME"))
}

func longFunctionThatExceedsTheLineLengthLimitForDemonstrationPurposes() {}
EOF
fi

echo "1. Normal run (no logging)"
echo "----------------------------------------"
time $WOOF_BIN check "$TEST_FILE" 2>&1 | head -10
echo ""

echo "2. With GOLOGGING=perf (performance tracking)"
echo "----------------------------------------"
GOLOGGING=perf time $WOOF_BIN check "$TEST_FILE" 2>&1
echo ""

echo "3. With GOLOGGING=debug (verbose debug)"
echo "----------------------------------------"
GOLOGGING=debug time $WOOF_BIN check "$TEST_FILE" 2>&1
echo ""

echo "4. Multiple files (directory)"
echo "----------------------------------------"
GOLOGGING=perf time $WOOF_BIN check "${PROJECT_ROOT}/testdata" 2>&1 | tail -20
echo ""

echo "============================================"
echo "Demo complete!"
echo "============================================"
echo ""
echo "Usage examples:"
echo "  GOLOGGING=perf  woof check .    # Performance tracking"
echo "  GOLOGGING=debug woof check .    # Debug output"
echo "  GOLOGGING=trace woof check .    # Very verbose"
echo ""
