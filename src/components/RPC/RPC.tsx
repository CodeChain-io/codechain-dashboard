import { Component } from "react";
import * as React from "react";
import "./RPC.css";
import { RPCLeftPanel } from "./RPCLeftPanel/RPCLeftPanel";
import RPCRightPanel from "./RPCRightPanel/RPCRightPanel";
export default class RPC extends Component {
  public render() {
    return (
      <div className="rpc-container d-flex">
        <RPCLeftPanel className="left-panel" />
        <RPCRightPanel className="right-panel" />
      </div>
    );
  }
}
