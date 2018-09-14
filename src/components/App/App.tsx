import * as React from "react";
import { connect, DispatchProp } from "react-redux";
import { BrowserRouter as Router, Route } from "react-router-dom";
import RequestAgent from "../../request";
import Dashboard from "../Dashboard/Dashboard";
import { GlobalNavigationBar } from "../GlobalNavigationBar/GlobalNavigationBar";
import { Header } from "../Header/Header";
import Nodelist from "../Nodelist/Nodelist";
import "./App.css";

class App extends React.Component<DispatchProp> {
  public componentWillMount() {
    RequestAgent.getInstance().setDispatch(this.props.dispatch);
    RequestAgent.getInstance()
      .call("dashboard_getNetwork", [])
      .then((result: any) => {
        console.log(result);
      });
  }
  public componentWillUnmount() {
    RequestAgent.getInstance().close();
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
export default connect()(App);
