import { describe, it, expect } from 'vitest';
import {
  sanitizeInput,
  validateInput,
  validateForm,
  INPUT_LIMITS,
  RateLimiter,
} from '../validation';

describe('Security Validation Utilities', () => {
  describe('sanitizeInput', () => {
    it('should sanitize malicious text input', () => {
      const maliciousInput = '<script>alert("xss")</script>Hello & "World"';
      const sanitized = sanitizeInput.text(maliciousInput);
      
      expect(sanitized).toBe('scriptalert(xss)/scriptHello World');
      expect(sanitized).not.toContain('<');
      expect(sanitized).not.toContain('>');
      expect(sanitized).not.toContain('"');
      expect(sanitized).not.toContain('&');
    });

    it('should sanitize blockchain addresses', () => {
      const address = '  0x1234567890123456789012345678901234567890  ';
      const sanitized = sanitizeInput.address(address);
      
      expect(sanitized).toBe('0x1234567890123456789012345678901234567890');
      expect(sanitized).not.toContain(' ');
    });

    it('should sanitize URLs', () => {
      const url = 'example.com';
      const sanitized = sanitizeInput.url(url);
      
      expect(sanitized).toBe('https://example.com');
    });

    it('should handle Twitter handles', () => {
      const handle = 'testuser';
      const sanitized = sanitizeInput.handle(handle, 'twitter');
      
      expect(sanitized).toBe('@testuser');
    });
  });

  describe('validateInput', () => {
    it('should validate string lengths', () => {
      // Valid length
      const validResult = validateInput.length('Valid Company Name', 'COMPANY_NAME');
      expect(validResult.valid).toBe(true);

      // Too short
      const shortResult = validateInput.length('A', 'COMPANY_NAME');
      expect(shortResult.valid).toBe(false);
      expect(shortResult.error).toContain('Minimum');

      // Too long
      const longResult = validateInput.length('A'.repeat(200), 'COMPANY_NAME');
      expect(longResult.valid).toBe(false);
      expect(longResult.error).toContain('Maximum');
    });

    it('should validate Ethereum addresses', () => {
      // Valid Ethereum address
      const validResult = validateInput.blockchainAddress(
        '0x1234567890123456789012345678901234567890',
        'ETHEREUM'
      );
      expect(validResult.valid).toBe(true);

      // Invalid Ethereum address
      const invalidResult = validateInput.blockchainAddress('invalid', 'ETHEREUM');
      expect(invalidResult.valid).toBe(false);
      expect(invalidResult.error).toContain('Invalid ethereum address');
    });

    it('should validate Bitcoin addresses', () => {
      // Valid Bitcoin address (legacy format)
      const validResult = validateInput.blockchainAddress(
        '1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa',
        'BITCOIN'
      );
      expect(validResult.valid).toBe(true);

      // Invalid Bitcoin address
      const invalidResult = validateInput.blockchainAddress('invalid', 'BITCOIN');
      expect(invalidResult.valid).toBe(false);
    });

    it('should validate ICP canister IDs', () => {
      // Valid ICP canister ID
      const validResult = validateInput.blockchainAddress(
        'rdmx6-jaaaa-aaaah-qcaiq-cai',
        'ICP_CANISTER'
      );
      expect(validResult.valid).toBe(true);

      // Invalid ICP canister ID
      const invalidResult = validateInput.blockchainAddress('invalid', 'ICP_CANISTER');
      expect(invalidResult.valid).toBe(false);
    });

    it('should validate team size', () => {
      // Valid team size
      const validResult = validateInput.teamSize(5);
      expect(validResult.valid).toBe(true);

      // Invalid team size (too small)
      const tooSmallResult = validateInput.teamSize(0);
      expect(tooSmallResult.valid).toBe(false);

      // Invalid team size (too large)
      const tooLargeResult = validateInput.teamSize(20000);
      expect(tooLargeResult.valid).toBe(false);
    });

    it('should validate URLs', () => {
      // Valid website URL
      const validResult = validateInput.url('https://example.com', 'WEBSITE');
      expect(validResult.valid).toBe(true);

      // Invalid URL (empty string should fail)
      const invalidResult = validateInput.url('', 'WEBSITE');
      expect(invalidResult.valid).toBe(false);
    });
  });

  describe('validateForm', () => {
    it('should validate company registration form', () => {
      const validFormData = {
        basic_info: {
          name: 'Test Company',
          description: 'A legitimate test company with proper description length',
          website: 'https://test.com',
          team_size: 5,
        },
        web3_identity: {
          github_org: 'test-org',
        },
        cross_chain_presence: {
          ethereum_contracts: ['0x1234567890123456789012345678901234567890'],
          bitcoin_addresses: ['1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa'],
          icp_canisters: ['rdmx6-jaaaa-aaaah-qcaiq-cai'],
        },
      };

      const result = validateForm.companyRegistration(validFormData);
      expect(result.valid).toBe(true);
      expect(Object.keys(result.errors)).toHaveLength(0);
    });

    it('should reject invalid company registration form', () => {
      const invalidFormData = {
        basic_info: {
          name: '', // Too short
          description: 'Short', // Too short
          website: 'invalid-url',
          team_size: 0, // Too small
        },
        cross_chain_presence: {
          ethereum_contracts: ['invalid-address'],
        },
      };

      const result = validateForm.companyRegistration(invalidFormData);
      expect(result.valid).toBe(false);
      expect(Object.keys(result.errors).length).toBeGreaterThan(0);
    });

    it('should validate endorsement messages', () => {
      const validMessage = 'This is a great company with excellent team and products.';
      const validResult = validateForm.endorsement(validMessage);
      expect(validResult.valid).toBe(true);

      const invalidMessage = 'Short';
      const invalidResult = validateForm.endorsement(invalidMessage);
      expect(invalidResult.valid).toBe(false);
    });
  });

  describe('RateLimiter', () => {
    it('should allow requests within limit', () => {
      const limiter = new RateLimiter(5, 60000); // 5 requests per minute
      
      expect(limiter.canProceed('test-key')).toBe(true);
      expect(limiter.canProceed('test-key')).toBe(true);
      expect(limiter.canProceed('test-key')).toBe(true);
    });

    it('should block requests over limit', () => {
      const limiter = new RateLimiter(2, 60000); // 2 requests per minute
      
      expect(limiter.canProceed('test-key')).toBe(true);
      expect(limiter.canProceed('test-key')).toBe(true);
      expect(limiter.canProceed('test-key')).toBe(false); // Should be blocked
    });

    it('should track remaining attempts', () => {
      const limiter = new RateLimiter(3, 60000);
      
      expect(limiter.getRemainingAttempts('test-key')).toBe(3);
      limiter.canProceed('test-key');
      expect(limiter.getRemainingAttempts('test-key')).toBe(2);
      limiter.canProceed('test-key');
      expect(limiter.getRemainingAttempts('test-key')).toBe(1);
    });
  });
});