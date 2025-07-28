import React from 'react';
import './VerificationProofDisplay.scss';

// URL sanitization utility for XSS protection
const sanitizeUrl = (url: string): string => {
  // Basic URL validation and sanitization
  try {
    const urlObj = new URL(url);
    
    // Only allow https protocol
    if (urlObj.protocol !== 'https:') {
      return '#';
    }
    
    // Whitelist allowed domains for security
    const allowedDomains = [
      'twitter.com', 'x.com', 'mobile.twitter.com',
      'discord.gg', 'discord.com', 'discordapp.com',
      't.me', 'telegram.me'
    ];
    
    const hostname = urlObj.hostname.toLowerCase();
    const isAllowed = allowedDomains.some(domain => 
      hostname === domain || hostname.endsWith(`.${domain}`)
    );
    
    if (!isAllowed) {
      return '#';
    }
    
    return urlObj.toString();
  } catch {
    return '#';
  }
};

// Text sanitization for preventing XSS in displayed content
const sanitizeText = (text: string): string => {
  return text
    .replace(/[<>'"&]/g, (match) => {
      const entities: { [key: string]: string } = {
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#x27;',
        '&': '&amp;'
      };
      return entities[match] || match;
    })
    .slice(0, 500); // Limit length
};

export interface VerificationProof {
  verification_type: 'GitHub' | 'Domain' | 'Twitter' | 'Discord' | 'Telegram';
  proof_url: string;
  verified_at: bigint;
  verification_method: 'Automated' | 'CommunityVote' | 'ProofVisible';
  challenge_data?: string;
  status: 'Active' | 'Removed' | 'Disputed';
}

interface VerificationProofDisplayProps {
  proofs: VerificationProof[];
  companyId: string;
  companyName: string;
}

const VerificationProofDisplay: React.FC<VerificationProofDisplayProps> = ({ 
  proofs, 
  companyId, 
  companyName 
}) => {
  const formatVerificationDate = (timestamp: bigint) => {
    return new Date(Number(timestamp) / 1_000_000).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'Active':
        return '✅';
      case 'Removed':
        return '❌';
      case 'Disputed':
        return '⚠️';
      default:
        return '❓';
    }
  };

  const getPlatformIcon = (platform: string) => {
    switch (platform) {
      case 'Twitter':
        return '🐦';
      case 'Discord':
        return '💬';
      case 'Telegram':
        return '📱';
      case 'GitHub':
        return '🐙';
      case 'Domain':
        return '🌐';
      default:
        return '🔗';
    }
  };

  const getVerificationMethodBadge = (method: string) => {
    const badges = {
      'Automated': { text: 'Auto-Verified', class: 'automated' },
      'CommunityVote': { text: 'Community Verified', class: 'community' },
      'ProofVisible': { text: 'Proof-Based', class: 'proof-based' }
    };
    return badges[method as keyof typeof badges] || { text: method, class: 'default' };
  };

  if (!proofs || proofs.length === 0) {
    return (
      <div className="verification-proofs empty">
        <h3>🔗 Verification Proofs</h3>
        <div className="empty-state">
          <p>No verification proofs available yet.</p>
          <small>Verification proofs will appear here once the company completes social media verification.</small>
        </div>
      </div>
    );
  }

  return (
    <div className="verification-proofs">
      <div className="proofs-header">
        <h3>🔗 Verification Proofs</h3>
        <div className="trust-indicator">
          <span className="indicator-icon">🛡️</span>
          <span className="indicator-text">
            All verification links are permanently stored and publicly auditable
          </span>
        </div>
      </div>

      <div className="transparency-notice">
        <div className="notice-content">
          <strong>🔍 Transparency Guarantee:</strong> These verification proofs are permanently linked 
          to {companyName}&apos;s profile. If any original posts are deleted, the community will be notified 
          and the company&apos;s trust score may be affected.
        </div>
      </div>

      <div className="proofs-list">
        {proofs.map((proof, index) => {
          const methodBadge = getVerificationMethodBadge(proof.verification_method);
          
          return (
            <div key={index} className={`proof-item status-${proof.status.toLowerCase()}`}>
              <div className="proof-header">
                <div className="platform-info">
                  <span className="platform-icon">{getPlatformIcon(proof.verification_type)}</span>
                  <span className="platform-name">{proof.verification_type}</span>
                  <span className={`method-badge ${methodBadge.class}`}>
                    {methodBadge.text}
                  </span>
                </div>
                <div className="verification-date">
                  <small>Verified: {formatVerificationDate(proof.verified_at)}</small>
                </div>
              </div>

              <div className="proof-content">
                <div className="proof-link-container">
                  {(() => {
                    const sanitizedUrl = sanitizeUrl(proof.proof_url);
                    const isValidUrl = sanitizedUrl !== '#';
                    
                    return isValidUrl ? (
                      <a 
                        href={sanitizedUrl}
                        target="_blank" 
                        rel="noopener noreferrer"
                        className="proof-link"
                      >
                        <span className="link-icon">🔗</span>
                        <span className="link-text">View Original Verification Post</span>
                        <span className="external-icon">↗</span>
                      </a>
                    ) : (
                      <div className="invalid-link">
                        <span className="warning-icon">⚠️</span>
                        <span className="warning-text">Invalid or unsafe URL detected</span>
                      </div>
                    );
                  })()}
                  <div className="status-indicator">
                    <span className="status-icon">{getStatusIcon(proof.status)}</span>
                    <span className={`status-text status-${proof.status.toLowerCase()}`}>
                      {proof.status}
                    </span>
                  </div>
                </div>

                {proof.status === 'Removed' && (
                  <div className="warning-message">
                    <span className="warning-icon">⚠️</span>
                    <span className="warning-text">
                      <strong>Warning:</strong> Original post was deleted after verification. 
                      This may indicate suspicious behavior. Community review has been triggered.
                    </span>
                  </div>
                )}

                {proof.status === 'Disputed' && (
                  <div className="disputed-message">
                    <span className="disputed-icon">🔍</span>
                    <span className="disputed-text">
                      <strong>Under Review:</strong> This verification proof has been flagged 
                      by the community and is currently under investigation.
                    </span>
                  </div>
                )}

                {proof.challenge_data && (
                  <div className="challenge-data">
                    <div className="challenge-label">Required Verification Text:</div>
                    <div className="challenge-text">"{sanitizeText(proof.challenge_data)}"</div>
                    <small className="challenge-note">
                      The original post must contain this exact text to maintain verification status.
                    </small>
                  </div>
                )}
              </div>

              <div className="proof-footer">
                <div className="proof-url">
                  <small className="url-label">Proof URL:</small>
                  <code className="url-text">{sanitizeText(proof.proof_url)}</code>
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <div className="community-actions">
        <div className="actions-header">
          <h4>Community Monitoring</h4>
          <p>Help maintain trust by monitoring verification proofs</p>
        </div>
        <div className="action-buttons">
          <button 
            className="action-button verify-button"
            onClick={() => {
              // TODO: Implement proof verification check
              console.log('Checking proof status...');
            }}
          >
            <span className="button-icon">🔍</span>
            Check Proof Status
          </button>
          <button 
            className="action-button report-button"
            onClick={() => {
              // TODO: Implement reporting functionality
              console.log('Report verification issue...');
            }}
          >
            <span className="button-icon">🚨</span>
            Report Issue
          </button>
        </div>
        <small className="actions-note">
          Community members can verify that proof links are still active and report any suspicious activity.
        </small>
      </div>
    </div>
  );
};

export default VerificationProofDisplay;