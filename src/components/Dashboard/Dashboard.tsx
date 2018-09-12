import { Component } from "react";
import * as React from "react";
import { ConnectionGraphContainer } from "./ConnectGraphContainer/ConnectionGraphContainer";

export default class Dashboard extends Component {
  constructor(props: {}) {
    super(props);
  }
  public render() {
    return (
      <div className="dashboard vh-100">
        <ConnectionGraphContainer className="animated fadeIn" />
      </div>
    );
  }
}
