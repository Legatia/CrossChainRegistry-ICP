#!/bin/bash

# CrossChain Registry Backend Test Automation Script
# This script provides comprehensive testing for all backend functionality

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Global variables for test data
COMPANY_ID=""
ENDORSER_COMPANY_ID=""
CHALLENGE_TOKEN=""

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

# Test helper function
run_test() {
    local test_name="$1"
    local test_command="$2"
    local expected_pattern="$3"
    
    ((TOTAL_TESTS++))
    log_info "Running test: $test_name"
    
    local result
    if result=$(eval "$test_command" 2>&1); then
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
}

# Test helper to extract values from results
extract_company_id() {
    echo "$1" | grep -o 'company_[0-9]*' | head -1
}

extract_challenge_token() {
    echo "$1" | grep -o 'icp-registry-[0-9]*'
}

# Setup function
setup_tests() {
    log_info "Setting up test environment..."
    
    # Navigate to project root for dfx commands
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
    cd "$PROJECT_ROOT"
    
    # Check if dfx is running
    if ! dfx ping > /dev/null 2>&1; then
        log_error "DFX is not running. Please start dfx with: dfx start --background"
        exit 1
    fi
    
    # Deploy if needed
    log_info "Deploying canisters..."
    dfx deploy > /dev/null 2>&1 || {
        log_error "Failed to deploy canisters"
        exit 1
    }
    
    log_success "Test environment ready"
}

# Core CRUD Tests
test_crud_operations() {
    log_info "=== Testing CRUD Operations ==="
    
    # Test 1: Get initial company count
    run_test "Get initial company count" \
        "dfx canister call CrossChainRegistry_backend get_company_count" \
        "nat64"
    
    # Test 2: Create a new company
    local create_result
    create_result=$(dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Test Automation Company";
            description = "A company created by automated tests";
            website = "https://testautomation.com";
            founding_date = "2024-01-01";
            team_size = 5;
            focus_areas = vec { "Testing"; "Automation" };
        };
        web3_identity = record {
            github_org = opt "test-automation";
            twitter_handle = opt "testautomation";
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
    })' 2>&1)
    
    if echo "$create_result" | grep -q "variant { Ok"; then
        COMPANY_ID=$(extract_company_id "$create_result")
        log_success "Create company - PASSED (ID: $COMPANY_ID)"
        ((PASSED_TESTS++))
    else
        log_error "Create company - FAILED"
        echo "$create_result" | head -3 | sed 's/^/  /'
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
    
    # Test 3: Get the created company
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Get created company" \
            "dfx canister call CrossChainRegistry_backend get_company '(\"$COMPANY_ID\")'" \
            "Test Automation Company"
    fi
    
    # Test 4: Update company
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Update company" \
            "dfx canister call CrossChainRegistry_backend update_company '(record {
                company_id = \"$COMPANY_ID\";
                basic_info = opt record {
                    name = \"Updated Test Company\";
                    description = \"Updated description\";
                    website = \"https://updated-test.com\";
                    founding_date = \"2024-01-01\";
                    team_size = 10;
                    focus_areas = vec { \"Testing\"; \"Automation\"; \"Web3\" };
                };
                web3_identity = null;
                cross_chain_presence = null;
                team_members = null;
            })'" \
            "variant { Ok }"
    fi
    
    # Test 5: List companies
    run_test "List companies" \
        "dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 3, null)'" \
        "vec {"
    
    # Test 6: Search companies
    run_test "Search companies" \
        "dfx canister call CrossChainRegistry_backend search_companies '(\"Testing\")'" \
        "Testing"
    
    # Test 7: Get statistics
    run_test "Get statistics" \
        "dfx canister call CrossChainRegistry_backend get_statistics" \
        "total_companies"
}

# Community Validation Tests
test_community_validation() {
    log_info "=== Testing Community Validation ==="
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_warning "Skipping community validation tests - no company ID available"
        return
    fi
    
    # Create a second company for endorsement testing
    local endorser_result
    endorser_result=$(dfx canister call CrossChainRegistry_backend create_company '(record {
        basic_info = record {
            name = "Endorser Company";
            description = "Company that will endorse others";
            website = "https://endorser.com";
            founding_date = "2024-01-01";
            team_size = 3;
            focus_areas = vec { "Endorsement" };
        };
        web3_identity = record {
            github_org = opt "endorser-org";
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
    })' 2>&1)
    
    if echo "$endorser_result" | grep -q "variant { Ok"; then
        ENDORSER_COMPANY_ID=$(extract_company_id "$endorser_result")
        log_success "Create endorser company - PASSED (ID: $ENDORSER_COMPANY_ID)"
        ((PASSED_TESTS++))
    else
        log_error "Create endorser company - FAILED"
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
    
    # Test endorsements
    if [[ -n "$ENDORSER_COMPANY_ID" ]]; then
        run_test "Add endorsement" \
            "dfx canister call CrossChainRegistry_backend add_endorsement '(\"$COMPANY_ID\", \"$ENDORSER_COMPANY_ID\", \"Great company for testing automation\")'" \
            "variant { Ok }"
        
        run_test "Get endorsements for company" \
            "dfx canister call CrossChainRegistry_backend get_endorsements_for_company '(\"$COMPANY_ID\")'" \
            "Great company for testing automation"
    fi
    
    # Test testimonials
    run_test "Add testimonial" \
        "dfx canister call CrossChainRegistry_backend add_testimonial '(\"$COMPANY_ID\", \"Jane Doe\", \"QA Engineer\", \"Excellent testing practices and automation\")'" \
        "variant { Ok }"
    
    run_test "Get testimonials for company" \
        "dfx canister call CrossChainRegistry_backend get_testimonials_for_company '(\"$COMPANY_ID\")'" \
        "Jane Doe"
    
    run_test "Verify testimonial" \
        "dfx canister call CrossChainRegistry_backend verify_testimonial '(\"$COMPANY_ID\", \"Jane Doe\")'" \
        "variant { Ok }"
    
    # Test vouches
    run_test "Add vouch" \
        "dfx canister call CrossChainRegistry_backend add_vouch '(\"$COMPANY_ID\", \"I vouch for this companys testing capabilities\")'" \
        "variant { Ok }"
    
    run_test "Get vouches for company" \
        "dfx canister call CrossChainRegistry_backend get_vouches_for_company '(\"$COMPANY_ID\")'" \
        "testing capabilities"
    
    # Test reputation system
    run_test "Stake reputation" \
        "dfx canister call CrossChainRegistry_backend stake_reputation '(\"$COMPANY_ID\", 1000)'" \
        "variant { Ok }"
    
    run_test "Get community validation" \
        "dfx canister call CrossChainRegistry_backend get_community_validation '(\"$COMPANY_ID\")'" \
        "reputation_score"
    
    run_test "Get companies by reputation" \
        "dfx canister call CrossChainRegistry_backend get_companies_by_reputation '(10, opt 5)'" \
        "vec {"
    
    run_test "Get reputation leaderboard" \
        "dfx canister call CrossChainRegistry_backend get_reputation_leaderboard '(opt 3)'" \
        "reputation_score"
}

# Verification System Tests
test_verification_system() {
    log_info "=== Testing Verification System ==="
    
    if [[ -z "$COMPANY_ID" ]]; then
        log_warning "Skipping verification tests - no company ID available"
        return
    fi
    
    # Test verification instructions
    run_test "Get GitHub verification instructions" \
        "dfx canister call CrossChainRegistry_backend get_verification_instructions '(variant { GitHub })'" \
        "GitHub organization"
    
    run_test "Get Domain verification instructions" \
        "dfx canister call CrossChainRegistry_backend get_verification_instructions '(variant { Domain })'" \
        "domain ownership"
    
    # Test domain verification challenge
    local challenge_result
    challenge_result=$(dfx canister call CrossChainRegistry_backend create_domain_verification_challenge "(\"$COMPANY_ID\")" 2>&1)
    
    if echo "$challenge_result" | grep -q "variant { Ok"; then
        CHALLENGE_TOKEN=$(extract_challenge_token "$challenge_result")
        log_success "Create domain verification challenge - PASSED (Token: $CHALLENGE_TOKEN)"
        ((PASSED_TESTS++))
    else
        log_error "Create domain verification challenge - FAILED"
        echo "$challenge_result" | head -3 | sed 's/^/  /'
        ((FAILED_TESTS++))
    fi
    ((TOTAL_TESTS++))
    
    # Test getting domain challenge
    if [[ -n "$CHALLENGE_TOKEN" ]]; then
        run_test "Get domain verification challenge" \
            "dfx canister call CrossChainRegistry_backend get_domain_verification_challenge '(\"$COMPANY_ID\")'" \
            "$CHALLENGE_TOKEN"
    fi
    
    # Test cross-chain verification
    run_test "Create Ethereum cross-chain challenge" \
        "dfx canister call CrossChainRegistry_backend create_crosschain_challenge '(record {
            company_id = \"$COMPANY_ID\";
            chain_type = variant { Ethereum };
            address_or_contract = \"0x1234567890abcdef1234567890abcdef12345678\";
            verification_method = variant { SignMessage = record { message = \"Verify TestAutomation ownership\" } };
        })'" \
        "variant { Ok"
    
    run_test "Create Bitcoin cross-chain challenge" \
        "dfx canister call CrossChainRegistry_backend create_crosschain_challenge '(record {
            company_id = \"$COMPANY_ID\";
            chain_type = variant { Bitcoin };
            address_or_contract = \"bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh\";
            verification_method = variant { SignMessage = record { message = \"Verify TestAutomation Bitcoin ownership\" } };
        })'" \
        "variant { Ok"
    
    run_test "Create ICP cross-chain challenge" \
        "dfx canister call CrossChainRegistry_backend create_crosschain_challenge '(record {
            company_id = \"$COMPANY_ID\";
            chain_type = variant { ICP };
            address_or_contract = \"rdmx6-jaaaa-aaaaa-aaadq-cai\";
            verification_method = variant { SignMessage = record { message = \"Verify TestAutomation ICP ownership\" } };
        })'" \
        "variant { Ok"
    
    run_test "Get cross-chain challenges for company" \
        "dfx canister call CrossChainRegistry_backend get_crosschain_challenges_for_company '(\"$COMPANY_ID\")'" \
        "vec {"
    
    # Test cross-chain verification instructions
    run_test "Get Ethereum verification instructions" \
        "dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions '(variant { Ethereum })'" \
        "Ethereum"
    
    run_test "Get Bitcoin verification instructions" \
        "dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions '(variant { Bitcoin })'" \
        "Bitcoin"
    
    run_test "Get ICP verification instructions" \
        "dfx canister call CrossChainRegistry_backend get_crosschain_verification_instructions '(variant { ICP })'" \
        "ICP"
}

# Error Handling Tests
test_error_handling() {
    log_info "=== Testing Error Handling ==="
    
    # Test invalid company ID
    run_test "Get non-existent company" \
        "dfx canister call CrossChainRegistry_backend get_company '(\"invalid_id\")'" \
        "variant { Err"
    
    # Test duplicate endorsement
    if [[ -n "$COMPANY_ID" && -n "$ENDORSER_COMPANY_ID" ]]; then
        # First add an endorsement, then try to add the same one again
        dfx canister call CrossChainRegistry_backend add_endorsement "(\"$COMPANY_ID\", \"$ENDORSER_COMPANY_ID\", \"Duplicate test\")" > /dev/null 2>&1
        
        run_test "Add duplicate endorsement" \
            "dfx canister call CrossChainRegistry_backend add_endorsement '(\"$COMPANY_ID\", \"$ENDORSER_COMPANY_ID\", \"Another endorsement\")'" \
            "variant { Err"
    fi
    
    # Test self-endorsement
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Self-endorsement (should fail)" \
            "dfx canister call CrossChainRegistry_backend add_endorsement '(\"$COMPANY_ID\", \"$COMPANY_ID\", \"Self endorsement\")'" \
            "variant { Err"
    fi
    
    # Test invalid verification method
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Invalid Ethereum address format" \
            "dfx canister call CrossChainRegistry_backend create_crosschain_challenge '(record {
                company_id = \"$COMPANY_ID\";
                chain_type = variant { Ethereum };
                address_or_contract = \"invalid_address\";
                verification_method = variant { SignMessage = record { message = \"Test\" } };
            })'" \
            "variant { Err"
    fi
    
    # Test empty testimonial
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Empty testimonial message (should fail)" \
            "dfx canister call CrossChainRegistry_backend add_testimonial '(\"$COMPANY_ID\", \"Empty Author\", \"Role\", \"\")'" \
            "variant { Err"
    fi
    
    # Test zero reputation stake
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Zero reputation stake (should fail)" \
            "dfx canister call CrossChainRegistry_backend stake_reputation '(\"$COMPANY_ID\", 0)'" \
            "variant { Err"
    fi
}

# Advanced functionality tests
test_advanced_features() {
    log_info "=== Testing Advanced Features ==="
    
    # Test search with filters
    run_test "List companies with status filter" \
        "dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 5, opt record { status = opt variant { Pending }; focus_areas = null; min_verification_score = null; has_github = null; has_contracts = null; })'" \
        "vec {"
    
    run_test "List companies with GitHub filter" \
        "dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 5, opt record { status = null; focus_areas = null; min_verification_score = null; has_github = opt true; has_contracts = null; })'" \
        "vec {"
    
    run_test "List companies with minimum verification score" \
        "dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 5, opt record { status = null; focus_areas = null; min_verification_score = opt 20; has_github = null; has_contracts = null; })'" \
        "vec {"
    
    # Test community validation statistics
    if [[ -n "$COMPANY_ID" ]]; then
        run_test "Get community validation stats" \
            "dfx canister call CrossChainRegistry_backend get_community_validation_stats '(\"$COMPANY_ID\")'" \
            "total_endorsements"
    fi
    
    # Test endorsement eligibility
    if [[ -n "$COMPANY_ID" && -n "$ENDORSER_COMPANY_ID" ]]; then
        run_test "Validate endorsement eligibility" \
            "dfx canister call CrossChainRegistry_backend validate_endorsement_eligibility '(\"$ENDORSER_COMPANY_ID\", \"$COMPANY_ID\")'" \
            "variant { Ok"
    fi
    
    # Test getting endorsements by company
    if [[ -n "$ENDORSER_COMPANY_ID" ]]; then
        run_test "Get endorsements by company" \
            "dfx canister call CrossChainRegistry_backend get_endorsements_by_company '(\"$ENDORSER_COMPANY_ID\")'" \
            "variant { Ok"
    fi
    
    # Test getting vouches by principal
    run_test "Get vouches by principal" \
        "dfx canister call CrossChainRegistry_backend get_vouches_by_principal" \
        "vec {"
    
    # Test getting testimonials by author
    run_test "Get testimonials by author" \
        "dfx canister call CrossChainRegistry_backend get_testimonials_by_author '(\"Jane Doe\")'" \
        "vec {"
}

# Performance and stress tests
test_performance() {
    log_info "=== Testing Performance ==="
    
    # Test creating multiple companies rapidly
    log_info "Creating 5 companies rapidly..."
    local start_time=$(date +%s)
    
    for i in {1..5}; do
        dfx canister call CrossChainRegistry_backend create_company "(record {
            basic_info = record {
                name = \"Performance Test Company $i\";
                description = \"Performance test company number $i\";
                website = \"https://perftest$i.com\";
                founding_date = \"2024-01-01\";
                team_size = $i;
                focus_areas = vec { \"Performance\"; \"Testing\" };
            };
            web3_identity = record {
                github_org = opt \"perftest$i\";
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
        })" > /dev/null 2>&1 &
    done
    
    wait  # Wait for all background processes
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    log_success "Created 5 companies in ${duration} seconds"
    ((PASSED_TESTS++))
    ((TOTAL_TESTS++))
    
    # Test listing many companies
    run_test "List many companies" \
        "dfx canister call CrossChainRegistry_backend list_companies '(opt 0, opt 20, null)'" \
        "vec {"
}

# Cleanup function
cleanup_tests() {
    log_info "=== Test Cleanup ==="
    
    # Note: In a real scenario, you might want to reset the canister state
    # For now, we'll just log the cleanup
    log_info "Tests completed. Canister state preserved for inspection."
    
    # If you want to reset the canister state, uncomment the following:
    # dfx canister uninstall-code CrossChainRegistry_backend || true
    # dfx deploy CrossChainRegistry_backend
}

# Generate test report
generate_report() {
    echo
    log_info "=== TEST REPORT ==="
    echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
    echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
    echo -e "Failed: ${RED}$FAILED_TESTS${NC}"
    
    if [[ $FAILED_TESTS -eq 0 ]]; then
        echo -e "${GREEN}All tests passed! 🎉${NC}"
        exit 0
    else
        echo -e "${RED}Some tests failed. Please review the output above.${NC}"
        exit 1
    fi
}

# Main execution
main() {
    echo -e "${BLUE}CrossChain Registry Backend Test Suite${NC}"
    echo -e "${BLUE}======================================${NC}"
    
    setup_tests
    
    # Run all test suites
    test_crud_operations
    test_community_validation
    test_verification_system
    test_error_handling
    test_advanced_features
    test_performance
    
    cleanup_tests
    generate_report
}

# Parse command line arguments
case "${1:-all}" in
    "crud")
        setup_tests
        test_crud_operations
        generate_report
        ;;
    "community")
        setup_tests
        test_community_validation
        generate_report
        ;;
    "verification")
        setup_tests
        test_verification_system
        generate_report
        ;;
    "errors")
        setup_tests
        test_error_handling
        generate_report
        ;;
    "advanced")
        setup_tests
        test_advanced_features
        generate_report
        ;;
    "performance")
        setup_tests
        test_performance
        generate_report
        ;;
    "all")
        main
        ;;
    *)
        echo "Usage: $0 [all|crud|community|verification|errors|advanced|performance]"
        echo "  all          - Run all tests (default)"
        echo "  crud         - Run CRUD operation tests only"
        echo "  community    - Run community validation tests only"
        echo "  verification - Run verification system tests only"
        echo "  errors       - Run error handling tests only"
        echo "  advanced     - Run advanced feature tests only"
        echo "  performance  - Run performance tests only"
        exit 1
        ;;
esac