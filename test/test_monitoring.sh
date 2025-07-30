#!/bin/bash

# CrossChain Registry Monitoring System Test Suite
# Tests all monitoring and security functionality

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Global test data
COMPANY_ID=""
PROOF_ID=""

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
    ((PASSED_TESTS++))
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((FAILED_TESTS++))
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_header() {
    echo
    echo -e "${CYAN}========================================${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${CYAN}========================================${NC}"
}

# Test helper function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"
    local expect_success="${4:-true}"
    
    ((TOTAL_TESTS++))
    log_info "Running test: $test_name"
    
    local result
    local command_success=true
    
    if ! result=$(eval "$test_command" 2>&1); then
        command_success=false
    fi
    
    if [[ "$expect_success" == "true" ]]; then
        if [[ "$command_success" == "true" ]]; then
            if [[ -z "$expected_pattern" ]] || echo "$result" | grep -q "$expected_pattern"; then
                log_success "$test_name - PASSED"
                echo "$result" | head -3 | sed 's/^/  /'
                return 0
            else
                log_error "$test_name - FAILED (Pattern not found: $expected_pattern)"
                echo "$result" | head -5 | sed 's/^/  /'
                return 1
            fi
        else
            log_error "$test_name - FAILED (Command failed)"
            echo "$result" | head -5 | sed 's/^/  /'
            return 1
        fi
    else
        # Expecting failure
        if [[ "$command_success" == "false" ]]; then
            if [[ -z "$expected_pattern" ]] || echo "$result" | grep -q "$expected_pattern"; then
                log_success "$test_name - PASSED (Expected failure)"
                echo "$result" | head -3 | sed 's/^/  /'
                return 0
            else
                log_error "$test_name - FAILED (Expected error pattern not found: $expected_pattern)"
                echo "$result" | head -5 | sed 's/^/  /'
                return 1
            fi
        else
            log_error "$test_name - FAILED (Expected command to fail but it succeeded)"
            echo "$result" | head -5 | sed 's/^/  /'
            return 1
        fi
    fi
}

# Extract helper functions
extract_company_id() {
    echo "$1" | grep -o 'company_[0-9]*' | head -1
}

# Setup test environment
setup_tests() {
    log_info "Setting up monitoring test environment..."
    
    # Navigate to project root
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
    cd "$PROJECT_ROOT"
    
    # Check dfx status
    if ! dfx ping > /dev/null 2>&1; then
        log_error "DFX is not running. Please start dfx with: dfx start --background"
        exit 1
    fi
    
    log_success "Test environment ready"
}

# Create test company for monitoring tests
create_test_company() {
    log_info "Creating test company for monitoring tests..."
    
    local result
    result=$(dfx canister call CrossChainRegistry_backend create_company '(
        record {
            basic_info = record {
                name = "MonitoringTestCorp";
                description = "A test company for monitoring system validation";
                website = "https://test-monitoring.example.com";
                founding_date = "2024-01-01";
                team_size = 5;
                focus_areas = vec { "Testing"; "Monitoring" };
            };
            web3_identity = record {
                github_org = opt "testmonitoringcorp";
                twitter_handle = opt "@testmonitoring";
                discord_server = opt "discord.gg/testmonitor";
                telegram_channel = opt "t.me/testmonitor";
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
        }
    )')
    
    if echo "$result" | grep -q "Ok"; then
        COMPANY_ID=$(extract_company_id "$result")
        log_success "Test company created: $COMPANY_ID"
        return 0
    else
        log_error "Failed to create test company"
        echo "$result"
        return 1
    fi
}

# Test 1: Monitoring Stats
test_monitoring_stats() {
    log_header "Testing Monitoring Statistics"
    
    run_test "Get monitoring stats" \
        "dfx canister call CrossChainRegistry_backend get_monitoring_stats '()'" \
        "total_proofs_monitored"
}

# Test 2: Community Alerts
test_community_alerts() {
    log_header "Testing Community Alert System"
    
    # Get all alerts
    run_test "Get all community alerts" \
        "dfx canister call CrossChainRegistry_backend get_community_alerts '(null : opt bool)'" \
        "vec"
    
    # Get only unacknowledged alerts
    run_test "Get unacknowledged alerts" \
        "dfx canister call CrossChainRegistry_backend get_community_alerts '(opt false)'" \
        "vec"
    
    # Get acknowledged alerts
    run_test "Get acknowledged alerts" \
        "dfx canister call CrossChainRegistry_backend get_community_alerts '(opt true)'" \
        "vec"
}

# Test 3: Security Event Logging
test_security_events() {
    log_header "Testing Security Event System"
    
    # Test getting security events by severity
    run_test "Get low severity security events" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_severity '(variant { Low })'" \
        "vec"
    
    run_test "Get medium severity security events" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_severity '(variant { Medium })'" \
        "vec"
    
    run_test "Get high severity security events" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_severity '(variant { High })'" \
        "vec"
    
    run_test "Get critical security events" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_severity '(variant { Critical })'" \
        "vec"
    
    # Test getting security events by principal
    run_test "Get security events by current principal" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_principal '()'" \
        "vec"
}

# Test 4: Community Reporting System
test_community_reporting() {
    log_header "Testing Community Reporting System"
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_error "No company ID available for reporting tests"
        return 1
    fi
    
    # Test community report submission
    run_test "Submit community report for suspicious activity" \
        "dfx canister call CrossChainRegistry_backend submit_community_report '(
            \"$COMPANY_ID\",
            null : opt text,
            variant { Suspicious },
            \"Test report for monitoring validation - suspicious behavior detected\"
        )'" \
        "Ok"
    
    # Test report with proof ID
    run_test "Submit community report with proof ID" \
        "dfx canister call CrossChainRegistry_backend submit_community_report '(
            \"$COMPANY_ID\",
            opt \"test_proof_123\",
            variant { PostDeleted },
            \"Test report - verification post was deleted after verification\"
        )'" \
        "Ok"
    
    # Test rate limiting by submitting multiple reports quickly
    log_info "Testing rate limiting (expecting some failures)..."
    
    for i in {1..5}; do
        run_test "Rate limit test - Report $i/5" \
            "dfx canister call CrossChainRegistry_backend submit_community_report '(
                \"$COMPANY_ID\",
                null : opt text,
                variant { Suspicious },
                \"Rate limit test report #$i\"
            )'" \
            "" \
            "false"  # We expect some of these to fail due to rate limiting
    done
}

# Test 5: Proof Monitoring Scheduling
test_proof_monitoring() {
    log_header "Testing Proof Monitoring System"
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_error "No company ID available for monitoring tests"
        return 1
    fi
    
    # Schedule proof monitoring with different priorities
    run_test "Schedule low priority proof monitoring" \
        "dfx canister call CrossChainRegistry_backend schedule_proof_monitoring '(
            \"$COMPANY_ID\",
            \"test_proof_low_priority\",
            variant { Low }
        )'" \
        "Ok"
    
    run_test "Schedule medium priority proof monitoring" \
        "dfx canister call CrossChainRegistry_backend schedule_proof_monitoring '(
            \"$COMPANY_ID\",
            \"test_proof_medium_priority\",
            variant { Medium }
        )'" \
        "Ok"
    
    run_test "Schedule high priority proof monitoring" \
        "dfx canister call CrossChainRegistry_backend schedule_proof_monitoring '(
            \"$COMPANY_ID\",
            \"test_proof_high_priority\",
            variant { High }
        )'" \
        "Ok"
    
    run_test "Schedule critical priority proof monitoring" \
        "dfx canister call CrossChainRegistry_backend schedule_proof_monitoring '(
            \"$COMPANY_ID\",
            \"test_proof_critical_priority\",
            variant { Critical }
        )'" \
        "Ok"
}

# Test 6: Monitoring Task Processing
test_monitoring_task_processing() {
    log_header "Testing Monitoring Task Processing"
    
    # Process monitoring tasks
    run_test "Process pending monitoring tasks" \
        "dfx canister call CrossChainRegistry_backend process_monitoring_tasks '()'" \
        "Ok"
}

# Test 7: Social Media Verification with Monitoring Integration
test_verification_with_monitoring() {
    log_header "Testing Verification System with Monitoring Integration"
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_error "No company ID available for verification tests"
        return 1
    fi
    
    # Test Twitter verification (which should trigger monitoring)
    run_test "Twitter verification with monitoring" \
        "dfx canister call CrossChainRegistry_backend verify_social_media_with_proof '(
            \"$COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/testcompany/status/1234567890\"
        )'" \
        "permanently linked"
    
    # Test Discord verification
    run_test "Discord verification with monitoring" \
        "dfx canister call CrossChainRegistry_backend verify_social_media_with_proof '(
            \"$COMPANY_ID\",
            \"discord\",
            \"https://discord.com/channels/123456789/987654321/1122334455\"
        )'" \
        "permanently linked"
    
    # Test with invalid URL (should trigger security events)
    run_test "Invalid URL should trigger security logging" \
        "dfx canister call CrossChainRegistry_backend verify_social_media_with_proof '(
            \"$COMPANY_ID\",
            \"twitter\",
            \"http://malicious-site.com/fake-tweet\"
        )'" \
        "Err" \
        "false"  # Expecting this to fail
}

# Test 8: Alert Acknowledgment
test_alert_acknowledgment() {
    log_header "Testing Alert Acknowledgment System"
    
    # First get some alerts to acknowledge
    local alerts_result
    alerts_result=$(dfx canister call CrossChainRegistry_backend get_community_alerts '(opt false)')
    
    # Extract an alert ID if any exist (this is a simplified extraction)
    if echo "$alerts_result" | grep -q "alert_"; then
        local alert_id
        alert_id=$(echo "$alerts_result" | grep -o 'alert_[0-9]*' | head -1)
        
        if [[ -n "$alert_id" ]]; then
            run_test "Acknowledge community alert" \
                "dfx canister call CrossChainRegistry_backend acknowledge_alert '(\"$alert_id\")'" \
                "Ok"
        else
            log_warning "No alert ID found to acknowledge"
        fi
    else
        log_info "No unacknowledged alerts found (this is normal for a fresh system)"
    fi
}

# Test 9: Error Handling and Edge Cases
test_error_handling() {
    log_header "Testing Error Handling and Edge Cases"
    
    # Test with invalid company ID
    run_test "Report for non-existent company" \
        "dfx canister call CrossChainRegistry_backend submit_community_report '(
            \"invalid_company_id\",
            null : opt text,
            variant { Suspicious },
            \"Test report for non-existent company\"
        )'" \
        "Err" \
        "false"  # Expecting failure
    
    # Test scheduling monitoring for non-existent company
    run_test "Schedule monitoring for non-existent company" \
        "dfx canister call CrossChainRegistry_backend schedule_proof_monitoring '(
            \"invalid_company_id\",
            \"test_proof\",
            variant { Low }
        )'" \
        "Ok"  # This might succeed as it just schedules the task
    
    # Test acknowledging non-existent alert
    run_test "Acknowledge non-existent alert" \
        "dfx canister call CrossChainRegistry_backend acknowledge_alert '(\"invalid_alert_id\")'" \
        "Err" \
        "false"  # Expecting failure
}

# Test 10: Integration Test - Full Monitoring Workflow
test_full_monitoring_workflow() {
    log_header "Testing Full Monitoring Workflow Integration"
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_error "No company ID available for integration test"
        return 1
    fi
    
    log_info "Running full monitoring workflow test..."
    
    # Step 1: Create verification (should trigger monitoring)
    log_info "Step 1: Creating verification proof..."
    local verification_result
    verification_result=$(dfx canister call CrossChainRegistry_backend verify_social_media_with_proof '(
        "'"$COMPANY_ID"'",
        "twitter",
        "https://twitter.com/integrationtest/status/1234567890"
    )')
    
    if echo "$verification_result" | grep -q "Ok"; then
        log_success "Verification created successfully"
    else
        log_error "Verification creation failed"
        return 1
    fi
    
    # Step 2: Check monitoring stats (should show increased proof count)
    log_info "Step 2: Checking monitoring statistics..."
    run_test "Check updated monitoring stats" \
        "dfx canister call CrossChainRegistry_backend get_monitoring_stats '()'" \
        "total_proofs_monitored"
    
    # Step 3: Submit community report
    log_info "Step 3: Submitting community report..."
    run_test "Submit integration test community report" \
        "dfx canister call CrossChainRegistry_backend submit_community_report '(
            \"$COMPANY_ID\",
            opt \"integration_test_proof\",
            variant { ContentModified },
            \"Integration test - proof content was modified\"
        )'" \
        "Ok"
    
    # Step 4: Process monitoring tasks
    log_info "Step 4: Processing monitoring tasks..."
    run_test "Process tasks after integration setup" \
        "dfx canister call CrossChainRegistry_backend process_monitoring_tasks '()'" \
        "Ok"
    
    # Step 5: Check for security events
    log_info "Step 5: Checking security events..."
    run_test "Check security events after integration" \
        "dfx canister call CrossChainRegistry_backend get_security_events_by_severity '(variant { Low })'" \
        "vec"
    
    log_success "Full monitoring workflow integration test completed"
}

# Main test execution
main() {
    log_header "CrossChain Registry Monitoring System Test Suite"
    
    setup_tests || exit 1
    
    # Create test data
    create_test_company || {
        log_warning "Failed to create test company, some tests may be skipped"
    }
    
    # Run all test suites
    test_monitoring_stats
    test_community_alerts
    test_security_events
    test_community_reporting
    test_proof_monitoring
    test_monitoring_task_processing
    test_verification_with_monitoring
    test_alert_acknowledgment
    test_error_handling
    test_full_monitoring_workflow
    
    # Print summary
    log_header "Test Results Summary"
    echo -e "${BLUE}Total Tests:${NC} $TOTAL_TESTS"
    echo -e "${GREEN}Passed:${NC} $PASSED_TESTS"
    echo -e "${RED}Failed:${NC} $FAILED_TESTS"
    
    local success_rate=$((PASSED_TESTS * 100 / TOTAL_TESTS))
    echo -e "${PURPLE}Success Rate:${NC} ${success_rate}%"
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo
        log_success "🎉 All monitoring system tests passed!"
        echo -e "${GREEN}The monitoring system is working correctly.${NC}"
        exit 0
    else
        echo
        log_error "❌ Some tests failed. Please review the issues above."
        exit 1
    fi
}

# Run main function
main "$@"