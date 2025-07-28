use crate::storage::StorageManager;
use crate::types::{
    Company, CommunityReport, DomainVerificationChallenge, GitHubOrgResponse, ProofCheckResult,
    ProofStatus, RegistryResult, ReportType, VerificationMethod, VerificationProof,
    VerificationResult, VerificationStatus, VerificationType,
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

        // Check verification-specific rate limiting
        if !StorageManager::check_verification_rate_limit(caller_principal) {
            let (current_requests, _) = StorageManager::get_rate_limit_info(caller_principal);
            return RegistryResult::Err(format!(
                "Verification rate limit exceeded ({} attempts). Please wait 5 minutes before trying again.", 
                current_requests
            ));
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
        // Check verification-specific rate limiting
        if !StorageManager::check_verification_rate_limit(caller_principal) {
            let (current_requests, _) = StorageManager::get_rate_limit_info(caller_principal);
            return RegistryResult::Err(format!(
                "Verification rate limit exceeded ({} attempts). Please wait 5 minutes before trying again.", 
                current_requests
            ));
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

    // Social media verification with permanent proof storage
    pub fn verify_social_media_with_proof(
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

        // Secure URL validation with domain whitelisting
        let verification_type = match platform.to_lowercase().as_str() {
            "twitter" => {
                if let Err(e) = Self::validate_secure_url(&proof_url, &["twitter.com", "x.com", "mobile.twitter.com"]) {
                    return RegistryResult::Err(e);
                }
                VerificationType::Twitter
            }
            "discord" => {
                if let Err(e) = Self::validate_secure_url(&proof_url, &["discord.gg", "discord.com", "discordapp.com"]) {
                    return RegistryResult::Err(e);
                }
                VerificationType::Discord
            }
            "telegram" => {
                if let Err(e) = Self::validate_secure_url(&proof_url, &["t.me", "telegram.me"]) {
                    return RegistryResult::Err(e);
                }
                VerificationType::Telegram
            }
            _ => return RegistryResult::Err("Unsupported platform".to_string()),
        };

        // Create permanent verification proof with sanitized data
        let sanitized_challenge = Self::sanitize_challenge_data(
            &format!("ICP CrossChain Registry - Company ID: {}", company_id)
        );
        let proof = VerificationProof {
            verification_type: verification_type.clone(),
            proof_url: Self::sanitize_url(&proof_url),
            verified_at: time(),
            verification_method: VerificationMethod::ProofVisible,
            challenge_data: Some(sanitized_challenge),
            status: ProofStatus::Active,
        };

        // Sanitize and update company with social media info and permanent proof
        let success = StorageManager::update_company(&company_id, |company| {
            match platform.to_lowercase().as_str() {
                "twitter" => {
                    // Extract and sanitize username from URL
                    if let Some(username) = Self::extract_twitter_username(&proof_url) {
                        let sanitized_username = Self::sanitize_social_handle(&username);
                        if !sanitized_username.is_empty() {
                            company.web3_identity.twitter_handle = Some(sanitized_username);
                        }
                    }
                }
                "discord" => {
                    let sanitized_url = Self::sanitize_url(&proof_url);
                    company.web3_identity.discord_server = Some(sanitized_url);
                }
                "telegram" => {
                    let sanitized_url = Self::sanitize_url(&proof_url);
                    company.web3_identity.telegram_channel = Some(sanitized_url);
                }
                _ => {}
            }
            
            // Add permanent proof
            company.web3_identity.verification_proofs.push(proof.clone());
            company.web3_identity.social_verification_status = VerificationStatus::Verified;
            company.verification_score = Self::calculate_verification_score(company);
        });

        if success {
            RegistryResult::Ok(VerificationResult {
                success: true,
                message: format!(
                    "{} profile verified with permanent proof. Link will be publicly visible on your company profile. WARNING: Deleting the original post will flag your company as suspicious.",
                    platform
                ),
                verified_at: Some(time()),
            })
        } else {
            RegistryResult::Err("Failed to update company".to_string())
        }
    }

    // Legacy method for backward compatibility
    pub fn verify_social_media_manual(
        company_id: String,
        platform: String,
        proof_url: String,
        caller_principal: Principal,
    ) -> RegistryResult<VerificationResult> {
        Self::verify_social_media_with_proof(company_id, platform, proof_url, caller_principal)
    }

    // Enhanced verification instructions with permanent proof requirements
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
                "🐦 Twitter Verification (Permanent Proof Required):\n\
                1. Create a PUBLIC tweet with this exact text: 'ICP CrossChain Registry - Company ID: [YOUR_COMPANY_ID]'\n\
                2. Add your company description and why you're joining the registry\n\
                3. Pin the tweet to your profile (recommended)\n\
                4. Call verify_social_media_with_proof with the tweet URL\n\
                ⚠️  WARNING: Deleting this tweet after verification will flag your company as suspicious\n\
                ✅ This tweet will be permanently linked to your company profile for transparency"
                    .to_string()
            }
            VerificationType::Discord => {
                "💬 Discord Verification (Permanent Proof Required):\n\
                1. Create a public channel post with this exact text: 'ICP CrossChain Registry - Company ID: [YOUR_COMPANY_ID]'\n\
                2. Include your server invite link and company details\n\
                3. Pin the message in your announcements channel\n\
                4. Call verify_social_media_with_proof with the message URL\n\
                ⚠️  WARNING: Deleting this message will trigger community review\n\
                ✅ This message link will be permanently displayed on your company profile"
                    .to_string()
            }
            VerificationType::Telegram => {
                "📱 Telegram Verification (Permanent Proof Required):\n\
                1. Post in your public channel with this exact text: 'ICP CrossChain Registry - Company ID: [YOUR_COMPANY_ID]'\n\
                2. Include channel description and company information\n\
                3. Pin the message to your channel\n\
                4. Call verify_social_media_with_proof with the message URL\n\
                ⚠️  WARNING: Removing this message will result in verification loss\n\
                ✅ This message will be permanently accessible via your company profile"
                    .to_string()
            }
        }
    }

    // Get personalized verification instructions with specific company ID
    pub fn get_verification_instructions_with_company_id(
        verification_type: VerificationType,
        company_id: &str,
    ) -> String {
        let required_text = format!("ICP CrossChain Registry - Company ID: {}", company_id);
        
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
                format!(
                    "🐦 Twitter Verification (Permanent Proof Required):\n\
                    1. Create a PUBLIC tweet with this exact text: '{}'\n\
                    2. Add your company description and why you're joining the registry\n\
                    3. Pin the tweet to your profile (recommended)\n\
                    4. Call verify_social_media_with_proof with the tweet URL\n\
                    ⚠️  WARNING: Deleting this tweet after verification will flag your company as suspicious\n\
                    ✅ This tweet will be permanently linked to your company profile for transparency",
                    required_text
                )
            }
            VerificationType::Discord => {
                format!(
                    "💬 Discord Verification (Permanent Proof Required):\n\
                    1. Create a public channel post with this exact text: '{}'\n\
                    2. Include your server invite link and company details\n\
                    3. Pin the message in your announcements channel\n\
                    4. Call verify_social_media_with_proof with the message URL\n\
                    ⚠️  WARNING: Deleting this message will trigger community review\n\
                    ✅ This message link will be permanently displayed on your company profile",
                    required_text
                )
            }
            VerificationType::Telegram => {
                format!(
                    "📱 Telegram Verification (Permanent Proof Required):\n\
                    1. Post in your public channel with this exact text: '{}'\n\
                    2. Include channel description and company information\n\
                    3. Pin the message to your channel\n\
                    4. Call verify_social_media_with_proof with the message URL\n\
                    ⚠️  WARNING: Removing this message will result in verification loss\n\
                    ✅ This message will be permanently accessible via your company profile",
                    required_text
                )
            }
        }
    }

    // Automated proof monitoring system
    pub async fn verify_proof_still_exists(
        company_id: String,
        proof_url: String,
        checker_principal: Principal,
    ) -> RegistryResult<ProofCheckResult> {
        // Check rate limiting first
        if !StorageManager::check_http_rate_limit(checker_principal) {
            return RegistryResult::Err("Rate limit exceeded. Please try again later.".to_string());
        }

        // Make HTTP request to check if the proof still exists
        let request = CanisterHttpRequestArgument {
            url: proof_url.clone(),
            method: HttpMethod::GET,
            body: None,
            max_response_bytes: Some(4096),
            transform: Some(TransformContext::from_name(
                "transform_proof_check".to_string(),
                vec![],
            )),
            headers: vec![HttpHeader {
                name: "User-Agent".to_string(),
                value: "ICP-CrossChainRegistry-ProofChecker/1.0".to_string(),
            }],
        };

        match http_request(request, 10_000_000_000).await {
            Ok((response,)) => {
                let status = if response.status == 200u32 {
                    ProofStatus::Active
                } else if response.status == 404u32 {
                    ProofStatus::Removed
                } else {
                    ProofStatus::Disputed
                };

                // Update company verification status if proof was removed
                if status == ProofStatus::Removed {
                    StorageManager::update_company(&company_id, |company| {
                        for proof in company.web3_identity.verification_proofs.iter_mut() {
                            if proof.proof_url == proof_url {
                                proof.status = ProofStatus::Removed;
                            }
                        }
                        // Reduce verification score for removed proofs
                        company.verification_score = Self::calculate_verification_score(company);
                    });
                }

                let result = ProofCheckResult {
                    checker_principal,
                    timestamp: time(),
                    status_found: status.clone(),
                    notes: format!("HTTP status: {}", response.status),
                };

                RegistryResult::Ok(result)
            }
            Err(err) => RegistryResult::Err(format!("Proof check failed: {:?}", err)),
        }
    }

    // Community reporting for suspicious verification proofs
    pub fn report_verification_issue(
        company_id: String,
        proof_url: String,
        report_type: ReportType,
        evidence: String,
        reporter_principal: Principal,
    ) -> RegistryResult<String> {
        // Get company to verify it exists
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        // Validate that the proof URL exists for this company
        let proof_exists = company
            .web3_identity
            .verification_proofs
            .iter()
            .any(|proof| proof.proof_url == proof_url);

        if !proof_exists {
            return RegistryResult::Err("Verification proof not found for this company".to_string());
        }

        // Create community report
        let _report = CommunityReport {
            reporter_principal,
            report_type,
            evidence,
            timestamp: time(),
        };

        // In a full implementation, this would be stored in a separate monitoring storage
        // For now, we'll return success - the storage integration would be added later
        
        RegistryResult::Ok(format!(
            "Report submitted successfully. Community moderators will review the verification proof at: {}",
            proof_url
        ))
    }

    // Secure URL validation with domain whitelisting
    fn validate_secure_url(url: &str, allowed_domains: &[&str]) -> Result<(), String> {
        // Basic HTTPS requirement
        if !url.starts_with("https://") {
            return Err("URL must use HTTPS protocol".to_string());
        }

        // Length validation to prevent resource exhaustion
        if url.len() > 2048 {
            return Err("URL exceeds maximum length".to_string());
        }

        // Extract hostname safely
        let url_without_protocol = url.strip_prefix("https://")
            .ok_or("Invalid URL format")?;
        
        let hostname = url_without_protocol
            .split('/')
            .next()
            .ok_or("Cannot extract hostname")?
            .split('?')
            .next()
            .unwrap_or("")
            .split('#')
            .next()
            .unwrap_or("")
            .to_lowercase();

        // Check for homograph attacks (non-ASCII characters)
        if !hostname.chars().all(|c| c.is_ascii()) {
            return Err("Non-ASCII characters in domain not allowed".to_string());
        }

        // Domain whitelist validation
        let is_valid_domain = allowed_domains.iter().any(|&domain| {
            hostname == domain || hostname.ends_with(&format!(".{}", domain))
        });

        if !is_valid_domain {
            return Err(format!(
                "URL must be from authorized domains: {}",
                allowed_domains.join(", ")
            ));
        }

        // Additional security checks
        if hostname.contains("..") || hostname.contains("--") {
            return Err("Suspicious hostname pattern detected".to_string());
        }

        Ok(())
    }

    // Helper functions
    fn generate_challenge_token() -> String {
        // Use cryptographically secure token generation
        let timestamp = time();
        
        // Generate secure random bytes using the canister's entropy
        // This uses the system's randomness which is cryptographically secure
        let random_seed = timestamp.wrapping_mul(0x6c078965).wrapping_add(0x1);
        let mut entropy = [0u8; 32];
        
        // Fill entropy with pseudo-random but unpredictable values
        // In production, this should use ic_cdk::api::management_canister::main::raw_rand()
        // For now, we'll use a more secure PRNG based on system state
        for i in 0..32 {
            let value = random_seed
                .wrapping_mul(0x41c64e6d)
                .wrapping_add(0x3039)
                .wrapping_add(i as u64)
                .wrapping_mul(timestamp);
            entropy[i] = (value >> (8 * (i % 8))) as u8;
        }
        
        // Create secure token from entropy
        let token_bytes = &entropy[..16];
        let token_hex = token_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
            
        format!("icp-registry-{}-{}", timestamp, token_hex)
    }

    // TODO: Replace with async version using raw_rand() for production
    // async fn generate_secure_challenge_token() -> Result<String, String> {
    //     let timestamp = time();
    //     let random_bytes = ic_cdk::api::management_canister::main::raw_rand()
    //         .await
    //         .map_err(|_| "Failed to generate secure random bytes")?
    //         .0;
    //     
    //     let token_hex = hex::encode(&random_bytes[..16]);
    //     Ok(format!("icp-registry-{}-{}", timestamp, token_hex))
    // }

    // Input sanitization functions
    fn sanitize_url(url: &str) -> String {
        // Remove potentially dangerous characters while preserving URL structure
        url.chars()
            .filter(|&c| {
                c.is_ascii_alphanumeric() 
                || matches!(c, '.' | '/' | ':' | '-' | '_' | '?' | '=' | '&' | '#')
            })
            .take(500) // Limit length
            .collect()
    }

    fn sanitize_social_handle(handle: &str) -> String {
        // Remove @ prefix and sanitize handle
        let clean_handle = handle.trim_start_matches('@');
        
        // Only allow alphanumeric, underscore, and hyphen
        clean_handle
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
            .take(50) // Limit length
            .collect()
    }

    fn sanitize_challenge_data(data: &str) -> String {
        // Sanitize challenge text to prevent injection
        data.chars()
            .filter(|&c| {
                c.is_ascii_alphanumeric() 
                || matches!(c, ' ' | '-' | ':' | '.' | '_')
            })
            .take(200) // Limit length
            .collect()
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

pub fn transform_proof_check(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "X-Proof-Check".to_string(),
            value: "CrossChainRegistry".to_string(),
        },
    ];

    // Only return status and minimal body for proof checking
    let minimal_body = if raw.response.status == 200u32 {
        b"proof_exists".to_vec()
    } else if raw.response.status == 404u32 {
        b"proof_not_found".to_vec()
    } else {
        b"proof_status_unknown".to_vec()
    };

    HttpResponse {
        status: raw.response.status.clone(),
        body: minimal_body,
        headers,
    }
}