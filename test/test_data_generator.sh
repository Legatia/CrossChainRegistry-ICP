#!/bin/bash

# Test Data Generator for CrossChain Registry
# This script generates test data for comprehensive testing

set -e

# Source test configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_FILE="$SCRIPT_DIR/test_config.json"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Generate sample companies
generate_sample_companies() {
    log_info "Generating sample companies..."
    
    local companies=(
        "DeFi Protocol Inc|Leading DeFi protocol for automated market making|https://defiprotocol.io|2023-03-15|25|DeFi,AMM,Liquidity|defi-protocol|defiprotocol"
        "NFT Marketplace Corp|Premier NFT marketplace with cross-chain support|https://nftmarketplace.com|2022-08-20|15|NFTs,Marketplace,Web3|nft-marketplace|nftmarketplace"
        "Cross-Chain Bridge LLC|Secure and fast cross-chain asset transfers|https://crosschainbridge.net|2024-01-10|30|Bridge,Interoperability,Security|cross-chain-bridge|crosschainbridge"
        "Analytics Platform|Blockchain analytics and data insights|https://analyticsplatform.com|2023-11-05|12|Analytics,Data,Insights|analytics-platform|analyticsplatform"
        "Gaming DAO|Decentralized gaming platform and community|https://gamingdao.org|2023-07-22|18|Gaming,DAO,Community|gaming-dao|gamingdao"
    )
    
    local company_ids=()
    
    for company_data in "${companies[@]}"; do
        IFS='|' read -r name description website founding_date team_size focus_areas github_org twitter_handle <<< "$company_data"
        
        # Convert focus_areas to Candid format
        local focus_areas_candid=$(echo "$focus_areas" | sed 's/,/"; "/g' | sed 's/^/"/' | sed 's/$/"/')
        
        local result=$(dfx canister call CrossChainRegistry_backend create_company "(record {
            basic_info = record {
                name = \"$name\";
                description = \"$description\";
                website = \"$website\";
                founding_date = \"$founding_date\";
                team_size = $team_size;
                focus_areas = vec { $focus_areas_candid };
            };
            web3_identity = record {
                github_org = opt \"$github_org\";
                twitter_handle = opt \"$twitter_handle\";
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
        })" 2>&1)
        
        if echo "$result" | grep -q "variant { Ok"; then
            local company_id=$(echo "$result" | grep -o 'company_[0-9]*')
            company_ids+=("$company_id")
            log_success "Created company: $name (ID: $company_id)"
        else
            log_warning "Failed to create company: $name"
        fi
    done
    
    echo "${company_ids[@]}"
}

# Add team members to companies
add_team_members() {
    local company_ids=("$@")
    log_info "Adding team members to companies..."
    
    local team_members=(
        "John Doe|CEO|john-doe|john-doe-ceo"
        "Jane Smith|CTO|jane-smith-dev|jane-smith-cto"
        "Mike Johnson|Lead Developer|mike-johnson|mike-johnson-dev"
        "Sarah Williams|Product Manager|sarah-williams|sarah-williams-pm"
        "David Brown|Marketing Director|david-brown|david-brown-marketing"
    )
    
    for company_id in "${company_ids[@]}"; do
        if [[ -n "$company_id" ]]; then
            # Add 2-3 random team members to each company
            local num_members=$((2 + RANDOM % 2))
            local selected_members=()
            
            # Randomly select team members
            while [[ ${#selected_members[@]} -lt $num_members ]]; do
                local member_index=$((RANDOM % ${#team_members[@]}))
                local member="${team_members[$member_index]}"
                
                if [[ ! " ${selected_members[@]} " =~ " ${member} " ]]; then
                    selected_members+=("$member")
                fi
            done
            
            # Add each selected member
            for member_data in "${selected_members[@]}"; do
                IFS='|' read -r name role github linkedin <<< "$member_data"
                
                dfx canister call CrossChainRegistry_backend update_company "(record {
                    company_id = \"$company_id\";
                    basic_info = null;
                    web3_identity = null;
                    cross_chain_presence = null;
                    team_members = opt vec {
                        record {
                            name = \"$name\";
                            role = \"$role\";
                            github_profile = opt \"$github\";
                            linkedin_profile = opt \"$linkedin\";
                            verified = false;
                        };
                    };
                })" > /dev/null 2>&1
            done
            
            log_success "Added ${#selected_members[@]} team members to $company_id"
        fi
    done
}

# Generate cross-chain data
add_crosschain_presence() {
    local company_ids=("$@")
    log_info "Adding cross-chain presence data..."
    
    local eth_contracts=(
        "0x1234567890abcdef1234567890abcdef12345678"
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
        "0x9876543210fedcba9876543210fedcba98765432"
        "0x1111222233334444555566667777888899990000"
        "0xaaabbbcccdddeeefffaaabbbcccdddeeefffaaa"
    )
    
    local btc_addresses=(
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq"
        "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"
        "bc1qm34lsc65zpw79lxes69zkqmk6ee3ewf0j77s3h"
    )
    
    local icp_canisters=(
        "rdmx6-jaaaa-aaaaa-aaadq-cai"
        "rrkah-fqaaa-aaaah-qcuiq-cai"
        "ryjl3-tyaaa-aaaaa-aaaba-cai"
        "rdmx6-jaaaa-aaaaa-aaada-cai"
    )
    
    for company_id in "${company_ids[@]}"; do
        if [[ -n "$company_id" ]]; then
            # Randomly assign some cross-chain assets
            local eth_contract=""
            local btc_address=""
            local icp_canister=""
            
            if [[ $((RANDOM % 3)) -eq 0 ]]; then
                eth_contract="\"${eth_contracts[$((RANDOM % ${#eth_contracts[@]}))]}\""
            fi
            
            if [[ $((RANDOM % 3)) -eq 0 ]]; then
                btc_address="\"${btc_addresses[$((RANDOM % ${#btc_addresses[@]}))]}\""
            fi
            
            if [[ $((RANDOM % 2)) -eq 0 ]]; then
                icp_canister="\"${icp_canisters[$((RANDOM % ${#icp_canisters[@]}))]}\""
            fi
            
            # Build the cross-chain presence record
            local eth_contracts_vec="vec { $(echo "$eth_contract" | sed 's/^"//;s/"$//') }"
            local btc_addresses_vec="vec { $(echo "$btc_address" | sed 's/^"//;s/"$//') }"
            local icp_canisters_vec="vec { $(echo "$icp_canister" | sed 's/^"//;s/"$//') }"
            
            if [[ -z "$eth_contract" ]]; then eth_contracts_vec="vec {}"; fi
            if [[ -z "$btc_address" ]]; then btc_addresses_vec="vec {}"; fi
            if [[ -z "$icp_canister" ]]; then icp_canisters_vec="vec {}"; fi
            
            dfx canister call CrossChainRegistry_backend update_company "(record {
                company_id = \"$company_id\";
                basic_info = null;
                web3_identity = null;
                cross_chain_presence = opt record {
                    ethereum_contracts = $eth_contracts_vec;
                    bitcoin_addresses = $btc_addresses_vec;
                    icp_canisters = $icp_canisters_vec;
                    polygon_contracts = vec {};
                    treasury_wallets = vec {};
                    token_contracts = vec {};
                };
                team_members = null;
            })" > /dev/null 2>&1
            
            log_success "Added cross-chain presence to $company_id"
        fi
    done
}

# Generate community validation data
generate_community_data() {
    local company_ids=("$@")
    log_info "Generating community validation data..."
    
    local testimonials=(
        "Alice Johnson|Senior Developer|Excellent company with innovative blockchain solutions"
        "Bob Smith|Product Manager|Great team culture and cutting-edge technology stack"
        "Carol Williams|QA Engineer|High quality code and rigorous testing practices"
        "David Lee|Designer|Outstanding user experience and design thinking"
        "Emma Davis|Marketing|Strong community engagement and brand presence"
    )
    
    local vouch_messages=(
        "Reliable partner in the blockchain space"
        "Innovative solutions and strong technical team"
        "Trustworthy company with proven track record"
        "Excellent collaboration and professional service"
        "Leading technology and great community support"
    )
    
    # Add testimonials
    for company_id in "${company_ids[@]}"; do
        if [[ -n "$company_id" ]]; then
            local num_testimonials=$((1 + RANDOM % 3))
            
            for ((i=0; i<num_testimonials; i++)); do
                local testimonial_index=$((RANDOM % ${#testimonials[@]}))
                IFS='|' read -r author role message <<< "${testimonials[$testimonial_index]}"
                
                dfx canister call CrossChainRegistry_backend add_testimonial "(\"$company_id\", \"$author\", \"$role\", \"$message\")" > /dev/null 2>&1
                
                # Randomly verify some testimonials
                if [[ $((RANDOM % 2)) -eq 0 ]]; then
                    dfx canister call CrossChainRegistry_backend verify_testimonial "(\"$company_id\", \"$author\")" > /dev/null 2>&1
                fi
            done
            
            # Add vouches
            local num_vouches=$((1 + RANDOM % 2))
            for ((i=0; i<num_vouches; i++)); do
                local message_index=$((RANDOM % ${#vouch_messages[@]}))
                local message="${vouch_messages[$message_index]}"
                
                dfx canister call CrossChainRegistry_backend add_vouch "(\"$company_id\", \"$message\")" > /dev/null 2>&1
            done
            
            log_success "Added community data to $company_id"
        fi
    done
    
    # Add cross-endorsements between companies
    log_info "Creating cross-endorsements..."
    for ((i=0; i<${#company_ids[@]}; i++)); do
        for ((j=0; j<${#company_ids[@]}; j++)); do
            if [[ $i -ne $j ]] && [[ $((RANDOM % 4)) -eq 0 ]]; then
                local endorser_id="${company_ids[$i]}"
                local target_id="${company_ids[$j]}"
                
                if [[ -n "$endorser_id" && -n "$target_id" ]]; then
                    dfx canister call CrossChainRegistry_backend add_endorsement "(\"$target_id\", \"$endorser_id\", \"Great partner company with excellent service\")" > /dev/null 2>&1
                fi
            fi
        done
    done
}

# Main function
main() {
    log_info "Starting test data generation..."
    
    # Check if dfx is available
    if ! command -v dfx &> /dev/null; then
        echo "Error: dfx is not installed or not in PATH"
        exit 1
    fi
    
    # Navigate to project root for dfx commands
    cd "$PROJECT_ROOT"
    
    # Check if canister is deployed
    if ! dfx canister call CrossChainRegistry_backend get_company_count > /dev/null 2>&1; then
        echo "Error: CrossChainRegistry_backend canister is not deployed"
        echo "Please run 'dfx deploy' first"
        exit 1
    fi
    
    # Generate data
    local company_ids
    company_ids=($(generate_sample_companies))
    
    if [[ ${#company_ids[@]} -gt 0 ]]; then
        add_team_members "${company_ids[@]}"
        add_crosschain_presence "${company_ids[@]}"
        generate_community_data "${company_ids[@]}"
        
        log_success "Test data generation completed!"
        log_info "Generated ${#company_ids[@]} companies with full data sets"
        
        # Show summary
        echo
        echo "Company IDs created:"
        for id in "${company_ids[@]}"; do
            echo "  - $id"
        done
    else
        echo "No companies were created successfully"
        exit 1
    fi
}

# Parse command line options
case "${1:-generate}" in
    "generate")
        main
        ;;
    "clean")
        log_info "Cleaning up test data..."
        cd "$PROJECT_ROOT"
        # Note: This would require implementing a cleanup function
        # For now, the canister would need to be redeployed to clean data
        dfx canister stop CrossChainRegistry_backend || true
        dfx canister delete CrossChainRegistry_backend || true
        dfx deploy CrossChainRegistry_backend
        log_success "Test data cleaned (canister redeployed)"
        ;;
    *)
        echo "Usage: $0 [generate|clean]"
        echo "  generate - Generate test data (default)"
        echo "  clean    - Clean up test data by redeploying canister"
        exit 1
        ;;
esac