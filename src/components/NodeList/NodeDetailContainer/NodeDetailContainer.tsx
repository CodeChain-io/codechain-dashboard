import * as React from "react";
import { connect } from "react-redux";
import { Dispatch } from "redux";
import { Actions } from "../../../actions";
import { RootState } from "../../../reducers";
import { Apis } from "../../../requests";
import { NodeInfo } from "../../../requests/types";
import NodeDetail from "./NodeDetail/NodeDetail";

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
      <div className="animated fadeIn">
        <NodeDetail nodeInfo={nodeInfo} />
      </div>
    );
  }
}
const mapStateToProps = (state: RootState, ownProps: OwnProps) => ({
  nodeInfo: state.nodeInfo[ownProps.match.params.nodeId]
});
const mapDispatchToProps = (dispatch: Dispatch, ownProps: OwnProps) => ({
  getNodeInfo: async () => {
    const nodeInfo = await Apis.getNodeInfo(ownProps.match.params.nodeId);
    dispatch(Actions.setNodeInfo(ownProps.match.params.nodeId, nodeInfo));
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(NodeDetailContainer);
