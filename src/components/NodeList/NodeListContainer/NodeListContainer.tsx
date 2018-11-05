import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { fetchChainNetworksIfNeeded } from "../../../actions/chainNetworks";
import { ReducerConfigure } from "../../../reducers";
import { ChainNetworks, NetworkNodeInfo } from "../../../requests/types";
import NodeItem from "./NodeItem/NodeItem";
import "./NodeListContainer.css";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
}
interface DispatchProps {
  getChainNetworks: () => void;
}
type Props = DispatchProps & OwnProps;
class NodeListContainer extends React.Component<Props> {
  public componentDidMount() {
    this.props.getChainNetworks();
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

const mapStateToProps = (state: ReducerConfigure) => ({
  chainNetworks: state.chainNetworksReducer.chainNetworks
});
const mapDispatchToProps = (dispatch: any) => ({
  getChainNetworks: async () => {
    dispatch(fetchChainNetworksIfNeeded());
  }
});
export default connect(
  mapStateToProps,
  mapDispatchToProps
)(NodeListContainer);
