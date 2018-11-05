import * as React from "react";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import { setNodeInfo } from "../../../actions/nodeInfo";
import { ReducerConfigure } from "../../../reducers";
import { Apis } from "../../../requests";
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
  nodeInfo: NodeInfo | undefined;
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
    state.nodeInfoReducer.nodeInfo[decodeURI(ownProps.match.params.nodeId)]
});
const mapDispatchToProps = (dispatch: Dispatch, ownProps: OwnProps) => ({
  getNodeInfo: async () => {
    const nodeId = decodeURI(ownProps.match.params.nodeId);
    const nodeInfo = await Apis.getNodeInfo(nodeId);
    dispatch(setNodeInfo(nodeId, nodeInfo));
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(NodeDetailContainer);
