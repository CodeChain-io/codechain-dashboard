import * as _ from "lodash";
import * as React from "react";
import { connect, DispatchProp } from "react-redux";
import { Dispatch } from "redux";
import { Actions } from "../../../actions";
import { RootState } from "../../../reducers";
import { Apis } from "../../../requests";
import { ChainNetworks, NetworkNodeInfo } from "../../../requests/types";
import NodeItem from "./NodeItem/NodeItem";
import "./NodeListContainer.css";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
  getChainNetworks: () => void;
}
type Props = DispatchProp & OwnProps;
class NodeListContainer extends React.Component<Props> {
  public componentDidMount() {
    if (!this.props.chainNetworks) {
      this.props.getChainNetworks();
    }
  }
  public render() {
    const { chainNetworks } = this.props;
    if (!chainNetworks) {
      return <div>Loading...</div>;
    }
    return (
      <div className="node-list-container">
        {_.map(chainNetworks.nodes, (nodeInfo: NetworkNodeInfo) => {
          return (
            <NodeItem
              key={nodeInfo.name}
              nodeInfo={nodeInfo}
              className="mb-3 animated fadeIn"
            />
          );
        })}
      </div>
    );
  }
}

const mapStateToProps = (state: RootState) => ({
  chainNetworks: state.chainNetworks
});
const mapDispatchToProps = (dispatch: Dispatch) => ({
  getChainNetworks: async () => {
    const chainNetworks = await Apis.getChainNetworks();
    dispatch(Actions.setChainNetworks(chainNetworks));
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(NodeListContainer);
