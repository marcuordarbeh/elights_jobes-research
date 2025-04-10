import React, { useState } from 'react';
import axios from 'axios';

const AchPayment: React.FC = () => {
  const [message, setMessage] = useState('');
  const [loading, setLoading] = useState(false);

  const generateAchDetails = async () => {
    setLoading(true);
    setMessage('');
    try {
      // Call the backend endpoint for ACH generation.
      const response = await axios.post<{ message: string }>('/api/generate_ach');
      setMessage(response.data.message);
    } catch (error: any) {
      console.error('ACH generation failed', error);
      setMessage('Failed to generate ACH details');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Generate ACH Payment Details</h2>
      <button
        onClick={generateAchDetails}
        className="w-full bg-purple-500 text-white p-2 rounded hover:bg-purple-600"
        disabled={loading}
      >
        {loading ? 'Generating...' : 'Generate ACH Details'}
      </button>
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default AchPayment;
