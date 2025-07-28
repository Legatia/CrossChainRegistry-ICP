import React, { useState } from 'react';
import { registryApi } from '../../services/api';
import { Company, EndorsementFormData, TestimonialFormData, VouchFormData } from '../../types';
import { 
  sanitizeInput, 
  validateForm, 
  defaultRateLimiter,
  INPUT_LIMITS 
} from '../../utils/validation';
import './CommunityValidation.scss';

interface CommunityValidationProps {
  company: Company;
  onUpdate: () => void;
}

const CommunityValidation: React.FC<CommunityValidationProps> = ({ company, onUpdate }) => {
  const [activeTab, setActiveTab] = useState<'endorsements' | 'testimonials' | 'vouches'>('endorsements');
  const [showAddForm, setShowAddForm] = useState(false);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  // Form states
  const [endorsementForm, setEndorsementForm] = useState<EndorsementFormData>({
    company_id: company.id,
    message: ''
  });

  const [testimonialForm, setTestimonialForm] = useState<TestimonialFormData>({
    company_id: company.id,
    author_name: '',
    role: '',
    message: ''
  });

  const [vouchForm, setVouchForm] = useState<VouchFormData>({
    company_id: company.id,
    message: ''
  });

  const formatDate = (timestamp: bigint) => {
    return new Date(Number(timestamp) / 1_000_000).toLocaleDateString();
  };

  const handleAddEndorsement = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Rate limiting check
    if (!defaultRateLimiter.canProceed('endorsement_submission')) {
      setError('Too many submission attempts. Please wait before trying again.');
      return;
    }
    
    // Validate endorsement
    const validation = validateForm.endorsement(endorsementForm.message);
    if (!validation.valid) {
      setValidationErrors({ message: validation.error! });
      setError('Please fix the validation errors.');
      return;
    }
    
    // Sanitize input
    const sanitizedForm = {
      ...endorsementForm,
      message: sanitizeInput.text(endorsementForm.message)
    };
    
    setLoading(true);
    setError(null);
    setValidationErrors({});

    try {
      const result = await registryApi.addEndorsement(sanitizedForm);
      if ('Ok' in result) {
        setEndorsementForm({ company_id: company.id, message: '' });
        setShowAddForm(false);
        onUpdate();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError(`Failed to add endorsement: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleAddTestimonial = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Rate limiting check
    if (!defaultRateLimiter.canProceed('testimonial_submission')) {
      setError('Too many submission attempts. Please wait before trying again.');
      return;
    }
    
    // Validate testimonial
    const validation = validateForm.testimonial(testimonialForm);
    if (!validation.valid) {
      setValidationErrors(validation.errors);
      setError('Please fix the validation errors.');
      return;
    }
    
    // Sanitize input
    const sanitizedForm = {
      ...testimonialForm,
      author_name: sanitizeInput.text(testimonialForm.author_name),
      role: sanitizeInput.text(testimonialForm.role),
      message: sanitizeInput.text(testimonialForm.message)
    };
    
    setLoading(true);
    setError(null);
    setValidationErrors({});

    try {
      const result = await registryApi.addTestimonial(sanitizedForm);
      if ('Ok' in result) {
        setTestimonialForm({ 
          company_id: company.id, 
          author_name: '', 
          role: '', 
          message: '' 
        });
        setShowAddForm(false);
        onUpdate();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError(`Failed to add testimonial: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleAddVouch = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Rate limiting check
    if (!defaultRateLimiter.canProceed('vouch_submission')) {
      setError('Too many submission attempts. Please wait before trying again.');
      return;
    }
    
    // Validate vouch
    const validation = validateForm.vouch(vouchForm.message);
    if (!validation.valid) {
      setValidationErrors({ message: validation.error! });
      setError('Please fix the validation errors.');
      return;
    }
    
    // Sanitize input
    const sanitizedForm = {
      ...vouchForm,
      message: sanitizeInput.text(vouchForm.message)
    };
    
    setLoading(true);
    setError(null);
    setValidationErrors({});

    try {
      const result = await registryApi.addVouch(sanitizedForm);
      if ('Ok' in result) {
        setVouchForm({ company_id: company.id, message: '' });
        setShowAddForm(false);
        onUpdate();
      } else {
        setError(result.Err);
      }
    } catch (err) {
      setError(`Failed to add vouch: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const renderEndorsements = () => (
    <div className="validation-content">
      <div className="validation-header">
        <h3>Peer Endorsements ({company.community_validation.peer_endorsements.length})</h3>
        <button 
          onClick={() => setShowAddForm(!showAddForm)}
          className="add-button"
        >
          {showAddForm ? 'Cancel' : 'Add Endorsement'}
        </button>
      </div>

      {showAddForm && (
        <form onSubmit={handleAddEndorsement} className="add-form">
          <div className="form-group">
            <label>Endorsement Message</label>
            <textarea
              value={endorsementForm.message}
              onChange={(e) => setEndorsementForm(prev => ({ ...prev, message: sanitizeInput.text(e.target.value) }))}
              placeholder="Write your endorsement for this company..."
              minLength={INPUT_LIMITS.ENDORSEMENT_MESSAGE.min}
              maxLength={INPUT_LIMITS.ENDORSEMENT_MESSAGE.max}
              required
              rows={4}
            />
            {validationErrors.message && (
              <div className="field-error">{validationErrors.message}</div>
            )}
          </div>
          <div className="form-actions">
            <button type="submit" disabled={loading}>
              {loading ? 'Adding...' : 'Add Endorsement'}
            </button>
          </div>
        </form>
      )}

      <div className="validation-list">
        {company.community_validation.peer_endorsements.length === 0 ? (
          <p className="empty-state">No endorsements yet. Be the first to endorse this company!</p>
        ) : (
          company.community_validation.peer_endorsements.map((endorsement, index) => (
            <div key={index} className="validation-item">
              <div className="validation-meta">
                <span className="validation-author">Company: {endorsement.endorser_company_id}</span>
                <span className="validation-date">{formatDate(endorsement.timestamp)}</span>
              </div>
              <p className="validation-message">{endorsement.message}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );

  const renderTestimonials = () => (
    <div className="validation-content">
      <div className="validation-header">
        <h3>Employee Testimonials ({company.community_validation.employee_testimonials.length})</h3>
        <button 
          onClick={() => setShowAddForm(!showAddForm)}
          className="add-button"
        >
          {showAddForm ? 'Cancel' : 'Add Testimonial'}
        </button>
      </div>

      {showAddForm && (
        <form onSubmit={handleAddTestimonial} className="add-form">
          <div className="form-row">
            <div className="form-group">
              <label>Your Name</label>
              <input
                type="text"
                value={testimonialForm.author_name}
                onChange={(e) => setTestimonialForm(prev => ({ ...prev, author_name: sanitizeInput.text(e.target.value) }))}
                minLength={INPUT_LIMITS.AUTHOR_NAME.min}
                maxLength={INPUT_LIMITS.AUTHOR_NAME.max}
                required
              />
              {validationErrors.author_name && (
                <div className="field-error">{validationErrors.author_name}</div>
              )}
            </div>
            <div className="form-group">
              <label>Your Role</label>
              <input
                type="text"
                value={testimonialForm.role}
                onChange={(e) => setTestimonialForm(prev => ({ ...prev, role: sanitizeInput.text(e.target.value) }))}
                placeholder="e.g., Former Developer, Current Marketing Lead"
                minLength={INPUT_LIMITS.TEAM_MEMBER_ROLE.min}
                maxLength={INPUT_LIMITS.TEAM_MEMBER_ROLE.max}
                required
              />
              {validationErrors.role && (
                <div className="field-error">{validationErrors.role}</div>
              )}
            </div>
          </div>
          <div className="form-group">
            <label>Testimonial</label>
            <textarea
              value={testimonialForm.message}
              onChange={(e) => setTestimonialForm(prev => ({ ...prev, message: sanitizeInput.text(e.target.value) }))}
              placeholder="Share your experience working with this company..."
              minLength={INPUT_LIMITS.TESTIMONIAL_MESSAGE.min}
              maxLength={INPUT_LIMITS.TESTIMONIAL_MESSAGE.max}
              required
              rows={4}
            />
            {validationErrors.message && (
              <div className="field-error">{validationErrors.message}</div>
            )}
          </div>
          <div className="form-actions">
            <button type="submit" disabled={loading}>
              {loading ? 'Adding...' : 'Add Testimonial'}
            </button>
          </div>
        </form>
      )}

      <div className="validation-list">
        {company.community_validation.employee_testimonials.length === 0 ? (
          <p className="empty-state">No testimonials yet. Share your experience working with this company!</p>
        ) : (
          company.community_validation.employee_testimonials.map((testimonial, index) => (
            <div key={index} className="validation-item">
              <div className="validation-meta">
                <span className="validation-author">
                  {testimonial.author_name} - {testimonial.role}
                </span>
                <span className="validation-date">{formatDate(testimonial.timestamp)}</span>
                <span className={`verification-badge ${testimonial.verified ? 'verified' : 'unverified'}`}>
                  {testimonial.verified ? '✓ Verified' : 'Unverified'}
                </span>
              </div>
              <p className="validation-message">{testimonial.message}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );

  const renderVouches = () => (
    <div className="validation-content">
      <div className="validation-header">
        <h3>Community Vouches ({company.community_validation.community_vouches.length})</h3>
        <button 
          onClick={() => setShowAddForm(!showAddForm)}
          className="add-button"
        >
          {showAddForm ? 'Cancel' : 'Add Vouch'}
        </button>
      </div>

      {showAddForm && (
        <form onSubmit={handleAddVouch} className="add-form">
          <div className="form-group">
            <label>Vouch Message</label>
            <textarea
              value={vouchForm.message}
              onChange={(e) => setVouchForm(prev => ({ ...prev, message: sanitizeInput.text(e.target.value) }))}
              placeholder="Vouch for this company based on your interactions or knowledge..."
              minLength={INPUT_LIMITS.VOUCH_MESSAGE.min}
              maxLength={INPUT_LIMITS.VOUCH_MESSAGE.max}
              required
              rows={4}
            />
            {validationErrors.message && (
              <div className="field-error">{validationErrors.message}</div>
            )}
          </div>
          <div className="form-actions">
            <button type="submit" disabled={loading}>
              {loading ? 'Adding...' : 'Add Vouch'}
            </button>
          </div>
        </form>
      )}

      <div className="validation-list">
        {company.community_validation.community_vouches.length === 0 ? (
          <p className="empty-state">No vouches yet. Vouch for this company if you trust them!</p>
        ) : (
          company.community_validation.community_vouches.map((vouch, index) => (
            <div key={index} className="validation-item">
              <div className="validation-meta">
                <span className="validation-author">Community Member</span>
                <span className="validation-date">{formatDate(vouch.timestamp)}</span>
                <span className="vouch-weight">Weight: {vouch.weight}</span>
              </div>
              <p className="validation-message">{vouch.message}</p>
            </div>
          ))
        )}
      </div>
    </div>
  );

  return (
    <section className="community-validation">
      <div className="section-header">
        <h2>Community Validation</h2>
        <div className="reputation-score">
          <span className="score-label">Reputation Score:</span>
          <span className="score-value">{company.community_validation.reputation_score}</span>
        </div>
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      <div className="validation-summary">
        <div className="summary-stats">
          <div className="stat">
            <span className="stat-value">{company.community_validation.peer_endorsements.length}</span>
            <span className="stat-label">Endorsements</span>
          </div>
          <div className="stat">
            <span className="stat-value">{company.community_validation.employee_testimonials.length}</span>
            <span className="stat-label">Testimonials</span>
          </div>
          <div className="stat">
            <span className="stat-value">{company.community_validation.community_vouches.length}</span>
            <span className="stat-label">Vouches</span>
          </div>
          <div className="stat">
            <span className="stat-value">{Number(company.community_validation.reputation_staked)}</span>
            <span className="stat-label">Tokens Staked</span>
          </div>
        </div>
      </div>

      <div className="validation-tabs">
        <button 
          onClick={() => { setActiveTab('endorsements'); setShowAddForm(false); }}
          className={`tab-button ${activeTab === 'endorsements' ? 'active' : ''}`}
        >
          Endorsements
        </button>
        <button 
          onClick={() => { setActiveTab('testimonials'); setShowAddForm(false); }}
          className={`tab-button ${activeTab === 'testimonials' ? 'active' : ''}`}
        >
          Testimonials
        </button>
        <button 
          onClick={() => { setActiveTab('vouches'); setShowAddForm(false); }}
          className={`tab-button ${activeTab === 'vouches' ? 'active' : ''}`}
        >
          Vouches
        </button>
      </div>

      <div className="validation-tab-content">
        {activeTab === 'endorsements' && renderEndorsements()}
        {activeTab === 'testimonials' && renderTestimonials()}
        {activeTab === 'vouches' && renderVouches()}
      </div>
    </section>
  );
};

export default CommunityValidation;