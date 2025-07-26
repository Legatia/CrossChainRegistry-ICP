import React from 'react';
import CompanyRegistrationForm from '../components/forms/CompanyRegistrationForm';
import './CompanyRegistrationPage.scss';

const CompanyRegistrationPage: React.FC = () => {
  return (
    <div className="company-registration-page">
      <div className="company-registration-page__container">
        <div className="company-registration-page__header">
          <h1>Register Your Web3 Company</h1>
          <p className="subtitle">
            Join the CrossChain Registry to build trust and credibility in the Web3 community
          </p>
        </div>

        <div className="registration-benefits">
          <h2>Benefits of Registration</h2>
          <div className="benefits-grid">
            <div className="benefit">
              <div className="benefit__icon">✅</div>
              <div className="benefit__content">
                <h3>Build Trust</h3>
                <p>Establish credibility through multi-signal verification</p>
              </div>
            </div>
            <div className="benefit">
              <div className="benefit__icon">🔍</div>
              <div className="benefit__content">
                <h3>Enhanced Visibility</h3>
                <p>Be discovered by job seekers, investors, and partners</p>
              </div>
            </div>
            <div className="benefit">
              <div className="benefit__icon">🌐</div>
              <div className="benefit__content">
                <h3>Cross-Chain Presence</h3>
                <p>Showcase your presence across multiple blockchain networks</p>
              </div>
            </div>
            <div className="benefit">
              <div className="benefit__icon">👥</div>
              <div className="benefit__content">
                <h3>Community Validation</h3>
                <p>Receive endorsements and testimonials from the community</p>
              </div>
            </div>
          </div>
        </div>

        <div className="verification-process">
          <h2>Verification Process</h2>
          <div className="process-steps">
            <div className="process-step">
              <div className="step-number">1</div>
              <div className="step-content">
                <h3>Submit Information</h3>
                <p>Provide company details, Web3 identity, and cross-chain presence</p>
              </div>
            </div>
            <div className="process-step">
              <div className="step-number">2</div>
              <div className="step-content">
                <h3>Complete Verifications</h3>
                <p>Verify GitHub organization, domain ownership, and blockchain addresses</p>
              </div>
            </div>
            <div className="process-step">
              <div className="step-number">3</div>
              <div className="step-content">
                <h3>Build Reputation</h3>
                <p>Receive community endorsements and increase your reputation score</p>
              </div>
            </div>
          </div>
        </div>

        <CompanyRegistrationForm />
      </div>
    </div>
  );
};

export default CompanyRegistrationPage;