import React, { useState } from 'react';
import axios from 'axios';

const CryptoConversion: React.FC = () => {
  const [cryptoWallet, setCryptoWallet] = useState('');
  const [amount, setAmount] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleConversion = async () => {
    setLoading(true);
    setMessage('');
    try {
      // The backend should perform the conversion and return a crypto wallet address
      const response = await axios.post('/api/convert_to_crypto', { amount });
      const { walletAddress } = response.data;
      setCryptoWallet(walletAddress);
      setMessage('Conversion successful');
    } catch (error: any) {
      console.error('Crypto conversion failed', error);
      setMessage('Crypto conversion failed');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Convert to Crypto Wallet</h2>
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
      {cryptoWallet && (
        <div className="mt-4">
          <p>
            <strong>Crypto Wallet Address:</strong> {cryptoWallet}
          </p>
        </div>
      )}
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default CryptoConversion;
