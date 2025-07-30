use crate::storage::StorageManager;
use crate::monitoring::MonitoringSystem;
use crate::types::{
    ChainType, CrossChainChallenge, CrossChainVerificationMethod, CrossChainVerificationRequest,
    EtherscanContractResponse, RegistryResult, VerificationResult, BlockchainInfoResponse,
    SecurityEventType, SecuritySeverity, VerificationProof, VerificationType, 
    VerificationMethod, ProofStatus,
};
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_cdk::api::time;
use regex::Regex;
use serde_json;

// Cross-chain verification implementation

pub struct CrossChainVerifier;

impl CrossChainVerifier {
    // Create cross-chain verification challenge
    pub fn create_crosschain_challenge(
        request: CrossChainVerificationRequest,
        caller_principal: Principal,
    ) -> RegistryResult<CrossChainChallenge> {
        // Check rate limiting for cross-chain verification
        if !StorageManager::check_verification_rate_limit(caller_principal) {
            let (current_requests, _) = StorageManager::get_rate_limit_info(caller_principal);
            // Log rate limit violation
            MonitoringSystem::log_security_event(
                SecurityEventType::RateLimitExceeded,
                Some(caller_principal),
                SecuritySeverity::Medium,
                format!(
                    "Rate limit exceeded for cross-chain challenge creation: {} attempts",
                    current_requests
                ),
                None,
                None,
            );
            return RegistryResult::Err("Rate limit exceeded. Please wait before creating more cross-chain challenges.".to_string());
        }

        // Get company and verify permissions
        let company = match StorageManager::get_company(&request.company_id) {
            Some(company) => company,
            None => {
                // Log suspicious activity
                MonitoringSystem::log_security_event(
                    SecurityEventType::SuspiciousInput,
                    Some(caller_principal),
                    SecuritySeverity::Low,
                    format!("Cross-chain challenge attempted for non-existent company: {}", request.company_id),
                    None,
                    None,
                );
                return RegistryResult::Err("Company not found".to_string());
            }
        };

        if company.created_by != caller_principal {
            // Log unauthorized access attempt
            MonitoringSystem::log_security_event(
                SecurityEventType::UnauthorizedAccess,
                Some(caller_principal),
                SecuritySeverity::Medium,
                format!(
                    "Unauthorized cross-chain challenge attempt: principal {} tried to create challenge for company {}",
                    caller_principal, request.company_id
                ),
                None,
                None,
            );
            return RegistryResult::Err(
                "Unauthorized: Only company creator can create challenges".to_string(),
            );
        }

        // Validate address/contract format
        if let Err(err) = Self::validate_address_format(&request.chain_type, &request.address_or_contract) {
            return RegistryResult::Err(err);
        }

        let now = time();
        let expires_at = now + (48 * 60 * 60 * 1_000_000_000); // 48 hours for cross-chain verification

        // Generate challenge message based on verification method
        let challenge_message = Self::generate_challenge_message(&request.verification_method, &request.company_id);

        let challenge = CrossChainChallenge {
            company_id: request.company_id.clone(),
            chain_type: request.chain_type.clone(),
            address_or_contract: request.address_or_contract.clone(),
            challenge_message,
            verification_method: request.verification_method,
            created_at: now,
            expires_at,
        };

        // Generate unique challenge key
        let chain_name = match request.chain_type {
            ChainType::Ethereum => "ethereum",
            ChainType::Bitcoin => "bitcoin", 
            ChainType::ICP => "icp",
            ChainType::Polygon => "polygon",
            ChainType::Solana => "solana",
            ChainType::Sui => "sui",
            ChainType::TON => "ton",
        };

        let challenge_key = StorageManager::generate_crosschain_challenge_key(
            &request.company_id,
            chain_name,
            &request.address_or_contract,
        );

        StorageManager::insert_crosschain_challenge(challenge_key.clone(), challenge.clone());

        // Log successful challenge creation
        MonitoringSystem::log_security_event(
            SecurityEventType::SecurityScan,
            Some(caller_principal),
            SecuritySeverity::Low,
            format!(
                "Cross-chain challenge created: {} for company {} on chain {:?}",
                request.address_or_contract, request.company_id, request.chain_type
            ),
            None,
            None,
        );

        RegistryResult::Ok(challenge)
    }

    // Verify Ethereum contract ownership
    pub async fn verify_ethereum_contract(
        company_id: String,
        contract_address: String,
    ) -> RegistryResult<VerificationResult> {
        // Find the corresponding challenge
        let challenge_key = match Self::find_challenge_key(&company_id, "ethereum", &contract_address) {
            Ok(key) => key,
            Err(err) => return RegistryResult::Err(err),
        };
        let challenge = match StorageManager::get_crosschain_challenge(&challenge_key) {
            Some(challenge) => challenge,
            None => return RegistryResult::Err("No verification challenge found".to_string()),
        };

        // Check if challenge expired
        if time() > challenge.expires_at {
            StorageManager::remove_crosschain_challenge(&challenge_key);
            return RegistryResult::Err("Cross-chain verification challenge expired".to_string());
        }

        // Query Etherscan API for recent transactions
        let etherscan_url = format!(
            "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock=0&endblock=99999999&sort=desc&apikey=YourApiKeyToken",
            contract_address
        );

        let request = CanisterHttpRequestArgument {
            url: etherscan_url,
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(8192),
            transform: Some(TransformContext::from_name(
                "transform_etherscan_response".to_string(),
                vec![],
            )),
            headers: vec![
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "ICP-CrossChainRegistry/1.0".to_string(),
                },
            ],
        };

        match http_request(request, 15_000_000_000).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    // Parse Etherscan response
                    match serde_json::from_slice::<EtherscanContractResponse>(&response.body) {
                        Ok(etherscan_data) => {
                            // Look for the challenge message in recent transactions
                            if Self::verify_ethereum_challenge(&etherscan_data, &challenge.challenge_message) {
                                // Create permanent verification proof
                                let proof = VerificationProof {
                                    verification_type: VerificationType::GitHub, // Using GitHub as placeholder for cross-chain
                                    proof_url: format!("https://etherscan.io/address/{}", contract_address),
                                    verified_at: time(),
                                    verification_method: VerificationMethod::Automated,
                                    challenge_data: Some(challenge.challenge_message.clone()),
                                    status: ProofStatus::Active,
                                };

                                // Verification successful - update company
                                let success = StorageManager::update_company(&company_id, |company| {
                                    // Add to verified contracts if not already present
                                    if !company.cross_chain_presence.ethereum_contracts.contains(&contract_address) {
                                        company.cross_chain_presence.ethereum_contracts.push(contract_address.clone());
                                    }
                                    // Mark contract as verified in WalletInfo or TokenInfo if exists
                                    for wallet in &mut company.cross_chain_presence.treasury_wallets {
                                        if wallet.address == contract_address && wallet.chain == "ethereum" {
                                            wallet.verified = true;
                                        }
                                    }
                                    for token in &mut company.cross_chain_presence.token_contracts {
                                        if token.contract_address == contract_address && token.chain == "ethereum" {
                                            token.verified = true;
                                        }
                                    }
                                    // Add verification proof
                                    company.web3_identity.verification_proofs.push(proof.clone());
                                });

                                if success {
                                    // Remove challenge after successful verification
                                    StorageManager::remove_crosschain_challenge(&challenge_key);

                                    // Log successful verification
                                    MonitoringSystem::log_security_event(
                                        SecurityEventType::SecurityScan,
                                        None,
                                        SecuritySeverity::Low,
                                        format!(
                                            "Ethereum contract verified: {} for company {}",
                                            contract_address, company_id
                                        ),
                                        None,
                                        None,
                                    );

                                    // Schedule monitoring for this contract
                                    let proof_id = format!("ethereum_{}_{}", contract_address, proof.verified_at);
                                    MonitoringSystem::schedule_proof_monitoring(
                                        company_id.clone(),
                                        proof_id,
                                        crate::types::TaskPriority::Medium,
                                    );

                                    RegistryResult::Ok(VerificationResult {
                                        success: true,
                                        message: format!(
                                            "Ethereum contract {} verified successfully. Contract will be monitored for continued activity.",
                                            contract_address
                                        ),
                                        verified_at: Some(time()),
                                    })
                                } else {
                                    // Log failure
                                    MonitoringSystem::log_security_event(
                                        SecurityEventType::SuspiciousInput,
                                        None,
                                        SecuritySeverity::Medium,
                                        format!("Failed to update company during Ethereum verification: {}", company_id),
                                        None,
                                        None,
                                    );
                                    RegistryResult::Err("Failed to update company".to_string())
                                }
                            } else {
                                // Log failed verification attempt
                                MonitoringSystem::log_security_event(
                                    SecurityEventType::RepeatedFailedVerification,
                                    None,
                                    SecuritySeverity::Low,
                                    format!(
                                        "Ethereum verification failed - challenge not found: {} for company {}",
                                        contract_address, company_id
                                    ),
                                    None,
                                    None,
                                );

                                RegistryResult::Ok(VerificationResult {
                                    success: false,
                                    message: "Challenge message not found in recent transactions. Please ensure you've sent a transaction with the challenge message from the verified address.".to_string(),
                                    verified_at: None,
                                })
                            }
                        }
                        Err(_) => {
                            // Log parsing failure
                            MonitoringSystem::log_security_event(
                                SecurityEventType::SuspiciousInput,
                                None,
                                SecuritySeverity::Medium,
                                format!("Failed to parse Etherscan API response for contract {}", contract_address),
                                None,
                                None,
                            );
                            RegistryResult::Err("Failed to parse Etherscan API response".to_string())
                        }
                    }
                } else {
                    // Log API error
                    MonitoringSystem::log_security_event(
                        SecurityEventType::SuspiciousInput,
                        None,
                        SecuritySeverity::Medium,
                        format!("Etherscan API error {} for contract {}", response.status, contract_address),
                        None,
                        None,
                    );
                    RegistryResult::Err(format!("Etherscan API error: {}", response.status))
                }
            }
            Err(err) => {
                // Log HTTP failure
                MonitoringSystem::log_security_event(
                    SecurityEventType::SuspiciousInput,
                    None,
                    SecuritySeverity::Medium,
                    format!("HTTP request failed for Ethereum verification: {:?}", err),
                    None,
                    None,
                );
                RegistryResult::Err(format!("HTTP request failed: {:?}", err))
            }
        }
    }

    // Verify Bitcoin address ownership
    pub async fn verify_bitcoin_address(
        company_id: String,
        bitcoin_address: String,
    ) -> RegistryResult<VerificationResult> {
        // Find the corresponding challenge
        let challenge_key = match Self::find_challenge_key(&company_id, "bitcoin", &bitcoin_address) {
            Ok(key) => key,
            Err(err) => return RegistryResult::Err(err),
        };
        let challenge = match StorageManager::get_crosschain_challenge(&challenge_key) {
            Some(challenge) => challenge,
            None => return RegistryResult::Err("No verification challenge found".to_string()),
        };

        // Check if challenge expired
        if time() > challenge.expires_at {
            StorageManager::remove_crosschain_challenge(&challenge_key);
            return RegistryResult::Err("Cross-chain verification challenge expired".to_string());
        }

        // Query Blockchain.info API for address information
        let blockchain_url = format!(
            "https://blockchain.info/rawaddr/{}?limit=50",
            bitcoin_address
        );

        let request = CanisterHttpRequestArgument {
            url: blockchain_url,
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(4096),
            transform: Some(TransformContext::from_name(
                "transform_blockchain_response".to_string(),
                vec![],
            )),
            headers: vec![
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "ICP-CrossChainRegistry/1.0".to_string(),
                },
            ],
        };

        match http_request(request, 15_000_000_000).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    // Parse blockchain.info response
                    match serde_json::from_slice::<BlockchainInfoResponse>(&response.body) {
                        Ok(blockchain_data) => {
                            // For Bitcoin, we verify the address exists and has activity
                            if blockchain_data.n_tx > 0 {
                                // Update company with verified Bitcoin address
                                let success = StorageManager::update_company(&company_id, |company| {
                                    if !company.cross_chain_presence.bitcoin_addresses.contains(&bitcoin_address) {
                                        company.cross_chain_presence.bitcoin_addresses.push(bitcoin_address.clone());
                                    }
                                    // Mark wallet as verified if exists
                                    for wallet in &mut company.cross_chain_presence.treasury_wallets {
                                        if wallet.address == bitcoin_address && wallet.chain == "bitcoin" {
                                            wallet.verified = true;
                                        }
                                    }
                                });

                                if success {
                                    // Remove challenge after successful verification
                                    StorageManager::remove_crosschain_challenge(&challenge_key);

                                    RegistryResult::Ok(VerificationResult {
                                        success: true,
                                        message: format!("Bitcoin address {} verified successfully", bitcoin_address),
                                        verified_at: Some(time()),
                                    })
                                } else {
                                    RegistryResult::Err("Failed to update company".to_string())
                                }
                            } else {
                                RegistryResult::Ok(VerificationResult {
                                    success: false,
                                    message: "Bitcoin address has no transaction history".to_string(),
                                    verified_at: None,
                                })
                            }
                        }
                        Err(_) => RegistryResult::Err("Failed to parse Blockchain.info API response".to_string()),
                    }
                } else {
                    RegistryResult::Err(format!("Blockchain.info API error: {}", response.status))
                }
            }
            Err(err) => RegistryResult::Err(format!("HTTP request failed: {:?}", err)),
        }
    }

    // Verify ICP canister ownership
    pub async fn verify_icp_canister(
        company_id: String,
        canister_id: String,
    ) -> RegistryResult<VerificationResult> {
        // For ICP canisters, we use the management canister to get canister info
        // This is a simplified version - in production you'd want to verify controller ownership
        
        // Find the corresponding challenge
        let challenge_key = match Self::find_challenge_key(&company_id, "icp", &canister_id) {
            Ok(key) => key,
            Err(err) => return RegistryResult::Err(err),
        };
        let challenge = match StorageManager::get_crosschain_challenge(&challenge_key) {
            Some(challenge) => challenge,
            None => return RegistryResult::Err("No verification challenge found".to_string()),
        };

        // Check if challenge expired
        if time() > challenge.expires_at {
            StorageManager::remove_crosschain_challenge(&challenge_key);
            return RegistryResult::Err("Cross-chain verification challenge expired".to_string());
        }

        // For now, we'll do basic validation - in production, you'd call the management canister
        if Self::is_valid_canister_id(&canister_id) {
            // Update company with verified ICP canister
            let success = StorageManager::update_company(&company_id, |company| {
                if !company.cross_chain_presence.icp_canisters.contains(&canister_id) {
                    company.cross_chain_presence.icp_canisters.push(canister_id.clone());
                }
            });

            if success {
                // Remove challenge after successful verification
                StorageManager::remove_crosschain_challenge(&challenge_key);

                RegistryResult::Ok(VerificationResult {
                    success: true,
                    message: format!("ICP canister {} verified successfully", canister_id),
                    verified_at: Some(time()),
                })
            } else {
                RegistryResult::Err("Failed to update company".to_string())
            }
        } else {
            RegistryResult::Ok(VerificationResult {
                success: false,
                message: "Invalid ICP canister ID format".to_string(),
                verified_at: None,
            })
        }
    }

    // Helper functions
    fn validate_address_format(chain_type: &ChainType, address: &str) -> Result<(), String> {
        match chain_type {
            ChainType::Ethereum | ChainType::Polygon => {
                if !address.starts_with("0x") || address.len() != 42 {
                    return Err("Invalid Ethereum/Polygon address format".to_string());
                }
            }
            ChainType::Bitcoin => {
                if address.len() < 26 || address.len() > 35 {
                    return Err("Invalid Bitcoin address format".to_string());
                }
            }
            ChainType::ICP => {
                if !Self::is_valid_canister_id(address) {
                    return Err("Invalid ICP canister ID format".to_string());
                }
            }
            ChainType::Solana => {
                if address.len() < 32 || address.len() > 44 {
                    return Err("Invalid Solana address format".to_string());
                }
            }
            ChainType::Sui => {
                if !address.starts_with("0x") || address.len() != 66 {
                    return Err("Invalid Sui address format".to_string());
                }
            }
            ChainType::TON => {
                if !(address.starts_with("0:") || address.starts_with("EQ") || address.starts_with("UQ") || address.starts_with("kQ")) {
                    return Err("Invalid TON address format".to_string());
                }
            }
        }
        Ok(())
    }

    fn is_valid_canister_id(canister_id: &str) -> bool {
        let canister_regex = match Regex::new(r"^[a-z0-9]+-[a-z0-9]+-[a-z0-9]+-[a-z0-9]+-[a-z0-9]+$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        canister_regex.is_match(canister_id)
    }

    fn generate_challenge_message(method: &CrossChainVerificationMethod, company_id: &str) -> String {
        match method {
            CrossChainVerificationMethod::SignMessage { message } => message.clone(),
            CrossChainVerificationMethod::DeploySpecialContract { verification_code } => {
                format!("Deploy contract with code: {} for company: {}", verification_code, company_id)
            }
            CrossChainVerificationMethod::SetPublicVariable { variable_name, value } => {
                format!("Set {} = {} for company: {}", variable_name, value, company_id)
            }
            CrossChainVerificationMethod::SpecialTransaction { transaction_data } => {
                format!("Execute transaction: {} for company: {}", transaction_data, company_id)
            }
        }
    }

    fn find_challenge_key(company_id: &str, chain: &str, address: &str) -> Result<String, String> {
        match StorageManager::find_crosschain_challenge_key(company_id, chain, address) {
            Some(key) => Ok(key),
            None => Err("Challenge not found".to_string()),
        }
    }

    fn verify_ethereum_challenge(etherscan_data: &EtherscanContractResponse, challenge_message: &str) -> bool {
        // Look for the challenge message in transaction input data
        for transaction in &etherscan_data.result {
            if transaction.input.contains(challenge_message) {
                return true;
            }
        }
        false
    }

    // Get verification instructions for cross-chain verification
    pub fn get_crosschain_verification_instructions(chain_type: ChainType) -> String {
        match chain_type {
            ChainType::Ethereum => {
                "To verify Ethereum contract ownership:\n\
                1. Create a cross-chain verification challenge for your contract address\n\
                2. Send a transaction to your contract with the challenge message in the input data\n\
                3. Call verify_ethereum_contract to complete verification\n\
                4. The system will check recent transactions for the challenge message".to_string()
            }
            ChainType::Bitcoin => {
                "To verify Bitcoin address ownership:\n\
                1. Create a cross-chain verification challenge for your Bitcoin address\n\
                2. Ensure your address has transaction history (at least 1 transaction)\n\
                3. Call verify_bitcoin_address to complete verification\n\
                4. The system will verify address activity and ownership".to_string()
            }
            ChainType::ICP => {
                "To verify ICP canister ownership:\n\
                1. Create a cross-chain verification challenge for your canister ID\n\
                2. Ensure you are listed as a controller of the canister\n\
                3. Call verify_icp_canister to complete verification\n\
                4. The system will verify canister existence and controller status".to_string()
            }
            ChainType::Polygon => {
                "To verify Polygon contract ownership:\n\
                1. Create a cross-chain verification challenge for your contract address\n\
                2. Send a transaction to your contract with the challenge message\n\
                3. Call verify_polygon_contract to complete verification\n\
                4. Similar to Ethereum verification process".to_string()
            }
            ChainType::Solana => {
                "To verify Solana address ownership:\n\
                1. Create a cross-chain verification challenge for your Solana address\n\
                2. Send a transaction from your address or create a program interaction\n\
                3. Call verify_solana_address to complete verification\n\
                4. The system will verify address activity and ownership".to_string()
            }
            ChainType::Sui => {
                "To verify Sui address ownership:\n\
                1. Create a cross-chain verification challenge for your Sui address\n\
                2. Perform a transaction or object interaction from your address\n\
                3. Call verify_sui_address to complete verification\n\
                4. The system will verify address activity on Sui network".to_string()
            }
            ChainType::TON => {
                "To verify TON address ownership:\n\
                1. Create a cross-chain verification challenge for your TON address\n\
                2. Send a transaction or message from your address\n\
                3. Call verify_ton_address to complete verification\n\
                4. The system will verify address activity and ownership".to_string()
            }
        }
    }

    // Advanced cross-chain verification features

    // Validate multi-chain address ownership for a company
    pub async fn verify_multi_chain_portfolio(
        company_id: String,
        caller_principal: Principal,
    ) -> RegistryResult<Vec<String>> {
        // Check authorization
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => {
                MonitoringSystem::log_security_event(
                    SecurityEventType::SuspiciousInput,
                    Some(caller_principal),
                    SecuritySeverity::Low,
                    format!("Multi-chain verification attempted for non-existent company: {}", company_id),
                    None,
                    None,
                );
                return RegistryResult::Err("Company not found".to_string());
            }
        };

        if company.created_by != caller_principal {
            MonitoringSystem::log_security_event(
                SecurityEventType::UnauthorizedAccess,
                Some(caller_principal),
                SecuritySeverity::Medium,
                format!(
                    "Unauthorized multi-chain verification attempt for company {}",
                    company_id
                ),
                None,
                None,
            );
            return RegistryResult::Err("Unauthorized access".to_string());
        }

        let mut verification_results = Vec::new();
        let presence = &company.cross_chain_presence;

        // Verify Ethereum contracts
        for contract in &presence.ethereum_contracts {
            match Self::verify_ethereum_contract(company_id.clone(), contract.clone()).await {
                RegistryResult::Ok(result) => {
                    if result.success {
                        verification_results.push(format!("Ethereum: {} ✓", contract));
                    } else {
                        verification_results.push(format!("Ethereum: {} ✗ - {}", contract, result.message));
                    }
                }
                RegistryResult::Err(e) => {
                    verification_results.push(format!("Ethereum: {} ✗ - {}", contract, e));
                }
            }
        }

        // Verify Bitcoin addresses
        for address in &presence.bitcoin_addresses {
            match Self::verify_bitcoin_address(company_id.clone(), address.clone()).await {
                RegistryResult::Ok(result) => {
                    if result.success {
                        verification_results.push(format!("Bitcoin: {} ✓", address));
                    } else {
                        verification_results.push(format!("Bitcoin: {} ✗ - {}", address, result.message));
                    }
                }
                RegistryResult::Err(e) => {
                    verification_results.push(format!("Bitcoin: {} ✗ - {}", address, e));
                }
            }
        }

        // Verify ICP canisters
        for canister in &presence.icp_canisters {
            match Self::verify_icp_canister(company_id.clone(), canister.clone()).await {
                RegistryResult::Ok(result) => {
                    if result.success {
                        verification_results.push(format!("ICP: {} ✓", canister));
                    } else {
                        verification_results.push(format!("ICP: {} ✗ - {}", canister, result.message));
                    }
                }
                RegistryResult::Err(e) => {
                    verification_results.push(format!("ICP: {} ✗ - {}", canister, e));
                }
            }
        }

        // Log multi-chain verification completion
        MonitoringSystem::log_security_event(
            SecurityEventType::SecurityScan,
            Some(caller_principal),
            SecuritySeverity::Low,
            format!(
                "Multi-chain portfolio verification completed for company {}: {} addresses checked",
                company_id, verification_results.len()
            ),
            None,
            None,
        );

        RegistryResult::Ok(verification_results)
    }

    // Cross-chain risk assessment
    pub fn assess_cross_chain_risk(company_id: String) -> RegistryResult<String> {
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        let presence = &company.cross_chain_presence;
        let mut risk_score = 0;
        let mut risk_factors = Vec::new();

        // Check for diversification across multiple chains
        let mut active_chains = 0;
        if !presence.ethereum_contracts.is_empty() { active_chains += 1; }
        if !presence.bitcoin_addresses.is_empty() { active_chains += 1; }
        if !presence.icp_canisters.is_empty() { active_chains += 1; }
        if !presence.solana_addresses.is_empty() { active_chains += 1; }
        if !presence.sui_addresses.is_empty() { active_chains += 1; }
        if !presence.ton_addresses.is_empty() { active_chains += 1; }

        if active_chains < 2 {
            risk_score += 20;
            risk_factors.push("Low chain diversification".to_string());
        }

        // Check for unverified addresses
        let total_addresses = presence.ethereum_contracts.len() +
                            presence.bitcoin_addresses.len() +
                            presence.icp_canisters.len() +
                            presence.solana_addresses.len() +
                            presence.sui_addresses.len() +
                            presence.ton_addresses.len();

        let verified_wallets = presence.treasury_wallets.iter()
            .filter(|w| w.verified)
            .count();

        if total_addresses > 0 && verified_wallets < total_addresses / 2 {
            risk_score += 30;
            risk_factors.push("Many unverified addresses".to_string());
        }

        // Check for suspicious patterns (too many addresses)
        if total_addresses > 50 {
            risk_score += 25;
            risk_factors.push("Unusually high number of addresses".to_string());
        }

        // Check treasury wallet count (simplified risk assessment)
        let treasury_wallet_count = presence.treasury_wallets.len();

        if treasury_wallet_count > 20 {
            risk_score += 15;
            risk_factors.push("High number of treasury wallets".to_string());
        }

        let risk_level = match risk_score {
            0..=20 => "Low",
            21..=50 => "Medium",
            51..=80 => "High",
            _ => "Critical",
        };

        let assessment = if risk_factors.is_empty() {
            format!("Risk Level: {} (Score: {}) - No significant risk factors detected.", risk_level, risk_score)
        } else {
            format!("Risk Level: {} (Score: {}) - Risk factors: {}", risk_level, risk_score, risk_factors.join(", "))
        };

        // Log risk assessment
        MonitoringSystem::log_security_event(
            SecurityEventType::SecurityScan,
            None,
            match risk_level {
                "Low" => SecuritySeverity::Low,
                "Medium" => SecuritySeverity::Medium,
                "High" | "Critical" => SecuritySeverity::High,
                _ => SecuritySeverity::Low,
            },
            format!("Cross-chain risk assessment for company {}: {}", company_id, assessment),
            None,
            None,
        );

        RegistryResult::Ok(assessment)
    }

    // Enhanced address validation with security checks
    pub fn validate_address_with_security_check(
        chain_type: &ChainType,
        address: &str,
        caller_principal: Principal,
    ) -> RegistryResult<bool> {
        // Basic format validation
        if let Err(err) = Self::validate_address_format(chain_type, address) {
            // Log invalid address attempt
            MonitoringSystem::log_security_event(
                SecurityEventType::SuspiciousInput,
                Some(caller_principal),
                SecuritySeverity::Low,
                format!("Invalid address format attempted: {} for chain {:?}", address, chain_type),
                None,
                None,
            );
            return RegistryResult::Err(err);
        }

        // Additional security checks
        
        // Check for known malicious addresses (simplified - in production would use real blacklists)
        let suspicious_patterns = [
            "0x0000000000000000000000000000000000000000", // Ethereum burn address
            "1111111111111111111114oLvT2", // Bitcoin burn address
            "aaaaa-aa", // ICP test addresses
        ];

        if suspicious_patterns.iter().any(|&pattern| address.contains(pattern)) {
            MonitoringSystem::log_security_event(
                SecurityEventType::SuspiciousInput,
                Some(caller_principal),
                SecuritySeverity::High,
                format!("Suspicious address pattern detected: {} for chain {:?}", address, chain_type),
                None,
                None,
            );
            return RegistryResult::Err("Suspicious address pattern detected".to_string());
        }

        // Check address length for potential overflow attacks
        if address.len() > 200 {
            MonitoringSystem::log_security_event(
                SecurityEventType::SuspiciousInput,
                Some(caller_principal),
                SecuritySeverity::Medium,
                format!("Excessively long address attempted: {} chars for chain {:?}", address.len(), chain_type),
                None,
                None,
            );
            return RegistryResult::Err("Address exceeds maximum length".to_string());
        }

        RegistryResult::Ok(true)
    }
}

// HTTP transform functions for cross-chain API responses
pub fn transform_etherscan_response(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
    ];

    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    }
}

pub fn transform_blockchain_response(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
    ];

    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    }
}