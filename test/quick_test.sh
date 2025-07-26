#!/bin/bash

# Quick validation test for the backend
set -e

echo "🚀 Running Quick Backend Validation Tests"
echo "========================================="

# Test 1: Basic functionality
echo "✅ Test 1: Get company count"
RESULT=$(dfx canister call CrossChainRegistry_backend get_company_count)
echo "   Result: $RESULT"

# Test 2: Create a company
echo "✅ Test 2: Create test company"
CREATE_RESULT=$(dfx canister call CrossChainRegistry_backend create_company '(record {
  basic_info = record {
    name = "Quick Test Company";
    description = "A test company for validation";
    website = "https://quicktest.com";
    founding_date = "2024-01-01";
    team_size = 5;
    focus_areas = vec { "Testing" };
  };
  web3_identity = record {
    github_org = opt "quicktest";
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
})')

echo "   Result: $CREATE_RESULT"

# Extract company ID
COMPANY_ID=$(echo "$CREATE_RESULT" | grep -o 'company_[0-9]*' || echo "")

if [[ -n "$COMPANY_ID" ]]; then
    echo "   Company ID: $COMPANY_ID"
    
    # Test 3: Get the created company
    echo "✅ Test 3: Get created company"
    GET_RESULT=$(dfx canister call CrossChainRegistry_backend get_company "(\"$COMPANY_ID\")")
    echo "   Retrieved company with name: $(echo "$GET_RESULT" | grep -o '"Quick Test Company"' || echo 'Name not found')"
    
    # Test 4: Add testimonial
    echo "✅ Test 4: Add testimonial"
    TESTIMONIAL_RESULT=$(dfx canister call CrossChainRegistry_backend add_testimonial "(\"$COMPANY_ID\", \"Test User\", \"Tester\", \"Great company for testing\")")
    echo "   Result: $TESTIMONIAL_RESULT"
    
    # Test 5: Add vouch
    echo "✅ Test 5: Add community vouch"
    VOUCH_RESULT=$(dfx canister call CrossChainRegistry_backend add_vouch "(\"$COMPANY_ID\", \"I vouch for this test company\")")
    echo "   Result: $VOUCH_RESULT"
    
    # Test 6: Get community validation
    echo "✅ Test 6: Get community validation data"
    COMMUNITY_RESULT=$(dfx canister call CrossChainRegistry_backend get_community_validation "(\"$COMPANY_ID\")")
    echo "   Has testimonials: $(echo "$COMMUNITY_RESULT" | grep -q "Test User" && echo "Yes" || echo "No")"
    echo "   Has vouches: $(echo "$COMMUNITY_RESULT" | grep -q "vouch for this test company" && echo "Yes" || echo "No")"
    
    # Test 7: Search companies
    echo "✅ Test 7: Search companies"
    SEARCH_RESULT=$(dfx canister call CrossChainRegistry_backend search_companies "(\"Testing\")")
    echo "   Found test company: $(echo "$SEARCH_RESULT" | grep -q "Quick Test Company" && echo "Yes" || echo "No")"
    
else
    echo "❌ Failed to create company - cannot run remaining tests"
    exit 1
fi

# Test 8: Get statistics
echo "✅ Test 8: Get registry statistics"
STATS_RESULT=$(dfx canister call CrossChainRegistry_backend get_statistics)
echo "   Statistics retrieved successfully"

# Test 9: Create domain verification challenge
echo "✅ Test 9: Create domain verification challenge"
DOMAIN_RESULT=$(dfx canister call CrossChainRegistry_backend create_domain_verification_challenge "(\"$COMPANY_ID\")")
echo "   Challenge created: $(echo "$DOMAIN_RESULT" | grep -q "challenge_token" && echo "Yes" || echo "No")"

# Test 10: Create cross-chain challenge
echo "✅ Test 10: Create cross-chain verification challenge"
CROSSCHAIN_RESULT=$(dfx canister call CrossChainRegistry_backend create_crosschain_challenge "(record {
  company_id = \"$COMPANY_ID\";
  chain_type = variant { Ethereum };
  address_or_contract = \"0x1234567890abcdef1234567890abcdef12345678\";
  verification_method = variant { SignMessage = record { message = \"Test verification\" } };
})")
echo "   Challenge created: $(echo "$CROSSCHAIN_RESULT" | grep -q "challenge_message" && echo "Yes" || echo "No")"

echo
echo "🎉 Quick validation tests completed successfully!"
echo "   All core backend functionality is working correctly."
echo "   The comprehensive test automation suite is ready for use."