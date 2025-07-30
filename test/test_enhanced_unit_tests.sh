#!/bin/bash

# Enhanced Unit Tests
# Tests internal logic and validation functions for new features

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

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

run_test() {
    local test_name="$1"
    local test_command="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    log_info "Running test: $test_name"
    
    if eval "$test_command"; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "✓ $test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "✗ $test_name"
        return 1
    fi
}

# Test Address Validation Functions
test_address_validation() {
    log_info "Testing address validation functions..."
    
    # Test Ethereum address validation
    run_test "Valid Ethereum address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\"
        )" 2>/dev/null | grep -q "true"
    '
    
    run_test "Invalid Ethereum address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0xinvalid\"
        )" 2>/dev/null | grep -q "false"
    '
    
    # Test Bitcoin address validation
    run_test "Valid Bitcoin Legacy address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"bitcoin\",
            \"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa\"
        )" 2>/dev/null | grep -q "true"
    '
    
    run_test "Valid Bitcoin SegWit address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"bitcoin\", 
            \"bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test Solana address validation
    run_test "Valid Solana address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"solana\",
            \"7dHbWXmci3dT8UFYWGGWnSZwJa8ACHWrAhwRgBAuR7a1\"
        )" 2>/dev/null | grep -q "true"
    '
    
    run_test "Invalid Solana address (contains invalid chars)" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"solana\",
            \"7dHbWXmci3dT8UFYWGGWnSZwJa8ACHWrAhwRgBAu0Il1\"
        )" 2>/dev/null | grep -q "false"
    '
    
    # Test ICP principal validation
    run_test "Valid ICP principal validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"icp\",
            \"rdmx6-jaaaa-aaaah-qcaiq-cai\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test Sui address validation
    run_test "Valid Sui address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"sui\",
            \"0x2d3d1d6e5f7c8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test TON address validation
    run_test "Valid TON raw address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ton\",
            \"0:83dfd552e63729b472fcbcc8c45ebcc6691702558b68ec7527e1ba403a0f31a8\"
        )" 2>/dev/null | grep -q "true"
    '
    
    run_test "Valid TON user-friendly address validation" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ton\",
            \"EQD2NmD_lH5f5u1Kj3KfGyTvhZSX0Eg6qp2a5IQUKXxOG21n\"
        )" 2>/dev/null | grep -q "true"
    '
}

# Test Verification Instructions
test_verification_instructions() {
    log_info "Testing verification instructions..."
    
    run_test "GitHub verification instructions" '
        dfx canister call CrossChainRegistry_backend get_verification_instructions "(variant { GitHub })" 2>/dev/null | grep -q "organization"
    '
    
    run_test "Domain verification instructions" '
        dfx canister call CrossChainRegistry_backend get_verification_instructions "(variant { Domain })" 2>/dev/null | grep -q "TXT record"
    '
    
    run_test "Twitter verification instructions" '
        dfx canister call CrossChainRegistry_backend get_verification_instructions "(variant { Twitter })" 2>/dev/null | grep -q "tweet"
    '
    
    run_test "LinkedIn verification instructions" '
        dfx canister call CrossChainRegistry_backend get_verification_instructions "(variant { LinkedIn })" 2>/dev/null | grep -q "company page"
    '
    
    run_test "Medium verification instructions" '
        dfx canister call CrossChainRegistry_backend get_verification_instructions "(variant { Medium })" 2>/dev/null | grep -q "article"
    '
}

# Test Cross-Chain Instructions
test_crosschain_instructions() {
    log_info "Testing cross-chain verification instructions..."
    
    run_test "Ethereum cross-chain instructions" '
        dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions "(variant { Ethereum })" 2>/dev/null | grep -q "contract"
    '
    
    run_test "Bitcoin cross-chain instructions" '
        dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions "(variant { Bitcoin })" 2>/dev/null | grep -q "transaction"
    '
    
    run_test "ICP cross-chain instructions" '
        dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions "(variant { ICP })" 2>/dev/null | grep -q "canister"
    '
    
    run_test "Polygon cross-chain instructions" '
        dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions "(variant { Polygon })" 2>/dev/null | grep -q "contract"
    '
}

# Test Address Validation Rules
test_address_validation_rules() {
    log_info "Testing address validation rules..."
    
    run_test "Ethereum address validation rules" '
        dfx canister call CrossChainRegistry_backend get_address_validation_rules "(\"ethereum\")" 2>/dev/null | grep -q "0x"
    '
    
    run_test "Bitcoin address validation rules" '
        dfx canister call CrossChainRegistry_backend get_address_validation_rules "(\"bitcoin\")" 2>/dev/null | grep -q "Legacy format"
    '
    
    run_test "Solana address validation rules" '
        dfx canister call CrossChainRegistry_backend get_address_validation_rules "(\"solana\")" 2>/dev/null | grep -q "base58"
    '
    
    run_test "ICP address validation rules" '
        dfx canister call CrossChainRegistry_backend get_address_validation_rules "(\"icp\")" 2>/dev/null | grep -q "Principal"
    '
    
    run_test "Unsupported chain validation rules" '
        dfx canister call CrossChainRegistry_backend get_address_validation_rules "(\"unsupported\")" 2>/dev/null | grep -q "Unsupported"
    '
}

# Test Supported Chains
test_supported_chains() {
    log_info "Testing supported chains..."
    
    run_test "Get supported chains list" '
        result=$(dfx canister call CrossChainRegistry_backend get_supported_chains "()" 2>/dev/null)
        echo "$result" | grep -q "ethereum" && 
        echo "$result" | grep -q "bitcoin" &&
        echo "$result" | grep -q "solana" &&
        echo "$result" | grep -q "icp"
    '
}

# Test Statistics and Analytics
test_statistics() {
    log_info "Testing statistics and analytics..."
    
    run_test "Get general statistics" '
        dfx canister call CrossChainRegistry_backend get_statistics "()" 2>/dev/null | grep -q "Ok"
    '
    
    run_test "Get company count" '
        dfx canister call CrossChainRegistry_backend get_company_count "()" 2>/dev/null | grep -E "[0-9]+"
    '
    
    run_test "Get monitoring statistics" '
        dfx canister call CrossChainRegistry_backend get_monitoring_stats "()" 2>/dev/null | grep -q "Ok"
    '
}

# Test Input Validation Edge Cases
test_input_validation_edge_cases() {
    log_info "Testing input validation edge cases..."
    
    # Test extremely long addresses
    run_test "Extremely long address rejection" '
        LONG_ADDR=$(printf "A%.0s" {1..300})
        ! dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"$LONG_ADDR\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test empty addresses
    run_test "Empty address rejection" '
        ! dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test addresses with special characters
    run_test "Special characters in address rejection" '
        ! dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E!@#$%\"
        )" 2>/dev/null | grep -q "true"
    '
    
    # Test case sensitivity
    run_test "Case insensitive Ethereum address" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0X742D35CC6634C0532925A3B8D4D3C12DE56D0D9E\"
        )" 2>/dev/null | grep -q "true"
    '
}

# Test Monitoring and Security Features
test_monitoring_features() {
    log_info "Testing monitoring and security features..."
    
    run_test "Process monitoring tasks" '
        dfx canister call CrossChainRegistry_backend process_monitoring_tasks "()" 2>/dev/null | grep -q "Ok"
    '
    
    run_test "Get security events by principal" '
        dfx canister call CrossChainRegistry_backend get_security_events_by_principal "()" 2>/dev/null | grep -q "Ok"
    '
    
    run_test "Get security events by severity" '
        dfx canister call CrossChainRegistry_backend get_security_events_by_severity "(variant { Low })" 2>/dev/null | grep -q "Ok"
    '
}

# Test Error Messages and Response Format
test_error_messages() {
    log_info "Testing error messages and response formats..."
    
    run_test "Clear error message for invalid chain" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"invalidchain\",
            \"someaddress\"
        )" 2>/dev/null | grep -q "false"
    '
    
    run_test "Clear error message for malformed address" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"notanaddress\"
        )" 2>/dev/null | grep -q "false"
    '
}

# Main test execution
main() {
    log_info "Starting Enhanced Unit Tests"
    log_info "=============================="
    
    # Ensure canister is running
    if ! dfx canister status CrossChainRegistry_backend >/dev/null 2>&1; then
        log_error "CrossChainRegistry_backend canister is not running"
        log_info "Please run: dfx start --background && dfx deploy"
        exit 1
    fi
    
    # Run test suites
    test_address_validation
    test_verification_instructions  
    test_crosschain_instructions
    test_address_validation_rules
    test_supported_chains
    test_statistics
    test_input_validation_edge_cases
    test_monitoring_features
    test_error_messages
    
    # Print summary
    echo
    log_info "=============================="
    log_info "Unit Test Summary"
    log_info "=============================="
    log_info "Tests Run: $TESTS_RUN"
    log_success "Tests Passed: $TESTS_PASSED"
    if [ $TESTS_FAILED -gt 0 ]; then
        log_error "Tests Failed: $TESTS_FAILED"
    else
        log_success "Tests Failed: $TESTS_FAILED"
    fi
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "🎉 All unit tests passed!"
        exit 0
    else
        log_error "❌ Some unit tests failed. Please review the output above."
        exit 1
    fi
}

# Run main function
main "$@"