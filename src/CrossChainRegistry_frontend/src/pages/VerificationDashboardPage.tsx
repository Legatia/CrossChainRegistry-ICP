import React from 'react';
import './VerificationDashboardPage.scss';

const VerificationDashboardPage: React.FC = () => {
  return (
    <div className="verification-dashboard-page">
      <div className="verification-dashboard-page__container">
        <div className="dashboard-header">
          <h1>Verification Dashboard</h1>
          <p>Manage your company's verification status and improve your reputation score</p>
        </div>

        <div className="dashboard-content">
          <div className="coming-soon">
            <h2>🚧 Dashboard Coming Soon</h2>
            <p>
              The verification dashboard is currently under development. This will include:
            </p>
            <ul>
              <li>Company verification status overview</li>
              <li>Domain verification management</li>
              <li>Cross-chain verification tools</li>
              <li>GitHub organization verification</li>
              <li>Social media verification status</li>
              <li>Community validation metrics</li>
              <li>Reputation score breakdown</li>
            </ul>
            <p>
              In the meantime, you can browse companies and see examples of verified profiles.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default VerificationDashboardPage;