import React from 'react';
import { Link } from 'react-router-dom';

const Dashboard: React.FC = () => {
  return (
    <div className="p-8">
      <h2 className="text-2xl font-bold mb-4">Dashboard</h2>
      <div className="space-y-4">
        <Link to="/payment" className="text-blue-500 underline block">
          Card Payment
        </Link>
        <Link to="/ach" className="text-blue-500 underline block">
          Generate ACH Details
        </Link>
        <Link to="/wire" className="text-blue-500 underline block">
          Generate Wire Transfer Details
        </Link>
        <Link to="/crypto" className="text-blue-500 underline block">
          Convert Fiat to Monero
        </Link>
      </div>
    </div>
  );
};

export default Dashboard;
