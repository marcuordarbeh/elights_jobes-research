import React from 'react';
import { Link } from 'react-router-dom';

const Dashboard: React.FC = () => {
  return (
    <div className="container mx-auto p-4">
      <h2 className="text-2xl font-bold mb-4">Dashboard</h2>
      <div className="flex flex-col space-y-2">
        <Link to="/card" className="text-blue-500 hover:underline">
          Process Card Payment
        </Link>
        <Link to="/ach" className="text-blue-500 hover:underline">
          Generate ACH Payment
        </Link>
        <Link to="/wire" className="text-blue-500 hover:underline">
          Process Wire Transfer
        </Link>
        <Link to="/crypto" className="text-blue-500 hover:underline">
          Convert to Crypto Wallet
        </Link>
      </div>
    </div>
  );
};

export default Dashboard;
