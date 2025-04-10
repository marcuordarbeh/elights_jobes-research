import React, { useState } from 'react';
import axios from 'axios';

const CryptoConversion: React.FC = () => {
  const [amount, setAmount] = useState('');
  const [walletAddress, setWalletAddress] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleConversion = async () => {
    setLoading(true);
    setMessage('');
    try {
      const response = await axios.post<{ status: string; result?: any }>('/api/convert_to_crypto', { amount });
      setMessage(response.data.status);
      if (response.data.result && response.data.result.walletAddress) {
        setWalletAddress(response.data.result.walletAddress);
      }
    } catch (error: any) {
      console.error('Crypto conversion failed', error);
      setMessage('Crypto conversion failed');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Convert Fiat to Monero</h2>
      <input
        type="text"
        placeholder="Amount in USD"
        value={amount}
        onChange={e => setAmount(e.target.value)}
        className="w-full p-2 mb-4 border rounded"
      />
      <button
        onClick={handleConversion}
        className="w-full bg-red-500 text-white p-2 rounded hover:bg-red-600"
        disabled={loading}
      >
        {loading ? 'Converting...' : 'Convert'}
      </button>
      {walletAddress && (
        <div className="mt-4">
          <p>
            <strong>Monero Wallet Address:</strong> {walletAddress}
          </p>
        </div>
      )}
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default CryptoConversion;
