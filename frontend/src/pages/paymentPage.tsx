import React, { useEffect, useState } from "react";
import axios from "axios";

interface ACHResponse {
  message: string;
}

interface WireResponse {
  message: string;
}

interface CardResponse {
  card: any;
  monero_conversion: any;
  status: string;
}

interface CryptoResponse {
  status: string;
}

const PaymentPage: React.FC = () => {
  const [achMsg, setAchMsg] = useState("");
  const [wireMsg, setWireMsg] = useState("");
  const [cardResult, setCardResult] = useState<CardResponse | null>(null);
  const [cryptoMsg, setCryptoMsg] = useState("");

  useEffect(() => {
    // Generate ACH details
    axios.post<ACHResponse>("/api/generate_ach")
      .then(response => setAchMsg(response.data.message))
      .catch(err => console.error(err));

    // Generate Wire transfer details
    axios.post<WireResponse>("/api/receive_bank_transfer")
      .then(response => setWireMsg(response.data.message))
      .catch(err => console.error(err));

    // Process a sample card transaction (simulate a debit card transaction for $10.00)
    axios.post<CardResponse>("/api/process_card", {
      card_number: "4111111111111111",
      expiry_date: "12/30",
      cvv: "123"
    }).then(response => setCardResult(response.data))
      .catch(err => console.error(err));

    // Trigger fiat-to-Monero conversion (this endpoint returns a status message)
    axios.post<CryptoResponse>("/api/convert_to_crypto")
      .then(response => setCryptoMsg(response.data.status))
      .catch(err => console.error(err));
  }, []);

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-6">Anonymous Payment Gateway</h1>

      <section className="mb-8">
        <h2 className="text-xl font-semibold mb-2">ACH Generation</h2>
        <p>{achMsg}</p>
      </section>

      <section className="mb-8">
        <h2 className="text-xl font-semibold mb-2">Wire Transfer Generation</h2>
        <p>{wireMsg}</p>
      </section>

      <section className="mb-8">
        <h2 className="text-xl font-semibold mb-2">Card Transaction Processing</h2>
        <pre className="bg-gray-100 p-4 rounded">
          {cardResult ? JSON.stringify(cardResult, null, 2) : "Processing..."}
        </pre>
      </section>

      <section className="mb-8">
        <h2 className="text-xl font-semibold mb-2">Fiat-to-Monero Conversion</h2>
        <p>{cryptoMsg}</p>
      </section>
    </div>
  );
};

export default PaymentPage;

// paymentPage.tsx - Combines ACH, Wire Transfer, Card Processing, and Crypto Conversion.
// import React from 'react';
// import AchPayment from '../components/AchPayment';
// import WireTransfer from '../components/WireTransfer';
// import Payment from '../components/Payment';
// import CryptoConversion from '../components/CryptoConversion';

// const PaymentPage: React.FC = () => {
//   return (
//     <div className="space-y-10 p-8">
//       <h1 className="text-3xl font-bold text-center">Anonymous Payment Gateway</h1>
//       <AchPayment />
//       <WireTransfer />
//       <Payment />
//       <CryptoConversion />
//     </div>
//   );
// };

// export default PaymentPage;
