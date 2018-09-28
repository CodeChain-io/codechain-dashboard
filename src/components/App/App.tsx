import * as React from "react";
import * as ReactModal from "react-modal";
import { connect, DispatchProp } from "react-redux";
import { BrowserRouter as Router, Route } from "react-router-dom";
import { ToastContainer } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import RequestAgent from "../../RequestAgent";
import Dashboard from "../Dashboard/Dashboard";
import { GlobalNavigationBar } from "../GlobalNavigationBar/GlobalNavigationBar";
import { Header } from "../Header/Header";
import NodeList from "../NodeList/NodeList";
import RPC from "../RPC/RPC";
import "./App.css";

class App extends React.Component<DispatchProp> {
  public componentDidMount() {
    ReactModal.setAppElement("#app");
  }
  public componentWillMount() {
    RequestAgent.getInstance().setDispatch(this.props.dispatch);
  }
  public componentWillUnmount() {
    RequestAgent.getInstance().close();
  }
  public render() {
    return (
      <Router>
        <div id="app" className="app">
          <Header />
          <GlobalNavigationBar />
          <div className="content-container">
            <Route exact={true} path="/" component={Dashboard} />
            <Route path="/nodelist" component={NodeList} />
            <Route path="/rpc" component={RPC} />
          </div>
          <ToastContainer autoClose={false} />
        </div>
      </Router>
    );
  }
}
export default connect()(App);
