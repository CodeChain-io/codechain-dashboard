import { Component } from "react";
import * as React from "react";
import { Route } from "react-router-dom";
import NodeDetailContainer from "./NodeDetailContainer/NodeDetailContainer";
import NodeListContainer from "./NodeListContainer/NodeListContainer";

interface Props {
  match: any;
}
export default class NodeList extends Component<Props> {
  public render() {
    const { match } = this.props;
    return (
      <div>
        <Route path={`${match.url}/:nodeId`} component={NodeDetailContainer} />
        <Route exact={true} path={match.url} component={NodeListContainer} />
      </div>
    );
  }
}
