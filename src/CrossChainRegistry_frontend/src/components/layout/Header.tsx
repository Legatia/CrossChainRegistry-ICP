import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import './Header.scss';

const Header: React.FC = () => {
  const location = useLocation();

  const isActivePath = (path: string) => {
    return location.pathname === path;
  };

  return (
    <header className="header">
      <div className="header__container">
        <Link to="/" className="header__logo">
          <span className="header__logo-text">CrossChain Registry</span>
        </Link>
        
        <nav className="header__nav">
          <Link 
            to="/" 
            className={`header__nav-link ${isActivePath('/') ? 'active' : ''}`}
          >
            Home
          </Link>
          <Link 
            to="/companies" 
            className={`header__nav-link ${isActivePath('/companies') ? 'active' : ''}`}
          >
            Browse Companies
          </Link>
          <Link 
            to="/register" 
            className={`header__nav-link ${isActivePath('/register') ? 'active' : ''}`}
          >
            Register Company
          </Link>
          <Link 
            to="/dashboard" 
            className={`header__nav-link ${isActivePath('/dashboard') ? 'active' : ''}`}
          >
            Dashboard
          </Link>
        </nav>
      </div>
    </header>
  );
};

export default Header;