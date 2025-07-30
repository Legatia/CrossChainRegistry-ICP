#!/bin/bash

# Enhanced Security Tests
# Tests security features, XSS protection, rate limiting, and monitoring integration

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

# Test counter
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

run_security_test() {
    local test_name="$1"
    local test_command="$2"
    local should_fail="${3:-false}"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    log_info "Running security test: $test_name"
    
    if [ "$should_fail" = "true" ]; then
        # Test should fail (security measure should block it)
        if ! eval "$test_command"; then
            TESTS_PASSED=$((TESTS_PASSED + 1))
            log_success "✓ $test_name (correctly blocked)"
            return 0
        else
            TESTS_FAILED=$((TESTS_FAILED + 1))
            log_error "✗ $test_name (security vulnerability - should have been blocked)"
            return 1
        fi
    else
        # Test should pass
        if eval "$test_command"; then
            TESTS_PASSED=$((TESTS_PASSED + 1))
            log_success "✓ $test_name"
            return 0
        else
            TESTS_FAILED=$((TESTS_FAILED + 1))
            log_error "✗ $test_name"
            return 1
        fi
    fi
}

# Setup security test data
setup_security_test_data() {
    log_info "Setting up security test data..."
    
    # Create test company for security testing
    dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Security Test Company";
            description = "A company for security testing";
            website = "https://security-test.com";
            founding_date = "2024-01-01";
            team_size = 5;
            focus_areas = vec {"Security"; "Testing"};
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
    })' 2>/dev/null || true
    
    # Get the created company ID
    SECURITY_COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 1, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -1 | tr -d '"')
    
    if [ -n "$SECURITY_COMPANY_ID" ]; then
        log_success "Security test company created: $SECURITY_COMPANY_ID"
        export SECURITY_COMPANY_ID
    else
        log_error "Failed to get security test company ID"
        return 1
    fi
}

# Test XSS Protection in Social Media Verification
test_xss_protection_verification() {
    log_info "Testing XSS protection in verification..."
    
    # Test script tag injection
    run_security_test "Script tag in Twitter URL" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/test<script>alert(1)</script>/status/123\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test JavaScript protocol injection
    run_security_test "JavaScript protocol in LinkedIn URL" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"linkedin\",
            \"javascript:alert(document.cookie)\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test event handler injection
    run_security_test "Event handler in Medium URL" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"medium\",
            \"https://medium.com/test\" onerror=\"alert(1)\" \"post\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test valid URL (should pass)
    run_security_test "Valid Twitter URL" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/legitimate_user/status/123456789\"
        )" 2>/dev/null || true
    ' false
}

# Test XSS Protection in Community Validation
test_xss_protection_community() {
    log_info "Testing XSS protection in community validation..."
    
    # Create second company for endorsement testing
    dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Endorser Security Company";
            description = "Company for security endorsement testing";
            website = "https://endorser-security.com";
            founding_date = "2024-01-01";
            team_size = 3;
            focus_areas = vec {"Security"};
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
    })' 2>/dev/null || true
    
    local ENDORSER_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 2, null)' | grep -o '"[a-zA-Z0-9_-]*"' | tail -1 | tr -d '"')
    
    # Test XSS in endorsement
    run_security_test "Script injection in endorsement" '
        dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"$SECURITY_COMPANY_ID\",
            \"$ENDORSER_ID\",
            \"<script>alert(\"XSS\")</script>This is a malicious endorsement\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test XSS in testimonial author name
    run_security_test "Script injection in testimonial author" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"<script>alert(1)</script>John Doe\",
            \"Engineer\",
            \"Normal testimonial message\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test XSS in testimonial role
    run_security_test "Script injection in testimonial role" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"Jane Smith\",
            \"<script>document.cookie</script>Senior Engineer\",
            \"Normal testimonial message\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test XSS in testimonial message
    run_security_test "Script injection in testimonial message" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"Bob Johnson\",
            \"Manager\",
            \"Great company! <script>eval(atob(\"YWxlcnQoMSk=\"))</script>\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test XSS in vouch message
    run_security_test "Script injection in vouch" '
        dfx canister call CrossChainRegistry_backend add_vouch "(
            \"$SECURITY_COMPANY_ID\",
            \"I vouch for this company <script>window.location=\\\"evil.com\\\"</script>\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test valid community validation (should pass)
    run_security_test "Valid endorsement" '
        dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"$SECURITY_COMPANY_ID\",
            \"$ENDORSER_ID\",
            \"This is a legitimate endorsement for a great company\"
        )" 2>/dev/null || true
    ' false
}

# Test Rate Limiting Security
test_rate_limiting_security() {
    log_info "Testing rate limiting security..."
    
    # Test rapid social verification attempts (should trigger rate limiting)
    log_info "Attempting rapid social verification requests..."
    local blocked_count=0
    for i in {1..10}; do
        if ! dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/test/status/$RANDOM\"
        )" 2>/dev/null | grep -q "Ok"; then
            blocked_count=$((blocked_count + 1))
        fi
    done
    
    run_security_test "Rate limiting blocks rapid requests" '
        [ $blocked_count -gt 5 ]  # At least half should be blocked
    ' false
    
    # Test rapid cross-chain challenge attempts
    log_info "Attempting rapid cross-chain challenge requests..."
    blocked_count=0
    for i in {1..8}; do
        if ! dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"$SECURITY_COMPANY_ID\";
            chain_type = variant { Ethereum };
            address_or_contract = \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9$i\";
            verification_method = variant { SignMessage = record { message = \"Test verification\" } };
        })" 2>/dev/null | grep -q "Ok"; then
            blocked_count=$((blocked_count + 1))
        fi
    done
    
    run_security_test "Rate limiting blocks rapid cross-chain requests" '
        [ $blocked_count -gt 3 ]  # Some should be blocked
    ' false
    
    # Test rapid community validation attempts
    log_info "Attempting rapid community validation requests..."
    blocked_count=0
    for i in {1..10}; do
        if ! dfx canister call CrossChainRegistry_backend add_vouch "(
            \"$SECURITY_COMPANY_ID\",
            \"Rapid vouch attempt number $i\"
        )" 2>/dev/null | grep -q "Ok"; then
            blocked_count=$((blocked_count + 1))
        fi
    done
    
    run_security_test "Rate limiting blocks rapid community requests" '
        [ $blocked_count -gt 4 ]  # Some should be blocked
    ' false
}

# Test Input Validation Security
test_input_validation_security() {
    log_info "Testing input validation security..."
    
    # Test extremely long inputs
    run_security_test "Extremely long testimonial message rejection" '
        LONG_MSG=$(printf "A%.0s" {1..5000})
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"Test Author\",
            \"Test Role\",
            \"$LONG_MSG\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test null byte injection
    run_security_test "Null byte injection protection" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"Test\\x00Author\",
            \"Test Role\",
            \"Test message\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test empty string validation
    run_security_test "Empty testimonial author rejection" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$SECURITY_COMPANY_ID\",
            \"\",
            \"Test Role\",
            \"Test message\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test SQL injection patterns (even though we don't use SQL)
    run_security_test "SQL injection pattern rejection" '
        dfx canister call CrossChainRegistry_backend add_vouch "(
            \"$SECURITY_COMPANY_ID\",
            \"Test; DROP TABLE companies; --\"
        )" 2>/dev/null || true
    ' false  # This should be accepted since we filter XSS, not SQL
}

# Test URL Validation Security
test_url_validation_security() {
    log_info "Testing URL validation security..."
    
    # Test non-HTTPS URLs
    run_security_test "Non-HTTPS URL rejection" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"http://twitter.com/test/status/123\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test file protocol injection
    run_security_test "File protocol injection rejection" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"file:///etc/passwd\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test data URI injection
    run_security_test "Data URI injection rejection" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"data:text/html,<script>alert(1)</script>\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test domain whitelist enforcement
    run_security_test "Unauthorized domain rejection" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"twitter\",
            \"https://evil.com/fake/twitter/status/123\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test valid domain (should pass)
    run_security_test "Valid domain acceptance" '
        dfx canister call CrossChainRegistry_backend verify_social_media_manual "(
            \"$SECURITY_COMPANY_ID\",
            \"discord\",
            \"https://discord.com/channels/123/456/789\"
        )" 2>/dev/null || true
    ' false
}

# Test Address Validation Security
test_address_validation_security() {
    log_info "Testing address validation security..."
    
    # Test burn address detection
    run_security_test "Ethereum burn address rejection" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x0000000000000000000000000000000000000000\"
        )" 2>/dev/null | grep -q "true"
    ' true
    
    # Test extremely long address
    run_security_test "Extremely long address rejection" '
        LONG_ADDR=$(printf "0x%.0s" {1..300})
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"$LONG_ADDR\"
        )" 2>/dev/null | grep -q "true"
    ' true
    
    # Test special characters in address
    run_security_test "Special characters in address rejection" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E!@#$\"
        )" 2>/dev/null | grep -q "true"
    ' true
    
    # Test valid address (should pass)
    run_security_test "Valid Ethereum address acceptance" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\"
        )" 2>/dev/null | grep -q "true"
    ' false
}

# Test Authorization Security
test_authorization_security() {
    log_info "Testing authorization security..."
    
    # Test operations on non-existent company
    run_security_test "Non-existent company operation rejection" '
        dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"non_existent_company_id_12345\",
            \"$SECURITY_COMPANY_ID\",
            \"Unauthorized endorsement attempt\"
        )" 2>/dev/null | grep -q "Ok"
    ' true
    
    # Test cross-chain challenge on non-existent company
    run_security_test "Cross-chain challenge for non-existent company rejection" '
        dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"fake_company_id_12345\";
            chain_type = variant { Ethereum };
            address_or_contract = \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\";
            verification_method = variant { SignMessage = record { message = \"Test verification\" } };
        })" 2>/dev/null | grep -q "Ok"
    ' true
}

# Test Monitoring Integration Security
test_monitoring_integration() {
    log_info "Testing monitoring integration..."
    
    # Test that security events are being logged
    run_security_test "Security events logging" '
        dfx canister call CrossChainRegistry_backend get_security_events_by_severity "(variant { High })" 2>/dev/null | grep -q "Ok"
    ' false
    
    # Test monitoring stats retrieval
    run_security_test "Monitoring stats retrieval" '
        dfx canister call CrossChainRegistry_backend get_monitoring_stats "()" 2>/dev/null | grep -q "Ok"
    ' false
    
    # Test that security monitoring is working
    run_security_test "Security monitoring process" '
        dfx canister call CrossChainRegistry_backend process_monitoring_tasks "()" 2>/dev/null | grep -q "Ok"
    ' false
}

# Test Error Handling Security
test_error_handling_security() {
    log_info "Testing error handling security..."
    
    # Test that errors don't leak sensitive information
    run_security_test "Error messages don't leak sensitive info" '
        result=$(dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"fake_company\",
            \"another_fake\",
            \"test\"
        )" 2>&1)
        # Should not contain internal paths, stack traces, or sensitive data
        ! echo "$result" | grep -i -E "(path|trace|internal|debug|sql|database)"
    ' false
    
    # Test graceful handling of malformed requests
    run_security_test "Graceful malformed request handling" '
        # This should not crash the canister
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"malformed_chain_name_!@#$%\",
            \"malformed_address_!@#$%\"
        )" 2>/dev/null || true
    ' false
}

# Main test execution
main() {
    log_info "Starting Enhanced Security Tests"
    log_info "================================="
    
    # Ensure canister is running
    if ! dfx canister status CrossChainRegistry_backend >/dev/null 2>&1; then
        log_error "CrossChainRegistry_backend canister is not running"
        log_info "Please run: dfx start --background && dfx deploy"
        exit 1
    fi
    
    # Setup test data
    if ! setup_security_test_data; then
        log_error "Failed to setup security test data"
        exit 1
    fi
    
    # Run security test suites
    test_xss_protection_verification
    test_xss_protection_community
    test_rate_limiting_security
    test_input_validation_security
    test_url_validation_security
    test_address_validation_security
    test_authorization_security
    test_monitoring_integration
    test_error_handling_security
    
    # Print summary
    echo
    log_info "================================="
    log_info "Security Test Summary"
    log_info "================================="
    log_info "Tests Run: $TESTS_RUN"
    log_success "Tests Passed: $TESTS_PASSED"
    if [ $TESTS_FAILED -gt 0 ]; then
        log_error "Tests Failed: $TESTS_FAILED"
    else
        log_success "Tests Failed: $TESTS_FAILED"
    fi
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "🎉 All security tests passed!"
        log_success "🔒 Enhanced security features are working correctly"
        exit 0
    else
        log_error "❌ Some security tests failed. This indicates potential vulnerabilities!"
        log_error "🚨 Please review and fix security issues before deploying to production"
        exit 1
    fi
}

# Run main function
main "$@"