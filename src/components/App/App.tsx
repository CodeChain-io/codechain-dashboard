import * as React from "react";
import { BrowserRouter as Router, Route } from "react-router-dom";
import Dashboard from "../Dashboard/Dashboard";
import GlobalNavigationBar from "../GlobalNavigationBar/GlobalNavigationBar";
import Header from "../Header/Header";
import Nodelist from "../Nodelist/Nodelist";
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

export default App;
