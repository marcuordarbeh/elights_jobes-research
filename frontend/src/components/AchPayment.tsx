import React, { useState } from 'react';
import axios from 'axios';

const AchPayment: React.FC = () => {
  const [accountNumber, setAccountNumber] = useState('');
  const [routingNumber, setRoutingNumber] = useState('');
  const [bankName, setBankName] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const generateAchDetails = async () => {
    setLoading(true);
    setMessage('');
    try {
      // The backend should return random ACH details
      const response = await axios.get('/api/generate_ach');
      const { accountNumber, routingNumber, bankName } = response.data;
      setAccountNumber(accountNumber);
      setRoutingNumber(routingNumber);
      setBankName(bankName);
      setMessage('ACH details generated successfully');
    } catch (error: any) {
      console.error('ACH generation failed', error);
      setMessage('Failed to generate ACH details');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Generate ACH Payment Details</h2>
      <button
        onClick={generateAchDetails}
        className="w-full bg-purple-500 text-white p-2 rounded hover:bg-purple-600"
        disabled={loading}
      >
        {loading ? 'Generating...' : 'Generate ACH Details'}
      </button>
      {accountNumber && (
        <div className="mt-4">
          <p>
            <strong>Account Number:</strong> {accountNumber}
          </p>
          <p>
            <strong>Routing Number:</strong> {routingNumber}
          </p>
          <p>
            <strong>Bank Name:</strong> {bankName}
          </p>
        </div>
      )}
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default AchPayment;
