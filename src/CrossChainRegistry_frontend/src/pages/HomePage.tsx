import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { registryApi } from '../services/api';
import { RegistryStatistics } from '../types';
import './HomePage.scss';

const HomePage: React.FC = () => {
  const [stats, setStats] = useState<RegistryStatistics | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStatistics();
  }, []);

  const loadStatistics = async () => {
    try {
      const statistics = await registryApi.getRegistryStatistics();
      setStats(statistics);
    } catch (error) {
      console.error('Failed to load statistics:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="home-page">
      {/* Hero Section */}
      <section className="hero">
        <div className="hero__container">
          <h1 className="hero__title">
            CrossChain Registry
          </h1>
          <p className="hero__subtitle">
            The definitive registry for verified Web3 companies and cross-chain presence
          </p>
          <p className="hero__description">
            Discover legitimate Web3 companies, verify their cross-chain presence, and build trust 
            through community validation. Protect yourself from scams and connect with authentic projects.
          </p>
          
          <div className="hero__actions">
            <Link to="/companies" className="button button--primary">
              Browse Companies
            </Link>
            <Link to="/register" className="button button--secondary">
              Register Your Company
            </Link>
          </div>
        </div>
      </section>

      {/* Statistics Section */}
      <section className="stats-section">
        <div className="stats-section__container">
          <h2>Registry Statistics</h2>
          {loading ? (
            <div className="loading">Loading statistics...</div>
          ) : stats ? (
            <div className="stats-grid">
              <div className="stat-card">
                <div className="stat-card__value">{stats.total_companies}</div>
                <div className="stat-card__label">Total Companies</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.verified_companies}</div>
                <div className="stat-card__label">Verified Companies</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.trusted_companies}</div>
                <div className="stat-card__label">Trusted Companies</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.companies_with_github}</div>
                <div className="stat-card__label">With GitHub</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.companies_with_contracts}</div>
                <div className="stat-card__label">With Contracts</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.companies_with_domain_verification}</div>
                <div className="stat-card__label">Domain Verified</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{stats.total_endorsements}</div>
                <div className="stat-card__label">Peer Endorsements</div>
              </div>
              <div className="stat-card">
                <div className="stat-card__value">{Math.round(stats.average_reputation_score)}</div>
                <div className="stat-card__label">Avg. Reputation</div>
              </div>
            </div>
          ) : (
            <div className="error">Failed to load statistics</div>
          )}
        </div>
      </section>

      {/* Features Section */}
      <section className="features-section">
        <div className="features-section__container">
          <h2>Why Use CrossChain Registry?</h2>
          <div className="features-grid">
            <div className="feature-card">
              <div className="feature-card__icon">🔍</div>
              <h3>Scam Protection</h3>
              <p>
                Advanced verification systems help identify and filter out fraudulent companies 
                targeting Web3 job seekers and investors.
              </p>
            </div>
            
            <div className="feature-card">
              <div className="feature-card__icon">🌐</div>
              <h3>Cross-Chain Verification</h3>
              <p>
                Verify company presence across Ethereum, Bitcoin, ICP, Polygon and other blockchain networks 
                with comprehensive contract and wallet verification.
              </p>
            </div>
            
            <div className="feature-card">
              <div className="feature-card__icon">👥</div>
              <h3>Community Validation</h3>
              <p>
                Peer endorsements, employee testimonials, and community vouches create 
                a trust network backed by reputation scoring.
              </p>
            </div>
            
            <div className="feature-card">
              <div className="feature-card__icon">🔗</div>
              <h3>Web3 Identity</h3>
              <p>
                GitHub organization ownership, domain verification, and social media 
                authentication for complete Web3 identity verification.
              </p>
            </div>
            
            <div className="feature-card">
              <div className="feature-card__icon">📊</div>
              <h3>Transparency Metrics</h3>
              <p>
                Open source contributions, development activity, and financial transparency 
                provide comprehensive company insights.
              </p>
            </div>
            
            <div className="feature-card">
              <div className="feature-card__icon">🏆</div>
              <h3>Reputation System</h3>
              <p>
                Multi-factor reputation scoring combines verification signals, community validation, 
                and staking mechanisms for trust assessment.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* How It Works Section */}
      <section className="how-it-works">
        <div className="how-it-works__container">
          <h2>How It Works</h2>
          <div className="steps">
            <div className="step">
              <div className="step__number">1</div>
              <div className="step__content">
                <h3>Register Your Company</h3>
                <p>Submit basic information, Web3 identity, and cross-chain presence details.</p>
              </div>
            </div>
            
            <div className="step">
              <div className="step__number">2</div>
              <div className="step__content">
                <h3>Complete Verification</h3>
                <p>Verify GitHub organization, domain ownership, and cross-chain addresses/contracts.</p>
              </div>
            </div>
            
            <div className="step">
              <div className="step__number">3</div>
              <div className="step__content">
                <h3>Build Community Trust</h3>
                <p>Receive endorsements, testimonials, and vouches to increase your reputation score.</p>
              </div>
            </div>
            
            <div className="step">
              <div className="step__number">4</div>
              <div className="step__content">
                <h3>Gain Trusted Status</h3>
                <p>Achieve verified or trusted status to unlock better opportunities and partnerships.</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="cta-section">
        <div className="cta-section__container">
          <h2>Ready to Get Started?</h2>
          <p>
            Join the growing network of verified Web3 companies and build trust through transparency.
          </p>
          <div className="cta-actions">
            <Link to="/register" className="button button--primary button--large">
              Register Your Company
            </Link>
            <Link to="/companies" className="button button--secondary button--large">
              Explore Companies
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
};

export default HomePage;