import * as React from "react";
import { BrowserRouter as Router, Route } from "react-router-dom";
import RequestAgent from "../../requests";
import Dashboard from "../Dashboard/Dashboard";
import { GlobalNavigationBar } from "../GlobalNavigationBar/GlobalNavigationBar";
import { Header } from "../Header/Header";
import Nodelist from "../Nodelist/Nodelist";
import "./App.css";

export default class App extends React.Component {
  private requestAgent: RequestAgent;
  public componentDidMount() {
    this.requestAgent = new RequestAgent("localhost:3012");
  }
  public componentWillUnmount() {
    this.requestAgent.close();
  }
  public render() {
    return (
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
  }
}
