import React, { useState } from 'react';
import { registryApi } from '../../services/api';
import { CompanyFormData, VerificationStatus } from '../../types';
import { 
  sanitizeInput, 
  validateInput, 
  validateForm, 
  defaultRateLimiter,
  INPUT_LIMITS 
} from '../../utils/validation';
import './CompanyRegistrationForm.scss';

const CompanyRegistrationForm: React.FC = () => {
  const [formData, setFormData] = useState<CompanyFormData>({
    basic_info: {
      name: '',
      description: '',
      website: '',
      founding_date: '',
      team_size: 1,
      focus_areas: []
    },
    web3_identity: {
      github_org: '',
      twitter_handle: '',
      discord_server: '',
      telegram_channel: '',
      domain_verified: false,
      social_verification_status: VerificationStatus.Pending,
      verification_proofs: []
    },
    cross_chain_presence: {
      ethereum_contracts: [],
      bitcoin_addresses: [],
      icp_canisters: [],
      polygon_contracts: [],
      solana_addresses: [],
      sui_addresses: [],
      ton_addresses: [],
      treasury_wallets: [],
      token_contracts: []
    },
    team_members: []
  });

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const focusAreaOptions = [
    'DeFi', 'NFTs', 'Gaming', 'Infrastructure', 'DAOs', 'Metaverse',
    'Layer 2', 'Cross-Chain', 'Privacy', 'Social', 'Tools', 'Education'
  ];

  const handleBasicInfoChange = (field: string, value: any) => {
    // Sanitize input based on field type
    let sanitizedValue = value;
    if (typeof value === 'string') {
      if (field === 'website') {
        sanitizedValue = sanitizeInput.url(value);
      } else {
        sanitizedValue = sanitizeInput.text(value);
      }
    }
    
    setFormData(prev => ({
      ...prev,
      basic_info: {
        ...prev.basic_info,
        [field]: sanitizedValue
      }
    }));
    
    // Clear validation error for this field
    if (validationErrors[`basic_info.${field}`]) {
      setValidationErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[`basic_info.${field}`];
        return newErrors;
      });
    }
  };

  const handleWeb3IdentityChange = (field: string, value: any) => {
    // Sanitize input based on field type
    let sanitizedValue = value;
    if (typeof value === 'string') {
      if (field === 'twitter_handle') {
        sanitizedValue = sanitizeInput.handle(value, 'twitter');
      } else if (field === 'discord_server') {
        sanitizedValue = sanitizeInput.handle(value, 'discord');
      } else if (field === 'telegram_channel') {
        sanitizedValue = sanitizeInput.handle(value, 'telegram');
      } else {
        sanitizedValue = sanitizeInput.text(value);
      }
    }
    
    setFormData(prev => ({
      ...prev,
      web3_identity: {
        ...prev.web3_identity,
        [field]: sanitizedValue
      }
    }));
    
    // Clear validation error for this field
    if (validationErrors[`web3_identity.${field}`]) {
      setValidationErrors(prev => {
        const newErrors = { ...prev };
        delete newErrors[`web3_identity.${field}`];
        return newErrors;
      });
    }
  };

  const handleFocusAreaToggle = (area: string) => {
    setFormData(prev => ({
      ...prev,
      basic_info: {
        ...prev.basic_info,
        focus_areas: prev.basic_info.focus_areas.includes(area)
          ? prev.basic_info.focus_areas.filter(a => a !== area)
          : [...prev.basic_info.focus_areas, area]
      }
    }));
  };

  const addTeamMember = () => {
    setFormData(prev => ({
      ...prev,
      team_members: [
        ...prev.team_members,
        {
          name: '',
          role: '',
          github_profile: '',
          linkedin_profile: '',
          verified: false
        }
      ]
    }));
  };

  const updateTeamMember = (index: number, field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      team_members: prev.team_members.map((member, i) => 
        i === index ? { ...member, [field]: value } : member
      )
    }));
  };

  const removeTeamMember = (index: number) => {
    setFormData(prev => ({
      ...prev,
      team_members: prev.team_members.filter((_, i) => i !== index)
    }));
  };

  const addEthereumContract = () => {
    const contract = prompt('Enter Ethereum contract address (0x followed by 40 hex characters):');
    if (contract) {
      const sanitized = sanitizeInput.address(contract);
      const validation = validateInput.blockchainAddress(sanitized, 'ETHEREUM');
      
      if (!validation.valid) {
        alert(`Invalid Ethereum address: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          ethereum_contracts: [...prev.cross_chain_presence.ethereum_contracts, sanitized]
        }
      }));
    }
  };

  const addBitcoinAddress = () => {
    const address = prompt('Enter Bitcoin address (Legacy, P2SH, or Bech32 format):');
    if (address) {
      const sanitized = sanitizeInput.address(address);
      const validation = validateInput.blockchainAddress(sanitized, 'BITCOIN');
      
      if (!validation.valid) {
        alert(`Invalid Bitcoin address: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          bitcoin_addresses: [...prev.cross_chain_presence.bitcoin_addresses, sanitized]
        }
      }));
    }
  };

  const addIcpCanister = () => {
    const canister = prompt('Enter ICP canister ID (format: xxxxx-xxxxx-xxxxx-xxxxx-xxx):');
    if (canister) {
      const sanitized = sanitizeInput.address(canister);
      const validation = validateInput.blockchainAddress(sanitized, 'ICP_CANISTER');
      
      if (!validation.valid) {
        alert(`Invalid ICP canister ID: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          icp_canisters: [...prev.cross_chain_presence.icp_canisters, sanitized]
        }
      }));
    }
  };

  const addSolanaAddress = () => {
    const address = prompt('Enter Solana address (Base58 string, 32-44 characters):');
    if (address) {
      const sanitized = sanitizeInput.address(address);
      const validation = validateInput.blockchainAddress(sanitized, 'SOLANA');
      
      if (!validation.valid) {
        alert(`Invalid Solana address: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          solana_addresses: [...prev.cross_chain_presence.solana_addresses, sanitized]
        }
      }));
    }
  };

  const addSuiAddress = () => {
    const address = prompt('Enter Sui address (0x followed by 64 hex characters):');
    if (address) {
      const sanitized = sanitizeInput.address(address);
      const validation = validateInput.blockchainAddress(sanitized, 'SUI');
      
      if (!validation.valid) {
        alert(`Invalid Sui address: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          sui_addresses: [...prev.cross_chain_presence.sui_addresses, sanitized]
        }
      }));
    }
  };

  const addTonAddress = () => {
    const address = prompt('Enter TON address (Base64url format, 48 characters):');
    if (address) {
      const sanitized = sanitizeInput.address(address);
      const validation = validateInput.blockchainAddress(sanitized, 'TON');
      
      if (!validation.valid) {
        alert(`Invalid TON address: ${validation.error}`);
        return;
      }
      
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          ton_addresses: [...prev.cross_chain_presence.ton_addresses, sanitized]
        }
      }));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Rate limiting check
    if (!defaultRateLimiter.canProceed('form_submission')) {
      const remaining = defaultRateLimiter.getRemainingAttempts('form_submission');
      setError(`Too many submission attempts. Please wait before trying again. Remaining attempts: ${remaining}`);
      return;
    }
    
    // Comprehensive form validation
    const validation = validateForm.companyRegistration(formData);
    if (!validation.valid) {
      setValidationErrors(validation.errors);
      setError('Please fix the validation errors below.');
      return;
    }
    
    setLoading(true);
    setError(null);
    setValidationErrors({});

    try {
      const result = await registryApi.createCompany(formData);
      
      if ('Ok' in result) {
        setSuccess(true);
        setError(null);
        // Reset form or redirect
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError(`Failed to register company: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  if (success) {
    return (
      <div className="registration-success">
        <h2>🎉 Company Registered Successfully!</h2>
        <p>Your company has been registered in the CrossChain Registry. You can now proceed to verify your information.</p>
        <button onClick={() => window.location.href = '/dashboard'}>
          Go to Dashboard
        </button>
      </div>
    );
  }

  return (
    <form className="company-registration-form" onSubmit={handleSubmit}>
      <h2>Register Your Web3 Company</h2>
      
      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      {/* Basic Information */}
      <section className="form-section">
        <h3>Basic Information</h3>
        
        <div className="form-group">
          <label htmlFor="company-name">Company Name *</label>
          <input
            type="text"
            id="company-name"
            value={formData.basic_info.name}
            onChange={(e) => handleBasicInfoChange('name', e.target.value)}
            minLength={INPUT_LIMITS.COMPANY_NAME.min}
            maxLength={INPUT_LIMITS.COMPANY_NAME.max}
            required
          />
          {validationErrors['basic_info.name'] && (
            <div className="field-error">{validationErrors['basic_info.name']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="description">Description *</label>
          <textarea
            id="description"
            value={formData.basic_info.description}
            onChange={(e) => handleBasicInfoChange('description', e.target.value)}
            minLength={INPUT_LIMITS.DESCRIPTION.min}
            maxLength={INPUT_LIMITS.DESCRIPTION.max}
            rows={4}
            required
          />
          {validationErrors['basic_info.description'] && (
            <div className="field-error">{validationErrors['basic_info.description']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="website">Website *</label>
          <input
            type="url"
            id="website"
            value={formData.basic_info.website}
            onChange={(e) => handleBasicInfoChange('website', e.target.value)}
            minLength={INPUT_LIMITS.WEBSITE_URL.min}
            maxLength={INPUT_LIMITS.WEBSITE_URL.max}
            pattern="https?://.*"
            placeholder="https://example.com"
            required
          />
          {validationErrors['basic_info.website'] && (
            <div className="field-error">{validationErrors['basic_info.website']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="founding-date">Founding Date</label>
          <input
            type="date"
            id="founding-date"
            value={formData.basic_info.founding_date}
            onChange={(e) => handleBasicInfoChange('founding_date', e.target.value)}
          />
        </div>

        <div className="form-group">
          <label htmlFor="team-size">Team Size</label>
          <input
            type="number"
            id="team-size"
            min="1"
            max="10000"
            value={formData.basic_info.team_size}
            onChange={(e) => handleBasicInfoChange('team_size', parseInt(e.target.value))}
          />
          {validationErrors['basic_info.team_size'] && (
            <div className="field-error">{validationErrors['basic_info.team_size']}</div>
          )}
        </div>

        <div className="form-group">
          <label>Focus Areas</label>
          <div className="focus-areas">
            {focusAreaOptions.map(area => (
              <label key={area} className="checkbox-label">
                <input
                  type="checkbox"
                  checked={formData.basic_info.focus_areas.includes(area)}
                  onChange={() => handleFocusAreaToggle(area)}
                />
                {area}
              </label>
            ))}
          </div>
        </div>
      </section>

      {/* Web3 Identity */}
      <section className="form-section">
        <h3>Web3 Identity</h3>
        
        <div className="form-group">
          <label htmlFor="github-org">GitHub Organization</label>
          <input
            type="text"
            id="github-org"
            value={formData.web3_identity.github_org || ''}
            onChange={(e) => handleWeb3IdentityChange('github_org', e.target.value)}
            maxLength={INPUT_LIMITS.GITHUB_ORG.max}
            placeholder="your-org-name"
          />
          {validationErrors['web3_identity.github_org'] && (
            <div className="field-error">{validationErrors['web3_identity.github_org']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="twitter">Twitter Handle</label>
          <input
            type="text"
            id="twitter"
            value={formData.web3_identity.twitter_handle || ''}
            onChange={(e) => handleWeb3IdentityChange('twitter_handle', e.target.value)}
            maxLength={INPUT_LIMITS.TWITTER_HANDLE.max}
            pattern="@?[a-zA-Z0-9_]{1,15}"
            placeholder="@yourcompany"
          />
          {validationErrors['web3_identity.twitter_handle'] && (
            <div className="field-error">{validationErrors['web3_identity.twitter_handle']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="discord">Discord Server</label>
          <input
            type="text"
            id="discord"
            value={formData.web3_identity.discord_server || ''}
            onChange={(e) => handleWeb3IdentityChange('discord_server', e.target.value)}
            maxLength={INPUT_LIMITS.DISCORD_SERVER.max}
            placeholder="discord.gg/yourserver"
          />
          {validationErrors['web3_identity.discord_server'] && (
            <div className="field-error">{validationErrors['web3_identity.discord_server']}</div>
          )}
        </div>

        <div className="form-group">
          <label htmlFor="telegram">Telegram Channel</label>
          <input
            type="text"
            id="telegram"
            value={formData.web3_identity.telegram_channel || ''}
            onChange={(e) => handleWeb3IdentityChange('telegram_channel', e.target.value)}
            maxLength={INPUT_LIMITS.TELEGRAM_CHANNEL.max}
            placeholder="t.me/yourcompany"
          />
          {validationErrors['web3_identity.telegram_channel'] && (
            <div className="field-error">{validationErrors['web3_identity.telegram_channel']}</div>
          )}
        </div>
      </section>

      {/* Cross-Chain Presence */}
      <section className="form-section">
        <h3>Cross-Chain Presence</h3>
        
        <div className="form-group">
          <label>Ethereum Contracts</label>
          <div className="address-list">
            {formData.cross_chain_presence.ethereum_contracts.map((contract, index) => (
              <div key={index} className="address-item">
                <span>{contract}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      ethereum_contracts: prev.cross_chain_presence.ethereum_contracts.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addEthereumContract}>Add Ethereum Contract</button>
          </div>
        </div>

        <div className="form-group">
          <label>Bitcoin Addresses</label>
          <div className="address-list">
            {formData.cross_chain_presence.bitcoin_addresses.map((address, index) => (
              <div key={index} className="address-item">
                <span>{address}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      bitcoin_addresses: prev.cross_chain_presence.bitcoin_addresses.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addBitcoinAddress}>Add Bitcoin Address</button>
          </div>
        </div>

        <div className="form-group">
          <label>ICP Canisters</label>
          <div className="address-list">
            {formData.cross_chain_presence.icp_canisters.map((canister, index) => (
              <div key={index} className="address-item">
                <span>{canister}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      icp_canisters: prev.cross_chain_presence.icp_canisters.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addIcpCanister}>Add ICP Canister</button>
          </div>
        </div>

        <div className="form-group">
          <label>Solana Addresses</label>
          <div className="address-list">
            {formData.cross_chain_presence.solana_addresses.map((address, index) => (
              <div key={index} className="address-item">
                <span>{address}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      solana_addresses: prev.cross_chain_presence.solana_addresses.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addSolanaAddress}>Add Solana Address</button>
          </div>
        </div>

        <div className="form-group">
          <label>Sui Addresses</label>
          <div className="address-list">
            {formData.cross_chain_presence.sui_addresses.map((address, index) => (
              <div key={index} className="address-item">
                <span>{address}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      sui_addresses: prev.cross_chain_presence.sui_addresses.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addSuiAddress}>Add Sui Address</button>
          </div>
        </div>

        <div className="form-group">
          <label>TON Addresses</label>
          <div className="address-list">
            {formData.cross_chain_presence.ton_addresses.map((address, index) => (
              <div key={index} className="address-item">
                <span>{address}</span>
                <button 
                  type="button" 
                  onClick={() => setFormData(prev => ({
                    ...prev,
                    cross_chain_presence: {
                      ...prev.cross_chain_presence,
                      ton_addresses: prev.cross_chain_presence.ton_addresses.filter((_, i) => i !== index)
                    }
                  }))}
                >
                  Remove
                </button>
              </div>
            ))}
            <button type="button" onClick={addTonAddress}>Add TON Address</button>
          </div>
        </div>
      </section>

      {/* Team Members */}
      <section className="form-section">
        <h3>Team Members</h3>
        
        {formData.team_members.map((member, index) => (
          <div key={index} className="team-member">
            <div className="form-group">
              <label>Name</label>
              <input
                type="text"
                value={member.name}
                onChange={(e) => updateTeamMember(index, 'name', e.target.value)}
              />
            </div>
            <div className="form-group">
              <label>Role</label>
              <input
                type="text"
                value={member.role}
                onChange={(e) => updateTeamMember(index, 'role', e.target.value)}
              />
            </div>
            <div className="form-group">
              <label>GitHub Profile</label>
              <input
                type="text"
                value={member.github_profile || ''}
                onChange={(e) => updateTeamMember(index, 'github_profile', e.target.value)}
              />
            </div>
            <div className="form-group">
              <label>LinkedIn Profile</label>
              <input
                type="text"
                value={member.linkedin_profile || ''}
                onChange={(e) => updateTeamMember(index, 'linkedin_profile', e.target.value)}
              />
            </div>
            <button type="button" onClick={() => removeTeamMember(index)}>
              Remove Team Member
            </button>
          </div>
        ))}
        
        <button type="button" onClick={addTeamMember}>
          Add Team Member
        </button>
      </section>

      <div className="form-actions">
        <button type="submit" disabled={loading} className="submit-button">
          {loading ? 'Registering...' : 'Register Company'}
        </button>
      </div>
    </form>
  );
};

export default CompanyRegistrationForm;