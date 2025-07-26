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
    // Core CRUD operations
    pub fn create_company(
        request: CreateCompanyRequest,
        caller_principal: Principal,
    ) -> RegistryResult<String> {
        let now = time();
        let company_id = StorageManager::generate_company_id();

        // Initialize company with default values
        let company = Company {
            id: company_id.clone(),
            basic_info: request.basic_info,
            web3_identity: request.web3_identity,
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
}