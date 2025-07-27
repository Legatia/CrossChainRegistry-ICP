import { describe, it, expect, vi, beforeEach } from 'vitest';
import { screen, waitFor } from '../../test/utils/testUtils';
import { renderWithRouter } from '../../test/utils/routerTestUtils';
import HomePage from '../HomePage';
import { registryApi } from '../../services/api';
import { mockRegistryStatistics } from '../../test/mocks/mockData';

// Mock the API service
vi.mock('../../services/api', () => ({
  registryApi: {
    getRegistryStatistics: vi.fn(),
  },
}));

describe('HomePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Static Content Rendering', () => {
    it('should render hero section with main title and subtitle', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText('CrossChain Registry')).toBeInTheDocument();
      expect(screen.getByText('The definitive registry for verified Web3 companies and cross-chain presence')).toBeInTheDocument();
    });

    it('should render hero description', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText(/discover legitimate web3 companies/i)).toBeInTheDocument();
      expect(screen.getByText(/protect yourself from scams/i)).toBeInTheDocument();
    });

    it('should render call-to-action buttons in hero', () => {
      renderWithRouter(<HomePage />);

      const browseButton = screen.getByRole('link', { name: 'Browse Companies' });
      const registerButtons = screen.getAllByRole('link', { name: 'Register Your Company' });
      const heroRegisterButton = registerButtons.find(btn => btn.closest('.hero'));

      expect(browseButton).toBeInTheDocument();
      expect(browseButton).toHaveAttribute('href', '/companies');

      expect(heroRegisterButton).toBeInTheDocument();
      expect(heroRegisterButton).toHaveAttribute('href', '/register');
    });

    it('should render features section', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText('Why Use CrossChain Registry?')).toBeInTheDocument();

      const features = [
        'Scam Protection',
        'Cross-Chain Verification',
        'Community Validation',
        'Web3 Identity',
        'Transparency Metrics',
        'Reputation System',
      ];

      features.forEach(feature => {
        expect(screen.getByText(feature)).toBeInTheDocument();
      });
    });

    it('should render how it works section', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText('How It Works')).toBeInTheDocument();

      const steps = [
        'Complete Verification',
        'Build Community Trust',
        'Gain Trusted Status',
      ];

      // Check for the "Register Your Company" step specifically in the how-it-works section
      const howItWorksSection = screen.getByText('How It Works').closest('.how-it-works');
      expect(howItWorksSection).toBeInTheDocument();
      if (howItWorksSection) {
        expect(howItWorksSection).toHaveTextContent('Register Your Company');
      }

      steps.forEach(step => {
        expect(screen.getByText(step)).toBeInTheDocument();
      });
    });

    it('should render final CTA section', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText('Ready to Get Started?')).toBeInTheDocument();
      
      const ctaButtons = screen.getAllByRole('link');
      const registerCTA = ctaButtons.find(btn => btn.textContent === 'Register Your Company');
      const exploreCTA = ctaButtons.find(btn => btn.textContent === 'Explore Companies');

      expect(registerCTA).toHaveAttribute('href', '/register');
      expect(exploreCTA).toHaveAttribute('href', '/companies');
    });
  });

  describe('Statistics Section', () => {
    it('should show loading state initially', () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockImplementation(() => new Promise(() => {})); // Never resolves

      renderWithRouter(<HomePage />);

      expect(screen.getByText('Loading statistics...')).toBeInTheDocument();
    });

    it('should render statistics when loaded successfully', async () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockResolvedValue(mockRegistryStatistics);

      renderWithRouter(<HomePage />);

      await waitFor(() => {
        expect(screen.getByText(mockRegistryStatistics.total_companies.toString())).toBeInTheDocument();
        expect(screen.getByText(mockRegistryStatistics.verified_companies.toString())).toBeInTheDocument();
        expect(screen.getByText(mockRegistryStatistics.trusted_companies.toString())).toBeInTheDocument();
        expect(screen.getByText(mockRegistryStatistics.companies_with_github.toString())).toBeInTheDocument();
      });
    });

    it('should render all statistics labels', async () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockResolvedValue(mockRegistryStatistics);

      renderWithRouter(<HomePage />);

      await waitFor(() => {
        expect(screen.getByText('Total Companies')).toBeInTheDocument();
        expect(screen.getByText('Verified Companies')).toBeInTheDocument();
        expect(screen.getByText('Trusted Companies')).toBeInTheDocument();
        expect(screen.getByText('With GitHub')).toBeInTheDocument();
        expect(screen.getByText('With Contracts')).toBeInTheDocument();
        expect(screen.getByText('Domain Verified')).toBeInTheDocument();
        expect(screen.getByText('Peer Endorsements')).toBeInTheDocument();
        expect(screen.getByText('Avg. Reputation')).toBeInTheDocument();
      });
    });

    it('should round average reputation score', async () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockResolvedValue({
        ...mockRegistryStatistics,
        average_reputation_score: 67.8,
      });

      renderWithRouter(<HomePage />);

      await waitFor(() => {
        expect(screen.getByText('68')).toBeInTheDocument();
      });
    });

    it('should show error state when statistics fail to load', async () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockRejectedValue(new Error('Network error'));

      renderWithRouter(<HomePage />);

      await waitFor(() => {
        expect(screen.getByText('Failed to load statistics')).toBeInTheDocument();
      });
    });

    it('should call API to load statistics on mount', () => {
      const mockGetStats = vi.mocked(registryApi.getRegistryStatistics);
      mockGetStats.mockResolvedValue(mockRegistryStatistics);

      renderWithRouter(<HomePage />);

      expect(mockGetStats).toHaveBeenCalledTimes(1);
    });
  });

  describe('Feature Cards', () => {
    it('should render feature cards with emojis', () => {
      renderWithRouter(<HomePage />);

      const emojiElements = [
        '🔍', // Scam Protection
        '🌐', // Cross-Chain Verification
        '👥', // Community Validation
        '🔗', // Web3 Identity
        '📊', // Transparency Metrics
        '🏆', // Reputation System
      ];

      emojiElements.forEach(emoji => {
        expect(screen.getByText(emoji)).toBeInTheDocument();
      });
    });

    it('should render feature descriptions', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText(/advanced verification systems help identify/i)).toBeInTheDocument();
      expect(screen.getByText(/verify company presence across ethereum/i)).toBeInTheDocument();
      expect(screen.getByText(/peer endorsements, employee testimonials/i)).toBeInTheDocument();
    });
  });

  describe('How It Works Steps', () => {
    it('should render numbered steps', () => {
      renderWithRouter(<HomePage />);

      // Check for step numbers
      expect(screen.getByText('1')).toBeInTheDocument();
      expect(screen.getByText('2')).toBeInTheDocument();
      expect(screen.getByText('3')).toBeInTheDocument();
      expect(screen.getByText('4')).toBeInTheDocument();
    });

    it('should render step descriptions', () => {
      renderWithRouter(<HomePage />);

      expect(screen.getByText(/submit basic information, web3 identity/i)).toBeInTheDocument();
      expect(screen.getByText(/verify github organization, domain ownership/i)).toBeInTheDocument();
      expect(screen.getByText(/receive endorsements, testimonials/i)).toBeInTheDocument();
      expect(screen.getByText(/achieve verified or trusted status/i)).toBeInTheDocument();
    });
  });

  describe('Links and Navigation', () => {
    it('should have correct navigation links', () => {
      renderWithRouter(<HomePage />);

      const browseLinks = screen.getAllByRole('link', { name: /browse companies|explore companies/i });
      const registerLinks = screen.getAllByRole('link', { name: /register your company/i });

      browseLinks.forEach(link => {
        expect(link).toHaveAttribute('href', '/companies');
      });

      registerLinks.forEach(link => {
        expect(link).toHaveAttribute('href', '/register');
      });
    });

    it('should have proper button styling classes', () => {
      renderWithRouter(<HomePage />);

      const registerButtons = screen.getAllByRole('link', { name: /register your company/i });
      const browseButtons = screen.getAllByRole('link', { name: /browse companies/i });
      const exploreButtons = screen.getAllByRole('link', { name: /explore companies/i });

      // Hero register button should be secondary, CTA register button should be primary
      const heroRegisterButton = registerButtons.find(btn => btn.closest('.hero'));
      const ctaRegisterButton = registerButtons.find(btn => btn.closest('.cta-section'));

      if (heroRegisterButton) {
        expect(heroRegisterButton).toHaveClass('button', 'button--secondary');
      }
      if (ctaRegisterButton) {
        expect(ctaRegisterButton).toHaveClass('button', 'button--primary');
      }

      // Browse/explore buttons styling
      browseButtons.forEach(button => {
        expect(button).toHaveClass('button', 'button--primary');
      });

      exploreButtons.forEach(button => {
        expect(button).toHaveClass('button', 'button--secondary');
      });
    });
  });

  describe('Component Structure', () => {
    it('should have proper semantic HTML structure', () => {
      renderWithRouter(<HomePage />);

      // Check for main sections
      const heroSection = screen.getByText('CrossChain Registry').closest('section');
      const statsSection = screen.getByText('Registry Statistics').closest('section');
      const featuresSection = screen.getByText('Why Use CrossChain Registry?').closest('section');
      const howItWorksSection = screen.getByText('How It Works').closest('section');
      const ctaSection = screen.getByText('Ready to Get Started?').closest('section');

      expect(heroSection).toHaveClass('hero');
      expect(statsSection).toHaveClass('stats-section');
      expect(featuresSection).toHaveClass('features-section');
      expect(howItWorksSection).toHaveClass('how-it-works');
      expect(ctaSection).toHaveClass('cta-section');
    });

    it('should render in the correct order', () => {
      renderWithRouter(<HomePage />);

      const sections = document.querySelectorAll('section');
      
      expect(sections[0]).toHaveClass('hero');
      expect(sections[1]).toHaveClass('stats-section');
      expect(sections[2]).toHaveClass('features-section');
      expect(sections[3]).toHaveClass('how-it-works');
      expect(sections[4]).toHaveClass('cta-section');
    });
  });
});