import React, { useState } from 'react';
import axios from 'axios';

const WireTransfer: React.FC = () => {
  const [message, setMessage] = useState('');
  const [loading, setLoading] = useState(false);

  const handleWireTransfer = async () => {
    setLoading(true);
    setMessage('');
    try {
      // Call the backend endpoint for wire transfer generation.
      const response = await axios.post<{ message: string }>('/api/receive_bank_transfer');
      setMessage(response.data.message);
    } catch (error: any) {
      console.error('Wire transfer failed', error);
      setMessage('Wire transfer processing failed');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Generate Wire Transfer Details</h2>
      <button
        onClick={handleWireTransfer}
        className="w-full bg-yellow-500 text-white p-2 rounded hover:bg-yellow-600"
        disabled={loading}
      >
        {loading ? 'Processing...' : 'Generate Wire Transfer Details'}
      </button>
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default WireTransfer;
