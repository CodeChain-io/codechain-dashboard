import { Component } from "react";
import * as React from "react";
import { ConnectChart } from "./ConnectChart/ConnectChart";

export default class Dashboard extends Component {
  constructor(props: {}) {
    super(props);
  }
  public render() {
    return (
      <div className="dashboard">
        <ConnectChart className="animated fadeIn" />
      </div>
    );
  }
}
