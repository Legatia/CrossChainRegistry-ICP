import { describe, it, expect } from 'vitest';
import { screen } from '../../../test/utils/testUtils';
import { renderWithRouter } from '../../../test/utils/routerTestUtils';
import CompanyCard from '../CompanyCard';
import { mockCompany, mockCompanies } from '../../../test/mocks/mockData';
import { CompanyStatus } from '../../../types';

describe('CompanyCard', () => {
  it('should render company basic information', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    expect(screen.getByText(mockCompany.basic_info.name)).toBeInTheDocument();
    expect(screen.getByText(mockCompany.basic_info.description)).toBeInTheDocument();
    expect(screen.getByText(mockCompany.basic_info.website)).toBeInTheDocument();
    expect(screen.getByText(mockCompany.basic_info.team_size.toString())).toBeInTheDocument();
  });

  it('should render company status with correct styling', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const statusElement = screen.getByText(mockCompany.status);
    expect(statusElement).toBeInTheDocument();
    expect(statusElement).toHaveClass('company-card__status--green');
  });

  it('should render different status colors correctly', () => {
    const statuses = [
      { status: CompanyStatus.Verified, expectedClass: 'company-card__status--green' },
      { status: CompanyStatus.Trusted, expectedClass: 'company-card__status--blue' },
      { status: CompanyStatus.Pending, expectedClass: 'company-card__status--orange' },
      { status: CompanyStatus.Flagged, expectedClass: 'company-card__status--red' },
    ];

    statuses.forEach(({ status, expectedClass }) => {
      const { unmount } = renderWithRouter(
        <CompanyCard company={{ ...mockCompany, status }} />
      );

      const statusElement = screen.getByText(status);
      expect(statusElement).toHaveClass(expectedClass);

      unmount();
    });
  });

  it('should render verification score', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const scoreText = screen.getByText(`${mockCompany.verification_score}/100`);
    expect(scoreText).toBeInTheDocument();
    expect(scoreText).toHaveClass('company-card__score');
  });

  it('should render focus areas', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    mockCompany.basic_info.focus_areas.forEach(area => {
      expect(screen.getByText(area)).toBeInTheDocument();
    });
  });

  it('should limit focus areas display to 3 and show "more" indicator', () => {
    const companyWithManyAreas = {
      ...mockCompany,
      basic_info: {
        ...mockCompany.basic_info,
        focus_areas: ['DeFi', 'NFTs', 'Gaming', 'Infrastructure', 'DAOs'],
      },
    };

    renderWithRouter(<CompanyCard company={companyWithManyAreas} />);

    // Should show first 3 areas
    expect(screen.getByText('DeFi')).toBeInTheDocument();
    expect(screen.getByText('NFTs')).toBeInTheDocument();
    expect(screen.getByText('Gaming')).toBeInTheDocument();

    // Should show "+2 more" indicator
    expect(screen.getByText('+2 more')).toBeInTheDocument();

    // Should not show the 4th and 5th areas directly
    expect(screen.queryByText('Infrastructure')).not.toBeInTheDocument();
    expect(screen.queryByText('DAOs')).not.toBeInTheDocument();
  });

  it('should render verification badges for different features', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    // Check for GitHub badge
    if (mockCompany.web3_identity.github_org) {
      expect(screen.getByText('GitHub')).toBeInTheDocument();
    }

    // Check for domain verification badge
    if (mockCompany.web3_identity.domain_verified) {
      expect(screen.getByText('Domain ✓')).toBeInTheDocument();
    }

    // Check for blockchain badges
    if (mockCompany.cross_chain_presence.ethereum_contracts.length > 0) {
      expect(screen.getByText('Ethereum')).toBeInTheDocument();
    }

    if (mockCompany.cross_chain_presence.bitcoin_addresses.length > 0) {
      expect(screen.getByText('Bitcoin')).toBeInTheDocument();
    }

    if (mockCompany.cross_chain_presence.icp_canisters.length > 0) {
      expect(screen.getByText('ICP')).toBeInTheDocument();
    }
  });

  it('should render community validation statistics', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const endorsementsCount = mockCompany.community_validation.peer_endorsements.length;
    const testimonialsCount = mockCompany.community_validation.employee_testimonials.length;
    const vouchesCount = mockCompany.community_validation.community_vouches.length;

    // Use more specific selectors to avoid conflicts with multiple "1" values
    expect(screen.getByText('Endorsements')).toBeInTheDocument();
    expect(screen.getByText('Testimonials')).toBeInTheDocument();
    expect(screen.getByText('Vouches')).toBeInTheDocument();
    
    // Check that the statistics section contains the correct numbers
    const statisticsSection = screen.getByText('Endorsements').closest('.community-stats') || 
                             screen.getByText('Endorsements').parentElement;
    if (statisticsSection) {
      expect(statisticsSection).toHaveTextContent(endorsementsCount.toString());
      expect(statisticsSection).toHaveTextContent(testimonialsCount.toString());
      expect(statisticsSection).toHaveTextContent(vouchesCount.toString());
    }
  });

  it('should render founding date when available', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    if (mockCompany.basic_info.founding_date) {
      expect(screen.getByText(mockCompany.basic_info.founding_date)).toBeInTheDocument();
    }
  });

  it('should show "Not specified" when founding date is missing', () => {
    const companyWithoutDate = {
      ...mockCompany,
      basic_info: {
        ...mockCompany.basic_info,
        founding_date: '',
      },
    };

    renderWithRouter(<CompanyCard company={companyWithoutDate} />);

    expect(screen.getByText('Not specified')).toBeInTheDocument();
  });

  it('should truncate long descriptions', () => {
    const longDescription = 'A'.repeat(200);
    const companyWithLongDesc = {
      ...mockCompany,
      basic_info: {
        ...mockCompany.basic_info,
        description: longDescription,
      },
    };

    renderWithRouter(<CompanyCard company={companyWithLongDesc} />);

    const truncatedText = screen.getByText(/A{150}\.\.\.$/);
    expect(truncatedText).toBeInTheDocument();
  });

  it('should render website as clickable link', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const websiteLink = screen.getByRole('link', { name: mockCompany.basic_info.website });
    expect(websiteLink).toHaveAttribute('href', mockCompany.basic_info.website);
    expect(websiteLink).toHaveAttribute('target', '_blank');
    expect(websiteLink).toHaveAttribute('rel', 'noopener noreferrer');
  });

  it('should have clickable company name that links to profile', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const nameLink = screen.getByRole('link', { name: mockCompany.basic_info.name });
    expect(nameLink).toHaveAttribute('href', `/company/${mockCompany.id}`);
  });

  it('should format creation date correctly', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const registeredText = screen.getByText(/Registered:/);
    expect(registeredText).toBeInTheDocument();
  });

  it('should have proper CSS classes for styling', () => {
    renderWithRouter(<CompanyCard company={mockCompany} />);

    const cardElement = screen.getByText(mockCompany.basic_info.name).closest('.company-card');
    expect(cardElement).toHaveClass('company-card');
  });
});