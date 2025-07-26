use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::Storable;
use serde::Serialize;
use std::borrow::Cow;

// Core Data Structures

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CompanyBasicInfo {
    pub name: String,
    pub description: String,
    pub website: String,
    pub founding_date: String,
    pub team_size: u32,
    pub focus_areas: Vec<String>, // DeFi, NFTs, Infrastructure, etc.
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Web3Identity {
    pub github_org: Option<String>,
    pub twitter_handle: Option<String>,
    pub discord_server: Option<String>,
    pub telegram_channel: Option<String>,
    pub domain_verified: bool,
    pub social_verification_status: VerificationStatus,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CrossChainPresence {
    pub ethereum_contracts: Vec<String>,
    pub bitcoin_addresses: Vec<String>,
    pub icp_canisters: Vec<String>,
    pub polygon_contracts: Vec<String>,
    pub treasury_wallets: Vec<WalletInfo>,
    pub token_contracts: Vec<TokenInfo>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct WalletInfo {
    pub chain: String,
    pub address: String,
    pub wallet_type: String, // treasury, operational, etc.
    pub verified: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TokenInfo {
    pub chain: String,
    pub contract_address: String,
    pub symbol: String,
    pub name: String,
    pub verified: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
    pub github_profile: Option<String>,
    pub linkedin_profile: Option<String>,
    pub verified: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CommunityValidation {
    pub peer_endorsements: Vec<Endorsement>,
    pub employee_testimonials: Vec<Testimonial>,
    pub community_vouches: Vec<Vouch>,
    pub reputation_score: u32,
    pub reputation_staked: u64, // tokens staked for credibility
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Endorsement {
    pub endorser_company_id: String,
    pub message: String,
    pub timestamp: u64,
    pub endorser_principal: Principal,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Testimonial {
    pub author_name: String,
    pub role: String,
    pub message: String,
    pub timestamp: u64,
    pub verified: bool,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Vouch {
    pub voucher_principal: Principal,
    pub message: String,
    pub timestamp: u64,
    pub weight: u32, // based on voucher's reputation
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
    Expired,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum CompanyStatus {
    Pending,      // Initial registration
    Verified,     // Basic verification complete
    Trusted,      // High reputation, community validated
    Flagged,      // Community reported issues
    Suspended,    // Admin action or severe violations
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Company {
    pub id: String,
    pub basic_info: CompanyBasicInfo,
    pub web3_identity: Web3Identity,
    pub cross_chain_presence: CrossChainPresence,
    pub team_members: Vec<TeamMember>,
    pub community_validation: CommunityValidation,
    pub status: CompanyStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub created_by: Principal,
    pub verification_score: u32, // Composite score based on all verifications
}

// API Request/Response Types

#[derive(CandidType, Deserialize)]
pub enum RegistryResult<T> {
    Ok(T),
    Err(String),
}

// Community Validation Request Types

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateEndorsementRequest {
    pub company_id: String,
    pub endorser_company_id: String,
    pub message: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateTestimonialRequest {
    pub company_id: String,
    pub author_name: String,
    pub role: String,
    pub message: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateVouchRequest {
    pub company_id: String,
    pub message: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct StakeReputationRequest {
    pub company_id: String,
    pub amount: u64,
}

// Community Validation Response Types

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CommunityValidationStats {
    pub total_endorsements: u32,
    pub total_testimonials: u32,
    pub verified_testimonials: u32,
    pub total_vouches: u32,
    pub reputation_score: u32,
    pub reputation_staked: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ReputationLeaderboard {
    pub company_id: String,
    pub company_name: String,
    pub reputation_score: u32,
    pub reputation_staked: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct CreateCompanyRequest {
    pub basic_info: CompanyBasicInfo,
    pub web3_identity: Web3Identity,
    pub cross_chain_presence: CrossChainPresence,
    pub team_members: Vec<TeamMember>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UpdateCompanyRequest {
    pub company_id: String,
    pub basic_info: Option<CompanyBasicInfo>,
    pub web3_identity: Option<Web3Identity>,
    pub cross_chain_presence: Option<CrossChainPresence>,
    pub team_members: Option<Vec<TeamMember>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SearchFilters {
    pub status: Option<CompanyStatus>,
    pub focus_areas: Option<Vec<String>>,
    pub min_verification_score: Option<u32>,
    pub has_github: Option<bool>,
    pub has_contracts: Option<bool>,
}

// Verification System Types

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct VerificationRequest {
    pub company_id: String,
    pub verification_type: VerificationType,
    pub proof_data: String, // Challenge response or proof
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum VerificationType {
    GitHub,
    Domain,
    Twitter,
    Discord,
    Telegram,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct VerificationResult {
    pub success: bool,
    pub message: String,
    pub verified_at: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct DomainVerificationChallenge {
    pub company_id: String,
    pub domain: String,
    pub challenge_token: String,
    pub created_at: u64,
    pub expires_at: u64,
}

//Cross-Chain Verification Types

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum ChainType {
    Ethereum,
    Bitcoin,
    ICP,
    Polygon,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CrossChainVerificationRequest {
    pub company_id: String,
    pub chain_type: ChainType,
    pub address_or_contract: String,
    pub verification_method: CrossChainVerificationMethod,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum CrossChainVerificationMethod {
    SignMessage { message: String },           // For wallet address verification
    DeploySpecialContract { verification_code: String }, // For contract ownership
    SetPublicVariable { variable_name: String, value: String }, // For existing contracts
    SpecialTransaction { transaction_data: String }, // For Bitcoin/other chains
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CrossChainChallenge {
    pub company_id: String,
    pub chain_type: ChainType,
    pub address_or_contract: String,
    pub challenge_message: String,
    pub verification_method: CrossChainVerificationMethod,
    pub created_at: u64,
    pub expires_at: u64,
}

// API Response structures for different chains
#[derive(Deserialize, Debug)]
pub struct EtherscanContractResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<EtherscanTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct EtherscanTransaction {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub input: String,
    #[serde(rename = "timeStamp")]
    pub timestamp: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockchainInfoResponse {
    pub address: String,
    pub hash160: String,
    pub n_tx: u32,
    pub n_unredeemed: u32,
    pub total_received: u64,
    pub total_sent: u64,
    pub final_balance: u64,
}

#[derive(Deserialize, Debug)]
pub struct ICPCanisterResponse {
    pub canister_id: String,
    pub status: String,
    pub controllers: Vec<String>,
    pub memory_size: u64,
    pub cycles: u64,
}

// GitHub API Response structures
#[derive(Deserialize)]
pub struct GitHubOrgResponse {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub blog: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct GitHubRepoResponse {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
}

// Implement Storable for types that need to be stored in stable structures

impl Storable for Company {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for DomainVerificationChallenge {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for CrossChainChallenge {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}