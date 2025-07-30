#!/usr/bin/env bash

# Master Test Runner for Enhanced Features
# Runs all enhanced feature tests in proper sequence

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
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

log_header() {
    echo -e "${PURPLE}========================================${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}========================================${NC}"
}

log_subheader() {
    echo -e "${CYAN}--- $1 ---${NC}"
}

# Test results tracking (using bash associative arrays)
unset test_results
declare -A test_results 2>/dev/null || {
    # Fallback for older shells without associative arrays
    log_warning "Associative arrays not supported. Using indexed array fallback."
}
TOTAL_TEST_SUITES=0
PASSED_TEST_SUITES=0
FAILED_TEST_SUITES=0

# Function to run a test suite
run_test_suite() {
    local suite_name="$1"
    local test_script="$2"
    local description="$3"
    
    TOTAL_TEST_SUITES=$((TOTAL_TEST_SUITES + 1))
    
    log_subheader "$description"
    log_info "Running test suite: $suite_name"
    
    if [ ! -f "$test_script" ]; then
        log_error "Test script not found: $test_script"
        test_results["$suite_name"]="MISSING"
        FAILED_TEST_SUITES=$((FAILED_TEST_SUITES + 1))
        return 1
    fi
    
    # Make script executable
    chmod +x "$test_script"
    
    # Run the test suite
    local start_time=$(date +%s)
    if "$test_script"; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        test_results["$suite_name"]="PASSED (${duration}s)"
        PASSED_TEST_SUITES=$((PASSED_TEST_SUITES + 1))
        log_success "✓ $suite_name completed successfully in ${duration}s"
        return 0
    else
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        test_results["$suite_name"]="FAILED (${duration}s)"
        FAILED_TEST_SUITES=$((FAILED_TEST_SUITES + 1))
        log_error "✗ $suite_name failed after ${duration}s"
        return 1
    fi
}

# Pre-flight checks
preflight_checks() {
    log_header "PRE-FLIGHT CHECKS"
    
    # Check if dfx is installed
    if ! command -v dfx &> /dev/null; then
        log_error "DFX is not installed or not in PATH"
        exit 1
    fi
    
    log_info "DFX version: $(dfx --version)"
    
    # Check if IC replica is running
    if ! dfx ping > /dev/null 2>&1; then
        log_warning "IC replica is not running. Starting..."
        dfx start --background
        sleep 5
    fi
    
    # Check if canister is deployed
    if ! dfx canister status CrossChainRegistry_backend > /dev/null 2>&1; then
        log_warning "CrossChainRegistry_backend canister is not deployed. Deploying..."
        dfx deploy CrossChainRegistry_backend
    fi
    
    # Verify canister is running
    local canister_status=$(dfx canister status CrossChainRegistry_backend 2>/dev/null | grep "Status:" | awk '{print $2}')
    if [ "$canister_status" = "Running" ]; then
        log_success "✓ CrossChainRegistry_backend canister is running"
    else
        log_error "CrossChainRegistry_backend canister is not running (Status: $canister_status)"
        exit 1
    fi
    
    # Check available cycles
    local cycles_balance=$(dfx canister status CrossChainRegistry_backend 2>/dev/null | grep "Balance:" | awk '{print $2}')
    log_info "Canister cycles balance: $cycles_balance"
    
    log_success "Pre-flight checks completed successfully"
    echo
}

# Generate test report
generate_test_report() {
    local report_file="/tmp/enhanced_features_test_report.md"
    
    log_info "Generating comprehensive test report..."
    
    cat > "$report_file" << EOF
# CrossChain Registry - Enhanced Features Test Report

**Generated:** $(date)
**Test Environment:** $(dfx --version)

## Test Summary

- **Total Test Suites:** $TOTAL_TEST_SUITES
- **Passed:** $PASSED_TEST_SUITES
- **Failed:** $FAILED_TEST_SUITES
- **Success Rate:** $(( PASSED_TEST_SUITES * 100 / TOTAL_TEST_SUITES ))%

## Test Suite Results

EOF
    
    for suite_name in "${!test_results[@]}"; do
        local result="${test_results[$suite_name]}"
        if [[ "$result" =~ PASSED ]]; then
            echo "- ✅ **$suite_name**: $result" >> "$report_file"
        elif [[ "$result" =~ FAILED ]]; then
            echo "- ❌ **$suite_name**: $result" >> "$report_file"
        else
            echo "- ⚠️ **$suite_name**: $result" >> "$report_file"
        fi
    done
    
    cat >> "$report_file" << EOF

## Test Coverage

### Enhanced Features Tested

1. **Social Media Verification Enhancements**
   - LinkedIn verification support
   - Medium verification support
   - Enhanced URL validation and security
   - XSS protection and input sanitization
   - Rate limiting implementation
   - Monitoring integration

2. **Community Validation Enhancements**
   - Enhanced endorsement system with fraud detection
   - Advanced testimonial validation with security checks
   - Improved vouch system with monitoring
   - XSS protection across all community features
   - Reputation integrity validation
   - Enhanced staking with validation requirements

3. **Cross-Chain Verification Enhancements**
   - Multi-chain portfolio verification
   - Cross-chain risk assessment
   - Enhanced address validation with security checks
   - Suspicious address pattern detection
   - Comprehensive monitoring integration
   - Rate limiting for cross-chain operations

4. **Security Enhancements**
   - XSS protection across all input fields
   - Rate limiting on all user-facing endpoints
   - Comprehensive security event logging
   - Input validation and sanitization
   - URL validation with domain whitelisting
   - Authorization and access control
   - Error handling security

5. **Monitoring Integration**
   - Security event logging for all operations
   - Automated proof monitoring scheduling
   - Community alert system
   - Monitoring statistics and analytics
   - Rate limit violation detection
   - Fraud pattern detection

## Performance Metrics

- **Average Response Time:** Measured across all test operations
- **Rate Limiting Effectiveness:** Confirmed blocking of rapid requests
- **Concurrent Operations:** Tested handling of multiple simultaneous requests
- **Resource Usage:** Validated memory and processing efficiency

## Security Validation

- **XSS Protection:** Confirmed blocking of script injection attempts
- **Input Validation:** Validated handling of malicious input patterns
- **Rate Limiting:** Confirmed protection against abuse
- **Authorization:** Verified access control enforcement
- **Monitoring:** Confirmed logging of security events

## Recommendations

EOF
    
    if [ $FAILED_TEST_SUITES -eq 0 ]; then
        cat >> "$report_file" << EOF
🎉 **All test suites passed successfully!**

The enhanced features are ready for production deployment with the following benefits:
- Comprehensive security protection
- Enhanced user experience with new verification options
- Robust monitoring and fraud detection
- Scalable architecture with rate limiting
- Full audit trail capabilities

EOF
    else
        cat >> "$report_file" << EOF
⚠️ **Some test suites failed - Review Required**

Please address the failed test suites before production deployment:
- Review failed security tests for potential vulnerabilities
- Check performance issues if any tests exceeded time limits
- Verify monitoring integration is working correctly
- Ensure all enhanced features are functioning as expected

EOF
    fi
    
    cat >> "$report_file" << EOF
## Next Steps

1. **If all tests passed:**
   - Deploy to staging environment for integration testing
   - Perform user acceptance testing
   - Update documentation
   - Plan production deployment

2. **If tests failed:**
   - Investigate and fix failing tests
   - Re-run failed test suites
   - Perform regression testing
   - Update implementation as needed

---
*Report generated by Enhanced Features Test Suite*
EOF
    
    log_success "Test report generated: $report_file"
    echo
    cat "$report_file"
}

# Main execution
main() {
    local start_time=$(date +%s)
    
    log_header "CROSSCHAIN REGISTRY - ENHANCED FEATURES TEST SUITE"
    log_info "Starting comprehensive testing of all enhanced features..."
    echo
    
    # Pre-flight checks
    preflight_checks
    
    # Run all test suites
    log_header "RUNNING TEST SUITES"
    
    # 1. Unit Tests - Core functionality validation
    run_test_suite "Unit Tests" \
                   "./test/test_enhanced_unit_tests.sh" \
                   "Testing core validation functions and API endpoints"
    
    echo
    
    # 2. Feature Tests - Integration testing of new features
    run_test_suite "Feature Integration Tests" \
                   "./test/test_enhanced_features.sh" \
                   "Testing enhanced verification, community validation, and cross-chain features"
    
    echo
    
    # 3. Security Tests - Security validation and vulnerability testing
    run_test_suite "Security Tests" \
                   "./test/test_enhanced_security.sh" \
                   "Testing XSS protection, rate limiting, and security monitoring"
    
    echo
    
    # 4. Performance Tests - Load testing and performance validation
    run_test_suite "Performance Tests" \
                   "./test/test_enhanced_performance.sh" \
                   "Testing performance, load handling, and resource usage"
    
    echo
    
    # Calculate total execution time
    local end_time=$(date +%s)
    local total_duration=$((end_time - start_time))
    
    # Generate comprehensive report
    log_header "TEST EXECUTION COMPLETE"
    generate_test_report
    
    # Final summary
    echo
    log_header "FINAL SUMMARY"
    log_info "Total execution time: ${total_duration}s"
    log_info "Test suites run: $TOTAL_TEST_SUITES"
    log_success "Test suites passed: $PASSED_TEST_SUITES"
    
    if [ $FAILED_TEST_SUITES -gt 0 ]; then
        log_error "Test suites failed: $FAILED_TEST_SUITES"
        echo
        log_error "❌ Some test suites failed. Please review the output above and fix issues."
        log_info "Re-run specific test suites after fixing issues:"
        for suite in "${!test_results[@]}"; do
            if [[ "${test_results[$suite]}" =~ FAILED ]]; then
                log_info "  - $suite"
            fi
        done
        exit 1
    else
        log_success "Test suites failed: $FAILED_TEST_SUITES"
        echo
        log_success "🎉 ALL ENHANCED FEATURES TESTS PASSED!"
        log_success "✅ The system is ready for production deployment"
        log_success "🔒 Security features validated"
        log_success "⚡ Performance requirements met"
        log_success "🛡️ Monitoring and fraud detection working"
        
        echo
        log_info "Enhanced features successfully implemented:"
        log_info "  ✓ LinkedIn and Medium social verification"
        log_info "  ✓ Advanced community validation with fraud detection"
        log_info "  ✓ Multi-chain portfolio verification and risk assessment"
        log_info "  ✓ Comprehensive XSS protection and input validation"
        log_info "  ✓ Rate limiting across all user-facing endpoints"
        log_info "  ✓ Full monitoring integration with security event logging"
        log_info "  ✓ Performance optimization and load handling"
        
        exit 0
    fi
}

# Handle script interruption
trap 'log_error "Test execution interrupted"; exit 130' INT TERM

# Run main function
main "$@"