import React from 'react';

interface LayoutProps {
  children: React.ReactNode;
}

/**
 * Basic layout wrapper component.
 */
export function Layout({ children }: LayoutProps) {
  return <div className="quilix-layout">{children}</div>;
}
