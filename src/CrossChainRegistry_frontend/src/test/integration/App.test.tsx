import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import App from '../../App';
import { registryApi } from '../../services/api';

// Mock the API service
vi.mock('../../services/api', () => ({
  registryApi: {
    getRegistryStatistics: vi.fn(),
    listCompanies: vi.fn(),
    getCompanyCount: vi.fn(),
  },
}));

describe('App Integration Tests', () => {
  const renderApp = (initialEntries = ['/']) => {
    return render(
      <MemoryRouter initialEntries={initialEntries}>
        <App />
      </MemoryRouter>
    );
  };

  describe('Routing', () => {
    it('should render HomePage on root route', () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockResolvedValue({
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

      renderApp(['/']);

      expect(screen.getByText('CrossChain Registry')).toBeInTheDocument();
      expect(screen.getByText('Registry Statistics')).toBeInTheDocument();
    });

    it('should render CompanyRegistrationPage on /register route', () => {
      renderApp(['/register']);

      expect(screen.getByText('Register Your Web3 Company')).toBeInTheDocument();
      expect(screen.getByText('Basic Information')).toBeInTheDocument();
    });

    it('should render CompanyListingPage on /companies route', () => {
      const mockListCompanies = vi.mocked(registryApi.listCompanies);
      const mockGetCompanyCount = vi.mocked(registryApi.getCompanyCount);
      
      mockListCompanies.mockResolvedValue([]);
      mockGetCompanyCount.mockResolvedValue(0);

      renderApp(['/companies']);

      expect(screen.getByText('Web3 Company Registry')).toBeInTheDocument();
      expect(screen.getByText('Discover verified Web3 companies')).toBeInTheDocument();
    });

    it('should render VerificationDashboardPage on /dashboard route', () => {
      renderApp(['/dashboard']);

      expect(screen.getByText('Verification Dashboard')).toBeInTheDocument();
      expect(screen.getByText('🚧 Dashboard Coming Soon')).toBeInTheDocument();
    });
  });

  describe('Header Navigation', () => {
    it('should render header on all routes', () => {
      const routes = ['/', '/register', '/companies', '/dashboard'];

      routes.forEach(route => {
        const { unmount } = renderApp([route]);
        
        expect(screen.getByText('CrossChain Registry')).toBeInTheDocument();
        expect(screen.getByRole('navigation')).toBeInTheDocument();
        
        unmount();
      });
    });

    it('should highlight active navigation links correctly', () => {
      const testCases = [
        { route: '/', expectedActive: 'Home' },
        { route: '/companies', expectedActive: 'Browse Companies' },
        { route: '/register', expectedActive: 'Register Company' },
        { route: '/dashboard', expectedActive: 'Dashboard' },
      ];

      testCases.forEach(({ route, expectedActive }) => {
        const { unmount } = renderApp([route]);
        
        const activeLink = screen.getByRole('link', { name: expectedActive });
        expect(activeLink).toHaveClass('active');
        
        unmount();
      });
    });
  });

  describe('App Structure', () => {
    it('should have proper semantic HTML structure', () => {
      renderApp(['/']);

      const app = document.querySelector('.app');
      const header = screen.getByRole('banner');
      const main = document.querySelector('.main-content');

      expect(app).toBeInTheDocument();
      expect(header).toBeInTheDocument();
      expect(main).toBeInTheDocument();
    });

    it('should apply correct CSS classes', () => {
      renderApp(['/']);

      const app = document.querySelector('.app');
      const mainContent = document.querySelector('.main-content');

      expect(app).toHaveClass('app');
      expect(mainContent).toHaveClass('main-content');
    });
  });

  describe('Error Boundaries', () => {
    it('should handle navigation between routes without errors', () => {
      const { rerender } = renderApp(['/']);

      // Navigate to different routes
      rerender(
        <MemoryRouter initialEntries={['/register']}>
          <App />
        </MemoryRouter>
      );

      expect(screen.getByText('Register Your Web3 Company')).toBeInTheDocument();

      rerender(
        <MemoryRouter initialEntries={['/companies']}>
          <App />
        </MemoryRouter>
      );

      expect(screen.getByText('Web3 Company Registry')).toBeInTheDocument();
    });
  });

  describe('API Integration', () => {
    it('should call appropriate APIs on different routes', () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      const mockListCompanies = vi.mocked(registryApi.listCompanies);
      const mockGetCompanyCount = vi.mocked(registryApi.getCompanyCount);

      mockGetStats.mockResolvedValue({
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
      mockListCompanies.mockResolvedValue([]);
      mockGetCompanyCount.mockResolvedValue(0);

      // Test home page API calls
      const { unmount: unmountHome } = renderApp(['/']);
      expect(mockGetStats).toHaveBeenCalled();
      unmountHome();

      // Test companies page API calls
      vi.clearAllMocks();
      const { unmount: unmountCompanies } = renderApp(['/companies']);
      expect(mockListCompanies).toHaveBeenCalled();
      expect(mockGetCompanyCount).toHaveBeenCalled();
      unmountCompanies();
    });
  });
});