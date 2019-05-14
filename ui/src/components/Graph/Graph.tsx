import { Component } from "react";
import * as React from "react";
import NetworkOutAllAVGGraph from "./NetworkOutAllAVGGraph/NetworkOutAllAVGGraph";
import NetworkOutAllGraph from "./NetworkOutAllGraph/NetworkOutAllGraph";

export default class Graph extends Component<any> {
  public render() {
    return (
      <div className="graph">
        <NetworkOutAllGraph />
        <NetworkOutAllAVGGraph />
      </div>
    );
  }
}
