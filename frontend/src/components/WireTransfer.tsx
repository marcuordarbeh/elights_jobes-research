// WireTransfer.tsx - Component to generate random wire transfer details.
import React, { useState } from 'react';
import axios from 'axios';

const WireTransfer: React.FC = () => {
  const [details, setDetails] = useState<{ accountNumber?: string; routingNumber?: string }>({});
  const [loading, setLoading] = useState(false);
  const [message, setMessage] = useState('');

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
    <div className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded">
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

// import React, { useState } from 'react';
// import axios from 'axios';

// const WireTransfer: React.FC = () => {
//   const [bank, setBank] = useState('Bank of America');
//   const [accountNumber, setAccountNumber] = useState('');
//   const [routingNumber, setRoutingNumber] = useState('');
//   const [loading, setLoading] = useState(false);
//   const [message, setMessage] = useState('');

//   const banks = ['Bank of America', 'Chase', 'Wells Fargo', 'Citibank'];

//   const handleWireTransfer = async () => {
//     setLoading(true);
//     setMessage('');
//     try {
//       // The backend should return bank-specific transfer details
//       const response = await axios.post('/api/wire_transfer', { bank });
//       const { accountNumber, routingNumber } = response.data;
//       setAccountNumber(accountNumber);
//       setRoutingNumber(routingNumber);
//       setMessage('Wire transfer details generated successfully');
//     } catch (error: any) {
//       console.error('Wire transfer failed', error);
//       setMessage('Wire transfer processing failed');
//     }
//     setLoading(false);
//   };

//   return (
//     <div className="max-w-md mx-auto mt-10 p-4 bg-white shadow rounded">
//       <h2 className="text-2xl font-bold mb-4">Process Wire Transfer</h2>
//       <div className="mb-4">
//         <label className="block mb-2 font-medium">Select Bank</label>
//         <select
//           value={bank}
//           onChange={e => setBank(e.target.value)}
//           className="w-full p-2 border rounded"
//         >
//           {banks.map(b => (
//             <option key={b} value={b}>
//               {b}
//             </option>
//           ))}
//         </select>
//       </div>
//       <button
//         onClick={handleWireTransfer}
//         className="w-full bg-yellow-500 text-white p-2 rounded hover:bg-yellow-600"
//         disabled={loading}
//       >
//         {loading ? 'Processing...' : 'Generate Wire Transfer Details'}
//       </button>
//       {accountNumber && (
//         <div className="mt-4">
//           <p>
//             <strong>Account Number:</strong> {accountNumber}
//           </p>
//           <p>
//             <strong>Routing Number:</strong> {routingNumber}
//           </p>
//         </div>
//       )}
//       {message && <div className="mt-4 text-center">{message}</div>}
//     </div>
//   );
// };

// export default WireTransfer;
