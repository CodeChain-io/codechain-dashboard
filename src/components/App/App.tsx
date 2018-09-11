import * as React from "react";
import { BrowserRouter as Router, Route } from "react-router-dom";
import GlobalNavigationBar from "../GlobalNavigationBar/GlobalNavigationBar";
import Header from "../Header/Header";
import "./App.css";

const App = () => (
  <Router>
    <div className="app">
      <Header />
      <GlobalNavigationBar />
      <div className="content-container">
        <Route exact={true} path="/" component={Dashboard} />
        <Route path="/nodelist" component={Nodelist} />
      </div>
    </div>
  </Router>
);

const Dashboard = () => (
  <div className="dashboard">
    <h2>Dashboard</h2>
  </div>
);

const Nodelist = () => (
  <div>
    <h2>Node list</h2>
  </div>
);

export default App;
