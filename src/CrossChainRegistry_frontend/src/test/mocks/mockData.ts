import { Company, CompanyStatus, VerificationStatus, CompanyBasicInfo, Web3Identity, CrossChainPresence, TeamMember, CommunityValidation, RegistryStatistics } from '../../types';

export const mockCompanyBasicInfo: CompanyBasicInfo = {
  name: 'Test Web3 Company',
  description: 'A revolutionary DeFi protocol building the future of decentralized finance.',
  website: 'https://testcompany.com',
  founding_date: '2023-01-15',
  team_size: 25,
  focus_areas: ['DeFi', 'Infrastructure'],
};

export const mockWeb3Identity: Web3Identity = {
  github_org: 'test-company-org',
  twitter_handle: '@testcompany',
  discord_server: 'discord.gg/testcompany',
  telegram_channel: 't.me/testcompany',
  domain_verified: true,
  social_verification_status: VerificationStatus.Verified,
};

export const mockCrossChainPresence: CrossChainPresence = {
  ethereum_contracts: ['0x1234567890123456789012345678901234567890'],
  bitcoin_addresses: ['1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa'],
  icp_canisters: ['rdmx6-jaaaa-aaaah-qcaiq-cai'],
  polygon_contracts: ['0xabcdefabcdefabcdefabcdefabcdefabcdefabcd'],
  treasury_wallets: [
    {
      chain: 'ethereum',
      address: '0x1234567890123456789012345678901234567890',
      wallet_type: 'treasury',
      verified: true,
    },
  ],
  token_contracts: [
    {
      chain: 'ethereum',
      contract_address: '0xTokenAddress123456789012345678901234567890',
      symbol: 'TEST',
      name: 'Test Token',
      verified: true,
    },
  ],
};

export const mockTeamMembers: TeamMember[] = [
  {
    name: 'John Doe',
    role: 'CEO & Founder',
    github_profile: 'https://github.com/johndoe',
    linkedin_profile: 'https://linkedin.com/in/johndoe',
    verified: true,
  },
  {
    name: 'Jane Smith',
    role: 'CTO',
    github_profile: 'https://github.com/janesmith',
    linkedin_profile: 'https://linkedin.com/in/janesmith',
    verified: false,
  },
];

export const mockCommunityValidation: CommunityValidation = {
  peer_endorsements: [
    {
      endorser_company_id: 'endorser-company-1',
      message: 'Great team with solid technical expertise.',
      timestamp: BigInt(Date.now() * 1_000_000),
      endorser_principal: {} as any,
    },
  ],
  employee_testimonials: [
    {
      author_name: 'Alice Johnson',
      role: 'Former Developer',
      message: 'Amazing company culture and innovative projects.',
      timestamp: BigInt(Date.now() * 1_000_000),
      verified: true,
    },
  ],
  community_vouches: [
    {
      voucher_principal: {} as any,
      message: 'Trustworthy project with transparent operations.',
      timestamp: BigInt(Date.now() * 1_000_000),
      weight: 5,
    },
  ],
  reputation_score: 85,
  reputation_staked: BigInt(1000),
};

export const mockCompany: Company = {
  id: 'test-company-123',
  basic_info: mockCompanyBasicInfo,
  web3_identity: mockWeb3Identity,
  cross_chain_presence: mockCrossChainPresence,
  team_members: mockTeamMembers,
  community_validation: mockCommunityValidation,
  verification_score: 92,
  status: CompanyStatus.Verified,
  created_at: BigInt(Date.now() * 1_000_000),
  updated_at: BigInt(Date.now() * 1_000_000),
  created_by: {} as any,
};

export const mockCompanies: Company[] = [
  mockCompany,
  {
    ...mockCompany,
    id: 'test-company-456',
    basic_info: {
      ...mockCompanyBasicInfo,
      name: 'Another Web3 Company',
      description: 'Building the next generation of NFT marketplaces.',
      focus_areas: ['NFTs', 'Metaverse'],
    },
    status: CompanyStatus.Trusted,
    verification_score: 78,
  },
  {
    ...mockCompany,
    id: 'test-company-789',
    basic_info: {
      ...mockCompanyBasicInfo,
      name: 'Gaming DAO',
      description: 'Decentralized gaming platform with play-to-earn mechanics.',
      focus_areas: ['Gaming', 'DAOs'],
    },
    status: CompanyStatus.Pending,
    verification_score: 45,
  },
];

export const mockRegistryStatistics: RegistryStatistics = {
  total_companies: 150,
  verified_companies: 89,
  trusted_companies: 34,
  pending_companies: 27,
  companies_with_github: 134,
  companies_with_contracts: 98,
  companies_with_domain_verification: 76,
  total_endorsements: 245,
  total_testimonials: 156,
  total_vouches: 89,
  average_reputation_score: 67.5,
};