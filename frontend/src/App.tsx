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

// import React from 'react';
// import { Switch, Route, Redirect } from 'react-router-dom';
// import Dashboard from './components/Dashboard';
// import PaymentPage from './pages/PaymentPage';

// function App() {
//   return (
//     <div className="App">
//       <Switch>
//         <Route path="/dashboard" component={Dashboard} />
//         <Route path="/payment" component={PaymentPage} />
//         <Redirect from="/" to="/dashboard" />
//       </Switch>
//     </div>
//   );
// }

// export default App;
