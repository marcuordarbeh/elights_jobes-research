import React from 'react';
import { Switch, Route, Redirect } from 'react-router-dom';
import LandingPage from './pages/LandingPage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';
import DashboardPage from './pages/DashboardPage';
import Dashboard from './components/Dashboard';
import Payment from './components/Payment';
import AchPayment from './components/AchPayment';
import WireTransfer from './components/WireTransfer';
import CryptoConversion from './components/CryptoConversion';

function App() {
  return (
    <div className="App">
      <Switch>
        {/* Landing page at root */}
        <Route exact path="/" component={LandingPage} />
        <Route path="/login" component={LoginPage} />
        <Route path="/register" component={RegisterPage} />
        <Route path="/dashboard" component={DashboardPage} />
        {/* Other payment related routes */}
        <Route path="/card" component={Payment} />
        <Route path="/ach" component={AchPayment} />
        <Route path="/wire" component={WireTransfer} />
        <Route path="/crypto" component={CryptoConversion} />
        <Route path="/home" component={Dashboard} />
        <Redirect to="/" />
      </Switch>
    </div>
  );
}

export default App;
