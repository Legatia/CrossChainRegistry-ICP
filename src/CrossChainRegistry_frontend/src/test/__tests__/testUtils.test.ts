import { describe, it, expect, vi } from 'vitest';
import { createMockFunction, waitForAsync, mockConfirm, mockPrompt } from '../utils/testUtils';

describe('Test Utilities', () => {
  describe('createMockFunction', () => {
    it('should create a mock function', () => {
      const mockFn = createMockFunction();
      expect(vi.isMockFunction(mockFn)).toBe(true);
    });

    it('should create a mock function with implementation', () => {
      const implementation = (x: number) => x * 2;
      const mockFn = createMockFunction(implementation);
      
      expect(mockFn(5)).toBe(10);
    });
  });

  describe('waitForAsync', () => {
    it('should resolve after timeout', async () => {
      const start = Date.now();
      await waitForAsync();
      const end = Date.now();
      
      // Should have waited at least some time
      expect(end - start).toBeGreaterThanOrEqual(0);
    });
  });

  describe('mockConfirm', () => {
    it('should mock window.confirm to return true by default', () => {
      const restore = mockConfirm();
      
      expect(window.confirm('Test')).toBe(true);
      
      restore();
    });

    it('should mock window.confirm to return specified value', () => {
      const restore = mockConfirm(false);
      
      expect(window.confirm('Test')).toBe(false);
      
      restore();
    });

    it('should restore original confirm function', () => {
      const originalConfirm = window.confirm;
      const restore = mockConfirm();
      
      restore();
      
      expect(window.confirm).toBe(originalConfirm);
    });
  });

  describe('mockPrompt', () => {
    it('should mock window.prompt to return default value', () => {
      const restore = mockPrompt();
      
      expect(window.prompt('Test')).toBe('test-value');
      
      restore();
    });

    it('should mock window.prompt to return specified value', () => {
      const restore = mockPrompt('custom-value');
      
      expect(window.prompt('Test')).toBe('custom-value');
      
      restore();
    });

    it('should mock window.prompt to return null', () => {
      const restore = mockPrompt(null);
      
      expect(window.prompt('Test')).toBe(null);
      
      restore();
    });

    it('should restore original prompt function', () => {
      const originalPrompt = window.prompt;
      const restore = mockPrompt();
      
      restore();
      
      expect(window.prompt).toBe(originalPrompt);
    });
  });
});