import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '../../../test/utils/testUtils';
import { MemoryRouter } from 'react-router-dom';
import Header from '../Header';

describe('Header', () => {
  const renderHeader = (pathname = '/') => {
    return render(<Header />, {
      wrapper: ({ children }) => (
        <MemoryRouter initialEntries={[pathname]}>
          {children}
        </MemoryRouter>
      ),
    });
  };

  it('should render the logo with correct text', () => {
    renderHeader();
    
    const logo = screen.getByRole('link', { name: /crosschain registry/i });
    expect(logo).toBeInTheDocument();
    expect(logo).toHaveAttribute('href', '/');
  });

  it('should render all navigation links', () => {
    renderHeader();

    const expectedLinks = [
      { text: 'Home', href: '/' },
      { text: 'Browse Companies', href: '/companies' },
      { text: 'Register Company', href: '/register' },
      { text: 'Dashboard', href: '/dashboard' },
    ];

    expectedLinks.forEach(({ text, href }) => {
      const link = screen.getByRole('link', { name: text });
      expect(link).toBeInTheDocument();
      expect(link).toHaveAttribute('href', href);
    });
  });

  it('should highlight the active link based on current path', () => {
    renderHeader('/companies');

    const activeLink = screen.getByRole('link', { name: 'Browse Companies' });
    const inactiveLink = screen.getByRole('link', { name: 'Home' });

    expect(activeLink).toHaveClass('active');
    expect(inactiveLink).not.toHaveClass('active');
  });

  it('should highlight home link when on root path', () => {
    renderHeader('/');

    const homeLink = screen.getByRole('link', { name: 'Home' });
    const companiesLink = screen.getByRole('link', { name: 'Browse Companies' });

    expect(homeLink).toHaveClass('active');
    expect(companiesLink).not.toHaveClass('active');
  });

  it('should highlight register link when on register path', () => {
    renderHeader('/register');

    const registerLink = screen.getByRole('link', { name: 'Register Company' });
    const homeLink = screen.getByRole('link', { name: 'Home' });

    expect(registerLink).toHaveClass('active');
    expect(homeLink).not.toHaveClass('active');
  });

  it('should highlight dashboard link when on dashboard path', () => {
    renderHeader('/dashboard');

    const dashboardLink = screen.getByRole('link', { name: 'Dashboard' });
    const homeLink = screen.getByRole('link', { name: 'Home' });

    expect(dashboardLink).toHaveClass('active');
    expect(homeLink).not.toHaveClass('active');
  });

  it('should not highlight any navigation link for unknown paths', () => {
    renderHeader('/unknown-path');

    const links = screen.getAllByRole('link').filter(link => 
      !link.textContent?.includes('CrossChain Registry')
    );

    links.forEach(link => {
      expect(link).not.toHaveClass('active');
    });
  });

  it('should have proper semantic HTML structure', () => {
    renderHeader();

    const header = screen.getByRole('banner');
    const navigation = screen.getByRole('navigation');

    expect(header).toBeInTheDocument();
    expect(navigation).toBeInTheDocument();
    expect(header).toContainElement(navigation);
  });

  it('should have accessible logo text', () => {
    renderHeader();

    const logoText = screen.getByText('CrossChain Registry');
    expect(logoText).toBeInTheDocument();
    expect(logoText.closest('a')).toHaveAttribute('href', '/');
  });

  it('should render with correct CSS classes', () => {
    renderHeader();

    const header = screen.getByRole('banner');
    expect(header).toHaveClass('header');

    const container = header.querySelector('.header__container');
    expect(container).toBeInTheDocument();

    const nav = screen.getByRole('navigation');
    expect(nav).toHaveClass('header__nav');
  });
});