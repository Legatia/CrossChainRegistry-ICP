use crate::types::{
    Company, DomainVerificationChallenge, CrossChainChallenge, SecurityEvent, MonitoringTask, 
    CommunityAlert, ProofMonitoring, SecuritySeverity, ProofStatus, VerificationProof, 
    CommunityReport
};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use std::collections::HashMap;
use candid::Principal;

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

    static SECURITY_EVENTS: RefCell<StableBTreeMap<String, SecurityEvent, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
        )
    );

    static MONITORING_TASKS: RefCell<StableBTreeMap<String, MonitoringTask, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
        )
    );

    static COMMUNITY_ALERTS: RefCell<StableBTreeMap<String, CommunityAlert, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
        )
    );

    static PROOF_MONITORING: RefCell<StableBTreeMap<String, ProofMonitoring, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
        )
    );

    // Rate limiting storage (in-memory, resets on canister upgrade)
    static HTTP_RATE_LIMITS: RefCell<HashMap<Principal, Vec<u64>>> = RefCell::new(HashMap::new());
    static REPORT_RATE_LIMITS: RefCell<HashMap<Principal, Vec<u64>>> = RefCell::new(HashMap::new());
    
    // Monitoring state (in-memory)
    static LAST_FULL_SCAN: RefCell<u64> = RefCell::new(0);
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
                        crate::types::ChainType::Solana => "solana",
                        crate::types::ChainType::Sui => "sui",
                        crate::types::ChainType::TON => "ton",
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

    // Enhanced rate limiting functions with security improvements
    pub fn check_http_rate_limit(principal: Principal) -> bool {
        Self::check_rate_limit_with_config(principal, 10, 60_000_000_000)
    }

    pub fn check_verification_rate_limit(principal: Principal) -> bool {
        // Stricter limit for verification attempts
        Self::check_rate_limit_with_config(principal, 5, 300_000_000_000) // 5 per 5 minutes
    }

    pub fn check_report_rate_limit(principal: Principal) -> bool {
        // Even stricter limit for reporting
        Self::check_rate_limit_with_config(principal, 3, 600_000_000_000) // 3 per 10 minutes
    }

    fn check_rate_limit_with_config(
        principal: Principal, 
        max_requests: usize, 
        window_size_ns: u64
    ) -> bool {
        HTTP_RATE_LIMITS.with(|limits| {
            let mut limits = limits.borrow_mut();
            let now = time();
            let window_start = now.saturating_sub(window_size_ns);

            // Get or create the request history for this principal
            let requests = limits.entry(principal).or_insert_with(Vec::new);

            // Remove requests older than the time window
            requests.retain(|&timestamp| timestamp > window_start);

            // Security: Prevent memory exhaustion by limiting history size
            if requests.len() > 1000 {
                requests.drain(..requests.len() - 100); // Keep only recent 100 requests
            }

            // Check if under the rate limit
            if requests.len() < max_requests {
                requests.push(now);
                true // Allow request
            } else {
                false // Rate limit exceeded
            }
        })
    }

    pub fn get_rate_limit_info(principal: Principal) -> (usize, u64) {
        HTTP_RATE_LIMITS.with(|limits| {
            let limits = limits.borrow();
            if let Some(requests) = limits.get(&principal) {
                let now = time();
                let window_start = now.saturating_sub(60_000_000_000); // 1 minute window
                let recent_requests = requests.iter().filter(|&&timestamp| timestamp > window_start).count();
                let oldest_request = requests.first().copied().unwrap_or(now);
                (recent_requests, now - oldest_request)
            } else {
                (0, 0)
            }
        })
    }

    // Clean up old rate limit data (called periodically)
    pub fn cleanup_rate_limits() {
        const CLEANUP_WINDOW_NS: u64 = 300_000_000_000; // 5 minutes in nanoseconds

        HTTP_RATE_LIMITS.with(|limits| {
            let mut limits = limits.borrow_mut();
            let now = time();
            let cleanup_threshold = now.saturating_sub(CLEANUP_WINDOW_NS);

            // Remove entries that are completely outside the cleanup window
            limits.retain(|_, requests| {
                requests.retain(|&timestamp| timestamp > cleanup_threshold);
                !requests.is_empty()
            });
        })
    }

    // Security Event Storage
    pub fn store_security_event(event: SecurityEvent) {
        SECURITY_EVENTS.with(|events| {
            events.borrow_mut().insert(event.event_id.clone(), event);
        });
    }

    pub fn get_security_events_by_severity(severity: SecuritySeverity) -> Vec<SecurityEvent> {
        SECURITY_EVENTS.with(|events| {
            events.borrow()
                .iter()
                .filter_map(|(_, event)| {
                    if std::mem::discriminant(&event.severity) == std::mem::discriminant(&severity) {
                        Some(event.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    pub fn get_security_events_by_principal(principal: Principal) -> Vec<SecurityEvent> {
        SECURITY_EVENTS.with(|events| {
            events.borrow()
                .iter()
                .filter_map(|(_, event)| {
                    if event.principal == Some(principal) {
                        Some(event.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    pub fn get_security_events_count_today() -> u32 {
        let today_start = time() - 86_400_000_000_000; // 24 hours in nanoseconds
        SECURITY_EVENTS.with(|events| {
            events.borrow()
                .iter()
                .filter(|(_, event)| event.timestamp > today_start)
                .count() as u32
        })
    }

    // Monitoring Task Storage
    pub fn store_monitoring_task(task: MonitoringTask) {
        MONITORING_TASKS.with(|tasks| {
            tasks.borrow_mut().insert(task.task_id.clone(), task);
        });
    }

    pub fn get_pending_monitoring_tasks(current_time: u64) -> Vec<MonitoringTask> {
        MONITORING_TASKS.with(|tasks| {
            tasks.borrow()
                .iter()
                .filter_map(|(_, task)| {
                    if task.scheduled_at <= current_time {
                        Some(task.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
    }

    pub fn update_monitoring_task(task: MonitoringTask) {
        MONITORING_TASKS.with(|tasks| {
            tasks.borrow_mut().insert(task.task_id.clone(), task);
        });
    }

    pub fn remove_monitoring_task(task_id: &str) {
        MONITORING_TASKS.with(|tasks| {
            tasks.borrow_mut().remove(&task_id.to_string());
        });
    }

    pub fn get_failed_monitoring_tasks_count() -> u32 {
        MONITORING_TASKS.with(|tasks| {
            tasks.borrow()
                .iter()
                .filter(|(_, task)| task.retry_count > 0)
                .count() as u32
        })
    }

    // Community Alert Storage
    pub fn store_community_alert(alert: CommunityAlert) {
        COMMUNITY_ALERTS.with(|alerts| {
            alerts.borrow_mut().insert(alert.alert_id.clone(), alert);
        });
    }

    pub fn get_community_alerts(acknowledged: Option<bool>) -> Vec<CommunityAlert> {
        COMMUNITY_ALERTS.with(|alerts| {
            alerts.borrow()
                .iter()
                .filter_map(|(_, alert)| {
                    match acknowledged {
                        Some(ack) if alert.acknowledged == ack => Some(alert.clone()),
                        None => Some(alert.clone()),
                        _ => None,
                    }
                })
                .collect()
        })
    }

    pub fn acknowledge_community_alert(alert_id: &str) -> Result<(), String> {
        COMMUNITY_ALERTS.with(|alerts| {
            let mut alerts = alerts.borrow_mut();
            if let Some(mut alert) = alerts.get(&alert_id.to_string()) {
                alert.acknowledged = true;
                alerts.insert(alert_id.to_string(), alert);
                Ok(())
            } else {
                Err("Alert not found".to_string())
            }
        })
    }

    // Proof Monitoring Storage
    pub fn update_proof_monitoring(monitoring: ProofMonitoring) {
        PROOF_MONITORING.with(|proof_monitoring| {
            proof_monitoring.borrow_mut().insert(monitoring.proof_id.clone(), monitoring);
        });
    }

    pub fn get_proof_monitoring(proof_id: &str) -> Option<ProofMonitoring> {
        PROOF_MONITORING.with(|proof_monitoring| {
            proof_monitoring.borrow().get(&proof_id.to_string()).map(|m| m.clone())
        })
    }

    pub fn add_community_report(company_id: &str, proof_id: &str, report: CommunityReport) {
        PROOF_MONITORING.with(|proof_monitoring| {
            let mut monitoring = proof_monitoring.borrow_mut();
            if let Some(mut proof_monitor) = monitoring.get(&proof_id.to_string()) {
                proof_monitor.community_reports.push(report);
                monitoring.insert(proof_id.to_string(), proof_monitor);
            } else {
                // Create new monitoring record
                let new_monitoring = ProofMonitoring {
                    proof_id: proof_id.to_string(),
                    company_id: company_id.to_string(),
                    last_checked: time(),
                    check_results: vec![],
                    community_reports: vec![report],
                };
                monitoring.insert(proof_id.to_string(), new_monitoring);
            }
        });
    }

    pub fn get_community_report_count(_company_id: &str, proof_id: &str) -> usize {
        PROOF_MONITORING.with(|proof_monitoring| {
            if let Some(monitoring) = proof_monitoring.borrow().get(&proof_id.to_string()) {
                monitoring.community_reports.len()
            } else {
                0
            }
        })
    }

    // Proof Status Updates
    pub fn update_proof_status(company_id: &str, proof: &VerificationProof, new_status: ProofStatus) -> Result<(), String> {
        let mut company = Self::get_company(company_id).ok_or("Company not found")?;
        
        for company_proof in &mut company.web3_identity.verification_proofs {
            if company_proof.verification_type == proof.verification_type 
                && company_proof.verified_at == proof.verified_at {
                company_proof.status = new_status.clone();
                let final_company = company.clone();
                Self::update_company(company_id, |comp| {
                    *comp = final_company;
                });
                return Ok(());
            }
        }
        
        Err("Proof not found".to_string())
    }

    pub fn update_proof_status_by_id(company_id: &str, proof_id: &str, new_status: ProofStatus) -> Result<(), String> {
        let mut company = Self::get_company(company_id).ok_or("Company not found")?;
        
        for proof in &mut company.web3_identity.verification_proofs {
            let generated_id = format!("{}_{}", proof.verification_type, proof.verified_at);
            if generated_id == proof_id {
                proof.status = new_status.clone();
                let final_company = company.clone();
                Self::update_company(company_id, |comp| {
                    *comp = final_company;
                });
                return Ok(());
            }
        }
        
        Err("Proof not found".to_string())
    }


    // Monitoring State
    pub fn get_last_full_scan_time() -> u64 {
        LAST_FULL_SCAN.with(|scan_time| *scan_time.borrow())
    }

    pub fn update_last_full_scan_time() {
        LAST_FULL_SCAN.with(|scan_time| {
            *scan_time.borrow_mut() = time();
        });
    }

}