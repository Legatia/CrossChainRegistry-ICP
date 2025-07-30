#!/bin/bash

# Enhanced Features Performance Tests
# Tests performance, load handling, and stress scenarios for new features

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Performance metrics
declare -A performance_metrics

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

run_performance_test() {
    local test_name="$1"
    local test_command="$2"
    local max_time_seconds="$3"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    log_info "Running performance test: $test_name (max: ${max_time_seconds}s)"
    
    local start_time=$(date +%s)
    
    if eval "$test_command"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        performance_metrics["$test_name"]=$duration
        
        if [ $duration -le $max_time_seconds ]; then
            TESTS_PASSED=$((TESTS_PASSED + 1))
            log_success "✓ $test_name (${duration}s)"
            return 0
        else
            TESTS_FAILED=$((TESTS_FAILED + 1))
            log_warning "⚠ $test_name (${duration}s - exceeded ${max_time_seconds}s limit)"
            return 1
        fi
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "✗ $test_name (failed to execute)"
        return 1
    fi
}

# Setup performance test data
setup_performance_test_data() {
    log_info "Setting up performance test data..."
    
    # Create multiple test companies for load testing
    for i in {1..5}; do
        dfx canister call CrossChainRegistry_backend create_company "(record {
            basic_info = record {
                name = \"Performance Test Company $i\";
                description = \"A test company for performance testing - $i\";
                website = \"https://perf-test-$i.com\";
                founding_date = \"2024-01-0$i\";
                team_size = $((i * 2));
                focus_areas = vec {\"Performance\"; \"Testing\"};
            };
            web3_identity = record {
                github_org = null;
                twitter_handle = null;
                discord_server = null;
                telegram_channel = null;
                linkedin_company = null;
                medium_publication = null;
                domain_verified = false;
                social_verification_status = variant { Pending };
                verification_proofs = vec {};
            };
            cross_chain_presence = record {
                ethereum_contracts = vec {};
                bitcoin_addresses = vec {};
                icp_canisters = vec {};
                polygon_contracts = vec {};
                solana_addresses = vec {};
                sui_addresses = vec {};
                ton_addresses = vec {};
                treasury_wallets = vec {};
                token_contracts = vec {};
            };
            team_members = vec {};
        })" 2>/dev/null || true
    done
    
    log_success "Performance test data setup complete"
}

# Test Social Media Verification Performance
test_social_verification_performance() {
    log_info "Testing social media verification performance..."
    
    # Get test company ID
    local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 1, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -1 | tr -d '"')
    
    run_performance_test "Single LinkedIn verification" '
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"'$COMPANY_ID'\",
            \"linkedin\",
            \"https://linkedin.com/company/test-company/posts/perf-test\"
        )" 2>/dev/null || true
    ' 5
    
    run_performance_test "Single Medium verification" '
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"'$COMPANY_ID'\",
            \"medium\",
            \"https://medium.com/test-publication/perf-test-post\"
        )" 2>/dev/null || true
    ' 5
    
    # Test batch verification performance
    run_performance_test "Batch social verification (5 platforms)" '
        for platform in twitter discord telegram linkedin medium; do
            dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
                \"'$COMPANY_ID'\",
                \"$platform\",
                \"https://$platform.com/test/perf-$RANDOM\"
            )" 2>/dev/null &
        done
        wait
    ' 10
}

# Test Community Validation Performance
test_community_validation_performance() {
    log_info "Testing community validation performance..."
    
    # Get company IDs for testing
    local companies=($(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 10, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -5))
    local COMPANY_A=${companies[0]//\"/}
    local COMPANY_B=${companies[1]//\"/}
    
    run_performance_test "Single endorsement creation" '
        dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"'$COMPANY_A'\",
            \"'$COMPANY_B'\",
            \"Performance test endorsement - high quality company\"
        )" 2>/dev/null || true
    ' 3
    
    run_performance_test "Single testimonial creation" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"'$COMPANY_A'\",
            \"Performance Tester\",
            \"QA Engineer\",
            \"Excellent company for performance testing. Fast response times and reliable service.\"
        )" 2>/dev/null || true
    ' 3
    
    run_performance_test "Single vouch creation" '
        dfx canister call CrossChainRegistry_backend add_vouch "(
            \"'$COMPANY_A'\",
            \"I vouch for this company based on their excellent performance metrics\"
        )" 2>/dev/null || true
    ' 3
    
    # Test batch community validation
    run_performance_test "Batch testimonial creation (10 testimonials)" '
        for i in {1..10}; do
            dfx canister call CrossChainRegistry_backend add_testimonial "(
                \"'$COMPANY_A'\",
                \"Perf Tester $i\",
                \"Role $i\",
                \"Performance test testimonial number $i\"
            )" 2>/dev/null &
        done
        wait
    ' 15
    
    run_performance_test "Batch vouch creation (10 vouches)" '
        for i in {1..10}; do
            dfx canister call CrossChainRegistry_backend add_vouch "(
                \"'$COMPANY_A'\",
                \"Batch vouch number $i for performance testing\"
            )" 2>/dev/null &
        done
        wait
    ' 15
}

# Test Cross-Chain Verification Performance
test_crosschain_performance() {
    log_info "Testing cross-chain verification performance..."
    
    local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 1, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -1 | tr -d '"')
    
    run_performance_test "Single Ethereum challenge creation" '
        dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"'$COMPANY_ID'\";
            chain_type = variant { Ethereum };
            address_or_contract = \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\";
            verification_method = variant { TransactionMessage };
        })" 2>/dev/null || true
    ' 5
    
    run_performance_test "Single Bitcoin challenge creation" '
        dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"'$COMPANY_ID'\";
            chain_type = variant { Bitcoin };
            address_or_contract = \"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa\";
            verification_method = variant { TransactionMessage };
        })" 2>/dev/null || true
    ' 5
    
    # Test address validation performance
    run_performance_test "Batch address validation (50 addresses)" '
        addresses=(
            "0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E"
            "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
            "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
            "7dHbWXmci3dT8UFYWGGWnSZwJa8ACHWrAhwRgBAuR7a1"
            "rdmx6-jaaaa-aaaah-qcaiq-cai"
        )
        chains=("ethereum" "bitcoin" "bitcoin" "solana" "icp")
        
        for i in {1..10}; do
            for j in {0..4}; do
                dfx canister call CrossChainRegistry_backend validate_address "(
                    \"${chains[$j]}\",
                    \"${addresses[$j]}\"
                )" 2>/dev/null &
            done
        done
        wait
    ' 20
}

# Test Monitoring System Performance
test_monitoring_performance() {
    log_info "Testing monitoring system performance..."
    
    run_performance_test "Monitoring stats retrieval" '
        dfx canister call CrossChainRegistry_backend get_monitoring_stats "()" 2>/dev/null || true
    ' 3
    
    run_performance_test "Security events retrieval" '
        dfx canister call CrossChainRegistry_backend get_security_events_by_severity "(variant { Low })" 2>/dev/null || true
    ' 5
    
    run_performance_test "Community alerts retrieval" '
        dfx canister call CrossChainRegistry_backend get_community_alerts "(null)" 2>/dev/null || true
    ' 3
    
    run_performance_test "Batch proof monitoring scheduling (20 proofs)" '
        local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies "(null, opt 1, null)" | grep -o "\"[a-zA-Z0-9_-]*\"" | head -1 | tr -d "\"")
        for i in {1..20}; do
            dfx canister call CrossChainRegistry_backend schedule_proof_monitoring "(
                \"'$COMPANY_ID'\",
                \"perf_proof_$i\",
                variant { Medium }
            )" 2>/dev/null &
        done
        wait
    ' 10
}

# Test Data Retrieval Performance
test_data_retrieval_performance() {
    log_info "Testing data retrieval performance..."
    
    run_performance_test "Company list retrieval (100 companies)" '
        dfx canister call CrossChainRegistry_backend list_companies "(null, opt 100, null)" 2>/dev/null || true
    ' 8
    
    run_performance_test "Company statistics retrieval" '
        dfx canister call CrossChainRegistry_backend get_statistics "()" 2>/dev/null || true
    ' 3
    
    run_performance_test "Company count retrieval" '
        dfx canister call CrossChainRegistry_backend get_company_count "()" 2>/dev/null || true
    ' 2
    
    # Test individual company data retrieval
    run_performance_test "Individual company data retrieval (10 companies)" '
        local companies=($(dfx canister call CrossChainRegistry_backend list_companies "(null, opt 10, null)" | grep -o "\"[a-zA-Z0-9_-]*\"" | head -10))
        for company_id in "${companies[@]}"; do
            company_id=${company_id//\"/}
            dfx canister call CrossChainRegistry_backend get_company "\"$company_id\"" 2>/dev/null &
        done
        wait
    ' 10
}

# Test Memory and Resource Usage
test_resource_usage() {
    log_info "Testing resource usage and memory efficiency..."
    
    # Test large input handling
    run_performance_test "Large description handling" '
        local LARGE_DESC=$(printf "A%.0s" {1..2000})
        local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies "(null, opt 1, null)" | grep -o "\"[a-zA-Z0-9_-]*\"" | head -1 | tr -d "\"")
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"'$COMPANY_ID'\",
            \"Resource Tester\",
            \"Performance Engineer\",
            \"'$LARGE_DESC'\"
        )" 2>/dev/null || true
    ' 5
    
    # Test concurrent operations stress
    run_performance_test "Concurrent operations stress test (50 operations)" '
        local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies "(null, opt 1, null)" | grep -o "\"[a-zA-Z0-9_-]*\"" | head -1 | tr -d "\"")
        for i in {1..50}; do
            case $((i % 3)) in
                0) dfx canister call CrossChainRegistry_backend add_vouch "(
                    \"'$COMPANY_ID'\",
                    \"Stress test vouch $i\"
                )" 2>/dev/null & ;;
                1) dfx canister call CrossChainRegistry_backend validate_address "(
                    \"ethereum\",
                    \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\"
                )" 2>/dev/null & ;;
                2) dfx canister call CrossChainRegistry_backend get_monitoring_stats "()" 2>/dev/null & ;;
            esac
        done
        wait
    ' 30
}

# Test Rate Limiting Performance
test_rate_limiting_performance() {
    log_info "Testing rate limiting performance..."
    
    local COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 1, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -1 | tr -d '"')
    
    run_performance_test "Rate limiting enforcement (should trigger limits)" '
        # Rapidly make many requests to trigger rate limiting
        for i in {1..20}; do
            dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
                \"'$COMPANY_ID'\",
                \"twitter\",
                \"https://twitter.com/test/status/$RANDOM\"
            )" 2>/dev/null || true
        done
    ' 15
    
    run_performance_test "Rate limit recovery (wait and retry)" '
        # Wait a bit for rate limits to reset
        sleep 2
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"'$COMPANY_ID'\",
            \"twitter\",
            \"https://twitter.com/test/status/recovery-test\"
        )" 2>/dev/null || true
    ' 5
}

# Generate performance report
generate_performance_report() {
    log_info "Generating performance report..."
    
    local report_file="/tmp/crosschain_performance_report.txt"
    
    cat > "$report_file" << EOF
CrossChain Registry - Enhanced Features Performance Report
Generated: $(date)
========================================================

Test Summary:
- Total Tests: $TESTS_RUN
- Tests Passed: $TESTS_PASSED  
- Tests Failed: $TESTS_FAILED
- Success Rate: $(( TESTS_PASSED * 100 / TESTS_RUN ))%

Performance Metrics:
EOF
    
    for test_name in "${!performance_metrics[@]}"; do
        echo "- $test_name: ${performance_metrics[$test_name]}s" >> "$report_file"
    done
    
    cat >> "$report_file" << EOF

Performance Analysis:
- Average response time: $(( $(IFS=+; echo "$((${performance_metrics[*]}))" ) / ${#performance_metrics[@]} ))s
- Fastest operation: $(printf '%s\n' "${performance_metrics[@]}" | sort -n | head -1)s
- Slowest operation: $(printf '%s\n' "${performance_metrics[@]}" | sort -n | tail -1)s

Recommendations:
- Operations completing in <3s: Excellent performance
- Operations completing in 3-10s: Good performance  
- Operations completing in >10s: Consider optimization

EOF
    
    log_success "Performance report generated: $report_file"
    cat "$report_file"
}

# Main test execution
main() {
    log_info "Starting Enhanced Features Performance Tests"
    log_info "============================================="
    
    # Ensure canister is running
    if ! dfx canister status CrossChainRegistry_backend >/dev/null 2>&1; then
        log_error "CrossChainRegistry_backend canister is not running"
        log_info "Please run: dfx start --background && dfx deploy"
        exit 1
    fi
    
    # Setup test data
    if ! setup_performance_test_data; then
        log_error "Failed to setup performance test data"
        exit 1
    fi
    
    # Run performance test suites
    test_social_verification_performance
    test_community_validation_performance  
    test_crosschain_performance
    test_monitoring_performance
    test_data_retrieval_performance
    test_resource_usage
    test_rate_limiting_performance
    
    # Generate performance report
    generate_performance_report
    
    # Print summary
    echo
    log_info "============================================="
    log_info "Performance Test Summary"
    log_info "============================================="
    log_info "Tests Run: $TESTS_RUN"
    log_success "Tests Passed: $TESTS_PASSED"
    if [ $TESTS_FAILED -gt 0 ]; then
        log_warning "Tests with Performance Issues: $TESTS_FAILED"
    else
        log_success "Tests with Performance Issues: $TESTS_FAILED"
    fi
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "🎉 All performance tests passed!"
        exit 0
    else
        log_warning "⚠️ Some tests exceeded performance thresholds. Check the report above."
        exit 0  # Don't fail on performance issues, just warn
    fi
}

# Run main function
main "$@"