import React from 'react';
import { Switch, Route, Redirect } from 'react-router-dom';
import LandingPage from './pages/LandingPage';
import DashboardPage from './pages/DashboardPage';
import PaymentPage from './pages/paymentPage';

function App() {
  return (
    <div className="App">
      <Switch>
        <Route path="/landing" component={LandingPage} />
        <Route path="/dashboard" component={DashboardPage} />
        <Route path="/payment" component={PaymentPage} />
        <Redirect from="/" to="/landing" />
      </Switch>
    </div>
  );
}

export default App;
