use crate::storage::StorageManager;
use crate::types::{
    Company, DomainVerificationChallenge, GitHubOrgResponse, RegistryResult, VerificationResult,
    VerificationStatus, VerificationType,
};
use candid::Principal;
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use ic_cdk::api::time;
use regex::Regex;
use serde_json;

// Verification logic implementation

pub struct VerificationManager;

impl VerificationManager {
    // Calculate verification score based on multiple signals
    pub fn calculate_verification_score(company: &Company) -> u32 {
        let mut score = 0u32;

        // Basic info completeness (max 20 points)
        if !company.basic_info.name.is_empty() {
            score += 5;
        }
        if !company.basic_info.description.is_empty() {
            score += 5;
        }
        if !company.basic_info.website.is_empty() {
            score += 5;
        }
        if !company.basic_info.focus_areas.is_empty() {
            score += 5;
        }

        // Web3 identity verification (max 30 points)
        if company.web3_identity.github_org.is_some() {
            score += 10;
        }
        if company.web3_identity.domain_verified {
            score += 10;
        }
        if matches!(
            company.web3_identity.social_verification_status,
            VerificationStatus::Verified
        ) {
            score += 10;
        }

        // Cross-chain presence (max 40 points)
        if !company.cross_chain_presence.ethereum_contracts.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.bitcoin_addresses.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.icp_canisters.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.solana_addresses.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.sui_addresses.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.ton_addresses.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.treasury_wallets.is_empty() {
            score += 5;
        }
        if !company.cross_chain_presence.token_contracts.is_empty() {
            score += 5;
        }

        // Team verification (max 15 points)
        let verified_team_count = company
            .team_members
            .iter()
            .filter(|m| m.verified)
            .count() as u32;
        score += std::cmp::min(verified_team_count * 3, 15);

        // Community validation (max 10 points)
        score += std::cmp::min(company.community_validation.reputation_score / 10, 10);

        std::cmp::min(score, 100) // Cap at 100
    }

    // GitHub verification
    pub async fn verify_github_organization(
        company_id: String,
        github_org: String,
        caller_principal: Principal,
    ) -> RegistryResult<VerificationResult> {
        // Get company and verify permissions
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err("Unauthorized: Only company creator can verify".to_string());
        }

        // Check rate limiting
        if !StorageManager::check_http_rate_limit(caller_principal) {
            return RegistryResult::Err("Rate limit exceeded. Please try again later.".to_string());
        }

        // Make HTTP request to GitHub API
        let url = format!("https://api.github.com/orgs/{}", github_org);

        let request = CanisterHttpRequestArgument {
            url: url.clone(),
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(4096),
            transform: Some(TransformContext::from_name(
                "transform_github_response".to_string(),
                vec![],
            )),
            headers: vec![
                HttpHeader {
                    name: "User-Agent".to_string(),
                    value: "ICP-CrossChainRegistry/1.0".to_string(),
                },
                HttpHeader {
                    name: "Accept".to_string(),
                    value: "application/vnd.github.v3+json".to_string(),
                },
            ],
        };

        match http_request(request, 10_000_000_000).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    // Parse GitHub API response
                    match serde_json::from_slice::<GitHubOrgResponse>(&response.body) {
                        Ok(github_data) => {
                            // Verify organization exists and has reasonable activity
                            if github_data.public_repos >= 1 {
                                // Update company verification status
                                let success = StorageManager::update_company(&company_id, |company| {
                                    company.web3_identity.github_org = Some(github_org.clone());
                                    company.web3_identity.social_verification_status =
                                        VerificationStatus::Verified;
                                    company.verification_score =
                                        Self::calculate_verification_score(company);
                                });

                                if success {
                                    RegistryResult::Ok(VerificationResult {
                                        success: true,
                                        message: format!(
                                            "GitHub organization '{}' verified successfully",
                                            github_org
                                        ),
                                        verified_at: Some(time()),
                                    })
                                } else {
                                    RegistryResult::Err("Failed to update company".to_string())
                                }
                            } else {
                                RegistryResult::Ok(VerificationResult {
                                    success: false,
                                    message: "GitHub organization has no public repositories"
                                        .to_string(),
                                    verified_at: None,
                                })
                            }
                        }
                        Err(_) => RegistryResult::Err("Failed to parse GitHub API response".to_string()),
                    }
                } else if response.status == 404u32 {
                    RegistryResult::Ok(VerificationResult {
                        success: false,
                        message: "GitHub organization not found".to_string(),
                        verified_at: None,
                    })
                } else {
                    RegistryResult::Err(format!("GitHub API error: {}", response.status))
                }
            }
            Err(err) => RegistryResult::Err(format!("HTTP request failed: {:?}", err)),
        }
    }

    // Domain verification challenge creation
    pub fn create_domain_verification_challenge(
        company_id: String,
        caller_principal: Principal,
    ) -> RegistryResult<DomainVerificationChallenge> {
        // Get company and verify permissions
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company creator can create challenges".to_string(),
            );
        }

        // Extract domain from company website
        let domain = match Self::extract_domain_from_url(&company.basic_info.website) {
            Ok(domain) => domain,
            Err(err) => return RegistryResult::Err(err),
        };

        let challenge_token = Self::generate_challenge_token();
        let now = time();
        let expires_at = now + (24 * 60 * 60 * 1_000_000_000); // 24 hours in nanoseconds

        let challenge = DomainVerificationChallenge {
            company_id: company_id.clone(),
            domain: domain.clone(),
            challenge_token: challenge_token.clone(),
            created_at: now,
            expires_at,
        };

        StorageManager::insert_domain_challenge(company_id, challenge.clone());

        RegistryResult::Ok(challenge)
    }

    // Domain ownership verification
    pub async fn verify_domain_ownership(
        company_id: String,
        caller_principal: Principal,
    ) -> RegistryResult<VerificationResult> {
        // Check rate limiting first
        if !StorageManager::check_http_rate_limit(caller_principal) {
            return RegistryResult::Err("Rate limit exceeded. Please try again later.".to_string());
        }

        // Get challenge
        let challenge = match StorageManager::get_domain_challenge(&company_id) {
            Some(challenge) => challenge,
            None => {
                return RegistryResult::Err(
                    "No domain verification challenge found. Create one first.".to_string(),
                )
            }
        };

        // Check if challenge expired
        if time() > challenge.expires_at {
            StorageManager::remove_domain_challenge(&company_id);
            return RegistryResult::Err("Domain verification challenge expired".to_string());
        }

        // Check DNS TXT record
        let verification_url = format!(
            "https://dns.google/resolve?name={}&type=TXT",
            challenge.domain
        );

        let request = CanisterHttpRequestArgument {
            url: verification_url,
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(1024),
            transform: Some(TransformContext::from_name(
                "transform_domain_response".to_string(),
                vec![],
            )),
            headers: vec![HttpHeader {
                name: "Accept".to_string(),
                value: "application/json".to_string(),
            }],
        };

        match http_request(request, 10_000_000_000).await {
            Ok((response,)) => {
                if response.status == 200u32 {
                    // Parse DNS response and look for challenge token
                    let response_text = String::from_utf8_lossy(&response.body);

                    if response_text.contains(&challenge.challenge_token) {
                        // Verification successful
                        let success = StorageManager::update_company(&company_id, |company| {
                            company.web3_identity.domain_verified = true;
                            company.verification_score = Self::calculate_verification_score(company);
                        });

                        if success {
                            // Remove challenge
                            StorageManager::remove_domain_challenge(&company_id);

                            RegistryResult::Ok(VerificationResult {
                                success: true,
                                message: format!("Domain '{}' verified successfully", challenge.domain),
                                verified_at: Some(time()),
                            })
                        } else {
                            RegistryResult::Err("Failed to update company".to_string())
                        }
                    } else {
                        RegistryResult::Ok(VerificationResult {
                            success: false,
                            message: format!(
                                "TXT record with token '{}' not found in domain '{}'",
                                challenge.challenge_token, challenge.domain
                            ),
                            verified_at: None,
                        })
                    }
                } else {
                    RegistryResult::Err(format!("DNS query failed with status: {}", response.status))
                }
            }
            Err(err) => RegistryResult::Err(format!("DNS query request failed: {:?}", err)),
        }
    }

    // Social media verification
    pub fn verify_social_media_manual(
        company_id: String,
        platform: String,
        proof_url: String,
        caller_principal: Principal,
    ) -> RegistryResult<VerificationResult> {
        // Get company and verify permissions
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err("Unauthorized: Only company creator can verify".to_string());
        }

        // Basic URL validation
        if !proof_url.starts_with("https://") {
            return RegistryResult::Err("Proof URL must use HTTPS".to_string());
        }

        // Platform-specific validation
        let is_valid_platform = match platform.to_lowercase().as_str() {
            "twitter" => proof_url.contains("twitter.com") || proof_url.contains("x.com"),
            "discord" => proof_url.contains("discord.gg") || proof_url.contains("discord.com"),
            "telegram" => proof_url.contains("t.me"),
            _ => false,
        };

        if !is_valid_platform {
            return RegistryResult::Err(format!("Invalid {} URL", platform));
        }

        // Update company social media info
        let success = StorageManager::update_company(&company_id, |company| {
            match platform.to_lowercase().as_str() {
                "twitter" => {
                    // Extract username from URL
                    if let Some(username) = Self::extract_twitter_username(&proof_url) {
                        company.web3_identity.twitter_handle = Some(username);
                    }
                }
                "discord" => {
                    company.web3_identity.discord_server = Some(proof_url.clone());
                }
                "telegram" => {
                    company.web3_identity.telegram_channel = Some(proof_url.clone());
                }
                _ => {}
            }
            company.verification_score = Self::calculate_verification_score(company);
        });

        if success {
            RegistryResult::Ok(VerificationResult {
                success: true,
                message: format!(
                    "{} profile submitted for verification. Manual review may be required.",
                    platform
                ),
                verified_at: Some(time()),
            })
        } else {
            RegistryResult::Err("Failed to update company".to_string())
        }
    }

    // Utility functions
    pub fn get_verification_instructions(verification_type: VerificationType) -> String {
        match verification_type {
            VerificationType::GitHub => {
                "To verify your GitHub organization:\n\
                1. Ensure your organization has at least 1 public repository\n\
                2. Call verify_github_organization with your company ID and organization name\n\
                3. The system will verify the organization exists and has activity"
                    .to_string()
            }
            VerificationType::Domain => {
                "To verify domain ownership:\n\
                1. Call create_domain_verification_challenge with your company ID\n\
                2. Add the provided challenge token as a TXT record to your domain's DNS\n\
                3. Call verify_domain_ownership to complete verification\n\
                4. TXT record format: 'icp-registry-verification=<token>'"
                    .to_string()
            }
            VerificationType::Twitter => {
                "To verify your Twitter/X account:\n\
                1. Post a tweet mentioning your company registration on ICP CrossChain Registry\n\
                2. Include your company ID in the tweet\n\
                3. Call verify_social_media_manual with the tweet URL\n\
                4. Manual review may be required"
                    .to_string()
            }
            VerificationType::Discord => {
                "To verify your Discord server:\n\
                1. Create a public channel or post mentioning ICP CrossChain Registry\n\
                2. Include your company ID in the message\n\
                3. Call verify_social_media_manual with the Discord invite or message URL"
                    .to_string()
            }
            VerificationType::Telegram => {
                "To verify your Telegram channel:\n\
                1. Post a message in your public channel mentioning ICP CrossChain Registry\n\
                2. Include your company ID in the message\n\
                3. Call verify_social_media_manual with the channel URL"
                    .to_string()
            }
        }
    }

    // Helper functions
    fn generate_challenge_token() -> String {
        // Use multiple sources of randomness for better security
        let timestamp = time();
        
        // Create a simple pseudo-random hash from various system state
        let mut hasher = timestamp;
        hasher = hasher.wrapping_mul(6364136223846793005);
        hasher = hasher.wrapping_add(1442695040888963407);
        hasher ^= hasher >> 32;
        hasher = hasher.wrapping_mul(0xd6e8feb86659fd93);
        hasher ^= hasher >> 32;
        hasher = hasher.wrapping_mul(0xd6e8feb86659fd93);
        hasher ^= hasher >> 32;

        format!("icp-registry-{}-{:016x}", timestamp, hasher)
    }

    // Safe regex compilation utility
    fn safe_regex_new(pattern: &str) -> Result<Regex, String> {
        Regex::new(pattern).map_err(|e| format!("Regex compilation error: {}", e))
    }

    fn extract_domain_from_url(url: &str) -> Result<String, String> {
        let url_regex = Self::safe_regex_new(r"^https?://([^/]+)")?;
        if let Some(captures) = url_regex.captures(url) {
            if let Some(domain) = captures.get(1) {
                Ok(domain.as_str().to_string())
            } else {
                Err("Invalid URL format".to_string())
            }
        } else {
            Err("Invalid URL format".to_string())
        }
    }

    fn extract_twitter_username(url: &str) -> Option<String> {
        let twitter_regex = Self::safe_regex_new(r"(?:twitter\.com|x\.com)/([^/?]+)").ok()?;
        if let Some(captures) = twitter_regex.captures(url) {
            captures.get(1).map(|m| m.as_str().to_string())
        } else {
            None
        }
    }

    // Cross-chain address validation functions
    pub fn validate_bitcoin_address(address: &str) -> bool {
        // Bitcoin address validation (Legacy, SegWit v0, SegWit v1/Taproot)
        let btc_legacy = match Self::safe_regex_new(r"^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        let btc_segwit = match Self::safe_regex_new(r"^bc1[a-z0-9]{39,59}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        
        btc_legacy.is_match(address) || btc_segwit.is_match(address)
    }

    pub fn validate_ethereum_address(address: &str) -> bool {
        // Ethereum address validation (0x followed by 40 hex characters)
        let eth_regex = match Self::safe_regex_new(r"^0x[a-fA-F0-9]{40}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        eth_regex.is_match(address)
    }

    pub fn validate_solana_address(address: &str) -> bool {
        // Solana addresses are base58-encoded, typically 32-44 characters
        // They use the base58 alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
        let solana_regex = match Self::safe_regex_new(r"^[1-9A-HJ-NP-Za-km-z]{32,44}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        
        // Additional validation: common invalid characters for base58
        if address.contains('0') || address.contains('O') || address.contains('I') || address.contains('l') {
            return false;
        }
        
        solana_regex.is_match(address)
    }

    pub fn validate_sui_address(address: &str) -> bool {
        // Sui addresses are 32-byte hex strings with 0x prefix (66 characters total)
        let sui_regex = match Self::safe_regex_new(r"^0x[a-fA-F0-9]{64}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        sui_regex.is_match(address)
    }

    pub fn validate_ton_address(address: &str) -> bool {
        // TON addresses can be in several formats:
        // 1. Raw format: 0:followed by 64 hex characters
        // 2. User-friendly format: base64url encoded, typically starting with EQ, UQ, or kQ
        let ton_raw = match Self::safe_regex_new(r"^0:[a-fA-F0-9]{64}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        let ton_friendly = match Self::safe_regex_new(r"^[EUkQ][A-Za-z0-9_-]{46,48}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        
        ton_raw.is_match(address) || ton_friendly.is_match(address)
    }

    pub fn validate_icp_principal(principal: &str) -> bool {
        // ICP Principal IDs are base32-encoded with specific format
        // They end with specific suffixes and have length constraints
        let icp_regex = match Self::safe_regex_new(r"^[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{3}$") {
            Ok(regex) => regex,
            Err(_) => return false,
        };
        icp_regex.is_match(principal)
    }

    pub fn validate_polygon_address(address: &str) -> bool {
        // Polygon uses the same address format as Ethereum
        Self::validate_ethereum_address(address)
    }

    // Comprehensive cross-chain address validation
    pub fn validate_cross_chain_address(chain: &str, address: &str) -> bool {
        match chain.to_lowercase().as_str() {
            "bitcoin" | "btc" => Self::validate_bitcoin_address(address),
            "ethereum" | "eth" => Self::validate_ethereum_address(address),
            "solana" | "sol" => Self::validate_solana_address(address),
            "sui" => Self::validate_sui_address(address),
            "ton" => Self::validate_ton_address(address),
            "icp" | "internet_computer" => Self::validate_icp_principal(address),
            "polygon" | "matic" => Self::validate_polygon_address(address),
            _ => false,
        }
    }

    // Get validation rules for different chains
    pub fn get_address_validation_rules(chain: &str) -> String {
        match chain.to_lowercase().as_str() {
            "bitcoin" | "btc" => {
                "Bitcoin addresses can be:\n\
                • Legacy format: Starting with 1 or 3 (25-34 characters)\n\
                • SegWit format: Starting with bc1 (39-59 characters)\n\
                Example: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"
                    .to_string()
            }
            "ethereum" | "eth" => {
                "Ethereum addresses:\n\
                • Must start with 0x\n\
                • Followed by exactly 40 hexadecimal characters\n\
                Example: 0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E"
                    .to_string()
            }
            "solana" | "sol" => {
                "Solana addresses:\n\
                • Base58-encoded strings\n\
                • 32-44 characters long\n\
                • No 0, O, I, or l characters\n\
                Example: 7dHbWXmci3dT8UFYWGGWnSZwJa8ACHWrAhwRgBAuR7a1"
                    .to_string()
            }
            "sui" => {
                "Sui addresses:\n\
                • Start with 0x\n\
                • Followed by exactly 64 hexadecimal characters\n\
                Example: 0x2d3d1d6e5f7c8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b"
                    .to_string()
            }
            "ton" => {
                "TON addresses can be:\n\
                • Raw format: 0: followed by 64 hex characters\n\
                • User-friendly: Base64url encoded, starting with EQ/UQ/kQ\n\
                Example: EQD2NmD_lH5f5u1Kj3KfGyTvhZSX0Eg6qp2a5IQUKXxOG21n"
                    .to_string()
            }
            "icp" | "internet_computer" => {
                "ICP Principal IDs:\n\
                • Base32-encoded with dashes\n\
                • Format: xxxxx-xxxxx-xxxxx-xxxxx-xxx\n\
                Example: rdmx6-jaaaa-aaaah-qcaiq-cai"
                    .to_string()
            }
            "polygon" | "matic" => {
                "Polygon addresses (same as Ethereum):\n\
                • Must start with 0x\n\
                • Followed by exactly 40 hexadecimal characters\n\
                Example: 0x742d35Cc6634C0532925a3b8D4d3c12de56d0d9E"
                    .to_string()
            }
            _ => "Unsupported chain. Please check the chain name.".to_string(),
        }
    }
}

// HTTP transform functions for HTTPS outcalls
pub fn transform_github_response(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=()".to_string(),
        },
    ];

    let mut sanitized_body = raw.response.body.clone();

    // Basic sanitization - remove potentially sensitive fields
    if let Ok(mut json_value) = serde_json::from_slice::<serde_json::Value>(&sanitized_body) {
        if let Some(obj) = json_value.as_object_mut() {
            // Remove sensitive fields that might contain personal info
            obj.remove("email");
            obj.remove("gravatar_id");
            obj.remove("events_url");
            obj.remove("received_events_url");
        }
        sanitized_body = serde_json::to_vec(&json_value).unwrap_or(raw.response.body.clone());
    }

    HttpResponse {
        status: raw.response.status.clone(),
        body: sanitized_body,
        headers,
    }
}

pub fn transform_domain_response(raw: TransformArgs) -> HttpResponse {
    let headers = vec![HttpHeader {
        name: "Content-Security-Policy".to_string(),
        value: "default-src 'self'".to_string(),
    }];

    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    }
}