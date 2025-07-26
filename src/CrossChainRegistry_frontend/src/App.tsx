import React from 'react';
import { Routes, Route } from 'react-router-dom';
import Header from './components/layout/Header';
import HomePage from './pages/HomePage';
import CompanyRegistrationPage from './pages/CompanyRegistrationPage';
import CompanyListingPage from './pages/CompanyListingPage';
import CompanyProfilePage from './pages/CompanyProfilePage';
import VerificationDashboardPage from './pages/VerificationDashboardPage';
import './App.scss';

const App: React.FC = () => {
  return (
    <div className="app">
      <Header />
      <main className="main-content">
        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/register" element={<CompanyRegistrationPage />} />
          <Route path="/companies" element={<CompanyListingPage />} />
          <Route path="/company/:companyId" element={<CompanyProfilePage />} />
          <Route path="/dashboard" element={<VerificationDashboardPage />} />
        </Routes>
      </main>
    </div>
  );
};

export default App;