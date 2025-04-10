import React from 'react';
import AchPayment from '../components/AchPayment';
import WireTransfer from '../components/WireTransfer';
import Payment from '../components/Payment';
import CryptoConversion from '../components/CryptoConversion';

const PaymentPage: React.FC = () => {
  return (
    <div className="space-y-10 p-8">
      <h1 className="text-3xl font-bold text-center">Anonymous Payment Gateway</h1>
      <AchPayment />
      <WireTransfer />
      <Payment />
      <CryptoConversion />
    </div>
  );
};

export default PaymentPage;
