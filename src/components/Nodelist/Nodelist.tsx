import { Component } from "react";
import * as React from "react";
import NodeItem from "./NodeItem/NodeItem";

export default class Nodelist extends Component {
  constructor(props: {}) {
    super(props);
  }
  public render() {
    return (
      <div>
        <NodeItem className="mb-3" />
        <NodeItem className="mb-3" />
        <NodeItem className="mb-3" />
        <NodeItem className="mb-3" />
      </div>
    );
  }
}
