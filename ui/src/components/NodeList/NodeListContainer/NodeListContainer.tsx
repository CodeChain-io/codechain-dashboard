import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { Combobox } from "react-widgets";
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
  sortBy: string;
  arrangement: string;
}
type Props = DispatchProps & OwnProps;
class NodeListContainer extends React.Component<Props, State> {
  private sortCriteria = [
    "name",
    "socketAddress",
    "blockNumber",
    "version",
    "status"
  ];

  public constructor(props: Props) {
    super(props);
    this.state = {
      isUpgradeNodeModalOpen: { type: "close" },
      isSelectNodesModalOpen: false,
      sortBy: "name",
      arrangement: "asc"
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
        <div className="row">
          <div className="ml-3">
            <h6>Sort by</h6>
            <Combobox
              data={this.sortCriteria}
              defaultValue={this.state.sortBy}
              onChange={this.handleOnChangeSortingCriterion}
            />
          </div>
          <div className="ml-3">
            <h6>Arrangement</h6>
            <input
              type="radio"
              id="asc"
              name="arrangement-radio"
              value="asc"
              checked={this.state.arrangement === "asc"}
              onChange={this.handleOnChangeArrangement}
            />
            <label>asc</label>
            <input
              type="radio"
              id="desc-1"
              name="arrangement-radio"
              value="desc"
              checked={this.state.arrangement === "desc"}
              onChange={this.handleOnChangeArrangement}
            />
            <label>desc</label>
          </div>
        </div>
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
        {_.map(
          chainNetworks.nodes.sort((fst, snd) =>
            compareNodes(fst, snd, this.state.sortBy, this.state.arrangement)
          ),
          (nodeInfo: NetworkNodeInfo) => {
            return (
              <NodeItem
                key={nodeInfo.name}
                nodeInfo={nodeInfo}
                className="mb-3 animated fadeIn"
              />
            );
          }
        )}
      </div>
    );
  }
  private handleOnChangeArrangement = (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    this.setState({ arrangement: event.target.value });
  };
  private handleOnChangeSortingCriterion = (newCriterion: string) => {
    this.setState({ sortBy: newCriterion });
  };
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

const compareNodes = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo,
  criterion: string,
  arrangement: string
): number => {
  let a: NetworkNodeInfo;
  let b: NetworkNodeInfo;

  if (arrangement === "asc") {
    a = fst;
    b = snd;
  } else {
    a = snd;
    b = fst;
  }
  switch (criterion) {
    case "name":
      return nodeNameComparator(a, b);
    case "socketAddress":
      return nodeSocketAddressComapartor(a, b);
    case "blockNumber":
      return nodeBlockNumberComparator(a, b);
    case "version":
      return nodeVersionComparator(a, b);
    case "status":
      return nodeStatusComparator(a, b);
    default:
      return 1;
  }
};

export const nodeNameComparator = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo
) => {
  if (fst.name > snd.name) {
    return 1;
  } else if (fst.name < snd.name) {
    return -1;
  } else {
    return 0;
  }
};

export const nodeBlockNumberComparator = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo
) => {
  const aBlockNumber = fst.bestBlockId ? fst.bestBlockId.blockNumber : 0;
  const bBlockNumber = snd.bestBlockId ? snd.bestBlockId.blockNumber : 0;
  return aBlockNumber - bBlockNumber;
};

export const nodeVersionComparator = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo
) => {
  const versionParsedDefault = [0, 0, 0];
  const fstVersionParsed = fst.version
    ? fst.version.version.split(".").map(numStr => parseInt(numStr, 10))
    : versionParsedDefault;
  const sndVersionParsed = snd.version
    ? snd.version.version.split(".").map(numStr => parseInt(numStr, 10))
    : versionParsedDefault;

  const zipped = _.zip(fstVersionParsed, sndVersionParsed);
  for (const [f, s] of zipped) {
    const fUnwrap = f ? f : 0;
    const sUnwrap = s ? s : 0;
    if (fUnwrap === sUnwrap) {
      continue;
    }
    return fUnwrap - sUnwrap;
  }
  return 0;
};

export const nodeSocketAddressComapartor = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo
) => {
  const sockAddressParsedDefault = [0, 0, 0, 0, 0];
  const fstSocAddressParsed = fst.address
    ? fst.address.split(/[.:]+/).map(numStr => parseInt(numStr, 10))
    : sockAddressParsedDefault;
  const sndSocAddressParsed = snd.address
    ? snd.address.split(/[.:]+/).map(numStr => parseInt(numStr, 10))
    : sockAddressParsedDefault;

  const zipped = _.zip(fstSocAddressParsed, sndSocAddressParsed);
  for (const [f, s] of zipped) {
    const fUnwrap = f ? f : 0;
    const sUnwrap = s ? s : 0;
    if (fUnwrap === sUnwrap) {
      continue;
    }
    return fUnwrap - sUnwrap;
  }
  return 0;
};

export const nodeStatusComparator = (
  fst: NetworkNodeInfo,
  snd: NetworkNodeInfo
) => {
  const statusOrder = {
    Error: 1,
    Stop: 2,
    Starting: 3,
    Updating: 4,
    Run: 5,
    UFO: 6
  };
  return statusOrder[fst.status] - statusOrder[snd.status];
};

const mapStateToProps = (state: ReducerConfigure) => ({
  chainNetworks: state.chainNetworksReducer.chainNetworks
});
const mapDispatchToProps = (dispatch: any) => ({
  getChainNetworks: async () => {
    dispatch(fetchChainNetworksIfNeeded());
  }
});
export default connect(mapStateToProps, mapDispatchToProps)(NodeListContainer);
