import React, { useState, useEffect } from 'react';
import { registryApi } from '../../services/api';
import { Company, SearchCompaniesRequest, CompanyStatus } from '../../types';
import CompanyCard from './CompanyCard';
import SearchFilters from './SearchFilters';
import './CompanyListing.scss';

const CompanyListing: React.FC = () => {
  const [companies, setCompanies] = useState<Company[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchRequest, setSearchRequest] = useState<SearchCompaniesRequest>({
    page: 0,
    limit: 20
  });
  const [totalCount, setTotalCount] = useState(0);

  const focusAreaOptions = [
    'DeFi', 'NFTs', 'Gaming', 'Infrastructure', 'DAOs', 'Metaverse',
    'Layer 2', 'Cross-Chain', 'Privacy', 'Social', 'Tools', 'Education'
  ];

  useEffect(() => {
    loadCompanies();
    loadTotalCount();
  }, [searchRequest]);

  const loadCompanies = async () => {
    try {
      setLoading(true);
      setError(null);
      
      let result: Company[];
      
      // Check if we have any search criteria
      const hasSearchCriteria = 
        searchRequest.query ||
        searchRequest.focus_areas?.length ||
        searchRequest.status_filter ||
        searchRequest.min_verification_score ||
        searchRequest.has_github ||
        searchRequest.has_contracts ||
        searchRequest.domain_verified;

      if (hasSearchCriteria) {
        result = await registryApi.searchCompanies(searchRequest);
      } else {
        result = await registryApi.listCompanies(searchRequest.page || 0, searchRequest.limit || 20);
      }
      
      setCompanies(result);
    } catch (err) {
      setError(`Failed to load companies: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const loadTotalCount = async () => {
    try {
      const count = await registryApi.getCompanyCount();
      setTotalCount(count);
    } catch (err) {
      console.error('Failed to load company count:', err);
    }
  };

  const handleSearchChange = (newSearchRequest: SearchCompaniesRequest) => {
    setSearchRequest({
      ...newSearchRequest,
      page: 0 // Reset to first page when search changes
    });
  };

  const handlePageChange = (newPage: number) => {
    setSearchRequest(prev => ({
      ...prev,
      page: newPage
    }));
  };

  const totalPages = Math.ceil(totalCount / (searchRequest.limit || 20));
  const currentPage = searchRequest.page || 0;

  if (loading && companies.length === 0) {
    return (
      <div className="company-listing">
        <div className="loading">Loading companies...</div>
      </div>
    );
  }

  return (
    <div className="company-listing">
      <div className="company-listing__header">
        <h1>Web3 Company Registry</h1>
        <p className="company-listing__subtitle">
          Discover verified Web3 companies and their cross-chain presence
        </p>
        
        <div className="company-listing__stats">
          <span className="stat">
            <span className="stat__value">{totalCount}</span>
            <span className="stat__label">Total Companies</span>
          </span>
          <span className="stat">
            <span className="stat__value">{companies.filter(c => c.status === CompanyStatus.Verified).length}</span>
            <span className="stat__label">Verified</span>
          </span>
          <span className="stat">
            <span className="stat__value">{companies.filter(c => c.status === CompanyStatus.Trusted).length}</span>
            <span className="stat__label">Trusted</span>
          </span>
        </div>
      </div>

      <SearchFilters
        searchRequest={searchRequest}
        onSearchChange={handleSearchChange}
        focusAreaOptions={focusAreaOptions}
      />

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      <div className="company-listing__content">
        {loading ? (
          <div className="loading">Searching companies...</div>
        ) : companies.length === 0 ? (
          <div className="no-results">
            <h3>No companies found</h3>
            <p>Try adjusting your search criteria or browse all companies.</p>
          </div>
        ) : (
          <>
            <div className="company-listing__results">
              <p className="results-count">
                Showing {companies.length} companies 
                {searchRequest.query && ` for "${searchRequest.query}"`}
              </p>
            </div>

            <div className="company-listing__grid">
              {companies.map(company => (
                <CompanyCard key={company.id} company={company} />
              ))}
            </div>

            {totalPages > 1 && (
              <div className="company-listing__pagination">
                <button 
                  onClick={() => handlePageChange(currentPage - 1)}
                  disabled={currentPage === 0}
                  className="pagination__button"
                >
                  Previous
                </button>
                
                <div className="pagination__info">
                  Page {currentPage + 1} of {totalPages}
                </div>
                
                <button 
                  onClick={() => handlePageChange(currentPage + 1)}
                  disabled={currentPage >= totalPages - 1}
                  className="pagination__button"
                >
                  Next
                </button>
              </div>
            )}
          </>
        )}
      </div>
    </div>
  );
};

export default CompanyListing;