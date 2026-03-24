#!/bin/bash
#
# Run woof analysis on professional test dataset
# 在专业测试数据集上运行woof分析

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECTS_DIR="$PROJECT_ROOT/projects"
RESULTS_DIR="$PROJECT_ROOT/results"
DATASET_FILE="$PROJECT_ROOT/dataset.yaml"

WOOF_BIN="${PROJECT_ROOT}/../../target/release/woofmt"
if [ ! -f "$WOOF_BIN" ]; then
    WOOF_BIN="${PROJECT_ROOT}/../../target/debug/woofmt"
fi

# Colors
RED='\033[0;31m'
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

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Ensure woof binary exists
check_woof() {
    if [ ! -f "$WOOF_BIN" ]; then
        log_error "woof binary not found at $WOOF_BIN"
        log_info "Please build woof first: cargo build --release"
        exit 1
    fi
    
    local version=$($WOOF_BIN --version 2>/dev/null || echo "unknown")
    log_info "Using woof: $version"
}

# Run analysis on a single project
analyze_project() {
    local name=$1
    local project_dir="$PROJECTS_DIR/$name"
    local result_dir="$RESULTS_DIR/$name"
    
    if [ ! -d "$project_dir" ]; then
        log_error "Project $name not found at $project_dir"
        return 1
    fi
    
    log_info "Analyzing $name..."
    mkdir -p "$result_dir"
    
    # Get project characteristics
    local cgo=$(yq eval ".projects[] | select(.name == \"$name\").cgo" "$DATASET_FILE")
    local reflection=$(yq eval ".projects[] | select(.name == \"$name\").reflection" "$DATASET_FILE")
    
    # Record system info
    cat > "$result_dir/system_info.json" << EOF
{
  "timestamp": "$(date -Iseconds)",
  "woof_version": "$($WOOF_BIN --version)",
  "go_version": "$(go version 2>/dev/null || echo 'N/A')",
  "cpus": $(nproc),
  "memory_mb": $(free -m 2>/dev/null | awk '/^Mem:/{print $2}' || echo '0'),
  "cgo_enabled": "$CGO_ENABLED"
}
EOF
    
    # Run analysis with memory profiling
    log_info "  Running full check..."
    
    local start_time=$(date +%s%N)
    local start_mem=$(get_memory_usage)
    
    # Run woof
    if ! $WOOF_BIN check "$project_dir" \
        --format json \
        --threads $(nproc) \
        > "$result_dir/diagnostics.json" 2> "$result_dir/stderr.log"; then
        log_warn "  woof exited with errors (see $result_dir/stderr.log)"
    fi
    
    local end_time=$(date +%s%N)
    local end_mem=$(get_memory_usage)
    
    # Calculate metrics
    local duration_ms=$(( (end_time - start_time) / 1000000 ))
    local mem_delta=$(( end_mem - start_mem ))
    
    # Count issues
    local errors=$(jq '[.[] | select(.severity == "error")] | length' "$result_dir/diagnostics.json" 2>/dev/null || echo 0)
    local warnings=$(jq '[.[] | select(.severity == "warning")] | length' "$result_dir/diagnostics.json" 2>/dev/null || echo 0)
    local infos=$(jq '[.[] | select(.severity == "info")] | length' "$result_dir/diagnostics.json" 2>/dev/null || echo 0)
    
    # Get file count
    local file_count=$(find "$project_dir" -name "*.go" -not -path "*/vendor/*" | wc -l)
    
    # Save metrics
    cat > "$result_dir/metrics.json" << EOF
{
  "project": "$name",
  "duration_ms": $duration_ms,
  "memory_delta_mb": $mem_delta,
  "files_checked": $file_count,
  "issues": {
    "error": $errors,
    "warning": $warnings,
    "info": $infos,
    "total": $((errors + warnings + infos))
  },
  "per_file_ms": $(( duration_ms / (file_count > 0 ? file_count : 1) )),
  "has_cgo": $cgo,
  "has_reflection": $reflection
}
EOF
    
    log_success "  Completed in ${duration_ms}ms, found $((errors + warnings + infos)) issues"
    log_info "    Memory: ${mem_delta}MB, Files: $file_count"
}

# Get current memory usage in MB
get_memory_usage() {
    if [ -f /proc/self/status ]; then
        grep VmRSS /proc/self/status | awk '{print $2 / 1024}' | cut -d. -f1
    else
        echo 0
    fi
}

# Run specific test scenario
run_scenario() {
    local scenario=$1
    
    log_info "Running scenario: $scenario"
    
    local projects=$(yq eval ".test_scenarios.$scenario.projects | .[]" "$DATASET_FILE")
    local iterations=$(yq eval ".test_scenarios.$scenario.iterations" "$DATASET_FILE")
    
    log_info "Projects: $(echo $projects | wc -w)"
    log_info "Iterations: $iterations"
    
    local scenario_dir="$RESULTS_DIR/scenarios/$scenario"
    mkdir -p "$scenario_dir"
    
    for ((i=1; i<=iterations; i++)); do
        log_info "Iteration $i/$iterations"
        
        while IFS= read -r project; do
            analyze_project "$project"
        done <<< "$projects"
    done
    
    # Aggregate results
    aggregate_scenario "$scenario"
}

# Aggregate scenario results
aggregate_scenario() {
    local scenario=$1
    local scenario_dir="$RESULTS_DIR/scenarios/$scenario"
    
    log_info "Aggregating results for $scenario..."
    
    # Find all metrics.json files
    local metrics_files=$(find "$RESULTS_DIR" -name "metrics.json" -path "*/$scenario/*" 2>/dev/null)
    
    if [ -z "$metrics_files" ]; then
        log_warn "No metrics found for scenario $scenario"
        return
    fi
    
    # Calculate averages
    local total_duration=0
    local total_memory=0
    local total_issues=0
    local count=0
    
    for file in $metrics_files; do
        local duration=$(jq '.duration_ms' "$file")
        local memory=$(jq '.memory_delta_mb' "$file")
        local issues=$(jq '.issues.total' "$file")
        
        total_duration=$((total_duration + duration))
        total_memory=$((total_memory + memory))
        total_issues=$((total_issues + issues))
        ((count++))
    done
    
    cat > "$scenario_dir/summary.json" << EOF
{
  "scenario": "$scenario",
  "iterations": $count,
  "avg_duration_ms": $((total_duration / count)),
  "avg_memory_mb": $((total_memory / count)),
  "avg_issues": $((total_issues / count)),
  "total_issues": $total_issues
}
EOF
    
    log_success "Summary saved to $scenario_dir/summary.json"
}

# Run all scenarios
run_all_scenarios() {
    log_info "Running all test scenarios..."
    
    local scenarios=$(yq eval '.test_scenarios | keys | .[]' "$DATASET_FILE")
    
    while IFS= read -r scenario; do
        run_scenario "$scenario"
        echo ""
    done <<< "$scenarios"
}

# Generate comparison report
generate_report() {
    log_info "Generating analysis report..."
    
    local report_file="$RESULTS_DIR/report.md"
    
    cat > "$report_file" << 'EOF'
# Woof Professional Dataset Analysis Report

Generated: $(date)

## Summary

EOF

    # Add project results
    echo "## Project Results" >> "$report_file"
    echo "" >> "$report_file"
    echo "| Project | Files | Duration | Memory | Issues |" >> "$report_file"
    echo "|---------|-------|----------|--------|--------|" >> "$report_file"
    
    for metrics_file in "$RESULTS_DIR"/*/metrics.json; do
        if [ -f "$metrics_file" ]; then
            local project=$(jq -r '.project' "$metrics_file")
            local files=$(jq '.files_checked' "$metrics_file")
            local duration=$(jq '.duration_ms' "$metrics_file")
            local memory=$(jq '.memory_delta_mb' "$metrics_file")
            local issues=$(jq '.issues.total' "$metrics_file")
            
            printf "| %s | %s | %sms | %sMB | %s |\n" \
                "$project" "$files" "$duration" "$memory" "$issues" >> "$report_file"
        fi
    done
    
    echo "" >> "$report_file"
    echo "## Detailed Results" >> "$report_file"
    echo "" >> "$report_file"
    echo "See individual project directories in results/" >> "$report_file"
    
    log_success "Report generated: $report_file"
}

# Clean results
clean_results() {
    log_warn "This will remove all analysis results!"
    read -p "Are you sure? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$RESULTS_DIR"
        log_success "Results cleaned"
    fi
}

# Usage
usage() {
    cat << EOF
Usage: $0 [COMMAND] [OPTIONS]

Commands:
  project <name>       Analyze specific project
  scenario <name>      Run specific test scenario
  all                  Run all test scenarios
  report               Generate analysis report
  clean                Clean all results
  list                 List available projects and scenarios

Examples:
  $0 project kubernetes          # Analyze kubernetes only
  $0 scenario cold_start         # Run cold start scenario
  $0 all                         # Run all scenarios
  $0 report                      # Generate report

EOF
}

# List available items
list_items() {
    echo "Available Projects:"
    yq eval '.projects[] | "  - \(.name): \(.description)"' "$DATASET_FILE"
    echo ""
    echo "Available Scenarios:"
    yq eval '.test_scenarios | keys | .[]' "$DATASET_FILE" | while read scenario; do
        local desc=$(yq eval ".test_scenarios.$scenario.description" "$DATASET_FILE")
        echo "  - $scenario: $desc"
    done
}

# Main
main() {
    check_woof
    mkdir -p "$RESULTS_DIR"
    
    case "${1:-}" in
        project)
            if [ -z "${2:-}" ]; then
                log_error "Please specify a project name"
                exit 1
            fi
            analyze_project "$2"
            ;;
        scenario)
            if [ -z "${2:-}" ]; then
                log_error "Please specify a scenario name"
                exit 1
            fi
            run_scenario "$2"
            ;;
        all)
            run_all_scenarios
            ;;
        report)
            generate_report
            ;;
        clean)
            clean_results
            ;;
        list)
            list_items
            ;;
        *)
            usage
            exit 1
            ;;
    esac
}

main "$@"
