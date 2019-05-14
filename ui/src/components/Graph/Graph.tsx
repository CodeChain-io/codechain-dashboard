import { Component } from "react";
import * as React from "react";
import { Route } from "react-router";
import GraphNode from "./GraphNode/GraphNode";
import NetworkOutAllAVGGraph from "./NetworkOutAllAVGGraph/NetworkOutAllAVGGraph";
import NetworkOutAllGraph from "./NetworkOutAllGraph/NetworkOutAllGraph";

interface Props {
  match: any;
  history: any;
}

export default class Graph extends Component<Props> {
  public render() {
    const { match } = this.props;
    return (
      <div>
        <Route exact={true} path={match.url} render={this.renderAllNodeGraph} />
        <Route path={`${match.url}/:nodeId`} component={GraphNode} />
      </div>
    );
  }

  private renderAllNodeGraph = () => {
    return (
      <div className="graph">
        <NetworkOutAllGraph history={this.props.history} />
        <NetworkOutAllAVGGraph />
      </div>
    );
  };
}
