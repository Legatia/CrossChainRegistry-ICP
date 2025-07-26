#!/bin/bash

# Master test runner for CrossChain Registry
# This script orchestrates all testing activities

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
TEST_SCRIPT="$SCRIPT_DIR/test_backend.sh"
DATA_GENERATOR="$SCRIPT_DIR/test_data_generator.sh"
LOG_DIR="$SCRIPT_DIR/logs"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Logging functions
log_header() {
    echo -e "${CYAN}======================================${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}======================================${NC}"
}

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Setup logging
setup_logging() {
    mkdir -p "$LOG_DIR"
    exec 1> >(tee -a "$LOG_DIR/test_run_$TIMESTAMP.log")
    exec 2> >(tee -a "$LOG_DIR/test_error_$TIMESTAMP.log" >&2)
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check dfx
    if ! command -v dfx &> /dev/null; then
        log_error "dfx is not installed. Please install DFINITY SDK first."
        exit 1
    fi
    
    # Check if dfx is running
    if ! dfx ping > /dev/null 2>&1; then
        log_warning "dfx is not running. Starting dfx..."
        dfx start --background --clean
        sleep 5
    fi
    
    # Check if scripts exist
    if [[ ! -f "$TEST_SCRIPT" ]]; then
        log_error "Test script not found: $TEST_SCRIPT"
        exit 1
    fi
    
    if [[ ! -f "$DATA_GENERATOR" ]]; then
        log_error "Data generator script not found: $DATA_GENERATOR"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

# Deploy canisters
deploy_canisters() {
    log_info "Deploying canisters..."
    
    cd "$PROJECT_ROOT"
    if dfx deploy > "$LOG_DIR/deploy_$TIMESTAMP.log" 2>&1; then
        log_success "Canisters deployed successfully"
    else
        log_error "Failed to deploy canisters. Check log: $LOG_DIR/deploy_$TIMESTAMP.log"
        exit 1
    fi
    cd "$SCRIPT_DIR"
}

# Run test suite
run_test_suite() {
    local test_type="$1"
    local log_file="$LOG_DIR/${test_type}_tests_$TIMESTAMP.log"
    
    log_info "Running $test_type tests..."
    
    if "$TEST_SCRIPT" "$test_type" > "$log_file" 2>&1; then
        log_success "$test_type tests completed successfully"
        
        # Extract test summary from log
        local total_tests=$(grep "Total Tests:" "$log_file" | grep -o '[0-9]*' | head -1)
        local passed_tests=$(grep "Passed:" "$log_file" | grep -o '[0-9]*' | head -1)
        local failed_tests=$(grep "Failed:" "$log_file" | grep -o '[0-9]*' | head -1)
        
        if [[ -n "$total_tests" && -n "$passed_tests" && -n "$failed_tests" ]]; then
            echo -e "  Total: $total_tests, Passed: ${GREEN}$passed_tests${NC}, Failed: ${RED}$failed_tests${NC}"
        fi
        
        return 0
    else
        log_error "$test_type tests failed. Check log: $log_file"
        return 1
    fi
}

# Generate test report
generate_test_report() {
    local report_file="$LOG_DIR/test_report_$TIMESTAMP.html"
    
    log_info "Generating test report..."
    
    cat > "$report_file" << EOF
<!DOCTYPE html>
<html>
<head>
    <title>CrossChain Registry Test Report - $TIMESTAMP</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .header { background-color: #f0f0f0; padding: 20px; border-radius: 5px; }
        .test-section { margin: 20px 0; padding: 15px; border: 1px solid #ddd; border-radius: 5px; }
        .success { background-color: #d4edda; border-color: #c3e6cb; }
        .failure { background-color: #f8d7da; border-color: #f5c6cb; }
        .warning { background-color: #fff3cd; border-color: #ffeaa7; }
        .log-content { background-color: #f8f9fa; padding: 10px; border-radius: 3px; font-family: monospace; white-space: pre-wrap; }
        table { width: 100%; border-collapse: collapse; margin: 10px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
    </style>
</head>
<body>
    <div class="header">
        <h1>CrossChain Registry Backend Test Report</h1>
        <p><strong>Timestamp:</strong> $TIMESTAMP</p>
        <p><strong>Test Environment:</strong> Local DFX Network</p>
    </div>
EOF

    # Add test results for each test type
    local test_types=("crud" "community" "verification" "errors" "advanced" "performance")
    local overall_status="success"
    
    for test_type in "${test_types[@]}"; do
        local log_file="$LOG_DIR/${test_type}_tests_$TIMESTAMP.log"
        
        if [[ -f "$log_file" ]]; then
            local test_status="success"
            if grep -q "Some tests failed" "$log_file"; then
                test_status="failure"
                overall_status="failure"
            fi
            
            cat >> "$report_file" << EOF
    <div class="test-section $test_status">
        <h2>$(echo ${test_type^} | sed 's/_/ /g') Tests</h2>
EOF
            
            # Extract test summary
            if grep -q "Total Tests:" "$log_file"; then
                local total=$(grep "Total Tests:" "$log_file" | grep -o '[0-9]*' | head -1)
                local passed=$(grep "Passed:" "$log_file" | grep -o '[0-9]*' | head -1)
                local failed=$(grep "Failed:" "$log_file" | grep -o '[0-9]*' | head -1)
                
                cat >> "$report_file" << EOF
        <table>
            <tr><th>Metric</th><th>Count</th></tr>
            <tr><td>Total Tests</td><td>$total</td></tr>
            <tr><td>Passed</td><td style="color: green;">$passed</td></tr>
            <tr><td>Failed</td><td style="color: red;">$failed</td></tr>
        </table>
EOF
            fi
            
            # Add log excerpt
            cat >> "$report_file" << EOF
        <h3>Log Excerpt</h3>
        <div class="log-content">$(tail -20 "$log_file" | head -10)</div>
    </div>
EOF
        fi
    done
    
    # Add summary
    cat >> "$report_file" << EOF
    <div class="test-section $overall_status">
        <h2>Overall Result</h2>
        <p><strong>Status:</strong> $(echo ${overall_status^})</p>
        <p><strong>Log Directory:</strong> $LOG_DIR</p>
        <p><strong>Generated:</strong> $(date)</p>
    </div>
</body>
</html>
EOF

    log_success "Test report generated: $report_file"
}

# Performance benchmarking
run_performance_benchmark() {
    log_info "Running performance benchmarks..."
    
    local benchmark_file="$LOG_DIR/benchmark_$TIMESTAMP.txt"
    
    echo "CrossChain Registry Performance Benchmark - $TIMESTAMP" > "$benchmark_file"
    echo "======================================================" >> "$benchmark_file"
    echo >> "$benchmark_file"
    
    # Benchmark company creation
    log_info "Benchmarking company creation..."
    local start_time=$(date +%s.%N)
    
    for i in {1..10}; do
        dfx canister call CrossChainRegistry_backend create_company "(record {
            basic_info = record {
                name = \"Benchmark Company $i\";
                description = \"Performance test company\";
                website = \"https://benchmark$i.com\";
                founding_date = \"2024-01-01\";
                team_size = 5;
                focus_areas = vec { \"Performance\" };
            };
            web3_identity = record {
                github_org = opt \"benchmark$i\";
                twitter_handle = null;
                discord_server = null;
                telegram_channel = null;
                domain_verified = false;
                social_verification_status = variant { Pending };
            };
            cross_chain_presence = record {
                ethereum_contracts = vec {};
                bitcoin_addresses = vec {};
                icp_canisters = vec {};
                polygon_contracts = vec {};
                treasury_wallets = vec {};
                token_contracts = vec {};
            };
            team_members = vec {};
        })" > /dev/null 2>&1
    done
    
    local end_time=$(date +%s.%N)
    local duration=$(echo "$end_time - $start_time" | bc)
    local avg_time=$(echo "scale=3; $duration / 10" | bc)
    
    echo "Company Creation Benchmark:" >> "$benchmark_file"
    echo "  - Created 10 companies in ${duration}s" >> "$benchmark_file"
    echo "  - Average time per company: ${avg_time}s" >> "$benchmark_file"
    echo >> "$benchmark_file"
    
    # Benchmark query operations
    log_info "Benchmarking query operations..."
    start_time=$(date +%s.%N)
    
    for i in {1..50}; do
        dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 10, null)' > /dev/null 2>&1
    done
    
    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc)
    avg_time=$(echo "scale=4; $duration / 50" | bc)
    
    echo "List Companies Benchmark:" >> "$benchmark_file"
    echo "  - Executed 50 list operations in ${duration}s" >> "$benchmark_file"
    echo "  - Average time per query: ${avg_time}s" >> "$benchmark_file"
    echo >> "$benchmark_file"
    
    log_success "Performance benchmark completed: $benchmark_file"
}

# Cleanup function
cleanup() {
    log_info "Cleaning up test environment..."
    
    # Optional: Clean up test data
    if [[ "$CLEANUP_DATA" == "true" ]]; then
        "$DATA_GENERATOR" clean > /dev/null 2>&1 || true
    fi
    
    log_success "Cleanup completed"
}

# Main execution function
run_all_tests() {
    local failed_tests=0
    
    log_header "CrossChain Registry Backend Test Suite"
    
    # Setup
    setup_logging
    check_prerequisites
    deploy_canisters
    
    # Generate test data if requested
    if [[ "$GENERATE_DATA" == "true" ]]; then
        log_info "Generating test data..."
        "$DATA_GENERATOR" generate > "$LOG_DIR/data_generation_$TIMESTAMP.log" 2>&1
    fi
    
    # Run test suites
    local test_suites=("crud" "community" "verification" "errors" "advanced")
    
    for test_suite in "${test_suites[@]}"; do
        if ! run_test_suite "$test_suite"; then
            ((failed_tests++))
        fi
    done
    
    # Run performance tests if requested
    if [[ "$RUN_PERFORMANCE" == "true" ]]; then
        run_performance_benchmark
        if ! run_test_suite "performance"; then
            ((failed_tests++))
        fi
    fi
    
    # Generate report
    generate_test_report
    
    # Summary
    log_header "Test Summary"
    if [[ $failed_tests -eq 0 ]]; then
        log_success "All test suites passed! 🎉"
        log_info "Test logs available in: $LOG_DIR"
    else
        log_error "$failed_tests test suite(s) failed"
        log_info "Check individual test logs in: $LOG_DIR"
        exit 1
    fi
    
    # Cleanup
    cleanup
}

# Parse command line arguments
show_help() {
    echo "Usage: $0 [OPTIONS] [TEST_TYPE]"
    echo ""
    echo "Options:"
    echo "  --generate-data    Generate test data before running tests"
    echo "  --performance      Include performance benchmarks"
    echo "  --cleanup          Clean up test data after tests"
    echo "  --help            Show this help message"
    echo ""
    echo "Test Types:"
    echo "  all               Run all test suites (default)"
    echo "  crud              Run CRUD operation tests only"
    echo "  community         Run community validation tests only"
    echo "  verification      Run verification system tests only"
    echo "  errors            Run error handling tests only"
    echo "  advanced          Run advanced feature tests only"
    echo "  performance       Run performance tests only"
    echo ""
    echo "Examples:"
    echo "  $0                           # Run all tests"
    echo "  $0 --generate-data           # Generate data and run all tests"
    echo "  $0 --performance crud        # Run CRUD tests with performance benchmarks"
    echo "  $0 verification              # Run only verification tests"
}

# Parse arguments
GENERATE_DATA="false"
RUN_PERFORMANCE="false"
CLEANUP_DATA="false"
TEST_TYPE="all"

while [[ $# -gt 0 ]]; do
    case $1 in
        --generate-data)
            GENERATE_DATA="true"
            shift
            ;;
        --performance)
            RUN_PERFORMANCE="true"
            shift
            ;;
        --cleanup)
            CLEANUP_DATA="true"
            shift
            ;;
        --help)
            show_help
            exit 0
            ;;
        crud|community|verification|errors|advanced|performance|all)
            TEST_TYPE="$1"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Execute based on test type
if [[ "$TEST_TYPE" == "all" ]]; then
    run_all_tests
else
    setup_logging
    check_prerequisites
    deploy_canisters
    
    if [[ "$GENERATE_DATA" == "true" ]]; then
        "$DATA_GENERATOR" generate > "$LOG_DIR/data_generation_$TIMESTAMP.log" 2>&1
    fi
    
    if run_test_suite "$TEST_TYPE"; then
        log_success "Test suite '$TEST_TYPE' completed successfully"
    else
        log_error "Test suite '$TEST_TYPE' failed"
        exit 1
    fi
fi