import { Component } from "react";
import * as React from "react";
import { Route } from "react-router-dom";
import NodeDetail from "./NodeDetail/NodeDetail";
import { NodeListContainer } from "./NodeListContainer/NodeListContainer";

interface Props {
  match: any;
}
export default class Nodelist extends Component<Props> {
  constructor(props: Props) {
    super(props);
  }
  public render() {
    const { match } = this.props;
    return (
      <div>
        <Route path={`${match.url}/:nodeId`} component={NodeDetail} />
        <Route exact={true} path={match.url} component={NodeListContainer} />
      </div>
    );
  }
}
