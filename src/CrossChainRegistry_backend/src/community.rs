use crate::storage::StorageManager;
use crate::types::{
    Company, CompanyStatus, CommunityValidation, CommunityValidationStats, Endorsement, 
    RegistryResult, ReputationLeaderboard, Testimonial, Vouch,
};
use candid::Principal;
use ic_cdk::api::time;

// Community validation business logic
pub struct CommunityValidationManager;

impl CommunityValidationManager {
    // Endorsement operations
    pub fn add_endorsement(
        company_id: String,
        endorser_company_id: String,
        message: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Validate that endorser company exists and caller is authorized
        let endorser_company = match StorageManager::get_company(&endorser_company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Endorser company not found".to_string()),
        };

        if endorser_company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company owner can create endorsements".to_string(),
            );
        }

        // Validate that target company exists
        if StorageManager::get_company(&company_id).is_none() {
            return RegistryResult::Err("Target company not found".to_string());
        }

        // Prevent self-endorsement
        if company_id == endorser_company_id {
            return RegistryResult::Err("Companies cannot endorse themselves".to_string());
        }

        // Check if endorsement already exists
        if let Some(company) = StorageManager::get_company(&company_id) {
            if company
                .community_validation
                .peer_endorsements
                .iter()
                .any(|e| e.endorser_company_id == endorser_company_id)
            {
                return RegistryResult::Err("Endorsement already exists".to_string());
            }
        }

        let endorsement = Endorsement {
            endorser_company_id,
            message,
            timestamp: time(),
            endorser_principal: caller_principal,
        };

        let success = StorageManager::update_company(&company_id, |company| {
            company
                .community_validation
                .peer_endorsements
                .push(endorsement);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to add endorsement".to_string())
        }
    }

    pub fn remove_endorsement(
        company_id: String,
        endorser_company_id: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Validate that endorser company exists and caller is authorized
        let endorser_company = match StorageManager::get_company(&endorser_company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Endorser company not found".to_string()),
        };

        if endorser_company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company owner can remove endorsements".to_string(),
            );
        }

        let success = StorageManager::update_company(&company_id, |company| {
            company
                .community_validation
                .peer_endorsements
                .retain(|e| e.endorser_company_id != endorser_company_id);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to remove endorsement".to_string())
        }
    }

    // Testimonial operations
    pub fn add_testimonial(
        company_id: String,
        author_name: String,
        role: String,
        message: String,
        _caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Validate that target company exists
        if StorageManager::get_company(&company_id).is_none() {
            return RegistryResult::Err("Company not found".to_string());
        }

        // Basic validation
        if author_name.trim().is_empty() {
            return RegistryResult::Err("Author name cannot be empty".to_string());
        }
        if role.trim().is_empty() {
            return RegistryResult::Err("Role cannot be empty".to_string());
        }
        if message.trim().is_empty() {
            return RegistryResult::Err("Message cannot be empty".to_string());
        }

        // Check if testimonial from this principal already exists
        if let Some(company) = StorageManager::get_company(&company_id) {
            if company
                .community_validation
                .employee_testimonials
                .iter()
                .any(|t| t.author_name == author_name)
            {
                return RegistryResult::Err("Testimonial from this author already exists".to_string());
            }
        }

        let testimonial = Testimonial {
            author_name,
            role,
            message,
            timestamp: time(),
            verified: false, // Default to unverified, can be verified later by admins
        };

        let success = StorageManager::update_company(&company_id, |company| {
            company
                .community_validation
                .employee_testimonials
                .push(testimonial);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to add testimonial".to_string())
        }
    }

    pub fn remove_testimonial(
        company_id: String,
        author_name: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Check if testimonial exists and if caller is authorized
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        // Allow company owner or testimonial author to remove testimonial
        let is_company_owner = company.created_by == caller_principal;
        let testimonial_exists = company
            .community_validation
            .employee_testimonials
            .iter()
            .any(|t| t.author_name == author_name);

        if !testimonial_exists {
            return RegistryResult::Err("Testimonial not found".to_string());
        }

        if !is_company_owner {
            return RegistryResult::Err(
                "Unauthorized: Only company owner can remove testimonials".to_string(),
            );
        }

        let success = StorageManager::update_company(&company_id, |company| {
            company
                .community_validation
                .employee_testimonials
                .retain(|t| t.author_name != author_name);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to remove testimonial".to_string())
        }
    }

    pub fn verify_testimonial(
        company_id: String,
        author_name: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Only allow company owner to verify testimonials for now
        // In a real system, this might be done by admin or through some verification process
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company owner can verify testimonials".to_string(),
            );
        }

        let success = StorageManager::update_company(&company_id, |company| {
            if let Some(testimonial) = company
                .community_validation
                .employee_testimonials
                .iter_mut()
                .find(|t| t.author_name == author_name)
            {
                testimonial.verified = true;
            }
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Testimonial not found".to_string())
        }
    }

    // Community vouch operations
    pub fn add_vouch(
        company_id: String,
        message: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Validate that target company exists
        if StorageManager::get_company(&company_id).is_none() {
            return RegistryResult::Err("Company not found".to_string());
        }

        if message.trim().is_empty() {
            return RegistryResult::Err("Message cannot be empty".to_string());
        }

        // Check if vouch from this principal already exists
        if let Some(company) = StorageManager::get_company(&company_id) {
            if company
                .community_validation
                .community_vouches
                .iter()
                .any(|v| v.voucher_principal == caller_principal)
            {
                return RegistryResult::Err("Vouch from this principal already exists".to_string());
            }
        }

        // Calculate voucher weight based on their activity/reputation
        let weight = Self::calculate_voucher_weight(caller_principal);

        let vouch = Vouch {
            voucher_principal: caller_principal,
            message,
            timestamp: time(),
            weight,
        };

        let success = StorageManager::update_company(&company_id, |company| {
            company.community_validation.community_vouches.push(vouch);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to add vouch".to_string())
        }
    }

    pub fn remove_vouch(
        company_id: String,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        let success = StorageManager::update_company(&company_id, |company| {
            company
                .community_validation
                .community_vouches
                .retain(|v| v.voucher_principal != caller_principal);
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to remove vouch".to_string())
        }
    }

    // Reputation management
    pub fn stake_reputation(
        company_id: String,
        amount: u64,
        caller_principal: Principal,
    ) -> RegistryResult<()> {
        // Validate that company exists and caller is authorized
        let company = match StorageManager::get_company(&company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Company not found".to_string()),
        };

        if company.created_by != caller_principal {
            return RegistryResult::Err(
                "Unauthorized: Only company owner can stake reputation".to_string(),
            );
        }

        if amount == 0 {
            return RegistryResult::Err("Stake amount must be greater than 0".to_string());
        }

        let success = StorageManager::update_company(&company_id, |company| {
            company.community_validation.reputation_staked += amount;
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Failed to stake reputation".to_string())
        }
    }

    // Utility functions
    fn calculate_voucher_weight(voucher_principal: Principal) -> u32 {
        // Calculate weight based on voucher's activity in the system
        // For now, use a simple heuristic based on how many companies they've vouched for
        let all_companies = StorageManager::get_all_companies();
        let vouch_count = all_companies
            .iter()
            .map(|company| {
                company
                    .community_validation
                    .community_vouches
                    .iter()
                    .filter(|v| v.voucher_principal == voucher_principal)
                    .count()
            })
            .sum::<usize>();

        // Base weight of 1, increased by activity
        match vouch_count {
            0..=2 => 1,
            3..=10 => 2,
            11..=25 => 3,
            _ => 5,
        }
    }

    fn update_reputation_score(company: &mut Company) {
        let mut score = 0u32;

        // Base score from verification
        score += company.verification_score / 4;

        // Endorsements (high weight)
        let endorsement_score = company
            .community_validation
            .peer_endorsements
            .len() as u32 * 10;
        score += endorsement_score;

        // Verified testimonials (medium weight)
        let verified_testimonial_score = company
            .community_validation
            .employee_testimonials
            .iter()
            .filter(|t| t.verified)
            .count() as u32 * 5;
        score += verified_testimonial_score;

        // Unverified testimonials (low weight)
        let unverified_testimonial_score = company
            .community_validation
            .employee_testimonials
            .iter()
            .filter(|t| !t.verified)
            .count() as u32 * 2;
        score += unverified_testimonial_score;

        // Community vouches (weighted by voucher reputation)
        let vouch_score: u32 = company
            .community_validation
            .community_vouches
            .iter()
            .map(|v| v.weight * 3)
            .sum();
        score += vouch_score;

        // Reputation staking bonus (logarithmic scale)
        let staking_bonus = if company.community_validation.reputation_staked > 0 {
            (company.community_validation.reputation_staked as f64).log10().ceil() as u32 * 2
        } else {
            0
        };
        score += staking_bonus;

        company.community_validation.reputation_score = score;

        // Update company status based on reputation score
        company.status = match score {
            0..=20 => CompanyStatus::Pending,
            21..=50 => CompanyStatus::Verified,
            51..=100 => CompanyStatus::Trusted,
            _ => CompanyStatus::Trusted,
        };
    }

    // Query functions
    pub fn get_community_validation(company_id: String) -> RegistryResult<CommunityValidation> {
        match StorageManager::get_company(&company_id) {
            Some(company) => RegistryResult::Ok(company.community_validation),
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    pub fn get_companies_by_reputation(min_score: u32, limit: Option<u32>) -> Vec<Company> {
        let limit = limit.unwrap_or(50) as usize;
        
        let mut companies = StorageManager::get_all_companies();
        companies.retain(|company| company.community_validation.reputation_score >= min_score);
        companies.sort_by(|a, b| {
            b.community_validation
                .reputation_score
                .cmp(&a.community_validation.reputation_score)
        });
        
        companies.into_iter().take(limit).collect()
    }

    pub fn get_endorsements_for_company(company_id: String) -> RegistryResult<Vec<Endorsement>> {
        match StorageManager::get_company(&company_id) {
            Some(company) => RegistryResult::Ok(company.community_validation.peer_endorsements),
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    pub fn get_testimonials_for_company(company_id: String) -> RegistryResult<Vec<Testimonial>> {
        match StorageManager::get_company(&company_id) {
            Some(company) => RegistryResult::Ok(company.community_validation.employee_testimonials),
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    pub fn get_vouches_for_company(company_id: String) -> RegistryResult<Vec<Vouch>> {
        match StorageManager::get_company(&company_id) {
            Some(company) => RegistryResult::Ok(company.community_validation.community_vouches),
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    // Statistics and analytics functions
    pub fn get_community_validation_stats(company_id: String) -> RegistryResult<CommunityValidationStats> {
        match StorageManager::get_company(&company_id) {
            Some(company) => {
                let cv = &company.community_validation;
                let stats = CommunityValidationStats {
                    total_endorsements: cv.peer_endorsements.len() as u32,
                    total_testimonials: cv.employee_testimonials.len() as u32,
                    verified_testimonials: cv.employee_testimonials.iter()
                        .filter(|t| t.verified)
                        .count() as u32,
                    total_vouches: cv.community_vouches.len() as u32,
                    reputation_score: cv.reputation_score,
                    reputation_staked: cv.reputation_staked,
                };
                RegistryResult::Ok(stats)
            }
            None => RegistryResult::Err("Company not found".to_string()),
        }
    }

    pub fn get_reputation_leaderboard(limit: Option<u32>) -> Vec<ReputationLeaderboard> {
        let limit = limit.unwrap_or(20) as usize;
        
        let mut companies = StorageManager::get_all_companies();
        companies.sort_by(|a, b| {
            b.community_validation
                .reputation_score
                .cmp(&a.community_validation.reputation_score)
        });
        
        companies
            .into_iter()
            .take(limit)
            .map(|company| ReputationLeaderboard {
                company_id: company.id,
                company_name: company.basic_info.name,
                reputation_score: company.community_validation.reputation_score,
                reputation_staked: company.community_validation.reputation_staked,
            })
            .collect()
    }

    pub fn get_endorsements_by_company(endorser_company_id: String) -> RegistryResult<Vec<(String, Endorsement)>> {
        // Get all endorsements made by a specific company
        let all_companies = StorageManager::get_all_companies();
        let mut endorsements = Vec::new();

        for company in all_companies {
            let company_id = company.id.clone();
            for endorsement in company.community_validation.peer_endorsements {
                if endorsement.endorser_company_id == endorser_company_id {
                    endorsements.push((company_id.clone(), endorsement));
                }
            }
        }

        RegistryResult::Ok(endorsements)
    }

    pub fn get_vouches_by_principal(voucher_principal: Principal) -> Vec<(String, Vouch)> {
        // Get all vouches made by a specific principal
        let all_companies = StorageManager::get_all_companies();
        let mut vouches = Vec::new();

        for company in all_companies {
            let company_id = company.id.clone();
            for vouch in company.community_validation.community_vouches {
                if vouch.voucher_principal == voucher_principal {
                    vouches.push((company_id.clone(), vouch));
                }
            }
        }

        vouches
    }

    pub fn get_testimonials_by_author(author_name: String) -> Vec<(String, Testimonial)> {
        // Get all testimonials written by a specific author
        let all_companies = StorageManager::get_all_companies();
        let mut testimonials = Vec::new();

        for company in all_companies {
            let company_id = company.id.clone();
            for testimonial in company.community_validation.employee_testimonials {
                if testimonial.author_name == author_name {
                    testimonials.push((company_id.clone(), testimonial));
                }
            }
        }

        testimonials
    }

    // Moderation functions (for future admin features)
    pub fn flag_testimonial(
        company_id: String,
        author_name: String,
        _admin_principal: Principal,
    ) -> RegistryResult<()> {
        // This could be used by moderators to flag inappropriate testimonials
        // For now, we'll just mark them as unverified
        let success = StorageManager::update_company(&company_id, |company| {
            if let Some(testimonial) = company
                .community_validation
                .employee_testimonials
                .iter_mut()
                .find(|t| t.author_name == author_name)
            {
                testimonial.verified = false;
            }
            Self::update_reputation_score(company);
        });

        if success {
            RegistryResult::Ok(())
        } else {
            RegistryResult::Err("Company or testimonial not found".to_string())
        }
    }

    // Validation helper functions
    pub fn validate_endorsement_eligibility(
        endorser_company_id: String,
        target_company_id: String,
    ) -> RegistryResult<bool> {
        let endorser = match StorageManager::get_company(&endorser_company_id) {
            Some(company) => company,
            None => return RegistryResult::Err("Endorser company not found".to_string()),
        };

        // Check if endorser company has sufficient reputation to endorse
        if endorser.community_validation.reputation_score < 10 {
            return RegistryResult::Ok(false);
        }

        // Check if endorsement already exists
        if let Some(target) = StorageManager::get_company(&target_company_id) {
            let already_endorsed = target
                .community_validation
                .peer_endorsements
                .iter()
                .any(|e| e.endorser_company_id == endorser_company_id);
            
            if already_endorsed {
                return RegistryResult::Ok(false);
            }
        }

        RegistryResult::Ok(true)
    }
}