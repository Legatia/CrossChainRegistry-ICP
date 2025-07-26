import React, { useState } from 'react';
import { SearchCompaniesRequest, CompanyStatus } from '../../types';
import './SearchFilters.scss';

interface SearchFiltersProps {
  searchRequest: SearchCompaniesRequest;
  onSearchChange: (request: SearchCompaniesRequest) => void;
  focusAreaOptions: string[];
}

const SearchFilters: React.FC<SearchFiltersProps> = ({
  searchRequest,
  onSearchChange,
  focusAreaOptions
}) => {
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [localQuery, setLocalQuery] = useState(searchRequest.query || '');

  const handleQuerySubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearchChange({
      ...searchRequest,
      query: localQuery.trim() || undefined
    });
  };

  const handleFilterChange = (field: keyof SearchCompaniesRequest, value: any) => {
    onSearchChange({
      ...searchRequest,
      [field]: value
    });
  };

  const handleFocusAreaToggle = (area: string) => {
    const currentAreas = searchRequest.focus_areas || [];
    const newAreas = currentAreas.includes(area)
      ? currentAreas.filter(a => a !== area)
      : [...currentAreas, area];
    
    handleFilterChange('focus_areas', newAreas.length > 0 ? newAreas : undefined);
  };

  const clearAllFilters = () => {
    setLocalQuery('');
    onSearchChange({
      page: 0,
      limit: searchRequest.limit
    });
  };

  const hasActiveFilters = 
    searchRequest.query ||
    searchRequest.focus_areas?.length ||
    searchRequest.status_filter ||
    searchRequest.min_verification_score ||
    searchRequest.has_github ||
    searchRequest.has_contracts ||
    searchRequest.domain_verified;

  return (
    <div className="search-filters">
      <form onSubmit={handleQuerySubmit} className="search-filters__main">
        <div className="search-input-group">
          <input
            type="text"
            placeholder="Search companies by name, description, or focus areas..."
            value={localQuery}
            onChange={(e) => setLocalQuery(e.target.value)}
            className="search-input"
          />
          <button type="submit" className="search-button">
            Search
          </button>
        </div>
      </form>

      <div className="search-filters__controls">
        <button 
          onClick={() => setShowAdvanced(!showAdvanced)}
          className="advanced-toggle"
        >
          {showAdvanced ? 'Hide' : 'Show'} Advanced Filters
        </button>
        
        {hasActiveFilters && (
          <button onClick={clearAllFilters} className="clear-filters">
            Clear All Filters
          </button>
        )}
      </div>

      {showAdvanced && (
        <div className="search-filters__advanced">
          {/* Status Filter */}
          <div className="filter-group">
            <label>Company Status</label>
            <select 
              value={searchRequest.status_filter || ''}
              onChange={(e) => handleFilterChange('status_filter', e.target.value || undefined)}
            >
              <option value="">All Statuses</option>
              <option value={CompanyStatus.Pending}>Pending</option>
              <option value={CompanyStatus.Verified}>Verified</option>
              <option value={CompanyStatus.Trusted}>Trusted</option>
              <option value={CompanyStatus.Flagged}>Flagged</option>
            </select>
          </div>

          {/* Verification Score Filter */}
          <div className="filter-group">
            <label>Minimum Verification Score</label>
            <input
              type="number"
              min="0"
              max="100"
              placeholder="0-100"
              value={searchRequest.min_verification_score || ''}
              onChange={(e) => handleFilterChange('min_verification_score', e.target.value ? parseInt(e.target.value) : undefined)}
            />
          </div>

          {/* Boolean Filters */}
          <div className="filter-group">
            <label>Verification Requirements</label>
            <div className="checkbox-group">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={searchRequest.has_github || false}
                  onChange={(e) => handleFilterChange('has_github', e.target.checked || undefined)}
                />
                Has GitHub Organization
              </label>
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={searchRequest.has_contracts || false}
                  onChange={(e) => handleFilterChange('has_contracts', e.target.checked || undefined)}
                />
                Has Smart Contracts
              </label>
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={searchRequest.domain_verified || false}
                  onChange={(e) => handleFilterChange('domain_verified', e.target.checked || undefined)}
                />
                Domain Verified
              </label>
            </div>
          </div>

          {/* Focus Areas Filter */}
          <div className="filter-group">
            <label>Focus Areas</label>
            <div className="focus-areas-filter">
              {focusAreaOptions.map(area => (
                <label key={area} className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={searchRequest.focus_areas?.includes(area) || false}
                    onChange={() => handleFocusAreaToggle(area)}
                  />
                  {area}
                </label>
              ))}
            </div>
          </div>

          {/* Results per page */}
          <div className="filter-group">
            <label>Results per page</label>
            <select 
              value={searchRequest.limit || 20}
              onChange={(e) => handleFilterChange('limit', parseInt(e.target.value))}
            >
              <option value={10}>10</option>
              <option value={20}>20</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
            </select>
          </div>
        </div>
      )}

      {/* Active Filters Display */}
      {hasActiveFilters && (
        <div className="search-filters__active">
          <span className="active-filters-label">Active filters:</span>
          <div className="active-filters">
            {searchRequest.query && (
              <span className="filter-tag">
                Query: "{searchRequest.query}"
                <button onClick={() => handleFilterChange('query', undefined)}>×</button>
              </span>
            )}
            {searchRequest.status_filter && (
              <span className="filter-tag">
                Status: {searchRequest.status_filter}
                <button onClick={() => handleFilterChange('status_filter', undefined)}>×</button>
              </span>
            )}
            {searchRequest.min_verification_score && (
              <span className="filter-tag">
                Min Score: {searchRequest.min_verification_score}
                <button onClick={() => handleFilterChange('min_verification_score', undefined)}>×</button>
              </span>
            )}
            {searchRequest.focus_areas?.map(area => (
              <span key={area} className="filter-tag">
                {area}
                <button onClick={() => handleFocusAreaToggle(area)}>×</button>
              </span>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default SearchFilters;