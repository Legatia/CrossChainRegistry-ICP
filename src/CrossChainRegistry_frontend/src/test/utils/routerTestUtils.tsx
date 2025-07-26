import React from 'react';
import { MemoryRouter } from 'react-router-dom';
import { render, RenderOptions } from '@testing-library/react';

// Wrapper for components that need router context
export const RouterWrapper: React.FC<{ children: React.ReactNode; initialEntries?: string[] }> = ({ 
  children, 
  initialEntries = ['/'] 
}) => (
  <MemoryRouter initialEntries={initialEntries}>
    {children}
  </MemoryRouter>
);

// Custom render for components that need router
export const renderWithRouter = (
  ui: React.ReactElement,
  options?: {
    initialEntries?: string[];
    renderOptions?: RenderOptions;
  }
) => {
  const { initialEntries = ['/'], renderOptions } = options || {};
  
  return render(ui, {
    wrapper: ({ children }) => (
      <RouterWrapper initialEntries={initialEntries}>
        {children}
      </RouterWrapper>
    ),
    ...renderOptions,
  });
};