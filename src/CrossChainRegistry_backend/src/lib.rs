mod api;
mod community;
mod crosschain;
mod storage;
mod types;
mod verification;

use api::RegistryAPI;
use community::CommunityValidationManager;
use crosschain::CrossChainVerifier;
use ic_cdk::api::management_canister::http_request::TransformArgs;
use storage::StorageManager;
use types::{
    ChainType, Company, CommunityValidation, CommunityValidationStats, CreateCompanyRequest, 
    CrossChainChallenge, CrossChainVerificationRequest, DomainVerificationChallenge, Endorsement, 
    ProofCheckResult, ProofStatus, RegistryResult, ReportType, ReputationLeaderboard, SearchFilters, 
    Testimonial, UpdateCompanyRequest, VerificationResult, VerificationType, Vouch,
};
use verification::VerificationManager;
use std::collections::HashMap;

// Core CRUD API endpoints
#[ic_cdk::update]
pub fn create_company(request: CreateCompanyRequest) -> RegistryResult<String> {
    let caller = ic_cdk::caller();
    RegistryAPI::create_company(request, caller)
}

#[ic_cdk::query]
pub fn get_company(company_id: String) -> RegistryResult<Company> {
    RegistryAPI::get_company(company_id)
}

#[ic_cdk::update]
pub fn update_company(request: UpdateCompanyRequest) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    RegistryAPI::update_company(request, caller)
}

#[ic_cdk::query]
pub fn list_companies(
    offset: Option<u32>,
    limit: Option<u32>,
    filters: Option<SearchFilters>,
) -> Vec<Company> {
    RegistryAPI::list_companies(offset, limit, filters)
}

#[ic_cdk::query]
pub fn search_companies(query: String) -> Vec<Company> {
    RegistryAPI::search_companies(query)
}

#[ic_cdk::query]
pub fn get_company_count() -> u64 {
    RegistryAPI::get_company_count()
}

#[ic_cdk::query]
pub fn get_statistics() -> HashMap<String, u64> {
    RegistryAPI::get_statistics()
}

// Verification API endpoints
#[ic_cdk::update]
async fn verify_github_organization(
    company_id: String,
    github_org: String,
) -> RegistryResult<VerificationResult> {
    let caller = ic_cdk::caller();
    VerificationManager::verify_github_organization(company_id, github_org, caller).await
}

#[ic_cdk::update]
fn create_domain_verification_challenge(
    company_id: String,
) -> RegistryResult<DomainVerificationChallenge> {
    let caller = ic_cdk::caller();
    VerificationManager::create_domain_verification_challenge(company_id, caller)
}

#[ic_cdk::update]
async fn verify_domain_ownership(company_id: String) -> RegistryResult<VerificationResult> {
    let caller = ic_cdk::caller();
    VerificationManager::verify_domain_ownership(company_id, caller).await
}

#[ic_cdk::update]
fn verify_social_media_manual(
    company_id: String,
    platform: String,
    proof_url: String,
) -> RegistryResult<VerificationResult> {
    let caller = ic_cdk::caller();
    VerificationManager::verify_social_media_manual(company_id, platform, proof_url, caller)
}

#[ic_cdk::update]
fn verify_social_media_with_proof(
    company_id: String,
    platform: String,
    proof_url: String,
) -> RegistryResult<VerificationResult> {
    let caller = ic_cdk::caller();
    VerificationManager::verify_social_media_with_proof(company_id, platform, proof_url, caller)
}

#[ic_cdk::update]
async fn verify_proof_still_exists(
    company_id: String,
    proof_url: String,
) -> RegistryResult<types::ProofCheckResult> {
    let caller = ic_cdk::caller();
    VerificationManager::verify_proof_still_exists(company_id, proof_url, caller).await
}

#[ic_cdk::update]
fn report_verification_issue(
    company_id: String,
    proof_url: String,
    report_type: types::ReportType,
    evidence: String,
) -> RegistryResult<String> {
    let caller = ic_cdk::caller();
    VerificationManager::report_verification_issue(company_id, proof_url, report_type, evidence, caller)
}

// Verification utility endpoints
#[ic_cdk::query]
fn get_domain_verification_challenge(company_id: String) -> Option<DomainVerificationChallenge> {
    StorageManager::get_domain_challenge(&company_id)
}

#[ic_cdk::query]
fn get_verification_instructions(verification_type: VerificationType) -> String {
    VerificationManager::get_verification_instructions(verification_type)
}

// Cross-chain verification API endpoints
#[ic_cdk::update]
fn create_crosschain_challenge(
    request: CrossChainVerificationRequest,
) -> RegistryResult<CrossChainChallenge> {
    let caller = ic_cdk::caller();
    CrossChainVerifier::create_crosschain_challenge(request, caller)
}

#[ic_cdk::update]
async fn verify_ethereum_contract(
    company_id: String,
    contract_address: String,
) -> RegistryResult<VerificationResult> {
    CrossChainVerifier::verify_ethereum_contract(company_id, contract_address).await
}

#[ic_cdk::update]
async fn verify_bitcoin_address(
    company_id: String,
    bitcoin_address: String,
) -> RegistryResult<VerificationResult> {
    CrossChainVerifier::verify_bitcoin_address(company_id, bitcoin_address).await
}

#[ic_cdk::update]
async fn verify_icp_canister(
    company_id: String,
    canister_id: String,
) -> RegistryResult<VerificationResult> {
    CrossChainVerifier::verify_icp_canister(company_id, canister_id).await
}

#[ic_cdk::query]
fn get_crosschain_verification_instructions(chain_type: ChainType) -> String {
    CrossChainVerifier::get_crosschain_verification_instructions(chain_type)
}

#[ic_cdk::query]
fn get_crosschain_challenges_for_company(company_id: String) -> Vec<CrossChainChallenge> {
    StorageManager::get_crosschain_challenges_for_company(&company_id)
}

// HTTP transform functions for HTTPS outcalls
#[ic_cdk::query]
fn transform_github_response(raw: TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    verification::transform_github_response(raw)
}

#[ic_cdk::query]
fn transform_domain_response(raw: TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    verification::transform_domain_response(raw)
}

#[ic_cdk::query]
fn transform_proof_check(raw: TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    verification::transform_proof_check(raw)
}

#[ic_cdk::query]
fn transform_etherscan_response(raw: TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    crosschain::transform_etherscan_response(raw)
}

#[ic_cdk::query]
fn transform_blockchain_response(raw: TransformArgs) -> ic_cdk::api::management_canister::http_request::HttpResponse {
    crosschain::transform_blockchain_response(raw)
}

// Community Validation API endpoints

// Endorsement endpoints
#[ic_cdk::update]
pub fn add_endorsement(
    company_id: String,
    endorser_company_id: String,
    message: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::add_endorsement(company_id, endorser_company_id, message, caller)
}

#[ic_cdk::update]
pub fn remove_endorsement(
    company_id: String,
    endorser_company_id: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::remove_endorsement(company_id, endorser_company_id, caller)
}

#[ic_cdk::query]
pub fn get_endorsements_for_company(company_id: String) -> RegistryResult<Vec<Endorsement>> {
    CommunityValidationManager::get_endorsements_for_company(company_id)
}

// Testimonial endpoints
#[ic_cdk::update]
pub fn add_testimonial(
    company_id: String,
    author_name: String,
    role: String,
    message: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::add_testimonial(company_id, author_name, role, message, caller)
}

#[ic_cdk::update]
pub fn remove_testimonial(
    company_id: String,
    author_name: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::remove_testimonial(company_id, author_name, caller)
}

#[ic_cdk::update]
pub fn verify_testimonial(
    company_id: String,
    author_name: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::verify_testimonial(company_id, author_name, caller)
}

#[ic_cdk::query]
pub fn get_testimonials_for_company(company_id: String) -> RegistryResult<Vec<Testimonial>> {
    CommunityValidationManager::get_testimonials_for_company(company_id)
}

// Community vouch endpoints
#[ic_cdk::update]
pub fn add_vouch(
    company_id: String,
    message: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::add_vouch(company_id, message, caller)
}

#[ic_cdk::update]
pub fn remove_vouch(company_id: String) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::remove_vouch(company_id, caller)
}

#[ic_cdk::query]
pub fn get_vouches_for_company(company_id: String) -> RegistryResult<Vec<Vouch>> {
    CommunityValidationManager::get_vouches_for_company(company_id)
}

// Reputation management endpoints
#[ic_cdk::update]
pub fn stake_reputation(
    company_id: String,
    amount: u64,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::stake_reputation(company_id, amount, caller)
}

// Community validation query endpoints
#[ic_cdk::query]
pub fn get_community_validation(company_id: String) -> RegistryResult<CommunityValidation> {
    CommunityValidationManager::get_community_validation(company_id)
}

#[ic_cdk::query]
pub fn get_companies_by_reputation(min_score: u32, limit: Option<u32>) -> Vec<Company> {
    CommunityValidationManager::get_companies_by_reputation(min_score, limit)
}

// Additional community validation endpoints

#[ic_cdk::query]
pub fn get_community_validation_stats(company_id: String) -> RegistryResult<CommunityValidationStats> {
    CommunityValidationManager::get_community_validation_stats(company_id)
}

#[ic_cdk::query]
pub fn get_reputation_leaderboard(limit: Option<u32>) -> Vec<ReputationLeaderboard> {
    CommunityValidationManager::get_reputation_leaderboard(limit)
}

#[ic_cdk::query]
pub fn get_endorsements_by_company(endorser_company_id: String) -> RegistryResult<Vec<(String, Endorsement)>> {
    CommunityValidationManager::get_endorsements_by_company(endorser_company_id)
}

#[ic_cdk::query]
pub fn get_vouches_by_principal() -> Vec<(String, Vouch)> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::get_vouches_by_principal(caller)
}

#[ic_cdk::query]
pub fn get_testimonials_by_author(author_name: String) -> Vec<(String, Testimonial)> {
    CommunityValidationManager::get_testimonials_by_author(author_name)
}

#[ic_cdk::query]
pub fn validate_endorsement_eligibility(
    endorser_company_id: String,
    target_company_id: String,
) -> RegistryResult<bool> {
    CommunityValidationManager::validate_endorsement_eligibility(endorser_company_id, target_company_id)
}

// Moderation endpoints (for future admin features)
#[ic_cdk::update]
pub fn flag_testimonial(
    company_id: String,
    author_name: String,
) -> RegistryResult<()> {
    let caller = ic_cdk::caller();
    CommunityValidationManager::flag_testimonial(company_id, author_name, caller)
}

// Cross-chain address validation endpoints
#[ic_cdk::query]
pub fn validate_address(chain: String, address: String) -> RegistryResult<bool> {
    RegistryAPI::validate_address(chain, address)
}

#[ic_cdk::query]
pub fn get_address_validation_rules(chain: String) -> RegistryResult<String> {
    RegistryAPI::get_address_validation_rules(chain)
}

#[ic_cdk::query]
pub fn get_supported_chains() -> RegistryResult<Vec<String>> {
    RegistryAPI::get_supported_chains()
}