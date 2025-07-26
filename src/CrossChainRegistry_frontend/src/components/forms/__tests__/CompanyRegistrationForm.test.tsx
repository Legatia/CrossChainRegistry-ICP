import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '../../../test/utils/testUtils';
import userEvent from '@testing-library/user-event';
import CompanyRegistrationForm from '../CompanyRegistrationForm';
import { registryApi } from '../../../services/api';
import { mockPrompt } from '../../../test/utils/testUtils';

// Mock the API service
vi.mock('../../../services/api', () => ({
  registryApi: {
    createCompany: vi.fn(),
  },
}));

describe('CompanyRegistrationForm', () => {
  const user = userEvent.setup();

  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('Form Rendering', () => {
    it('should render all required form sections', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByText('Register Your Web3 Company')).toBeInTheDocument();
      expect(screen.getByText('Basic Information')).toBeInTheDocument();
      expect(screen.getByText('Web3 Identity')).toBeInTheDocument();
      expect(screen.getByText('Cross-Chain Presence')).toBeInTheDocument();
      expect(screen.getByText('Team Members')).toBeInTheDocument();
    });

    it('should render all basic information fields', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByLabelText(/company name/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/website/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/founding date/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/team size/i)).toBeInTheDocument();
    });

    it('should render focus areas as checkboxes', () => {
      render(<CompanyRegistrationForm />);

      const focusAreas = ['DeFi', 'NFTs', 'Gaming', 'Infrastructure', 'DAOs', 'Metaverse'];
      
      focusAreas.forEach(area => {
        expect(screen.getByLabelText(area)).toBeInTheDocument();
      });
    });

    it('should render Web3 identity fields', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByLabelText(/github organization/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/twitter handle/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/discord server/i)).toBeInTheDocument();
      expect(screen.getByLabelText(/telegram channel/i)).toBeInTheDocument();
    });

    it('should render cross-chain presence section with add buttons', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByText('Ethereum Contracts')).toBeInTheDocument();
      expect(screen.getByText('Bitcoin Addresses')).toBeInTheDocument();
      expect(screen.getByText('ICP Canisters')).toBeInTheDocument();
      
      expect(screen.getByText('Add Ethereum Contract')).toBeInTheDocument();
      expect(screen.getByText('Add Bitcoin Address')).toBeInTheDocument();
      expect(screen.getByText('Add ICP Canister')).toBeInTheDocument();
    });

    it('should render team members section with add button', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByText('Add Team Member')).toBeInTheDocument();
    });

    it('should render submit button', () => {
      render(<CompanyRegistrationForm />);

      expect(screen.getByRole('button', { name: /register company/i })).toBeInTheDocument();
    });
  });

  describe('Form Interactions', () => {
    it('should handle basic information input changes', async () => {
      render(<CompanyRegistrationForm />);

      const nameInput = screen.getByLabelText(/company name/i);
      const descriptionInput = screen.getByLabelText(/description/i);
      const websiteInput = screen.getByLabelText(/website/i);

      await user.type(nameInput, 'Test Company');
      await user.type(descriptionInput, 'A test company description');
      await user.type(websiteInput, 'https://testcompany.com');

      expect(nameInput).toHaveValue('Test Company');
      expect(descriptionInput).toHaveValue('A test company description');
      expect(websiteInput).toHaveValue('https://testcompany.com');
    });

    it('should handle focus area selection', async () => {
      render(<CompanyRegistrationForm />);

      const defiCheckbox = screen.getByLabelText('DeFi');
      const nftCheckbox = screen.getByLabelText('NFTs');

      await user.click(defiCheckbox);
      await user.click(nftCheckbox);

      expect(defiCheckbox).toBeChecked();
      expect(nftCheckbox).toBeChecked();
    });

    it('should toggle focus areas on and off', async () => {
      render(<CompanyRegistrationForm />);

      const defiCheckbox = screen.getByLabelText('DeFi');

      await user.click(defiCheckbox);
      expect(defiCheckbox).toBeChecked();

      await user.click(defiCheckbox);
      expect(defiCheckbox).not.toBeChecked();
    });

    it('should handle Web3 identity input changes', async () => {
      render(<CompanyRegistrationForm />);

      const githubInput = screen.getByLabelText(/github organization/i);
      const twitterInput = screen.getByLabelText(/twitter handle/i);

      await user.type(githubInput, 'test-org');
      await user.type(twitterInput, '@testcompany');

      expect(githubInput).toHaveValue('test-org');
      expect(twitterInput).toHaveValue('@testcompany');
    });

    it('should add team members', async () => {
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Team Member');
      await user.click(addButton);

      expect(screen.getByLabelText('Name')).toBeInTheDocument();
      expect(screen.getByLabelText('Role')).toBeInTheDocument();
      expect(screen.getByLabelText('GitHub Profile')).toBeInTheDocument();
      expect(screen.getByLabelText('LinkedIn Profile')).toBeInTheDocument();
      expect(screen.getByText('Remove Team Member')).toBeInTheDocument();
    });

    it('should remove team members', async () => {
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Team Member');
      await user.click(addButton);

      const removeButton = screen.getByText('Remove Team Member');
      await user.click(removeButton);

      expect(screen.queryByLabelText('Name')).not.toBeInTheDocument();
      expect(screen.queryByText('Remove Team Member')).not.toBeInTheDocument();
    });

    it('should update team member information', async () => {
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Team Member');
      await user.click(addButton);

      const nameInput = screen.getByLabelText('Name');
      const roleInput = screen.getByLabelText('Role');

      await user.type(nameInput, 'John Doe');
      await user.type(roleInput, 'CEO');

      expect(nameInput).toHaveValue('John Doe');
      expect(roleInput).toHaveValue('CEO');
    });
  });

  describe('Cross-Chain Address Management', () => {
    it('should add Ethereum contract via prompt', async () => {
      const restorePrompt = mockPrompt('0x1234567890123456789012345678901234567890');
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Ethereum Contract');
      await user.click(addButton);

      await waitFor(() => {
        expect(screen.getByText('0x1234567890123456789012345678901234567890')).toBeInTheDocument();
      });

      restorePrompt();
    });

    it('should add Bitcoin address via prompt', async () => {
      const restorePrompt = mockPrompt('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa');
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Bitcoin Address');
      await user.click(addButton);

      await waitFor(() => {
        expect(screen.getByText('1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa')).toBeInTheDocument();
      });

      restorePrompt();
    });

    it('should add ICP canister via prompt', async () => {
      const restorePrompt = mockPrompt('rdmx6-jaaaa-aaaah-qcaiq-cai');
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add ICP Canister');
      await user.click(addButton);

      await waitFor(() => {
        expect(screen.getByText('rdmx6-jaaaa-aaaah-qcaiq-cai')).toBeInTheDocument();
      });

      restorePrompt();
    });

    it('should not add empty addresses when prompt is cancelled', async () => {
      const restorePrompt = mockPrompt(null);
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Ethereum Contract');
      await user.click(addButton);

      // Should not add any address item
      expect(screen.queryByText('Remove')).not.toBeInTheDocument();

      restorePrompt();
    });

    it('should remove added addresses', async () => {
      const restorePrompt = mockPrompt('0x1234567890123456789012345678901234567890');
      render(<CompanyRegistrationForm />);

      const addButton = screen.getByText('Add Ethereum Contract');
      await user.click(addButton);

      await waitFor(() => {
        const removeButton = screen.getByText('Remove');
        user.click(removeButton);
      });

      await waitFor(() => {
        expect(screen.queryByText('0x1234567890123456789012345678901234567890')).not.toBeInTheDocument();
      });

      restorePrompt();
    });
  });

  describe('Form Submission', () => {
    const fillBasicForm = async () => {
      const nameInput = screen.getByLabelText(/company name/i);
      const descriptionInput = screen.getByLabelText(/description/i);
      const websiteInput = screen.getByLabelText(/website/i);

      await user.type(nameInput, 'Test Company');
      await user.type(descriptionInput, 'A test company');
      await user.type(websiteInput, 'https://test.com');
    };

    it('should submit form with valid data', async () => {
      const mockCreateCompany = vi.mocked(registryApi.createCompany);
      mockCreateCompany.mockResolvedValue({ Ok: {} as any });

      render(<CompanyRegistrationForm />);

      await fillBasicForm();

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockCreateCompany).toHaveBeenCalledTimes(1);
      });
    });

    it('should show loading state during submission', async () => {
      const mockCreateCompany = vi.mocked(registryApi.createCompany);
      mockCreateCompany.mockImplementation(() => new Promise(() => {})); // Never resolves

      render(<CompanyRegistrationForm />);

      await fillBasicForm();

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText('Registering...')).toBeInTheDocument();
        expect(submitButton).toBeDisabled();
      });
    });

    it('should show success message after successful submission', async () => {
      const mockCreateCompany = vi.mocked(registryApi.createCompany);
      mockCreateCompany.mockResolvedValue({ Ok: {} as any });

      render(<CompanyRegistrationForm />);

      await fillBasicForm();

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText(/company registered successfully/i)).toBeInTheDocument();
        expect(screen.getByText('Go to Dashboard')).toBeInTheDocument();
      });
    });

    it('should show error message on submission failure', async () => {
      const mockCreateCompany = vi.mocked(registryApi.createCompany);
      mockCreateCompany.mockResolvedValue({ Err: 'Network error' });

      render(<CompanyRegistrationForm />);

      await fillBasicForm();

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(screen.getByText('Network error')).toBeInTheDocument();
      });
    });

    it('should require company name for submission', async () => {
      render(<CompanyRegistrationForm />);

      const descriptionInput = screen.getByLabelText(/description/i);
      const websiteInput = screen.getByLabelText(/website/i);

      await user.type(descriptionInput, 'A test company');
      await user.type(websiteInput, 'https://test.com');

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      // Form should not submit without required fields
      expect(registryApi.createCompany).not.toHaveBeenCalled();
    });

    it('should include form data in submission', async () => {
      const mockCreateCompany = vi.mocked(registryApi.createCompany);
      mockCreateCompany.mockResolvedValue({ Ok: {} as any });

      render(<CompanyRegistrationForm />);

      await fillBasicForm();

      const defiCheckbox = screen.getByLabelText('DeFi');
      await user.click(defiCheckbox);

      const githubInput = screen.getByLabelText(/github organization/i);
      await user.type(githubInput, 'test-org');

      const submitButton = screen.getByRole('button', { name: /register company/i });
      await user.click(submitButton);

      await waitFor(() => {
        expect(mockCreateCompany).toHaveBeenCalledWith(
          expect.objectContaining({
            basic_info: expect.objectContaining({
              name: 'Test Company',
              description: 'A test company',
              website: 'https://test.com',
              focus_areas: expect.arrayContaining(['DeFi']),
            }),
            web3_identity: expect.objectContaining({
              github_org: 'test-org',
            }),
          })
        );
      });
    });
  });

  describe('Form Validation', () => {
    it('should validate required fields', () => {
      render(<CompanyRegistrationForm />);

      const nameInput = screen.getByLabelText(/company name/i);
      const descriptionInput = screen.getByLabelText(/description/i);
      const websiteInput = screen.getByLabelText(/website/i);

      expect(nameInput).toHaveAttribute('required');
      expect(descriptionInput).toHaveAttribute('required');
      expect(websiteInput).toHaveAttribute('required');
    });

    it('should validate website URL format', () => {
      render(<CompanyRegistrationForm />);

      const websiteInput = screen.getByLabelText(/website/i);
      expect(websiteInput).toHaveAttribute('type', 'url');
    });

    it('should validate team size as number', () => {
      render(<CompanyRegistrationForm />);

      const teamSizeInput = screen.getByLabelText(/team size/i);
      expect(teamSizeInput).toHaveAttribute('type', 'number');
      expect(teamSizeInput).toHaveAttribute('min', '1');
    });
  });
});