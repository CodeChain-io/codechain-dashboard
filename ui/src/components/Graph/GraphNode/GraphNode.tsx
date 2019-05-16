import * as React from "react";
import NetworkOutNodeExtensionGraph from "./NetworkOutNodeExtensionGraph/NetworkOutNodeExtensionGraph";
import NetworkOutNodePeerGraph from "./NetworkOutNodePeerGraph/NetworkOutNodePeerGraph";

interface OwnProps {
  match: {
    params: {
      nodeId: string;
    };
  };
}

export default class GraphNode extends React.Component<OwnProps> {
  public render() {
    const { match } = this.props;
    return (
      <div>
        <h2>Node {match.params.nodeId}</h2>

        <NetworkOutNodeExtensionGraph nodeId={match.params.nodeId} />
        <NetworkOutNodePeerGraph nodeId={match.params.nodeId} />
      </div>
    );
  }
}
