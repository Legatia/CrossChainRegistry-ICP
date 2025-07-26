import '@testing-library/jest-dom';
import { vi } from 'vitest';

// Mock environment variables
Object.defineProperty(window, 'process', {
  value: {
    env: {
      DFX_NETWORK: 'local',
      NODE_ENV: 'test',
    },
  },
});

// Mock console methods to reduce noise in tests
global.console = {
  ...console,
  // log: vi.fn(),
  debug: vi.fn(),
  info: vi.fn(),
  warn: vi.fn(),
  error: vi.fn(),
};