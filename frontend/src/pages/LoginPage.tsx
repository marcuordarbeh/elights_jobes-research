import React from 'react';
import Login from '../components/Login';

const LoginPage: React.FC = () => {
  return (
    <div
      className="min-h-screen flex items-center justify-center bg-cover bg-center"
      style={{ backgroundImage: "url('/images/login.png')" }}
    >
      <div className="bg-white bg-opacity-75 p-8 rounded shadow-lg">
        <Login />
      </div>
    </div>
  );
};

export default LoginPage;
