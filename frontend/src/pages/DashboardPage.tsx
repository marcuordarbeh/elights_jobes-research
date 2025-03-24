import React from 'react';

const DashboardPage: React.FC = () => {
  return (
    <div className="min-h-screen relative">
      {/* Main background image */}
      <img
        src="/images/Capturelanding.JPG"
        alt="Dashboard Background"
        className="w-full h-full object-cover absolute inset-0"
      />
      {/* Overlay with semi-transparent layer */}
      <div className="absolute inset-0 bg-black opacity-50"></div>
      {/* Overlay content */}
      <div className="relative z-10 flex flex-col items-center justify-center min-h-screen text-white">
        <img src="/images/seame.png" alt="Seame" className="w-32 h-32 mb-4" />
        <img src="/images/check.png" alt="Check" className="w-16 h-16 mb-2" />
        <h1 className="text-4xl font-bold mb-4">Dashboard</h1>
        {/* Additional dashboard content */}
      </div>
    </div>
  );
};

export default DashboardPage;
