use crate::types::{Company, DomainVerificationChallenge, CrossChainChallenge};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;

// Global state management
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static COMPANIES: RefCell<StableBTreeMap<String, Company, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0)))
        )
    );

    static DOMAIN_CHALLENGES: RefCell<StableBTreeMap<String, DomainVerificationChallenge, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
        )
    );

    static CROSSCHAIN_CHALLENGES: RefCell<StableBTreeMap<String, CrossChainChallenge, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
        )
    );
}

// Storage abstraction layer
pub struct StorageManager;

impl StorageManager {
    // Company storage operations
    pub fn insert_company(company_id: String, company: Company) {
        COMPANIES.with(|companies| {
            companies.borrow_mut().insert(company_id, company);
        });
    }

    pub fn get_company(company_id: &str) -> Option<Company> {
        COMPANIES.with(|companies| {
            companies.borrow().get(&company_id.to_string())
        })
    }

    pub fn update_company<F>(company_id: &str, update_fn: F) -> bool 
    where 
        F: FnOnce(&mut Company)
    {
        COMPANIES.with(|companies| {
            let mut companies_map = companies.borrow_mut();
            if let Some(mut company) = companies_map.get(&company_id.to_string()) {
                update_fn(&mut company);
                company.updated_at = time();
                companies_map.insert(company_id.to_string(), company);
                true
            } else {
                false
            }
        })
    }

    pub fn get_all_companies() -> Vec<Company> {
        COMPANIES.with(|companies| {
            companies
                .borrow()
                .iter()
                .map(|(_, company)| company)
                .collect()
        })
    }

    pub fn get_companies_count() -> u64 {
        COMPANIES.with(|companies| companies.borrow().len())
    }

    // Domain challenge storage operations
    pub fn insert_domain_challenge(company_id: String, challenge: DomainVerificationChallenge) {
        DOMAIN_CHALLENGES.with(|challenges| {
            challenges.borrow_mut().insert(company_id, challenge);
        });
    }

    pub fn get_domain_challenge(company_id: &str) -> Option<DomainVerificationChallenge> {
        DOMAIN_CHALLENGES.with(|challenges| {
            challenges.borrow().get(&company_id.to_string())
        })
    }

    pub fn remove_domain_challenge(company_id: &str) -> Option<DomainVerificationChallenge> {
        DOMAIN_CHALLENGES.with(|challenges| {
            challenges.borrow_mut().remove(&company_id.to_string())
        })
    }

    // Cross-chain challenge storage operations
    pub fn insert_crosschain_challenge(challenge_key: String, challenge: CrossChainChallenge) {
        CROSSCHAIN_CHALLENGES.with(|challenges| {
            challenges.borrow_mut().insert(challenge_key, challenge);
        });
    }

    pub fn get_crosschain_challenge(challenge_key: &str) -> Option<CrossChainChallenge> {
        CROSSCHAIN_CHALLENGES.with(|challenges| {
            challenges.borrow().get(&challenge_key.to_string())
        })
    }

    pub fn remove_crosschain_challenge(challenge_key: &str) -> Option<CrossChainChallenge> {
        CROSSCHAIN_CHALLENGES.with(|challenges| {
            challenges.borrow_mut().remove(&challenge_key.to_string())
        })
    }

    pub fn get_crosschain_challenges_for_company(company_id: &str) -> Vec<CrossChainChallenge> {
        CROSSCHAIN_CHALLENGES.with(|challenges| {
            challenges
                .borrow()
                .iter()
                .filter_map(|(_, challenge)| {
                    if challenge.company_id == company_id {
                        Some(challenge)
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    // Utility functions
    pub fn generate_company_id() -> String {
        format!("company_{}", time())
    }

    pub fn generate_crosschain_challenge_key(company_id: &str, chain_type: &str, address: &str) -> String {
        format!("{}_{}_{}_{}", company_id, chain_type, address, time())
    }

    pub fn find_crosschain_challenge_key(company_id: &str, chain_type: &str, address: &str) -> Option<String> {
        CROSSCHAIN_CHALLENGES.with(|challenges| {
            challenges
                .borrow()
                .iter()
                .find_map(|(key, challenge)| {
                    let challenge_chain = match challenge.chain_type {
                        crate::types::ChainType::Ethereum => "ethereum",
                        crate::types::ChainType::Bitcoin => "bitcoin",
                        crate::types::ChainType::ICP => "icp",
                        crate::types::ChainType::Polygon => "polygon",
                    };
                    
                    if challenge.company_id == company_id 
                        && challenge_chain == chain_type 
                        && challenge.address_or_contract == address {
                        Some(key)
                    } else {
                        None
                    }
                })
        })
    }
}