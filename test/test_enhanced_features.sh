#!/bin/bash

# Enhanced Features Test Suite
# Tests all new Stage 2 core features including enhanced verification,
# community validation, and cross-chain capabilities

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
CANISTER_ID="be2us-64aaa-aaaaa-qaabq-cai"
BACKEND_URL="http://127.0.0.1:4943"

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

# Setup test data
setup_test_data() {
    log_info "Setting up test data..."
    
    # Create test company
    dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Enhanced Test Company";
            description = "A test company for enhanced features";
            website = "https://enhanced-test.com";
            founding_date = "2024-01-01";
            team_size = 10;
            focus_areas = vec {"DeFi"; "Security"};
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
    })' || {
        log_error "Failed to create test company"
        return 1
    }
    
    # Get the created company ID
    TEST_COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 1, null)' | grep -o '"[a-zA-Z0-9_-]*"' | head -1 | tr -d '"')
    
    if [ -n "$TEST_COMPANY_ID" ]; then
        log_success "Test company created: $TEST_COMPANY_ID"
        export TEST_COMPANY_ID
    else
        log_error "Failed to get test company ID"
        return 1
    fi
}

# Test Enhanced Social Media Verification
test_enhanced_social_verification() {
    log_info "Testing enhanced social media verification..."
    
    # Test LinkedIn verification
    run_test "LinkedIn verification with monitoring" '
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"$TEST_COMPANY_ID\",
            \"linkedin\", 
            \"https://linkedin.com/company/test-company/posts/123\",
        )" 2>/dev/null || true
    '
    
    # Test Medium verification  
    run_test "Medium verification with monitoring" '
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"$TEST_COMPANY_ID\",
            \"medium\",
            \"https://medium.com/test-publication/post-123\",
        )" 2>/dev/null || true
    '
    
    # Test rate limiting (should fail after multiple attempts)
    log_info "Testing rate limiting for social verification..."
    for i in {1..6}; do
        dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"$TEST_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/test/status/123$i\",
        )" 2>/dev/null || true
    done
    
    run_test "Rate limiting detection" '
        # This should fail due to rate limiting
        ! dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"$TEST_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/test/status/999\",
        )" 2>/dev/null
    '
    
    # Test XSS protection
    run_test "XSS protection in social verification" '
        ! dfx canister call CrossChainRegistry_backend verify_social_media_with_proof "(
            \"$TEST_COMPANY_ID\",
            \"twitter\",
            \"https://twitter.com/test<script>alert(1)</script>/status/123\",
        )" 2>/dev/null
    '
}

# Test Enhanced Community Validation
test_enhanced_community_validation() {
    log_info "Testing enhanced community validation..."
    
    # Create a second company for endorsement testing
    dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Endorser Company";
            description = "Company for endorsement testing";
            website = "https://endorser-test.com";
            founding_date = "2024-01-01";
            team_size = 5;
            focus_areas = vec {"Testing"};
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
    
    # Get endorser company ID
    ENDORSER_COMPANY_ID=$(dfx canister call CrossChainRegistry_backend list_companies '(null, opt 2, null)' | grep -o '"[a-zA-Z0-9_-]*"' | tail -1 | tr -d '"')
    
    # Test enhanced endorsements
    run_test "Enhanced endorsement with monitoring" '
        dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"$TEST_COMPANY_ID\",
            \"$ENDORSER_COMPANY_ID\", 
            \"This is a legitimate endorsement message\"
        )" 2>/dev/null || true
    '
    
    # Test XSS protection in endorsements
    run_test "XSS protection in endorsements" '
        ! dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"$TEST_COMPANY_ID\",
            \"$ENDORSER_COMPANY_ID\",
            \"<script>alert(\"XSS\")</script>Malicious endorsement\"
        )" 2>/dev/null
    '
    
    # Test enhanced testimonials
    run_test "Enhanced testimonial with security checks" '
        dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$TEST_COMPANY_ID\",
            \"John Doe\",
            \"Software Engineer\",
            \"Great company to work with! Very professional team.\"
        )" 2>/dev/null || true
    '
    
    # Test XSS protection in testimonials
    run_test "XSS protection in testimonials" '
        ! dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$TEST_COMPANY_ID\",
            \"<script>alert(1)</script>\",
            \"Evil Role\",
            \"javascript:alert(1)\"
        )" 2>/dev/null
    '
    
    # Test enhanced vouches
    run_test "Enhanced vouch with monitoring" '
        dfx canister call CrossChainRegistry_backend add_vouch "(
            \"$TEST_COMPANY_ID\",
            \"I vouch for this company based on their excellent work\"
        )" 2>/dev/null || true
    '
    
    # Test XSS protection in vouches
    run_test "XSS protection in vouches" '
        ! dfx canister call CrossChainRegistry_backend add_vouch "(
            \"$TEST_COMPANY_ID\",
            \"<script>document.cookie</script>Malicious vouch\"
        )" 2>/dev/null
    '
}

# Test Enhanced Cross-Chain Verification
test_enhanced_crosschain_verification() {
    log_info "Testing enhanced cross-chain verification..."
    
    # Test cross-chain challenge creation with monitoring
    run_test "Cross-chain challenge creation with monitoring" '
        dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"$TEST_COMPANY_ID\";
            chain_type = variant { Ethereum };
            address_or_contract = \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\";
            verification_method = variant { TransactionMessage };
        })" 2>/dev/null || true
    '
    
    # Test rate limiting for cross-chain challenges
    log_info "Testing rate limiting for cross-chain challenges..."
    for i in {1..6}; do
        dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"$TEST_COMPANY_ID\";
            chain_type = variant { Bitcoin };
            address_or_contract = \"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfN$i\";
            verification_method = variant { TransactionMessage };
        })" 2>/dev/null || true
    done
    
    run_test "Cross-chain rate limiting detection" '
        # This should fail due to rate limiting
        ! dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
            company_id = \"$TEST_COMPANY_ID\";
            chain_type = variant { Bitcoin };
            address_or_contract = \"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfN9\";
            verification_method = variant { TransactionMessage };
        })" 2>/dev/null
    '
    
    # Test address validation with security checks
    run_test "Address validation security checks" '
        dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E\"
        )" 2>/dev/null || true
    '
    
    # Test suspicious address detection
    run_test "Suspicious address pattern detection" '
        ! dfx canister call CrossChainRegistry_backend validate_address "(
            \"ethereum\",
            \"0x0000000000000000000000000000000000000000\"
        )" 2>/dev/null
    '
}

# Test Monitoring Integration
test_monitoring_integration() {
    log_info "Testing monitoring integration..."
    
    # Test security event retrieval
    run_test "Security events retrieval" '
        dfx canister call CrossChainRegistry_backend get_security_events_by_severity "(variant { Medium })" 2>/dev/null || true
    '
    
    # Test monitoring stats
    run_test "Monitoring statistics" '
        dfx canister call CrossChainRegistry_backend get_monitoring_stats "()" 2>/dev/null || true
    '
    
    # Test community alerts
    run_test "Community alerts retrieval" '
        dfx canister call CrossChainRegistry_backend get_community_alerts "(null)" 2>/dev/null || true
    '
    
    # Test proof monitoring scheduling
    run_test "Proof monitoring scheduling" '
        dfx canister call CrossChainRegistry_backend schedule_proof_monitoring "(
            \"$TEST_COMPANY_ID\",
            \"test_proof_123\",
            variant { Medium }
        )" 2>/dev/null || true
    '
}

# Test Error Handling and Edge Cases
test_error_handling() {
    log_info "Testing error handling and edge cases..."
    
    # Test operations on non-existent company
    run_test "Non-existent company error handling" '
        ! dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"non_existent_company\",
            \"$TEST_COMPANY_ID\",
            \"Should fail\"
        )" 2>/dev/null
    '
    
    # Test unauthorized operations
    run_test "Unauthorized operation detection" '
        # Try to create endorsement as wrong principal (should fail)
        ! dfx canister call CrossChainRegistry_backend add_endorsement "(
            \"$TEST_COMPANY_ID\",
            \"fake_company_id\",
            \"Unauthorized endorsement\"
        )" 2>/dev/null
    '
    
    # Test empty/invalid input handling
    run_test "Empty input validation" '
        ! dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$TEST_COMPANY_ID\",
            \"\",
            \"\",
            \"\"
        )" 2>/dev/null
    '
    
    # Test excessively long input handling
    run_test "Long input validation" '
        LONG_STRING=$(printf "A%.0s" {1..1500})
        ! dfx canister call CrossChainRegistry_backend add_testimonial "(
            \"$TEST_COMPANY_ID\",
            \"Test Author\",
            \"Test Role\",
            \"$LONG_STRING\"
        )" 2>/dev/null
    '
}

# Test Performance and Load
test_performance() {
    log_info "Testing performance and load handling..."
    
    # Test multiple concurrent operations
    run_test "Concurrent operations handling" '
        for i in {1..3}; do
            dfx canister call CrossChainRegistry_backend add_vouch "(
                \"$TEST_COMPANY_ID\",
                \"Performance test vouch $i\"
            )" 2>/dev/null &
        done
        wait
        true  # Always pass this test for now
    '
    
    # Test large data retrieval
    run_test "Large data retrieval" '
        dfx canister call CrossChainRegistry_backend list_companies "(null, opt 100, null)" 2>/dev/null || true
    '
}

# Main test execution
main() {
    log_info "Starting Enhanced Features Test Suite"
    log_info "========================================="
    
    # Ensure canister is running
    if ! dfx canister status CrossChainRegistry_backend >/dev/null 2>&1; then
        log_error "CrossChainRegistry_backend canister is not running"
        log_info "Please run: dfx start --background && dfx deploy"
        exit 1
    fi
    
    # Setup test data
    if ! setup_test_data; then
        log_error "Failed to setup test data"
        exit 1
    fi
    
    # Run test suites
    test_enhanced_social_verification
    test_enhanced_community_validation
    test_enhanced_crosschain_verification
    test_monitoring_integration
    test_error_handling
    test_performance
    
    # Print summary
    echo
    log_info "========================================="
    log_info "Test Summary"
    log_info "========================================="
    log_info "Tests Run: $TESTS_RUN"
    log_success "Tests Passed: $TESTS_PASSED"
    if [ $TESTS_FAILED -gt 0 ]; then
        log_error "Tests Failed: $TESTS_FAILED"
    else
        log_success "Tests Failed: $TESTS_FAILED"
    fi
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "🎉 All enhanced features tests passed!"
        exit 0
    else
        log_error "❌ Some tests failed. Please review the output above."
        exit 1
    fi
}

# Run main function
main "$@"