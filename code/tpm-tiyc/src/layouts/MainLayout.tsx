import React from 'react';
import { Outlet } from 'react-router-dom';

export const MainLayout: React.FC = () => {
  return (
    <div className="min-h-screen bg-background text-text-main flex flex-col items-center justify-center p-6">
      <div className="w-full max-w-5xl mx-auto flex flex-col">
        {/* We can add a header or global nav here if needed later */}
        <main className="flex-1 w-full animate-in fade-in zoom-in-95 duration-300">
          <Outlet />
        </main>
      </div>
    </div>
  );
};
