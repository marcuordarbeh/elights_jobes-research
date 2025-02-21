import React, { useState } from 'react';
import axios from 'axios';

const Payment = () => {
  const [cardNumber, setCardNumber] = useState('');
  const [expiryDate, setExpiryDate] = useState('');
  const [cvv, setCvv] = useState('');

  const handleProcessCard = async () => {
    try {
      await axios.post('/api/process_card', {
        card_number: cardNumber,
        expiry_date: expiryDate,
        cvv
      });
      alert('Payment processed successfully');
    } catch (error) {
      console.error('Payment processing failed', error);
    }
  };

  return (
    <div>
      <h2>Process Payment</h2>
      <input
        type="text"
        placeholder="Card Number"
        value={cardNumber}
        onChange={(e) => setCardNumber(e.target.value)}
      />
      <input
        type="text"
        placeholder="Expiry Date"
        value={expiryDate}
        onChange={(e) => setExpiryDate(e.target.value)}
      />
      <input
        type="