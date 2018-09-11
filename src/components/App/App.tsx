import * as React from "react";
import { BrowserRouter as Router, Link, Route } from "react-router-dom";

const App = () => (
  <Router>
    <div>
      <ul>
        <li>
          <Link to="/">Dashboard</Link>
        </li>
        <li>
          <Link to="/nodelist">Node list</Link>
        </li>
      </ul>

      <hr />

      <Route exact={true} path="/" component={Dashboard} />
      <Route path="/nodelist" component={Nodelist} />
    </div>
  </Router>
);

const Dashboard = () => (
  <div>
    <h2>Dashboard</h2>
  </div>
);

const Nodelist = () => (
  <div>
    <h2>Node list</h2>
  </div>
);

export default App;
