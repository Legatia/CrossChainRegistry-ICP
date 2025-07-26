import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { registryApi } from '../services/api';
import { Company, CompanyStatus } from '../types';
import CommunityValidation from '../components/community/CommunityValidation';
import './CompanyProfilePage.scss';

const CompanyProfilePage: React.FC = () => {
  const { companyId } = useParams<{ companyId: string }>();
  const [company, setCompany] = useState<Company | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (companyId) {
      loadCompany(companyId);
    }
  }, [companyId]);

  const loadCompany = async (id: string) => {
    try {
      setLoading(true);
      setError(null);
      const result = await registryApi.getCompany(id);
      if (result) {
        setCompany(result);
      } else {
        setError('Company not found');
      }
    } catch (err) {
      setError(`Failed to load company: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const formatDate = (timestamp: bigint) => {
    return new Date(Number(timestamp) / 1_000_000).toLocaleDateString();
  };

  const getStatusColor = (status: CompanyStatus) => {
    switch (status) {
      case CompanyStatus.Verified:
        return 'green';
      case CompanyStatus.Trusted:
        return 'blue';
      case CompanyStatus.Pending:
        return 'orange';
      case CompanyStatus.Flagged:
        return 'red';
      default:
        return 'gray';
    }
  };

  if (loading) {
    return (
      <div className="company-profile-page">
        <div className="loading">Loading company profile...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="company-profile-page">
        <div className="error-message">{error}</div>
      </div>
    );
  }

  if (!company) {
    return (
      <div className="company-profile-page">
        <div className="error-message">Company not found</div>
      </div>
    );
  }

  return (
    <div className="company-profile-page">
      <div className="company-profile-page__container">
        {/* Header Section */}
        <div className="company-header">
          <div className="company-header__main">
            <h1 className="company-header__name">{company.basic_info.name}</h1>
            <span className={`company-header__status company-header__status--${getStatusColor(company.status)}`}>
              {company.status}
            </span>
          </div>
          
          <div className="company-header__info">
            <div className="company-header__website">
              <a href={company.basic_info.website} target="_blank" rel="noopener noreferrer">
                {company.basic_info.website}
              </a>
            </div>
            
            <div className="company-header__meta">
              <span>Team Size: {company.basic_info.team_size}</span>
              {company.basic_info.founding_date && (
                <span>Founded: {company.basic_info.founding_date}</span>
              )}
              <span>Registered: {formatDate(company.created_at)}</span>
            </div>
          </div>

          <div className="company-header__score">
            <div className="score-circle">
              <span className="score-value">{company.verification_score}</span>
              <span className="score-label">Verification Score</span>
            </div>
          </div>
        </div>

        {/* Description */}
        <section className="company-section">
          <h2>About</h2>
          <p className="company-description">{company.basic_info.description}</p>
        </section>

        {/* Focus Areas */}
        {company.basic_info.focus_areas.length > 0 && (
          <section className="company-section">
            <h2>Focus Areas</h2>
            <div className="focus-areas">
              {company.basic_info.focus_areas.map((area, index) => (
                <span key={index} className="focus-area-tag">{area}</span>
              ))}
            </div>
          </section>
        )}

        {/* Web3 Identity */}
        <section className="company-section">
          <h2>Web3 Identity</h2>
          <div className="web3-identity">
            {company.web3_identity.github_org && (
              <div className="identity-item">
                <span className="label">GitHub Organization:</span>
                <a 
                  href={`https://github.com/${company.web3_identity.github_org}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="identity-link"
                >
                  {company.web3_identity.github_org}
                </a>
              </div>
            )}
            
            {company.web3_identity.twitter_handle && (
              <div className="identity-item">
                <span className="label">Twitter:</span>
                <a 
                  href={`https://twitter.com/${company.web3_identity.twitter_handle.replace('@', '')}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="identity-link"
                >
                  {company.web3_identity.twitter_handle}
                </a>
              </div>
            )}
            
            {company.web3_identity.discord_server && (
              <div className="identity-item">
                <span className="label">Discord:</span>
                <span className="identity-value">{company.web3_identity.discord_server}</span>
              </div>
            )}
            
            {company.web3_identity.telegram_channel && (
              <div className="identity-item">
                <span className="label">Telegram:</span>
                <span className="identity-value">{company.web3_identity.telegram_channel}</span>
              </div>
            )}
            
            <div className="identity-item">
              <span className="label">Domain Verified:</span>
              <span className={`verification-badge ${company.web3_identity.domain_verified ? 'verified' : 'unverified'}`}>
                {company.web3_identity.domain_verified ? '✓ Verified' : '✗ Not Verified'}
              </span>
            </div>
          </div>
        </section>

        {/* Cross-Chain Presence */}
        <section className="company-section">
          <h2>Cross-Chain Presence</h2>
          <div className="cross-chain-presence">
            {company.cross_chain_presence.ethereum_contracts.length > 0 && (
              <div className="chain-section">
                <h3>Ethereum Contracts</h3>
                <div className="address-list">
                  {company.cross_chain_presence.ethereum_contracts.map((contract, index) => (
                    <div key={index} className="address-item">
                      <a 
                        href={`https://etherscan.io/address/${contract}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="address-link"
                      >
                        {contract}
                      </a>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {company.cross_chain_presence.bitcoin_addresses.length > 0 && (
              <div className="chain-section">
                <h3>Bitcoin Addresses</h3>
                <div className="address-list">
                  {company.cross_chain_presence.bitcoin_addresses.map((address, index) => (
                    <div key={index} className="address-item">
                      <a 
                        href={`https://blockstream.info/address/${address}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="address-link"
                      >
                        {address}
                      </a>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {company.cross_chain_presence.icp_canisters.length > 0 && (
              <div className="chain-section">
                <h3>ICP Canisters</h3>
                <div className="address-list">
                  {company.cross_chain_presence.icp_canisters.map((canister, index) => (
                    <div key={index} className="address-item">
                      <span className="address-link">{canister}</span>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {company.cross_chain_presence.polygon_contracts.length > 0 && (
              <div className="chain-section">
                <h3>Polygon Contracts</h3>
                <div className="address-list">
                  {company.cross_chain_presence.polygon_contracts.map((contract, index) => (
                    <div key={index} className="address-item">
                      <a 
                        href={`https://polygonscan.com/address/${contract}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="address-link"
                      >
                        {contract}
                      </a>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {company.cross_chain_presence.treasury_wallets.length > 0 && (
              <div className="chain-section">
                <h3>Treasury Wallets</h3>
                <div className="wallet-list">
                  {company.cross_chain_presence.treasury_wallets.map((wallet, index) => (
                    <div key={index} className="wallet-item">
                      <div className="wallet-info">
                        <span className="wallet-chain">{wallet.chain}</span>
                        <span className="wallet-type">({wallet.wallet_type})</span>
                        <span className={`wallet-status ${wallet.verified ? 'verified' : 'unverified'}`}>
                          {wallet.verified ? '✓' : '?'}
                        </span>
                      </div>
                      <div className="wallet-address">{wallet.address}</div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {company.cross_chain_presence.token_contracts.length > 0 && (
              <div className="chain-section">
                <h3>Token Contracts</h3>
                <div className="token-list">
                  {company.cross_chain_presence.token_contracts.map((token, index) => (
                    <div key={index} className="token-item">
                      <div className="token-info">
                        <span className="token-name">{token.name} ({token.symbol})</span>
                        <span className="token-chain">{token.chain}</span>
                        <span className={`token-status ${token.verified ? 'verified' : 'unverified'}`}>
                          {token.verified ? '✓' : '?'}
                        </span>
                      </div>
                      <div className="token-address">{token.contract_address}</div>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        </section>

        {/* Team Members */}
        {company.team_members.length > 0 && (
          <section className="company-section">
            <h2>Team Members</h2>
            <div className="team-members">
              {company.team_members.map((member, index) => (
                <div key={index} className="team-member">
                  <div className="member-info">
                    <h4 className="member-name">{member.name}</h4>
                    <span className="member-role">{member.role}</span>
                    <span className={`member-status ${member.verified ? 'verified' : 'unverified'}`}>
                      {member.verified ? '✓ Verified' : 'Unverified'}
                    </span>
                  </div>
                  <div className="member-links">
                    {member.github_profile && (
                      <a href={member.github_profile} target="_blank" rel="noopener noreferrer">
                        GitHub
                      </a>
                    )}
                    {member.linkedin_profile && (
                      <a href={member.linkedin_profile} target="_blank" rel="noopener noreferrer">
                        LinkedIn
                      </a>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Community Validation */}
        <CommunityValidation company={company} onUpdate={() => loadCompany(company.id)} />
      </div>
    </div>
  );
};

export default CompanyProfilePage;