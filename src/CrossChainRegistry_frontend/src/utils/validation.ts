// Security validation utilities for CrossChain Registry
// Provides comprehensive input validation and sanitization

// Input length constraints
export const INPUT_LIMITS = {
  COMPANY_NAME: { min: 2, max: 100 },
  DESCRIPTION: { min: 10, max: 2000 },
  WEBSITE_URL: { min: 7, max: 500 }, // Minimum: http://x.co
  GITHUB_ORG: { min: 1, max: 39 }, // GitHub username limit
  TWITTER_HANDLE: { min: 1, max: 15 }, // Twitter handle limit
  DISCORD_SERVER: { min: 7, max: 100 }, // discord.gg/xxxxx
  TELEGRAM_CHANNEL: { min: 5, max: 100 }, // t.me/xxxxx
  TEAM_MEMBER_NAME: { min: 2, max: 50 },
  TEAM_MEMBER_ROLE: { min: 2, max: 50 },
  ENDORSEMENT_MESSAGE: { min: 10, max: 1000 },
  TESTIMONIAL_MESSAGE: { min: 10, max: 1000 },
  VOUCH_MESSAGE: { min: 10, max: 1000 },
  AUTHOR_NAME: { min: 2, max: 50 },
} as const;

// Blockchain address validation patterns
const BLOCKCHAIN_PATTERNS = {
  // Ethereum address: 0x followed by 40 hex characters
  ETHEREUM: /^0x[a-fA-F0-9]{40}$/,
  
  // Bitcoin address patterns (P2PKH, P2SH, Bech32)
  BITCOIN: /^([13][a-km-zA-HJ-NP-Z1-9]{25,34}|bc1[a-z0-9]{39,59})$/,
  
  // ICP Canister ID: 5 groups of alphanumeric characters separated by hyphens
  ICP_CANISTER: /^[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{5}-[a-z0-9]{3}$/,
  
  // Solana address: Base58 string of 32-44 characters
  SOLANA: /^[1-9A-HJ-NP-Za-km-z]{32,44}$/,
  
  // Sui address: 0x followed by 64 hex characters
  SUI: /^0x[a-fA-F0-9]{64}$/,
  
  // TON address: Base64url format
  TON: /^[A-Za-z0-9_-]{48}$/,
} as const;

// URL validation patterns
const URL_PATTERNS = {
  // Website URL: Must start with http:// or https://
  WEBSITE: /^https?:\/\/(?:[-\w.])+(?:\:[0-9]+)?(?:\/(?:[\w\._~!$&'()*+,;=:@]|%[0-9a-fA-F]{2})*)*(?:\?(?:[\w\._~!$&'()*+,;=:@/?]|%[0-9a-fA-F]{2})*)?(?:#(?:[\w\._~!$&'()*+,;=:@/?]|%[0-9a-fA-F]{2})*)?$/,
  
  // GitHub organization: github.com/orgname
  GITHUB: /^https?:\/\/(www\.)?github\.com\/[a-zA-Z0-9]([a-zA-Z0-9\-]){0,38}$/,
  
  // LinkedIn profile: linkedin.com/in/username
  LINKEDIN: /^https?:\/\/(www\.)?linkedin\.com\/in\/[a-zA-Z0-9\-._~:/?#[\]@!$&'()*+,;=%]+$/,
  
  // Twitter handle: Can start with @ or not
  TWITTER: /^@?[a-zA-Z0-9_]{1,15}$/,
  
  // Discord server: discord.gg/invite
  DISCORD: /^(https?:\/\/)?(www\.)?(discord\.gg\/|discordapp\.com\/invite\/)[a-zA-Z0-9\-._~:/?#[\]@!$&'()*+,;=%]+$/,
  
  // Telegram channel: t.me/channel
  TELEGRAM: /^(https?:\/\/)?(www\.)?(t\.me\/|telegram\.me\/)[a-zA-Z0-9_]{5,32}$/,
} as const;

// Sanitization functions
export const sanitizeInput = {
  /**
   * Basic text sanitization - removes potentially dangerous characters
   */
  text: (input: string): string => {
    if (!input) return '';
    return input
      .trim()
      .replace(/[<>\"'&]/g, '') // Remove HTML/script injection chars
      .replace(/\s+/g, ' ') // Normalize whitespace
      .substring(0, 2000); // Hard limit
  },

  /**
   * URL sanitization - ensures only safe URL schemes
   */
  url: (input: string): string => {
    if (!input) return '';
    const trimmed = input.trim();
    
    // Only allow http and https schemes
    if (!trimmed.match(/^https?:\/\//)) {
      return `https://${trimmed}`;
    }
    
    return trimmed;
  },

  /**
   * Blockchain address sanitization
   */
  address: (input: string): string => {
    if (!input) return '';
    return input.trim().replace(/\s/g, ''); // Remove all whitespace
  },

  /**
   * Social handle sanitization
   */
  handle: (input: string, platform: 'twitter' | 'discord' | 'telegram'): string => {
    if (!input) return '';
    const trimmed = input.trim();
    
    switch (platform) {
      case 'twitter':
        return trimmed.startsWith('@') ? trimmed : `@${trimmed}`;
      case 'discord':
        return trimmed.startsWith('discord.gg/') ? `https://${trimmed}` : trimmed;
      case 'telegram':
        return trimmed.startsWith('t.me/') ? `https://${trimmed}` : trimmed;
      default:
        return trimmed;
    }
  },
};

// Validation functions
export const validateInput = {
  /**
   * Validate string length
   */
  length: (input: string, field: keyof typeof INPUT_LIMITS): { valid: boolean; error?: string } => {
    if (!input) {
      return { valid: false, error: 'Field is required' };
    }
    
    const limits = INPUT_LIMITS[field];
    if (input.length < limits.min) {
      return { valid: false, error: `Minimum ${limits.min} characters required` };
    }
    
    if (input.length > limits.max) {
      return { valid: false, error: `Maximum ${limits.max} characters allowed` };
    }
    
    return { valid: true };
  },

  /**
   * Validate blockchain addresses
   */
  blockchainAddress: (address: string, type: keyof typeof BLOCKCHAIN_PATTERNS): { valid: boolean; error?: string } => {
    if (!address) {
      return { valid: false, error: 'Address is required' };
    }
    
    const sanitized = sanitizeInput.address(address);
    const pattern = BLOCKCHAIN_PATTERNS[type];
    
    if (!pattern.test(sanitized)) {
      return { valid: false, error: `Invalid ${type.toLowerCase()} address format` };
    }
    
    return { valid: true };
  },

  /**
   * Validate URLs
   */
  url: (url: string, type: keyof typeof URL_PATTERNS): { valid: boolean; error?: string } => {
    if (!url) {
      return { valid: false, error: 'URL is required' };
    }
    
    const sanitized = sanitizeInput.url(url);
    const pattern = URL_PATTERNS[type];
    
    if (!pattern.test(sanitized)) {
      return { valid: false, error: `Invalid ${type.toLowerCase()} URL format` };
    }
    
    return { valid: true };
  },

  /**
   * Validate team size
   */
  teamSize: (size: number): { valid: boolean; error?: string } => {
    if (!size || size < 1) {
      return { valid: false, error: 'Team size must be at least 1' };
    }
    
    if (size > 10000) {
      return { valid: false, error: 'Team size cannot exceed 10,000' };
    }
    
    return { valid: true };
  },

  /**
   * Validate email format (for future use)
   */
  email: (email: string): { valid: boolean; error?: string } => {
    if (!email) {
      return { valid: false, error: 'Email is required' };
    }
    
    const emailPattern = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
    
    if (!emailPattern.test(email)) {
      return { valid: false, error: 'Invalid email format' };
    }
    
    return { valid: true };
  },
};

// Comprehensive form validation
export const validateForm = {
  /**
   * Validate company registration form
   */
  companyRegistration: (formData: any): { valid: boolean; errors: Record<string, string> } => {
    const errors: Record<string, string> = {};
    
    // Basic info validation
    const nameValidation = validateInput.length(formData.basic_info?.name || '', 'COMPANY_NAME');
    if (!nameValidation.valid) {
      errors['basic_info.name'] = nameValidation.error!;
    }
    
    const descValidation = validateInput.length(formData.basic_info?.description || '', 'DESCRIPTION');
    if (!descValidation.valid) {
      errors['basic_info.description'] = descValidation.error!;
    }
    
    const websiteValidation = validateInput.url(formData.basic_info?.website || '', 'WEBSITE');
    if (!websiteValidation.valid) {
      errors['basic_info.website'] = websiteValidation.error!;
    }
    
    const teamSizeValidation = validateInput.teamSize(formData.basic_info?.team_size || 0);
    if (!teamSizeValidation.valid) {
      errors['basic_info.team_size'] = teamSizeValidation.error!;
    }
    
    // Web3 identity validation (optional fields)
    if (formData.web3_identity?.github_org) {
      const githubValidation = validateInput.length(formData.web3_identity.github_org, 'GITHUB_ORG');
      if (!githubValidation.valid) {
        errors['web3_identity.github_org'] = githubValidation.error!;
      }
    }
    
    // Cross-chain addresses validation
    if (formData.cross_chain_presence?.ethereum_contracts) {
      formData.cross_chain_presence.ethereum_contracts.forEach((addr: string, index: number) => {
        const validation = validateInput.blockchainAddress(addr, 'ETHEREUM');
        if (!validation.valid) {
          errors[`ethereum_contract_${index}`] = validation.error!;
        }
      });
    }
    
    if (formData.cross_chain_presence?.bitcoin_addresses) {
      formData.cross_chain_presence.bitcoin_addresses.forEach((addr: string, index: number) => {
        const validation = validateInput.blockchainAddress(addr, 'BITCOIN');
        if (!validation.valid) {
          errors[`bitcoin_address_${index}`] = validation.error!;
        }
      });
    }
    
    if (formData.cross_chain_presence?.icp_canisters) {
      formData.cross_chain_presence.icp_canisters.forEach((canister: string, index: number) => {
        const validation = validateInput.blockchainAddress(canister, 'ICP_CANISTER');
        if (!validation.valid) {
          errors[`icp_canister_${index}`] = validation.error!;
        }
      });
    }
    
    return {
      valid: Object.keys(errors).length === 0,
      errors,
    };
  },

  /**
   * Validate community validation forms
   */
  endorsement: (message: string): { valid: boolean; error?: string } => {
    return validateInput.length(message, 'ENDORSEMENT_MESSAGE');
  },

  testimonial: (data: { author_name: string; role: string; message: string }): { valid: boolean; errors: Record<string, string> } => {
    const errors: Record<string, string> = {};
    
    const nameValidation = validateInput.length(data.author_name, 'AUTHOR_NAME');
    if (!nameValidation.valid) {
      errors.author_name = nameValidation.error!;
    }
    
    const roleValidation = validateInput.length(data.role, 'TEAM_MEMBER_ROLE');
    if (!roleValidation.valid) {
      errors.role = roleValidation.error!;
    }
    
    const messageValidation = validateInput.length(data.message, 'TESTIMONIAL_MESSAGE');
    if (!messageValidation.valid) {
      errors.message = messageValidation.error!;
    }
    
    return {
      valid: Object.keys(errors).length === 0,
      errors,
    };
  },

  vouch: (message: string): { valid: boolean; error?: string } => {
    return validateInput.length(message, 'VOUCH_MESSAGE');
  },
};

// Rate limiting helper (client-side)
export class RateLimiter {
  private attempts: Map<string, number[]> = new Map();
  
  constructor(
    private maxAttempts: number = 5,
    private windowMs: number = 60000 // 1 minute
  ) {}
  
  canProceed(key: string): boolean {
    const now = Date.now();
    const attempts = this.attempts.get(key) || [];
    
    // Remove old attempts outside the window
    const recentAttempts = attempts.filter(time => now - time < this.windowMs);
    
    if (recentAttempts.length >= this.maxAttempts) {
      return false;
    }
    
    // Add current attempt
    recentAttempts.push(now);
    this.attempts.set(key, recentAttempts);
    
    return true;
  }
  
  getRemainingAttempts(key: string): number {
    const now = Date.now();
    const attempts = this.attempts.get(key) || [];
    const recentAttempts = attempts.filter(time => now - time < this.windowMs);
    
    return Math.max(0, this.maxAttempts - recentAttempts.length);
  }
}

// Export a default rate limiter instance
export const defaultRateLimiter = new RateLimiter();