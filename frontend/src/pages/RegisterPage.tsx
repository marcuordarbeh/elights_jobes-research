import React from 'react';
import Register from '../components/Register';

const RegisterPage: React.FC = () => {
  return (
    <div
      className="min-h-screen flex items-center justify-center bg-cover bg-center"
      style={{ backgroundImage: "url('/images/register.png')" }}
    >
      <div className="bg-white bg-opacity-75 p-8 rounded shadow-lg">
        <Register />
      </div>
    </div>
  );
};

export default RegisterPage;
