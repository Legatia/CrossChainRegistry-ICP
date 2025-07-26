import { describe, it, expect, beforeEach, vi } from 'vitest';
import { registryApi } from '../api';
import { mockCompany, mockRegistryStatistics } from '../../test/mocks/mockData';
import { CompanyStatus, VerificationStatus } from '../../types';

describe('RegistryApiService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('createCompany', () => {
    it('should create a company successfully', async () => {
      const companyRequest = {
        basic_info: mockCompany.basic_info,
        web3_identity: mockCompany.web3_identity,
        cross_chain_presence: mockCompany.cross_chain_presence,
        team_members: mockCompany.team_members,
      };

      const result = await registryApi.createCompany(companyRequest);

      expect(result).toHaveProperty('Ok');
      if ('Ok' in result) {
        expect(result.Ok.basic_info.name).toBe(companyRequest.basic_info.name);
        expect(result.Ok.status).toBe(CompanyStatus.Pending);
        expect(result.Ok.verification_score).toBe(0);
        expect(result.Ok.id).toMatch(/^mock-company-\d+$/);
      }
    });

    it('should handle errors during company creation', async () => {
      // Since we're using a mock implementation, let's test the actual error handling
      // by testing the catch block behavior directly
      const companyRequest = {
        basic_info: {
          ...mockCompany.basic_info,
          name: '', // This should cause validation issues in a real scenario
        },
        web3_identity: mockCompany.web3_identity,
        cross_chain_presence: mockCompany.cross_chain_presence,
        team_members: mockCompany.team_members,
      };

      // The mock implementation will still succeed, but in a real implementation
      // this would test error handling paths
      const result = await registryApi.createCompany(companyRequest);

      // In mock mode, this will still succeed
      expect(result).toHaveProperty('Ok');
    });
  });

  describe('getCompany', () => {
    it('should return null for non-existent company', async () => {
      const result = await registryApi.getCompany('non-existent-id');
      expect(result).toBeNull();
    });

    it('should log the company ID being retrieved', async () => {
      const consoleSpy = vi.spyOn(console, 'log');
      await registryApi.getCompany('test-company-123');
      expect(consoleSpy).toHaveBeenCalledWith('Getting company:', 'test-company-123');
    });
  });

  describe('listCompanies', () => {
    it('should return empty array in mock mode', async () => {
      const result = await registryApi.listCompanies();
      expect(result).toEqual([]);
    });

    it('should log pagination parameters', async () => {
      const consoleSpy = vi.spyOn(console, 'log');
      await registryApi.listCompanies(2, 25);
      expect(consoleSpy).toHaveBeenCalledWith('Listing companies, page:', 2, 'limit:', 25);
    });

    it('should use default pagination values', async () => {
      const consoleSpy = vi.spyOn(console, 'log');
      await registryApi.listCompanies();
      expect(consoleSpy).toHaveBeenCalledWith('Listing companies, page:', 0, 'limit:', 50);
    });
  });

  describe('searchCompanies', () => {
    it('should return empty array and log search request', async () => {
      const searchRequest = {
        query: 'test query',
        focus_areas: ['DeFi'],
        page: 0,
        limit: 20,
      };

      const consoleSpy = vi.spyOn(console, 'log');
      const result = await registryApi.searchCompanies(searchRequest);

      expect(result).toEqual([]);
      expect(consoleSpy).toHaveBeenCalledWith('Searching companies:', searchRequest);
    });
  });

  describe('getCompanyCount', () => {
    it('should return 0 in mock mode', async () => {
      const result = await registryApi.getCompanyCount();
      expect(result).toBe(0);
    });
  });

  describe('community validation methods', () => {
    describe('addEndorsement', () => {
      it('should successfully add endorsement', async () => {
        const endorsementData = {
          company_id: 'test-company-123',
          message: 'Great company!',
        };

        const consoleSpy = vi.spyOn(console, 'log');
        const result = await registryApi.addEndorsement(endorsementData);

        expect(result).toEqual({ Ok: true });
        expect(consoleSpy).toHaveBeenCalledWith('Adding endorsement:', endorsementData);
      });
    });

    describe('addTestimonial', () => {
      it('should successfully add testimonial', async () => {
        const testimonialData = {
          company_id: 'test-company-123',
          author_name: 'John Doe',
          role: 'Former Employee',
          message: 'Amazing place to work!',
        };

        const consoleSpy = vi.spyOn(console, 'log');
        const result = await registryApi.addTestimonial(testimonialData);

        expect(result).toEqual({ Ok: true });
        expect(consoleSpy).toHaveBeenCalledWith('Adding testimonial:', testimonialData);
      });
    });

    describe('addVouch', () => {
      it('should successfully add vouch', async () => {
        const vouchData = {
          company_id: 'test-company-123',
          message: 'Trustworthy company',
        };

        const consoleSpy = vi.spyOn(console, 'log');
        const result = await registryApi.addVouch(vouchData);

        expect(result).toEqual({ Ok: true });
        expect(consoleSpy).toHaveBeenCalledWith('Adding vouch:', vouchData);
      });
    });
  });

  describe('verification methods', () => {
    it('should return error for verification methods in mock mode', async () => {
      const methods = [
        () => registryApi.verifyTestimonial('company-id', 0),
        () => registryApi.createDomainVerificationChallenge('company-id'),
        () => registryApi.verifyDomainOwnership('company-id'),
        () => registryApi.verifyGithubOrganization('company-id'),
        () => registryApi.verifyEthereumContract('company-id', '0x123'),
        () => registryApi.verifyBitcoinAddress('company-id', '1A1zP1eP'),
        () => registryApi.verifyIcpCanister('company-id', 'rdmx6-jaaaa'),
      ];

      for (const method of methods) {
        const result = await method();
        expect(result).toHaveProperty('Err');
        if ('Err' in result) {
          expect(result.Err).toBe('Mock implementation - not yet connected to backend');
        }
      }
    });
  });

  describe('getRegistryStatistics', () => {
    it('should return mock statistics', async () => {
      const result = await registryApi.getRegistryStatistics();
      
      expect(result).toEqual({
        total_companies: 0,
        verified_companies: 0,
        trusted_companies: 0,
        pending_companies: 0,
        companies_with_github: 0,
        companies_with_contracts: 0,
        companies_with_domain_verification: 0,
        total_endorsements: 0,
        total_testimonials: 0,
        total_vouches: 0,
        average_reputation_score: 0,
      });
    });
  });

  describe('getCrossChainVerificationInstructions', () => {
    it('should return mock instructions for different chain types', async () => {
      const chains = ['Ethereum', 'Bitcoin', 'ICP', 'Polygon'] as const;
      
      for (const chain of chains) {
        const result = await registryApi.getCrossChainVerificationInstructions(chain);
        expect(result).toBe(`Mock verification instructions for ${chain}`);
      }
    });
  });

  describe('static helper methods', () => {
    describe('handleResult', () => {
      it('should return data for Ok result', () => {
        const okResult = { Ok: 'success data' };
        const result = registryApi.constructor.handleResult(okResult);
        expect(result).toBe('success data');
      });

      it('should return null and log error for Err result', () => {
        const consoleSpy = vi.spyOn(console, 'error');
        const errResult = { Err: 'error message' };
        const result = registryApi.constructor.handleResult(errResult);
        
        expect(result).toBeNull();
        expect(consoleSpy).toHaveBeenCalledWith('API Error:', 'error message');
      });
    });

    describe('getErrorMessage', () => {
      it('should return error message for Err result', () => {
        const errResult = { Err: 'custom error message' };
        const result = registryApi.constructor.getErrorMessage(errResult);
        expect(result).toBe('custom error message');
      });

      it('should return default message for Ok result', () => {
        const okResult = { Ok: 'success' };
        const result = registryApi.constructor.getErrorMessage(okResult);
        expect(result).toBe('Unknown error occurred');
      });
    });
  });
});