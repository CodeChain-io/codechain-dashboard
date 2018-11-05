import * as React from "react";
import { connect } from "react-redux";
import { fetchNodeInfoIfNeeded } from "../../../actions/nodeInfo";
import { ReducerConfigure } from "../../../reducers";
import { NodeInfo } from "../../../requests/types";
import NodeDetail from "./NodeDetail/NodeDetail";
import "./NodeDetailContainer.css";

interface OwnProps {
  match: {
    params: {
      nodeId: string;
    };
  };
}

interface StateProps {
  nodeInfo?: NodeInfo;
}

interface DispatchProps {
  getNodeInfo: () => void;
}

type Props = DispatchProps & OwnProps & StateProps;
class NodeDetailContainer extends React.Component<Props> {
  public componentDidMount() {
    if (!this.props.nodeInfo) {
      this.props.getNodeInfo();
    }
  }
  public render() {
    const { nodeInfo } = this.props;
    if (!nodeInfo) {
      return <div>Loading...</div>;
    }
    return (
      <div className="node-detail-container animated fadeIn">
        <NodeDetail nodeInfo={nodeInfo} />
      </div>
    );
  }
}
const mapStateToProps = (state: ReducerConfigure, ownProps: OwnProps) => ({
  nodeInfo:
    state.nodeInfoReducer.nodeInfos[decodeURI(ownProps.match.params.nodeId)] &&
    state.nodeInfoReducer.nodeInfos[decodeURI(ownProps.match.params.nodeId)]
      .info
});
const mapDispatchToProps = (dispatch: any, ownProps: OwnProps) => ({
  getNodeInfo: async () => {
    const nodeId = decodeURI(ownProps.match.params.nodeId);
    dispatch(fetchNodeInfoIfNeeded(nodeId));
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(NodeDetailContainer);
