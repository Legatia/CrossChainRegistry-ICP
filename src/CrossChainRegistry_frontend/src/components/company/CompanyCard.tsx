import React from 'react';
import { Link } from 'react-router-dom';
import { Company, CompanyStatus } from '../../types';
import './CompanyCard.scss';

interface CompanyCardProps {
  company: Company;
}

const CompanyCard: React.FC<CompanyCardProps> = ({ company }) => {
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

  const formatDate = (timestamp: bigint) => {
    return new Date(Number(timestamp) / 1_000_000).toLocaleDateString();
  };

  return (
    <div className="company-card">
      <div className="company-card__header">
        <h3 className="company-card__name">
          <Link to={`/company/${company.id}`}>
            {company.basic_info.name}
          </Link>
        </h3>
        <span 
          className={`company-card__status company-card__status--${getStatusColor(company.status)}`}
        >
          {company.status}
        </span>
      </div>

      <div className="company-card__info">
        <p className="company-card__description">
          {company.basic_info.description.length > 150 
            ? `${company.basic_info.description.substring(0, 150)}...`
            : company.basic_info.description
          }
        </p>

        <div className="company-card__details">
          <div className="company-card__detail">
            <span className="label">Website:</span>
            <a 
              href={company.basic_info.website} 
              target="_blank" 
              rel="noopener noreferrer"
              className="company-card__link"
            >
              {company.basic_info.website}
            </a>
          </div>

          <div className="company-card__detail">
            <span className="label">Team Size:</span>
            <span>{company.basic_info.team_size}</span>
          </div>

          <div className="company-card__detail">
            <span className="label">Founded:</span>
            <span>{company.basic_info.founding_date || 'Not specified'}</span>
          </div>

          <div className="company-card__detail">
            <span className="label">Verification Score:</span>
            <span className="company-card__score">{company.verification_score}/100</span>
          </div>
        </div>

        {company.basic_info.focus_areas.length > 0 && (
          <div className="company-card__focus-areas">
            <span className="label">Focus Areas:</span>
            <div className="company-card__tags">
              {company.basic_info.focus_areas.slice(0, 3).map((area, index) => (
                <span key={index} className="company-card__tag">
                  {area}
                </span>
              ))}
              {company.basic_info.focus_areas.length > 3 && (
                <span className="company-card__tag company-card__tag--more">
                  +{company.basic_info.focus_areas.length - 3} more
                </span>
              )}
            </div>
          </div>
        )}

        <div className="company-card__verification">
          <div className="company-card__verification-badges">
            {company.web3_identity.github_org && (
              <span className="badge badge--github">GitHub</span>
            )}
            {company.web3_identity.domain_verified && (
              <span className="badge badge--domain">Domain ✓</span>
            )}
            {company.cross_chain_presence.ethereum_contracts.length > 0 && (
              <span className="badge badge--ethereum">Ethereum</span>
            )}
            {company.cross_chain_presence.bitcoin_addresses.length > 0 && (
              <span className="badge badge--bitcoin">Bitcoin</span>
            )}
            {company.cross_chain_presence.icp_canisters.length > 0 && (
              <span className="badge badge--icp">ICP</span>
            )}
            {company.cross_chain_presence.solana_addresses.length > 0 && (
              <span className="badge badge--solana">Solana</span>
            )}
            {company.cross_chain_presence.sui_addresses.length > 0 && (
              <span className="badge badge--sui">Sui</span>
            )}
            {company.cross_chain_presence.ton_addresses.length > 0 && (
              <span className="badge badge--ton">TON</span>
            )}
          </div>
        </div>

        <div className="company-card__community">
          <div className="company-card__stats">
            <span className="stat">
              <span className="stat__value">{company.community_validation.peer_endorsements.length}</span>
              <span className="stat__label">Endorsements</span>
            </span>
            <span className="stat">
              <span className="stat__value">{company.community_validation.employee_testimonials.length}</span>
              <span className="stat__label">Testimonials</span>
            </span>
            <span className="stat">
              <span className="stat__value">{company.community_validation.community_vouches.length}</span>
              <span className="stat__label">Vouches</span>
            </span>
          </div>
        </div>

        <div className="company-card__footer">
          <span className="company-card__created">
            Registered: {formatDate(company.created_at)}
          </span>
        </div>
      </div>
    </div>
  );
};

export default CompanyCard;