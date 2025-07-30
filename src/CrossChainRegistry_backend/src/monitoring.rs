use candid::Principal;
use ic_cdk::api::time;

use crate::types::*;
use crate::storage::StorageManager;

pub struct MonitoringSystem;

impl MonitoringSystem {
    // Security Event Logging
    
    pub fn log_security_event(
        event_type: SecurityEventType,
        principal: Option<Principal>,
        severity: SecuritySeverity,
        details: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> String {
        let event_id = Self::generate_event_id();
        let event = SecurityEvent {
            event_id: event_id.clone(),
            event_type: event_type.clone(),
            principal,
            timestamp: time(),
            severity: severity.clone(),
            details: details.clone(),
            ip_address,
            user_agent,
        };
        
        StorageManager::store_security_event(event);
        
        // For critical events, create immediate alerts
        if matches!(severity, SecuritySeverity::Critical | SecuritySeverity::High) {
            Self::create_security_alert(&event_id, &event_type, &details);
        }
        
        event_id
    }
    
    pub fn get_security_events_by_severity(severity: SecuritySeverity) -> Vec<SecurityEvent> {
        StorageManager::get_security_events_by_severity(severity)
    }
    
    pub fn get_security_events_by_principal(principal: Principal) -> Vec<SecurityEvent> {
        StorageManager::get_security_events_by_principal(principal)
    }
    
    // Proof Monitoring System
    
    pub fn schedule_proof_monitoring(company_id: String, proof_id: String, priority: TaskPriority) {
        let task = MonitoringTask {
            task_id: Self::generate_task_id(),
            task_type: TaskType::ProofCheck,
            target_company_id: company_id,
            target_proof_id: Some(proof_id),
            scheduled_at: time() + 3600_000_000_000, // Check in 1 hour
            priority,
            retry_count: 0,
            max_retries: 3,
            last_error: None,
        };
        
        StorageManager::store_monitoring_task(task);
    }
    
    pub fn process_monitoring_tasks() -> Vec<String> {
        let current_time = time();
        let pending_tasks = StorageManager::get_pending_monitoring_tasks(current_time);
        let mut processed_tasks = Vec::new();
        
        for mut task in pending_tasks {
            match Self::execute_monitoring_task(&mut task) {
                Ok(_) => {
                    processed_tasks.push(task.task_id.clone());
                    StorageManager::remove_monitoring_task(&task.task_id);
                }
                Err(error) => {
                    task.retry_count += 1;
                    task.last_error = Some(error);
                    
                    if task.retry_count >= task.max_retries {
                        // Task failed permanently
                        Self::log_security_event(
                            SecurityEventType::ProofTampering,
                            None,
                            SecuritySeverity::High,
                            format!("Monitoring task failed permanently: {}", task.task_id),
                            None,
                            None,
                        );
                        StorageManager::remove_monitoring_task(&task.task_id);
                    } else {
                        // Reschedule with exponential backoff
                        task.scheduled_at = current_time + (task.retry_count as u64 * 3600_000_000_000);
                        StorageManager::update_monitoring_task(task);
                    }
                }
            }
        }
        
        processed_tasks
    }
    
    fn execute_monitoring_task(task: &MonitoringTask) -> Result<(), String> {
        match task.task_type {
            TaskType::ProofCheck => Self::check_proof_status(task),
            TaskType::ContentValidation => Self::validate_proof_content(task),
            TaskType::ReputationUpdate => Self::update_reputation_scores(task),
            TaskType::SecurityScan => Self::perform_security_scan(task),
            TaskType::CommunityAlert => Self::send_community_alert(task),
        }
    }
    
    fn check_proof_status(task: &MonitoringTask) -> Result<(), String> {
        let company = StorageManager::get_company(&task.target_company_id)
            .ok_or("Company not found")?;
            
        if let Some(proof_id) = &task.target_proof_id {
            // Find the specific proof to check
            for proof in &company.web3_identity.verification_proofs {
                if Self::generate_proof_id(proof) == *proof_id {
                    return Self::verify_proof_still_exists(proof, &task.target_company_id);
                }
            }
            Err("Proof not found".to_string())
        } else {
            // Check all proofs for the company
            for proof in &company.web3_identity.verification_proofs {
                if let Err(e) = Self::verify_proof_still_exists(proof, &task.target_company_id) {
                    Self::log_security_event(
                        SecurityEventType::ProofTampering,
                        None,
                        SecuritySeverity::High,
                        format!("Proof check failed for company {}: {}", task.target_company_id, e),
                        None,
                        None,
                    );
                }
            }
            Ok(())
        }
    }
    
    fn verify_proof_still_exists(proof: &VerificationProof, company_id: &str) -> Result<(), String> {
        // This would ideally make HTTP requests to check if the proof URLs are still valid
        // For now, we'll simulate the check and focus on the monitoring infrastructure
        
        // TODO: Implement actual HTTP verification for different platforms:
        // - Twitter: Check if tweet exists via API
        // - Discord: Check if message exists (more complex)
        // - Telegram: Check if message exists
        // - GitHub: Check if file/commit exists
        
        // Simulate some checks failing randomly for demonstration
        let url_accessible = Self::simulate_url_check(&proof.proof_url);
        
        if !url_accessible {
            // Proof appears to be deleted/inaccessible
            Self::handle_missing_proof(proof, company_id)?;
            return Err("Proof URL no longer accessible".to_string());
        }
        
        // Update monitoring record
        let monitoring_record = ProofMonitoring {
            proof_id: Self::generate_proof_id(proof),
            company_id: company_id.to_string(),
            last_checked: time(),
            check_results: vec![ProofCheckResult {
                checker_principal: ic_cdk::caller(),
                timestamp: time(),
                status_found: ProofStatus::Active,
                notes: "Proof verified as still accessible".to_string(),
            }],
            community_reports: vec![],
        };
        
        StorageManager::update_proof_monitoring(monitoring_record);
        
        Ok(())
    }
    
    fn handle_missing_proof(proof: &VerificationProof, company_id: &str) -> Result<(), String> {
        // Update proof status to Removed
        StorageManager::update_proof_status(company_id, proof, ProofStatus::Removed)?;
        
        // Create community alert
        let alert = CommunityAlert {
            alert_id: Self::generate_alert_id(),
            company_id: company_id.to_string(),
            alert_type: AlertType::ProofDeleted,
            message: format!(
                "🚨 TRUST ALERT: {} verification proof has been deleted after verification. Original post is no longer accessible.",
                proof.verification_type
            ),
            evidence: vec![proof.proof_url.clone()],
            created_at: time(),
            acknowledged: false,
            severity: AlertSeverity::Error,
        };
        
        StorageManager::store_community_alert(alert);
        
        // Log security event
        Self::log_security_event(
            SecurityEventType::ProofTampering,
            None,
            SecuritySeverity::High,
            format!("Verification proof deleted: {} for company {}", proof.proof_url, company_id),
            None,
            None,
        );
        
        // Schedule reputation update
        let reputation_task = MonitoringTask {
            task_id: Self::generate_task_id(),
            task_type: TaskType::ReputationUpdate,
            target_company_id: company_id.to_string(),
            target_proof_id: None,
            scheduled_at: time() + 300_000_000_000, // 5 minutes
            priority: TaskPriority::High,
            retry_count: 0,
            max_retries: 1,
            last_error: None,
        };
        
        StorageManager::store_monitoring_task(reputation_task);
        
        Ok(())
    }
    
    fn validate_proof_content(_task: &MonitoringTask) -> Result<(), String> {
        // TODO: Implement content validation
        // This would check if the verification post still contains the required challenge text
        Ok(())
    }
    
    fn update_reputation_scores(task: &MonitoringTask) -> Result<(), String> {
        let mut company = StorageManager::get_company(&task.target_company_id)
            .ok_or("Company not found")?;
            
        // Calculate reputation penalty for missing proofs
        let removed_proofs = company.web3_identity.verification_proofs
            .iter()
            .filter(|p| p.status == ProofStatus::Removed)
            .count();
            
        let disputed_proofs = company.web3_identity.verification_proofs
            .iter()
            .filter(|p| p.status == ProofStatus::Disputed)
            .count();
            
        // Apply reputation penalties
        let penalty = (removed_proofs * 20) + (disputed_proofs * 10);
        company.verification_score = company.verification_score.saturating_sub(penalty as u32);
        
        // Update company status if reputation is too low
        if company.verification_score < 30 {
            company.status = CompanyStatus::Flagged;
        }
        
        StorageManager::update_company(&task.target_company_id, |comp| {
            *comp = company.clone();
        });
        
        Ok(())
    }
    
    fn perform_security_scan(_task: &MonitoringTask) -> Result<(), String> {
        // TODO: Implement periodic security scanning
        // - Check for suspicious patterns
        // - Validate all active proofs
        // - Review recent security events
        Ok(())
    }
    
    fn send_community_alert(_task: &MonitoringTask) -> Result<(), String> {
        // TODO: Implement community notification system
        // This would notify community members about important events
        Ok(())
    }
    
    // Community Reporting
    
    pub fn submit_community_report(
        company_id: String,
        proof_id: Option<String>,
        report_type: ReportType,
        evidence: String,
        reporter: Principal,
    ) -> Result<String, String> {
        // Rate limit community reports
        if !StorageManager::check_report_rate_limit(reporter) {
            return Err("Rate limit exceeded for community reports".to_string());
        }
        
        let report = CommunityReport {
            reporter_principal: reporter,
            report_type: report_type.clone(),
            evidence: evidence.clone(),
            timestamp: time(),
        };
        
        // Add report to proof monitoring record
        if let Some(proof_id) = proof_id {
            StorageManager::add_community_report(&company_id, &proof_id, report);
            
            // If enough reports, mark proof as disputed
            let report_count = StorageManager::get_community_report_count(&company_id, &proof_id);
            if report_count >= 3 {
                StorageManager::update_proof_status_by_id(&company_id, &proof_id, ProofStatus::Disputed)?;
                
                // Create high-priority monitoring task
                Self::schedule_proof_monitoring(company_id.clone(), proof_id.clone(), TaskPriority::Critical);
            }
        }
        
        // Log the community report
        Self::log_security_event(
            SecurityEventType::CommunityReport,
            Some(reporter),
            SecuritySeverity::Medium,
            format!("Community report: {:?} - {}", report_type, evidence),
            None,
            None,
        );
        
        Ok("Report submitted successfully".to_string())
    }
    
    // Monitoring Statistics
    
    pub fn get_monitoring_stats() -> MonitoringStats {
        let companies = StorageManager::get_all_companies();
        let mut stats = MonitoringStats {
            total_proofs_monitored: 0,
            active_proofs: 0,
            removed_proofs: 0,
            disputed_proofs: 0,
            last_full_scan: StorageManager::get_last_full_scan_time(),
            security_events_today: StorageManager::get_security_events_count_today(),
            failed_checks_count: StorageManager::get_failed_monitoring_tasks_count(),
        };
        
        for company in companies {
            for proof in &company.web3_identity.verification_proofs {
                stats.total_proofs_monitored += 1;
                match proof.status {
                    ProofStatus::Active => stats.active_proofs += 1,
                    ProofStatus::Removed => stats.removed_proofs += 1,
                    ProofStatus::Disputed => stats.disputed_proofs += 1,
                }
            }
        }
        
        stats
    }
    
    pub fn get_community_alerts(acknowledged: Option<bool>) -> Vec<CommunityAlert> {
        StorageManager::get_community_alerts(acknowledged)
    }
    
    pub fn acknowledge_alert(alert_id: String) -> Result<(), String> {
        StorageManager::acknowledge_community_alert(&alert_id)
    }
    
    // Utility Functions
    
    fn generate_event_id() -> String {
        format!("evt_{}", time())
    }
    
    fn generate_task_id() -> String {
        format!("task_{}", time())
    }
    
    fn generate_alert_id() -> String {
        format!("alert_{}", time())
    }
    
    fn generate_proof_id(proof: &VerificationProof) -> String {
        format!("{}_{}", proof.verification_type, proof.verified_at)
    }
    
    fn simulate_url_check(url: &str) -> bool {
        // Simulate some URLs being inaccessible for testing
        // In real implementation, this would make actual HTTP requests
        !url.contains("deleted") && !url.contains("404")
    }
    
    fn create_security_alert(event_id: &str, event_type: &SecurityEventType, details: &str) {
        let alert = CommunityAlert {
            alert_id: Self::generate_alert_id(),
            company_id: "system".to_string(),
            alert_type: AlertType::SecurityBreach,
            message: format!("Security Event {}: {:?}", event_id, event_type),
            evidence: vec![details.to_string()],
            created_at: time(),
            acknowledged: false,
            severity: AlertSeverity::Critical,
        };
        
        StorageManager::store_community_alert(alert);
    }
}