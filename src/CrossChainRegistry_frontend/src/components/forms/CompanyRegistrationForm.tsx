import React, { useState } from 'react';
import { registryApi } from '../../services/api';
import { CompanyFormData, VerificationStatus } from '../../types';
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
      social_verification_status: VerificationStatus.Pending
    },
    cross_chain_presence: {
      ethereum_contracts: [],
      bitcoin_addresses: [],
      icp_canisters: [],
      polygon_contracts: [],
      treasury_wallets: [],
      token_contracts: []
    },
    team_members: []
  });

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  const focusAreaOptions = [
    'DeFi', 'NFTs', 'Gaming', 'Infrastructure', 'DAOs', 'Metaverse',
    'Layer 2', 'Cross-Chain', 'Privacy', 'Social', 'Tools', 'Education'
  ];

  const handleBasicInfoChange = (field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      basic_info: {
        ...prev.basic_info,
        [field]: value
      }
    }));
  };

  const handleWeb3IdentityChange = (field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      web3_identity: {
        ...prev.web3_identity,
        [field]: value
      }
    }));
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
    const contract = prompt('Enter Ethereum contract address:');
    if (contract && contract.trim()) {
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          ethereum_contracts: [...prev.cross_chain_presence.ethereum_contracts, contract.trim()]
        }
      }));
    }
  };

  const addBitcoinAddress = () => {
    const address = prompt('Enter Bitcoin address:');
    if (address && address.trim()) {
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          bitcoin_addresses: [...prev.cross_chain_presence.bitcoin_addresses, address.trim()]
        }
      }));
    }
  };

  const addIcpCanister = () => {
    const canister = prompt('Enter ICP canister ID:');
    if (canister && canister.trim()) {
      setFormData(prev => ({
        ...prev,
        cross_chain_presence: {
          ...prev.cross_chain_presence,
          icp_canisters: [...prev.cross_chain_presence.icp_canisters, canister.trim()]
        }
      }));
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

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
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="description">Description *</label>
          <textarea
            id="description"
            value={formData.basic_info.description}
            onChange={(e) => handleBasicInfoChange('description', e.target.value)}
            rows={4}
            required
          />
        </div>

        <div className="form-group">
          <label htmlFor="website">Website *</label>
          <input
            type="url"
            id="website"
            value={formData.basic_info.website}
            onChange={(e) => handleBasicInfoChange('website', e.target.value)}
            required
          />
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
            value={formData.basic_info.team_size}
            onChange={(e) => handleBasicInfoChange('team_size', parseInt(e.target.value))}
          />
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
            placeholder="your-org-name"
          />
        </div>

        <div className="form-group">
          <label htmlFor="twitter">Twitter Handle</label>
          <input
            type="text"
            id="twitter"
            value={formData.web3_identity.twitter_handle || ''}
            onChange={(e) => handleWeb3IdentityChange('twitter_handle', e.target.value)}
            placeholder="@yourcompany"
          />
        </div>

        <div className="form-group">
          <label htmlFor="discord">Discord Server</label>
          <input
            type="text"
            id="discord"
            value={formData.web3_identity.discord_server || ''}
            onChange={(e) => handleWeb3IdentityChange('discord_server', e.target.value)}
            placeholder="discord.gg/yourserver"
          />
        </div>

        <div className="form-group">
          <label htmlFor="telegram">Telegram Channel</label>
          <input
            type="text"
            id="telegram"
            value={formData.web3_identity.telegram_channel || ''}
            onChange={(e) => handleWeb3IdentityChange('telegram_channel', e.target.value)}
            placeholder="t.me/yourcompany"
          />
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