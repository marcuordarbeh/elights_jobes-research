import React from 'react';
import { Switch, Route, Redirect } from 'react-router-dom';
import Login from './components/Login';
import Register from './components/Register';
import Dashboard from './components/Dashboard';
import Payment from './components/Payment';

function App() {
  return (
    <div className="App">
      <Switch>
        <Route path="/login" component={Login} />
        <Route path="/register" component={Register} />
        <Route path="/dashboard" component={Dashboard} />
        <Route path="/payment" component={Payment} />
        <Redirect from="/" to="/login" />
      </Switch>
    </div>
  );
}

export default App;