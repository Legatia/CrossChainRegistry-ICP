import React from 'react';
import CompanyListing from '../components/company/CompanyListing';
import './CompanyListingPage.scss';

const CompanyListingPage: React.FC = () => {
  return (
    <div className="company-listing-page">
      <div className="company-listing-page__container">
        <CompanyListing />
      </div>
    </div>
  );
};

export default CompanyListingPage;