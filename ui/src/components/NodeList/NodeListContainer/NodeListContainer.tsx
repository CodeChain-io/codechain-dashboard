import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { fetchChainNetworksIfNeeded } from "../../../actions/chainNetworks";
import { ReducerConfigure } from "../../../reducers";
import { Apis } from "../../../requests";
import {
  ChainNetworks,
  NetworkNodeInfo,
  UpdateCodeChainRequest
} from "../../../requests/types";
import UpgradeNodeModal from "../UpgradeNodeModal/UpgradeNodeModal";
import NodeItem from "./NodeItem/NodeItem";
import "./NodeListContainer.css";
import SelectNodesModal from "./SelectNodesModal/SelectNodesModal";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
}
interface DispatchProps {
  getChainNetworks: () => void;
}
interface State {
  isSelectNodesModalOpen: boolean;
  isUpgradeNodeModalOpen:
    | {
        type: "close";
      }
    | {
        type: "open";
        selectedNodes: string[];
      };
}
type Props = DispatchProps & OwnProps;
class NodeListContainer extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      isUpgradeNodeModalOpen: { type: "close" },
      isSelectNodesModalOpen: false
    };
  }
  public componentDidMount() {
    this.props.getChainNetworks();
  }
  public render() {
    const { chainNetworks } = this.props;
    const { isUpgradeNodeModalOpen, isSelectNodesModalOpen } = this.state;
    if (!chainNetworks) {
      return <div>Loading...</div>;
    }
    return (
      <div className="node-list-container">
        <button
          type="button"
          className="btn mb-3"
          onClick={this.openSelectNodesModal}
        >
          Update
        </button>
        <SelectNodesModal
          onClose={this.handleOnCloseSelectNodesModal}
          isOpen={isSelectNodesModalOpen}
          onSelectNodes={this.handleOnSelectNodes}
          chainNetworks={chainNetworks}
        />
        <UpgradeNodeModal
          onClose={this.handleOnCloseUpgradeModal}
          isOpen={isUpgradeNodeModalOpen.type === "open"}
          onUpdateNode={this.handleOnUpgradeNode}
        />
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
  private openSelectNodesModal = () => {
    this.setState({ isSelectNodesModalOpen: true });
  };
  private handleOnCloseSelectNodesModal = () => {
    this.setState({ isSelectNodesModalOpen: false });
  };
  private handleOnSelectNodes = (nodes: string[]) => {
    this.openUpgradeNodeModal(nodes);
  };
  private openUpgradeNodeModal = (selectedNodes: string[]) => {
    this.setState({
      isSelectNodesModalOpen: false
    });
    if (selectedNodes.length === 0) {
      return;
    }
    this.setState({
      isUpgradeNodeModalOpen: {
        type: "open",
        selectedNodes
      }
    });
  };
  private handleOnCloseUpgradeModal = () => {
    this.setState({ isUpgradeNodeModalOpen: { type: "close" } });
  };
  private handleOnUpgradeNode = async (req: UpdateCodeChainRequest) => {
    if (this.state.isUpgradeNodeModalOpen.type === "close") {
      throw new Error("Invalid state");
    }
    const selectedNodes = this.state.isUpgradeNodeModalOpen.selectedNodes;

    await Promise.all(
      _.map(selectedNodes, nodeName => {
        return Apis.updateNode(nodeName, req);
      })
    );

    this.setState({ isUpgradeNodeModalOpen: { type: "close" } });
  };
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
