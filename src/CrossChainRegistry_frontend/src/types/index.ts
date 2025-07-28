import { Principal } from '@dfinity/principal';

// Core Data Structures matching backend types

export interface CompanyBasicInfo {
  name: string;
  description: string;
  website: string;
  founding_date: string;
  team_size: number;
  focus_areas: string[];
}

export interface VerificationProof {
  verification_type: 'GitHub' | 'Domain' | 'Twitter' | 'Discord' | 'Telegram';
  proof_url: string;
  verified_at: bigint;
  verification_method: 'Automated' | 'CommunityVote' | 'ProofVisible';
  challenge_data?: string;
  status: 'Active' | 'Removed' | 'Disputed';
}

export interface Web3Identity {
  github_org?: string;
  twitter_handle?: string;
  discord_server?: string;
  telegram_channel?: string;
  domain_verified: boolean;
  social_verification_status: VerificationStatus;
  verification_proofs: VerificationProof[];
}

export interface CrossChainPresence {
  ethereum_contracts: string[];
  bitcoin_addresses: string[];
  icp_canisters: string[];
  polygon_contracts: string[];
  solana_addresses: string[];
  sui_addresses: string[];
  ton_addresses: string[];
  treasury_wallets: WalletInfo[];
  token_contracts: TokenInfo[];
}

export interface WalletInfo {
  chain: string;
  address: string;
  wallet_type: string;
  verified: boolean;
}

export interface TokenInfo {
  chain: string;
  contract_address: string;
  symbol: string;
  name: string;
  verified: boolean;
}

export interface TeamMember {
  name: string;
  role: string;
  github_profile?: string;
  linkedin_profile?: string;
  verified: boolean;
}

export interface CommunityValidation {
  peer_endorsements: Endorsement[];
  employee_testimonials: Testimonial[];
  community_vouches: Vouch[];
  reputation_score: number;
  reputation_staked: bigint;
}

export interface Endorsement {
  endorser_company_id: string;
  message: string;
  timestamp: bigint;
  endorser_principal: Principal;
}

export interface Testimonial {
  author_name: string;
  role: string;
  message: string;
  timestamp: bigint;
  verified: boolean;
}

export interface Vouch {
  voucher_principal: Principal;
  message: string;
  timestamp: bigint;
  weight: number;
}

export enum VerificationStatus {
  Pending = 'Pending',
  Verified = 'Verified',
  Failed = 'Failed',
  Expired = 'Expired'
}

export enum CompanyStatus {
  Pending = 'Pending',
  Verified = 'Verified', 
  Trusted = 'Trusted',
  Flagged = 'Flagged'
}

export interface Company {
  id: string;
  basic_info: CompanyBasicInfo;
  web3_identity: Web3Identity;
  cross_chain_presence: CrossChainPresence;
  team_members: TeamMember[];
  community_validation: CommunityValidation;
  verification_score: number;
  status: CompanyStatus;
  created_at: bigint;
  updated_at: bigint;
  created_by: Principal;
}

// API Request/Response Types

export interface CreateCompanyRequest {
  basic_info: CompanyBasicInfo;
  web3_identity: Web3Identity;
  cross_chain_presence: CrossChainPresence;
  team_members: TeamMember[];
}

export interface UpdateCompanyRequest {
  company_id: string;
  basic_info?: CompanyBasicInfo;
  web3_identity?: Web3Identity;
  cross_chain_presence?: CrossChainPresence;
  team_members?: TeamMember[];
}

export interface SearchCompaniesRequest {
  query?: string;
  focus_areas?: string[];
  status_filter?: CompanyStatus;
  min_verification_score?: number;
  has_github?: boolean;
  has_contracts?: boolean;
  domain_verified?: boolean;
  page?: number;
  limit?: number;
}

export interface DomainVerificationChallenge {
  company_id: string;
  domain: string;
  challenge_token: string;
  created_at: bigint;
  expires_at: bigint;
}

export interface CrossChainChallenge {
  company_id: string;
  chain_type: ChainType;
  address_or_contract: string;
  challenge_message: string;
  verification_method: CrossChainVerificationMethod;
  created_at: bigint;
  expires_at: bigint;
}

export enum ChainType {
  Ethereum = 'Ethereum',
  Bitcoin = 'Bitcoin',
  ICP = 'ICP',
  Polygon = 'Polygon',
  Solana = 'Solana',
  Sui = 'Sui',
  TON = 'TON'
}

export interface CrossChainVerificationMethod {
  SignMessage?: { message: string };
  DeploySpecialContract?: { verification_code: string };
  SetPublicVariable?: { variable_name: string; value: string };
  SpecialTransaction?: { transaction_data: string };
}

export interface VerificationResult {
  success: boolean;
  message: string;
  verified_at?: bigint;
}

export interface RegistryStatistics {
  total_companies: number;
  verified_companies: number;
  trusted_companies: number;
  pending_companies: number;
  companies_with_github: number;
  companies_with_contracts: number;
  companies_with_domain_verification: number;
  total_endorsements: number;
  total_testimonials: number;
  total_vouches: number;
  average_reputation_score: number;
}

// Form Types for UI

export interface CompanyFormData extends CreateCompanyRequest {}

export interface EndorsementFormData {
  company_id: string;
  message: string;
}

export interface TestimonialFormData {
  company_id: string;
  author_name: string;
  role: string;
  message: string;
}

export interface VouchFormData {
  company_id: string;
  message: string;
}

// API Response wrapper
export type RegistryResult<T> = { Ok: T } | { Err: string };