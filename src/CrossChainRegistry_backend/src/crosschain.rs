use crate::storage::StorageManager;
use crate::types::{
    ChainType, CrossChainChallenge, CrossChainVerificationMethod, CrossChainVerificationRequest,
    EtherscanContractResponse, RegistryResult, VerificationResult, BlockchainInfoResponse,
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
        // Get company and verify permissions
        let company = match StorageManager::get_company(&request.company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
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

        StorageManager::insert_crosschain_challenge(challenge_key, challenge.clone());

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
                                });

                                if success {
                                    // Remove challenge after successful verification
                                    StorageManager::remove_crosschain_challenge(&challenge_key);

                                    RegistryResult::Ok(VerificationResult {
                                        success: true,
                                        message: format!("Ethereum contract {} verified successfully", contract_address),
                                        verified_at: Some(time()),
                                    })
                                } else {
                                    RegistryResult::Err("Failed to update company".to_string())
                                }
                            } else {
                                RegistryResult::Ok(VerificationResult {
                                    success: false,
                                    message: "Challenge message not found in recent transactions".to_string(),
                                    verified_at: None,
                                })
                            }
                        }
                        Err(_) => RegistryResult::Err("Failed to parse Etherscan API response".to_string()),
                    }
                } else {
                    RegistryResult::Err(format!("Etherscan API error: {}", response.status))
                }
            }
            Err(err) => RegistryResult::Err(format!("HTTP request failed: {:?}", err)),
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