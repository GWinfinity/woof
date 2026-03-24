#!/bin/bash
#
# Compare woof with golangci-lint on professional dataset
# 对比woof和golangci-lint在专业数据集上的表现

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECTS_DIR="$PROJECT_ROOT/projects"
RESULTS_DIR="$PROJECT_ROOT/results/comparison"

WOOF_BIN="${PROJECT_ROOT}/../../target/release/woofmt"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="${PROJECT_ROOT}/../../target/debug/woofmt"
fi

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check tools
check_tools() {
    if [ ! -f "$WOOF_BIN" ]; then
        log_error "woof binary not found"
        exit 1
    fi
    
    if ! command -v golangci-lint &> /dev/null; then
        log_warn "golangci-lint not found, installing..."
        go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
    fi
    
    log_info "woof: $($WOOF_BIN --version)"
    log_info "golangci-lint: $(golangci-lint --version)"
}

# Run comparison on a project
compare_project() {
    local name=$1
    local project_dir="$PROJECTS_DIR/$name"
    
    if [ ! -d "$project_dir" ]; then
        log_error "Project $name not found"
        return 1
    fi
    
    log_info "Comparing on $name..."
    mkdir -p "$RESULTS_DIR/$name"
    
    # Get file count
    local file_count=$(find "$project_dir" -name "*.go" -not -path "*/vendor/*" | wc -l)
    log_info "  Files to analyze: $file_count"
    
    # Run woof
    log_info "  Running woof..."
    local woof_start=$(date +%s%N)
    
    if ! $WOOF_BIN check "$project_dir" \
        --format json \
        > "$RESULTS_DIR/$name/woof.json" 2> "$RESULTS_DIR/$name/woof.err"; then
        log_warn "    woof exited with errors"
    fi
    
    local woof_end=$(date +%s%N)
    local woof_time=$(( (woof_end - woof_start) / 1000000 ))
    
    # Count woof issues
    local woof_issues=$(jq '[.[] | select(.severity != null)] | length' "$RESULTS_DIR/$name/woof.json" 2>/dev/null || echo 0)
    
    # Run golangci-lint
    log_info "  Running golangci-lint..."
    local golangci_start=$(date +%s%N)
    
    if ! golangci-lint run --timeout 30m --out-format json \
        "$project_dir/..." \
        > "$RESULTS_DIR/$name/golangci.json" 2> "$RESULTS_DIR/$name/golangci.err"; then
        log_warn "    golangci-lint found issues or exited with errors"
    fi
    
    local golangci_end=$(date +%s%N)
    local golangci_time=$(( (golangci_end - golangci_start) / 1000000 ))
    
    # Count golangci-lint issues
    local golangci_issues=$(jq '.Issues | length' "$RESULTS_DIR/$name/golangci.json" 2>/dev/null || echo 0)
    
    # Calculate speedup
    local speedup=$(echo "scale=2; $golangci_time / $woof_time" | bc 2>/dev/null || echo "N/A")
    
    # Save comparison
    cat > "$RESULTS_DIR/$name/comparison.json" << EOF
{
  "project": "$name",
  "files": $file_count,
  "woof": {
    "time_ms": $woof_time,
    "issues": $woof_issues
  },
  "golangci": {
    "time_ms": $golangci_time,
    "issues": $golangci_issues
  },
  "speedup": $speedup
}
EOF
    
    # Print results
    echo ""
    echo -e "${CYAN}Results for $name:${NC}"
    printf "  %-20s %10s %10s %10s\n" "Tool" "Time" "Issues" ""
    printf "  %-20s %10s %10s %10s\n" "----" "----" "------" ""
    printf "  %-20s %8dms %10d %10s\n" "woof" "$woof_time" "$woof_issues" ""
    printf "  %-20s %8dms %10d %10s\n" "golangci-lint" "$golangci_time" "$golangci_issues" ""
    printf "  %-20s %10s\n" "Speedup" "${speedup}x"
    echo ""
}

# Generate comparison report
generate_report() {
    log_info "Generating comparison report..."
    
    local report_file="$RESULTS_DIR/report.md"
    
    cat > "$report_file" << 'EOF'
# Woof vs golangci-lint Comparison Report

Generated: $(date)

## Summary

| Project | Files | woof | golangci-lint | Speedup | Issues (woof/gci) |
|---------|-------|------|---------------|---------|-------------------|
EOF

    for comp_file in "$RESULTS_DIR"/*/comparison.json; do
        if [ -f "$comp_file" ]; then
            local project=$(jq -r '.project' "$comp_file")
            local files=$(jq '.files' "$comp_file")
            local woof_time=$(jq '.woof.time_ms' "$comp_file")
            local golangci_time=$(jq '.golangci.time_ms' "$comp_file")
            local speedup=$(jq '.speedup' "$comp_file")
            local woof_issues=$(jq '.woof.issues' "$comp_file")
            local golangci_issues=$(jq '.golangci.issues' "$comp_file")
            
            printf "| %s | %s | %dms | %dms | %.1fx | %d/%d |\n" \
                "$project" "$files" "$woof_time" "$golangci_time" \
                "$speedup" "$woof_issues" "$golangci_issues" >> "$report_file"
        fi
    done
    
    # Calculate averages
    local total_speedup=0
    local count=0
    
    for comp_file in "$RESULTS_DIR"/*/comparison.json; do
        if [ -f "$comp_file" ]; then
            local speedup=$(jq '.speedup' "$comp_file")
            total_speedup=$(echo "$total_speedup + $speedup" | bc)
            ((count++))
        fi
    done
    
    local avg_speedup=$(echo "scale=2; $total_speedup / $count" | bc)
    
    cat >> "$report_file" << EOF

## Statistics

- **Average Speedup**: ${avg_speedup}x
- **Projects tested**: $count

## Conclusion

woof is on average ${avg_speedup}x faster than golangci-lint.

EOF
    
    log_success "Report generated: $report_file"
}

# Run comparison on all projects
compare_all() {
    log_info "Running full comparison..."
    
    local projects=("consul" "vault" "prometheus")
    
    for project in "${projects[@]}"; do
        if [ -d "$PROJECTS_DIR/$project" ]; then
            compare_project "$project"
        else
            log_warn "Skipping $project (not downloaded)"
        fi
    done
    
    generate_report
}

# Usage
usage() {
    cat << EOF
Usage: $0 [COMMAND] [OPTIONS]

Commands:
  project <name>    Compare on specific project
  all               Compare on all available projects
  report            Generate comparison report

Examples:
  $0 project consul           # Compare on consul only
  $0 all                      # Compare on all projects
  $0 report                   # Generate report from existing results

EOF
}

# Main
main() {
    check_tools
    mkdir -p "$RESULTS_DIR"
    
    case "${1:-}" in
        project)
            if [ -z "${2:-}" ]; then
                log_error "Please specify a project name"
                exit 1
            fi
            compare_project "$2"
            ;;
        all)
            compare_all
            ;;
        report)
            generate_report
            ;;
        *)
            usage
            exit 1
            ;;
    esac
}

main "$@"
