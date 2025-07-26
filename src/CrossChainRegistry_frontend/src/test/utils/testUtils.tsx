import React, { ReactElement } from 'react';
import { render, RenderOptions } from '@testing-library/react';

// Custom render function - let individual tests handle routing
const customRender = (
  ui: ReactElement,
  options?: RenderOptions
) => render(ui, options);

export * from '@testing-library/react';
export { customRender as render };

// Helper function to create mock functions
export const createMockFunction = <T extends (...args: any[]) => any>(
  implementation?: T
) => {
  const mockFn = vi.fn();
  if (implementation) {
    mockFn.mockImplementation(implementation);
  }
  return mockFn as unknown as T;
};

// Helper to wait for async operations
export const waitForAsync = () => new Promise(resolve => setTimeout(resolve, 0));

// Helper to mock window.confirm
export const mockConfirm = (returnValue: boolean = true) => {
  const originalConfirm = window.confirm;
  window.confirm = vi.fn(() => returnValue);
  return () => {
    window.confirm = originalConfirm;
  };
};

// Helper to mock window.prompt
export const mockPrompt = (returnValue: string | null = 'test-value') => {
  const originalPrompt = window.prompt;
  window.prompt = vi.fn(() => returnValue);
  return () => {
    window.prompt = originalPrompt;
  };
};