import React, { useState } from 'react';
import axios from 'axios';

const Payment: React.FC = () => {
  const [cardType, setCardType] = useState<'live' | 'virtual' | 'credit'>('live');
  const [cardNumber, setCardNumber] = useState('');
  const [expiryDate, setExpiryDate] = useState('');
  const [cvv, setCvv] = useState('');
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

  const handleProcessCard = async () => {
    setLoading(true);
    setMessage('');
    try {
      const response = await axios.post('/api/process_card', {
        card_number: cardNumber,
        expiry_date: expiryDate,
        cvv: cvv,
        card_type: cardType,
      });
      setMessage('Payment processed successfully');
      console.log('Response:', response.data);
    } catch (error: any) {
      console.error('Payment processing failed', error);
      setMessage('Payment processing failed');
    }
    setLoading(false);
  };

  return (
    <div className="max-w-md mx-auto p-4 bg-white shadow rounded">
      <h2 className="text-2xl font-bold mb-4">Card Payment</h2>
      <div className="mb-4">
        <label className="mr-4">
          <input
            type="radio"
            name="cardType"
            value="live"
            checked={cardType === 'live'}
            onChange={() => setCardType('live')}
          />
          Live Card
        </label>
        <label className="mr-4">
          <input
            type="radio"
            name="cardType"
            value="virtual"
            checked={cardType === 'virtual'}
            onChange={() => setCardType('virtual')}
          />
          Virtual Card
        </label>
        <label>
          <input
            type="radio"
            name="cardType"
            value="credit"
            checked={cardType === 'credit'}
            onChange={() => setCardType('credit')}
          />
          Credit Card
        </label>
      </div>
      <input
        type="text"
        placeholder="Card Number"
        value={cardNumber}
        onChange={e => setCardNumber(e.target.value)}
        className="w-full p-2 mb-4 border rounded"
      />
      <input
        type="text"
        placeholder="Expiry Date (MM/YY)"
        value={expiryDate}
        onChange={e => setExpiryDate(e.target.value)}
        className="w-full p-2 mb-4 border rounded"
      />
      <input
        type="text"
        placeholder="CVV"
        value={cvv}
        onChange={e => setCvv(e.target.value)}
        className="w-full p-2 mb-4 border rounded"
      />
      <button
        onClick={handleProcessCard}
        className="w-full bg-green-500 text-white p-2 rounded hover:bg-green-600"
        disabled={loading}
      >
        {loading ? 'Processing...' : 'Submit Payment'}
      </button>
      {message && <div className="mt-4 text-center">{message}</div>}
    </div>
  );
};

export default Payment;
