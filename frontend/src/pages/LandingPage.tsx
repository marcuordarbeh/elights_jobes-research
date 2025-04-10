import React from 'react';
import { Link } from 'react-router-dom';

const LandingPage: React.FC = () => {
  return (
    <div
      className="min-h-screen bg-cover bg-center flex flex-col items-center justify-center"
      style={{ backgroundImage: "url('/images/landing.png')" }}
    >
      <h1 className="text-5xl text-white font-bold mb-8 drop-shadow-lg">
        Welcome to Payment System
      </h1>
      <div className="flex space-x-6">
        <Link
          to="/dashboard"
          className="px-8 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition shadow"
        >
          Dashboard
        </Link>
      </div>
    </div>
  );
};

export default LandingPage;
