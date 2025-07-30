use crate::storage::StorageManager;
use crate::types::{
    Company, CompanyStatus, CommunityValidation, CreateCompanyRequest, RegistryResult,
    SearchFilters, UpdateCompanyRequest,
};
use crate::verification::VerificationManager;
use candid::Principal;
use ic_cdk::api::time;
use std::collections::HashMap;

// API layer for company registry operations

pub struct RegistryAPI;

impl RegistryAPI {
    // Security constants for input validation
    const MAX_NAME_LENGTH: usize = 100;
    const MAX_DESCRIPTION_LENGTH: usize = 2000;
    const MAX_URL_LENGTH: usize = 500;
    const MAX_MESSAGE_LENGTH: usize = 1000;
    const MAX_SOCIAL_HANDLE_LENGTH: usize = 100;
    const MAX_ADDRESS_LENGTH: usize = 100;
    const MAX_TEAM_MEMBERS: usize = 50;
    const MAX_ADDRESSES_PER_CHAIN: usize = 20;

    // Input validation functions
    fn validate_string_length(value: &str, max_length: usize, field_name: &str) -> Result<(), String> {
        if value.len() > max_length {
            return Err(format!("{} exceeds maximum length of {} characters", field_name, max_length));
        }
        Ok(())
    }

    fn validate_company_request(request: &CreateCompanyRequest) -> Result<(), String> {
        // Validate basic info
        Self::validate_string_length(&request.basic_info.name, Self::MAX_NAME_LENGTH, "Company name")?;
        Self::validate_string_length(&request.basic_info.description, Self::MAX_DESCRIPTION_LENGTH, "Description")?;
        Self::validate_string_length(&request.basic_info.website, Self::MAX_URL_LENGTH, "Website URL")?;
        Self::validate_string_length(&request.basic_info.founding_date, 20, "Founding date")?;

        if request.basic_info.name.trim().is_empty() {
            return Err("Company name cannot be empty".to_string());
        }

        if request.basic_info.team_size > 10000 {
            return Err("Team size cannot exceed 10,000 members".to_string());
        }

        if request.basic_info.focus_areas.len() > 10 {
            return Err("Cannot have more than 10 focus areas".to_string());
        }

        // Validate web3 identity
        if let Some(github_org) = &request.web3_identity.github_org {
            Self::validate_string_length(github_org, Self::MAX_SOCIAL_HANDLE_LENGTH, "GitHub organization")?;
        }
        if let Some(twitter) = &request.web3_identity.twitter_handle {
            Self::validate_string_length(twitter, Self::MAX_SOCIAL_HANDLE_LENGTH, "Twitter handle")?;
        }
        if let Some(discord) = &request.web3_identity.discord_server {
            Self::validate_string_length(discord, Self::MAX_URL_LENGTH, "Discord server")?;
        }
        if let Some(telegram) = &request.web3_identity.telegram_channel {
            Self::validate_string_length(telegram, Self::MAX_URL_LENGTH, "Telegram channel")?;
        }
        if let Some(linkedin) = &request.web3_identity.linkedin_company {
            Self::validate_string_length(linkedin, Self::MAX_SOCIAL_HANDLE_LENGTH, "LinkedIn company")?;
        }
        if let Some(medium) = &request.web3_identity.medium_publication {
            Self::validate_string_length(medium, Self::MAX_SOCIAL_HANDLE_LENGTH, "Medium publication")?;
        }

        // Validate cross-chain addresses
        if request.cross_chain_presence.ethereum_contracts.len() > Self::MAX_ADDRESSES_PER_CHAIN {
            return Err("Too many Ethereum contracts".to_string());
        }
        if request.cross_chain_presence.bitcoin_addresses.len() > Self::MAX_ADDRESSES_PER_CHAIN {
            return Err("Too many Bitcoin addresses".to_string());
        }
        if request.cross_chain_presence.solana_addresses.len() > Self::MAX_ADDRESSES_PER_CHAIN {
            return Err("Too many Solana addresses".to_string());
        }
        if request.cross_chain_presence.sui_addresses.len() > Self::MAX_ADDRESSES_PER_CHAIN {
            return Err("Too many Sui addresses".to_string());
        }
        if request.cross_chain_presence.ton_addresses.len() > Self::MAX_ADDRESSES_PER_CHAIN {
            return Err("Too many TON addresses".to_string());
        }

        for address in &request.cross_chain_presence.ethereum_contracts {
            Self::validate_string_length(address, Self::MAX_ADDRESS_LENGTH, "Ethereum contract")?;
        }
        for address in &request.cross_chain_presence.bitcoin_addresses {
            Self::validate_string_length(address, Self::MAX_ADDRESS_LENGTH, "Bitcoin address")?;
        }
        for address in &request.cross_chain_presence.solana_addresses {
            Self::validate_string_length(address, Self::MAX_ADDRESS_LENGTH, "Solana address")?;
        }

        // Validate team members
        if request.team_members.len() > Self::MAX_TEAM_MEMBERS {
            return Err("Too many team members".to_string());
        }

        for member in &request.team_members {
            Self::validate_string_length(&member.name, Self::MAX_NAME_LENGTH, "Team member name")?;
            Self::validate_string_length(&member.role, Self::MAX_NAME_LENGTH, "Team member role")?;
            if let Some(github) = &member.github_profile {
                Self::validate_string_length(github, Self::MAX_URL_LENGTH, "GitHub profile")?;
            }
            if let Some(linkedin) = &member.linkedin_profile {
                Self::validate_string_length(linkedin, Self::MAX_URL_LENGTH, "LinkedIn profile")?;
            }
        }

        Ok(())
    }

    // Core CRUD operations
    pub fn create_company(
        request: CreateCompanyRequest,
        caller_principal: Principal,
    ) -> RegistryResult<String> {
        // Validate input first
        if let Err(validation_error) = Self::validate_company_request(&request) {
            return RegistryResult::Err(validation_error);
        }
        let now = time();
        let company_id = StorageManager::generate_company_id();

        // Initialize company with default values
        let mut web3_identity = request.web3_identity;
        web3_identity.verification_proofs = Vec::new(); // Initialize empty verification proofs

        let company = Company {
            id: company_id.clone(),
            basic_info: request.basic_info,
            web3_identity,
            cross_chain_presence: request.cross_chain_presence,
            team_members: request.team_members,
            community_validation: CommunityValidation {
                peer_endorsements: Vec::new(),
                employee_testimonials: Vec::new(),
                community_vouches: Vec::new(),
                reputation_score: 0,
                reputation_staked: 0,
            },
            status: CompanyStatus::Pending,
            created_at: now,
            updated_at: now,
            created_by: caller_principal,
            verification_score: 0,
        };

        // Calculate initial verification score
        let mut updated_company = company;
        updated_company.verification_score =
            VerificationManager::calculate_verification_score(&updated_company);

        StorageManager::insert_company(company_id.clone(), updated_company);

        RegistryResult::Ok(company_id)
    }

    pub fn get_company(company_id: String) -> RegistryResult<Company> {
        match StorageManager::get_company(&company_id) {
            Some(company) => RegistryResult::Ok(company),
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    pub fn update_company(
        request: UpdateCompanyRequest,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Check if company exists and caller is authorized
        let company = match StorageManager::get_company(&request.company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company creator can update".to_string(),
            );
        }

        // Update company fields
        let success = StorageManager::update_company(&request.company_id, |company| {
            // Update fields if provided
            if let Some(basic_info) = request.basic_info {
                company.basic_info = basic_info;
            }
            if let Some(web3_identity) = request.web3_identity {
                company.web3_identity = web3_identity;
            }
            if let Some(cross_chain_presence) = request.cross_chain_presence {
                company.cross_chain_presence = cross_chain_presence;
            }
            if let Some(team_members) = request.team_members {
                company.team_members = team_members;
            }

            // Recalculate verification score
            company.verification_score = VerificationManager::calculate_verification_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Company not found".to_string())
        }
    }

    pub fn list_companies(
        offset: Option<u32>,
        limit: Option<u32>,
        filters: Option<SearchFilters>,
    ) -> Vec<Company> {
        let offset = offset.unwrap_or(0) as usize;
        let limit = limit.unwrap_or(50) as usize;

        let mut all_companies = StorageManager::get_all_companies();

        // Apply filters if provided
        if let Some(filters) = filters {
            all_companies.retain(|company| {
                let mut matches = true;

                if let Some(ref status_filter) = filters.status {
                    matches &= std::mem::discriminant(&company.status)
                        == std::mem::discriminant(status_filter);
                }

                if let Some(ref focus_areas) = filters.focus_areas {
                    matches &= focus_areas.iter().any(|area| {
                        company.basic_info.focus_areas.contains(area)
                    });
                }

                if let Some(min_score) = filters.min_verification_score {
                    matches &= company.verification_score >= min_score;
                }

                if let Some(has_github) = filters.has_github {
                    matches &= has_github == company.web3_identity.github_org.is_some();
                }

                if let Some(has_contracts) = filters.has_contracts {
                    let has_any_contracts = !company.cross_chain_presence.ethereum_contracts.is_empty()
                        || !company.cross_chain_presence.icp_canisters.is_empty()
                        || !company.cross_chain_presence.polygon_contracts.is_empty();
                    matches &= has_contracts == has_any_contracts;
                }

                matches
            });
        }

        // Sort by verification score (highest first), then by creation date
        all_companies.sort_by(|a, b| {
            b.verification_score
                .cmp(&a.verification_score)
                .then(b.created_at.cmp(&a.created_at))
        });

        // Apply pagination
        all_companies
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect()
    }

    pub fn search_companies(query: String) -> Vec<Company> {
        let query_lower = query.to_lowercase();

        StorageManager::get_all_companies()
            .into_iter()
            .filter(|company| {
                company.basic_info.name.to_lowercase().contains(&query_lower)
                    || company
                        .basic_info
                        .description
                        .to_lowercase()
                        .contains(&query_lower)
                    || company
                        .basic_info
                        .focus_areas
                        .iter()
                        .any(|area| area.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn get_company_count() -> u64 {
        StorageManager::get_companies_count()
    }

    pub fn get_statistics() -> HashMap<String, u64> {
        let mut stats = HashMap::new();

        let all_companies = StorageManager::get_all_companies();
        let total_count = all_companies.len() as u64;
        stats.insert("total_companies".to_string(), total_count);

        let mut pending_count = 0u64;
        let mut verified_count = 0u64;
        let mut trusted_count = 0u64;
        let mut flagged_count = 0u64;

        for company in all_companies {
            match company.status {
                CompanyStatus::Pending => pending_count += 1,
                CompanyStatus::Verified => verified_count += 1,
                CompanyStatus::Trusted => trusted_count += 1,
                CompanyStatus::Flagged => flagged_count += 1,
                CompanyStatus::Suspended => {}
            }
        }

        stats.insert("pending_companies".to_string(), pending_count);
        stats.insert("verified_companies".to_string(), verified_count);
        stats.insert("trusted_companies".to_string(), trusted_count);
        stats.insert("flagged_companies".to_string(), flagged_count);

        stats
    }

    // Cross-chain address validation utilities
    pub fn validate_address(chain: String, address: String) -> RegistryResult<bool> {
        let is_valid = VerificationManager::validate_cross_chain_address(&chain, &address);
        RegistryResult::Ok(is_valid)
    }

    pub fn get_address_validation_rules(chain: String) -> RegistryResult<String> {
        let rules = VerificationManager::get_address_validation_rules(&chain);
        RegistryResult::Ok(rules)
    }

    pub fn get_supported_chains() -> RegistryResult<Vec<String>> {
        let chains = vec![
            "bitcoin".to_string(),
            "ethereum".to_string(),
            "solana".to_string(),
            "sui".to_string(),
            "ton".to_string(),
            "icp".to_string(),
            "polygon".to_string(),
        ];
        RegistryResult::Ok(chains)
    }
}