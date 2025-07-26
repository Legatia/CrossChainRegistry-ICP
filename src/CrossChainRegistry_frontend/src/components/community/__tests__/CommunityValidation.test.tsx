import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '../../../test/utils/testUtils';
import userEvent from '@testing-library/user-event';
import CommunityValidation from '../CommunityValidation';
import { registryApi } from '../../../services/api';
import { mockCompany } from '../../../test/mocks/mockData';

// Mock the API service
vi.mock('../../../services/api', () => ({
  registryApi: {
    addEndorsement: vi.fn(),
    addTestimonial: vi.fn(),
    addVouch: vi.fn(),
  },
}));

describe('CommunityValidation', () => {
  const user = userEvent.setup();
  const mockOnUpdate = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  const renderComponent = () => {
    return render(
      <CommunityValidation company={mockCompany} onUpdate={mockOnUpdate} />
    );
  };

  describe('Component Rendering', () => {
    it('should render section header with reputation score', () => {
      renderComponent();

      expect(screen.getByText('Community Validation')).toBeInTheDocument();
      expect(screen.getByText('Reputation Score:')).toBeInTheDocument();
      expect(screen.getByText(mockCompany.community_validation.reputation_score.toString())).toBeInTheDocument();
    });

    it('should render validation summary statistics', () => {
      renderComponent();

      const endorsementsCount = mockCompany.community_validation.peer_endorsements.length;
      const testimonialsCount = mockCompany.community_validation.employee_testimonials.length;
      const vouchesCount = mockCompany.community_validation.community_vouches.length;
      const stakedTokens = Number(mockCompany.community_validation.reputation_staked);

      expect(screen.getByText(endorsementsCount.toString())).toBeInTheDocument();
      expect(screen.getByText(testimonialsCount.toString())).toBeInTheDocument();
      expect(screen.getByText(vouchesCount.toString())).toBeInTheDocument();
      expect(screen.getByText(stakedTokens.toString())).toBeInTheDocument();

      expect(screen.getByText('Endorsements')).toBeInTheDocument();
      expect(screen.getByText('Testimonials')).toBeInTheDocument();
      expect(screen.getByText('Vouches')).toBeInTheDocument();
      expect(screen.getByText('Tokens Staked')).toBeInTheDocument();
    });

    it('should render tab navigation', () => {
      renderComponent();

      expect(screen.getByRole('button', { name: 'Endorsements' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Testimonials' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Vouches' })).toBeInTheDocument();
    });

    it('should show endorsements tab as active by default', () => {
      renderComponent();

      const endorsementsTab = screen.getByRole('button', { name: 'Endorsements' });
      expect(endorsementsTab).toHaveClass('active');
    });
  });

  describe('Tab Navigation', () => {
    it('should switch to testimonials tab when clicked', async () => {
      renderComponent();

      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      expect(testimonialsTab).toHaveClass('active');
      expect(screen.getByRole('button', { name: 'Endorsements' })).not.toHaveClass('active');
    });

    it('should switch to vouches tab when clicked', async () => {
      renderComponent();

      const vouchesTab = screen.getByRole('button', { name: 'Vouches' });
      await user.click(vouchesTab);

      expect(vouchesTab).toHaveClass('active');
      expect(screen.getByRole('button', { name: 'Endorsements' })).not.toHaveClass('active');
    });

    it('should hide add form when switching tabs', async () => {
      renderComponent();

      // Open add form in endorsements tab
      const addEndorsementButton = screen.getByText('Add Endorsement');
      await user.click(addEndorsementButton);

      expect(screen.getByText('Endorsement Message')).toBeInTheDocument();

      // Switch to testimonials tab
      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      // Add form should be hidden
      expect(screen.queryByText('Endorsement Message')).not.toBeInTheDocument();
    });
  });

  describe('Endorsements Tab', () => {
    it('should render existing endorsements', () => {
      renderComponent();

      const endorsement = mockCompany.community_validation.peer_endorsements[0];
      expect(screen.getByText(endorsement.message)).toBeInTheDocument();
      expect(screen.getByText(`Company: ${endorsement.endorser_company_id}`)).toBeInTheDocument();
    });

    it('should show add endorsement form when button is clicked', async () => {
      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      expect(screen.getByLabelText('Endorsement Message')).toBeInTheDocument();
      expect(screen.getByText('Cancel')).toBeInTheDocument();
    });

    it('should hide add form when cancel is clicked', async () => {
      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const cancelButton = screen.getByText('Cancel');
      await user.click(cancelButton);

      expect(screen.queryByLabelText('Endorsement Message')).not.toBeInTheDocument();
    });

    it('should submit endorsement form', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockResolvedValue({ Ok: true });

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company with excellent team!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockAddEndorsement).toHaveBeenCalledWith({
          company_id: mockCompany.id,
          message: 'Great company with excellent team!',
        });
      });
    });

    it('should call onUpdate after successful endorsement submission', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockResolvedValue({ Ok: true });

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockOnUpdate).toHaveBeenCalledTimes(1);
      });
    });

    it('should show error message on endorsement submission failure', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockResolvedValue({ Err: 'Network error' });

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText('Network error')).toBeInTheDocument();
      });
    });
  });

  describe('Testimonials Tab', () => {
    it('should render existing testimonials with verification status', async () => {
      renderComponent();

      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      const testimonial = mockCompany.community_validation.employee_testimonials[0];
      expect(screen.getByText(testimonial.message)).toBeInTheDocument();
      expect(screen.getByText(`${testimonial.author_name} - ${testimonial.role}`)).toBeInTheDocument();
      
      if (testimonial.verified) {
        expect(screen.getByText('✓ Verified')).toBeInTheDocument();
      } else {
        expect(screen.getByText('Unverified')).toBeInTheDocument();
      }
    });

    it('should show add testimonial form with all required fields', async () => {
      renderComponent();

      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      const addButton = screen.getByText('Add Testimonial');
      await user.click(addButton);

      expect(screen.getByLabelText('Your Name')).toBeInTheDocument();
      expect(screen.getByLabelText('Your Role')).toBeInTheDocument();
      expect(screen.getByLabelText('Testimonial')).toBeInTheDocument();
    });

    it('should submit testimonial form with all data', async () => {
      const mockAddTestimonial = vi.mocked(registryApi.addTestimonial);
      mockAddTestimonial.mockResolvedValue({ Ok: true });

      renderComponent();

      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      const addButton = screen.getByText('Add Testimonial');
      await user.click(addButton);

      await user.type(screen.getByLabelText('Your Name'), 'John Doe');
      await user.type(screen.getByLabelText('Your Role'), 'Former Developer');
      await user.type(screen.getByLabelText('Testimonial'), 'Amazing place to work!');

      const submitButton = screen.getByText('Add Testimonial');
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockAddTestimonial).toHaveBeenCalledWith({
          company_id: mockCompany.id,
          author_name: 'John Doe',
          role: 'Former Developer',
          message: 'Amazing place to work!',
        });
      });
    });
  });

  describe('Vouches Tab', () => {
    it('should render existing vouches with weight', async () => {
      renderComponent();

      const vouchesTab = screen.getByRole('button', { name: 'Vouches' });
      await user.click(vouchesTab);

      const vouch = mockCompany.community_validation.community_vouches[0];
      expect(screen.getByText(vouch.message)).toBeInTheDocument();
      expect(screen.getByText(`Weight: ${vouch.weight}`)).toBeInTheDocument();
      expect(screen.getByText('Community Member')).toBeInTheDocument();
    });

    it('should show add vouch form', async () => {
      renderComponent();

      const vouchesTab = screen.getByRole('button', { name: 'Vouches' });
      await user.click(vouchesTab);

      const addButton = screen.getByText('Add Vouch');
      await user.click(addButton);

      expect(screen.getByLabelText('Vouch Message')).toBeInTheDocument();
    });

    it('should submit vouch form', async () => {
      const mockAddVouch = vi.mocked(registryApi.addVouch);
      mockAddVouch.mockResolvedValue({ Ok: true });

      renderComponent();

      const vouchesTab = screen.getByRole('button', { name: 'Vouches' });
      await user.click(vouchesTab);

      const addButton = screen.getByText('Add Vouch');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Vouch Message');
      await user.type(messageInput, 'Trustworthy company with great products');

      const submitButton = screen.getByText('Add Vouch');
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockAddVouch).toHaveBeenCalledWith({
          company_id: mockCompany.id,
          message: 'Trustworthy company with great products',
        });
      });
    });
  });

  describe('Empty States', () => {
    const emptyCompany = {
      ...mockCompany,
      community_validation: {
        peer_endorsements: [],
        employee_testimonials: [],
        community_vouches: [],
        reputation_score: 0,
        reputation_staked: BigInt(0),
      },
    };

    it('should show empty state for endorsements', () => {
      render(<CommunityValidation company={emptyCompany} onUpdate={mockOnUpdate} />);

      expect(screen.getByText('No endorsements yet. Be the first to endorse this company!')).toBeInTheDocument();
    });

    it('should show empty state for testimonials', async () => {
      render(<CommunityValidation company={emptyCompany} onUpdate={mockOnUpdate} />);

      const testimonialsTab = screen.getByRole('button', { name: 'Testimonials' });
      await user.click(testimonialsTab);

      expect(screen.getByText('No testimonials yet. Share your experience working with this company!')).toBeInTheDocument();
    });

    it('should show empty state for vouches', async () => {
      render(<CommunityValidation company={emptyCompany} onUpdate={mockOnUpdate} />);

      const vouchesTab = screen.getByRole('button', { name: 'Vouches' });
      await user.click(vouchesTab);

      expect(screen.getByText('No vouches yet. Vouch for this company if you trust them!')).toBeInTheDocument();
    });
  });

  describe('Loading States', () => {
    it('should show loading state during endorsement submission', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockImplementation(() => new Promise(() => {})); // Never resolves

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText('Adding...')).toBeInTheDocument();
      });
    });

    it('should disable submit button during loading', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockImplementation(() => new Promise(() => {})); // Never resolves

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        const addingButton = screen.getByText('Adding...');
        expect(addingButton).toBeDisabled();
      });
    });
  });

  describe('Form Reset', () => {
    it('should reset endorsement form after successful submission', async () => {
      const mockAddEndorsement = vi.mocked(registryApi.addEndorsement);
      mockAddEndorsement.mockResolvedValue({ Ok: true });

      renderComponent();

      const addButton = screen.getByText('Add Endorsement');
      await user.click(addButton);

      const messageInput = screen.getByLabelText('Endorsement Message');
      await user.type(messageInput, 'Great company!');

      const submitButton = screen.getByText('Add Endorsement');
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.queryByLabelText('Endorsement Message')).not.toBeInTheDocument();
      });
    });
  });
});