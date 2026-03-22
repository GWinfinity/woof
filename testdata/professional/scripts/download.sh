#!/bin/bash
#
# Download professional test dataset for woof
# 下载专业测试数据集

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROJECTS_DIR="$PROJECT_ROOT/projects"
DATASET_FILE="$PROJECT_ROOT/dataset.yaml"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Check dependencies
check_deps() {
    local deps=("git" "yq" "du" "wc")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            log_error "Missing dependency: $dep"
            if [ "$dep" = "yq" ]; then
                echo "  Install with: go install github.com/mikefarah/yq/v4@latest"
            fi
            exit 1
        fi
    done
}

# Parse dataset.yaml and extract project info
parse_dataset() {
    local name=$1
    local field=$2
    yq eval ".projects[] | select(.name == \"$name\").$field" "$DATASET_FILE"
}

# Clone or update a project
clone_project() {
    local name=$1
    local repo=$2
    local commit=$3
    local tag=$4
    
    local target_dir="$PROJECTS_DIR/$name"
    
    log_info "Processing $name..."
    
    if [ -d "$target_dir/.git" ]; then
        log_info "  Existing repository found, updating..."
        cd "$target_dir"
        
        # Fetch latest and checkout specific commit
        git fetch origin --depth=1 "$commit" 2>/dev/null || git fetch origin "$tag"
        git checkout "$commit" --quiet
        
        # Verify commit
        local current_commit=$(git rev-parse HEAD)
        if [ "$current_commit" != "$commit" ]; then
            log_error "  Commit mismatch! Expected: $commit, Got: $current_commit"
            return 1
        fi
        
        log_success "  Updated to $commit"
    else
        log_info "  Cloning from $repo..."
        mkdir -p "$target_dir"
        
        # Shallow clone with specific commit
        git clone --depth=1 --no-checkout "$repo" "$target_dir"
        cd "$target_dir"
        git fetch --depth=1 origin "$commit"
        git checkout "$commit" --quiet
        
        log_success "  Cloned at $commit"
    fi
    
    # Collect stats
    collect_stats "$name" "$target_dir"
}

# Collect project statistics
collect_stats() {
    local name=$1
    local dir=$2
    
    log_info "  Collecting statistics..."
    
    # Count Go files
    local go_files=$(find "$dir" -name "*.go" -not -path "*/vendor/*" -not -name "*_test.go" 2>/dev/null | wc -l)
    
    # Count CGO files
    local cgo_files=$(find "$dir" -name "*.go" -not -path "*/vendor/*" -exec grep -l "import \"C\"" {} \; 2>/dev/null | wc -l)
    
    # Count lines of code
    local lines=$(find "$dir" -name "*.go" -not -path "*/vendor/*" -not -name "*_test.go" -exec cat {} \; 2>/dev/null | wc -l)
    
    # Calculate size
    local size=$(du -sh "$dir" 2>/dev/null | cut -f1)
    
    # Save stats
    cat > "$PROJECTS_DIR/$name.stats.json" << EOF
{
  "name": "$name",
  "go_files": $go_files,
  "cgo_files": $cgo_files,
  "lines_of_code": $lines,
  "size": "$size",
  "collected_at": "$(date -Iseconds)"
}
EOF
    
    log_info "  Stats: $go_files Go files, $cgo_files CGO files, $lines lines, $size"
}

# Download all projects
download_all() {
    log_info "Reading dataset configuration..."
    
    local projects=$(yq eval '.projects[].name' "$DATASET_FILE")
    local total=$(echo "$projects" | wc -l)
    
    log_info "Found $total projects to download"
    echo ""
    
    mkdir -p "$PROJECTS_DIR"
    
    local i=1
    while IFS= read -r name; do
        log_info "[$i/$total] Processing $name..."
        
        local repo=$(parse_dataset "$name" "repo")
        local commit=$(parse_dataset "$name" "commit")
        local tag=$(parse_dataset "$name" "tag")
        
        clone_project "$name" "$repo" "$commit" "$tag"
        
        echo ""
        ((i++))
    done <<< "$projects"
    
    log_success "All projects downloaded successfully!"
}

# Download specific project
download_single() {
    local name=$1
    
    log_info "Downloading single project: $name"
    
    local repo=$(parse_dataset "$name" "repo")
    local commit=$(parse_dataset "$name" "commit")
    local tag=$(parse_dataset "$name" "tag")
    
    if [ -z "$repo" ]; then
        log_error "Project '$name' not found in dataset"
        exit 1
    fi
    
    clone_project "$name" "$repo" "$commit" "$tag"
}

# Verify all projects
verify_all() {
    log_info "Verifying all projects..."
    
    local failed=0
    local projects=$(yq eval '.projects[].name' "$DATASET_FILE")
    
    while IFS= read -r name; do
        local target_dir="$PROJECTS_DIR/$name"
        local expected_commit=$(parse_dataset "$name" "commit")
        
        if [ ! -d "$target_dir/.git" ]; then
            log_error "$name: Repository not found"
            ((failed++))
            continue
        fi
        
        cd "$target_dir"
        local actual_commit=$(git rev-parse HEAD)
        
        if [ "$actual_commit" != "$expected_commit" ]; then
            log_error "$name: Commit mismatch!"
            log_error "  Expected: $expected_commit"
            log_error "  Actual:   $actual_commit"
            ((failed++))
        else
            log_success "$name: Verified at $expected_commit"
        fi
    done <<< "$projects"
    
    if [ $failed -eq 0 ]; then
        log_success "All projects verified successfully!"
    else
        log_error "$failed project(s) failed verification"
        exit 1
    fi
}

# Show dataset info
show_info() {
    echo "Professional Test Dataset"
    echo "========================"
    echo ""
    
    local version=$(yq eval '.version' "$DATASET_FILE")
    local description=$(yq eval '.description' "$DATASET_FILE")
    
    echo "Version: $version"
    echo "Description: $description"
    echo ""
    
    echo "Projects:"
    yq eval '.projects[] | "  - \(.name): \(.tag) - \(.description) [\(.size)]"' "$DATASET_FILE"
    echo ""
    
    echo "Test Scenarios:"
    yq eval '.test_scenarios | keys | .[]' "$DATASET_FILE" | while read scenario; do
        local desc=$(yq eval ".test_scenarios.$scenario.description" "$DATASET_FILE")
        echo "  - $scenario: $desc"
    done
}

# Clean all projects
clean_all() {
    log_warn "This will remove all downloaded projects!"
    read -p "Are you sure? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$PROJECTS_DIR"
        log_success "Projects directory cleaned"
    else
        log_info "Cancelled"
    fi
}

# Usage
usage() {
    cat << EOF
Usage: $0 [COMMAND] [OPTIONS]

Commands:
  download [project]  Download all or specific project
  verify              Verify all projects at correct commits
  info                Show dataset information
  clean               Remove all downloaded projects
  stats               Show statistics for all projects

Examples:
  $0 download                    # Download all projects
  $0 download kubernetes         # Download only kubernetes
  $0 verify                      # Verify all projects
  $0 info                        # Show dataset info
  $0 stats                       # Show project statistics

EOF
}

# Show statistics
show_stats() {
    echo "Project Statistics"
    echo "=================="
    echo ""
    
    printf "%-15s %10s %10s %12s %8s\n" "Project" "Go Files" "CGO Files" "Lines" "Size"
    printf "%-15s %10s %10s %12s %8s\n" "---------------" "----------" "----------" "------------" "--------"
    
    for stats_file in "$PROJECTS_DIR"/*.stats.json; do
        if [ -f "$stats_file" ]; then
            local name=$(jq -r '.name' "$stats_file")
            local go_files=$(jq -r '.go_files' "$stats_file")
            local cgo_files=$(jq -r '.cgo_files' "$stats_file")
            local lines=$(jq -r '.lines_of_code' "$stats_file")
            local size=$(jq -r '.size' "$stats_file")
            
            printf "%-15s %10s %10s %12s %8s\n" "$name" "$go_files" "$cgo_files" "$lines" "$size"
        fi
    done
}

# Main
main() {
    check_deps
    
    case "${1:-}" in
        download)
            if [ -n "${2:-}" ]; then
                download_single "$2"
            else
                download_all
            fi
            ;;
        verify)
            verify_all
            ;;
        info)
            show_info
            ;;
        clean)
            clean_all
            ;;
        stats)
            show_stats
            ;;
        *)
            usage
            exit 1
            ;;
    esac
}

main "$@"
