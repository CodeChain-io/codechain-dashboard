import {
  faBroadcastTower,
  faCircle,
  faInfo
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { Link } from "react-router-dom";
import { Dispatch } from "redux";
import { Actions } from "../../actions";
import { RootState } from "../../reducers";
import { Apis } from "../../requests";
import { ChainNetworks, NetworkNodeInfo } from "../../requests/types";
import { getStatusClass } from "../../utils/getStatusClass";
import { ConnectionGraphContainer } from "./ConnectGraphContainer/ConnectionGraphContainer";
import "./Dashboard.css";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
}
interface State {
  selectedNode?: NetworkNodeInfo;
  selectedNetworkNodeList: NetworkNodeInfo[];
}

interface DispatchProps {
  getChainNetworks: () => void;
}

type Props = DispatchProps & OwnProps;
class Dashboard extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      selectedNode: undefined,
      selectedNetworkNodeList: []
    };
  }
  public componentDidMount() {
    if (!this.props.chainNetworks) {
      this.props.getChainNetworks();
    }
  }
  public render() {
    const { chainNetworks } = this.props;
    const { selectedNode, selectedNetworkNodeList } = this.state;
    if (!chainNetworks) {
      return <div>Loading...</div>;
    }
    return (
      <div className="dashboard">
        <div className="d-flex">
          <div className="left-panel mr-3">
            <ConnectionGraphContainer
              chainNetworks={chainNetworks}
              className="animated fadeIn"
              onSelectNode={this.onSelectNode}
              onDeselect={this.onDeselect}
            />
          </div>
          <div className="right-panel">
            <div className="dashboard-item animated fadeIn">
              <div className="dashboard-item-header">
                <h5 className="mb-0">
                  <FontAwesomeIcon className="mr-2" icon={faBroadcastTower} />
                  Status
                </h5>
              </div>
              <div className="dashboard-item-body ">
                <ul className="list-unstyled mb-0">
                  <li>
                    <div className="d-flex align-items-center mb-1">
                      <FontAwesomeIcon
                        icon={faCircle}
                        className="text-success mr-2"
                      />
                      <span className="mr-auto">Run</span>
                      <span>
                        {
                          _.filter(
                            chainNetworks.nodes,
                            node => node.status === "Run"
                          ).length
                        }
                      </span>
                    </div>
                  </li>
                  <li>
                    <div className="d-flex align-items-center mb-1">
                      <FontAwesomeIcon
                        icon={faCircle}
                        className="text-warning mr-2"
                      />
                      <span className="mr-auto">Starting</span>
                      <span>
                        {
                          _.filter(
                            chainNetworks.nodes,
                            node => node.status === "Starting"
                          ).length
                        }
                      </span>
                    </div>
                  </li>
                  <li>
                    <div className="d-flex align-items-center mb-1">
                      <FontAwesomeIcon
                        icon={faCircle}
                        className="text-secondary mr-2"
                      />
                      <span className="mr-auto">Stop</span>
                      <span>
                        {
                          _.filter(
                            chainNetworks.nodes,
                            node => node.status === "Stop"
                          ).length
                        }
                      </span>
                    </div>
                  </li>
                  <li>
                    <div className="d-flex align-items-center mb-1">
                      <FontAwesomeIcon
                        icon={faCircle}
                        className="text-danger mr-2"
                      />
                      <span className="mr-auto">Error</span>
                      <span>
                        {
                          _.filter(
                            chainNetworks.nodes,
                            node => node.status === "Error"
                          ).length
                        }
                      </span>
                    </div>
                  </li>
                  <li>
                    <div className="d-flex align-items-center mb-1">
                      <FontAwesomeIcon
                        icon={faCircle}
                        className="text-info mr-2"
                      />
                      <span className="mr-auto">UFO</span>
                      <span>
                        {
                          _.filter(
                            chainNetworks.nodes,
                            node => node.status === "UFO"
                          ).length
                        }
                      </span>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
            {selectedNode && (
              <div className="dashboard-item mt-3">
                <div className="dashboard-item-header">
                  <h5 className="mb-0">
                    <FontAwesomeIcon className="mr-2" icon={faInfo} />
                    Selected node info
                  </h5>
                </div>
                <div className="dashboard-item-body node-item-info-panel">
                  {this.getNodeInfoElem(selectedNode)}
                </div>
              </div>
            )}
          </div>
        </div>
        {selectedNetworkNodeList.length > 0 && (
          <div className="selected-network-node-list-container">
            {_.map(selectedNetworkNodeList, node => {
              return (
                <div key={node.name} className="network-node-info">
                  {this.getNodeInfoElem(node)}
                </div>
              );
            })}
          </div>
        )}
      </div>
    );
  }
  private onSelectNode = (node: { id: string; label: string }) => {
    const selectedNode = _.find(
      this.props.chainNetworks!.nodes,
      networkNodeInfo => networkNodeInfo.name === node.id
    );
    setTimeout(() => {
      let selectedNetworkNodeList = [node.id];
      let beforeNetworkNodeList = [node.id];
      const connections = this.props.chainNetworks!.connections;
      while (true) {
        for (const connection of connections) {
          if (_.includes(beforeNetworkNodeList, connection.nodeA)) {
            beforeNetworkNodeList.push(connection.nodeB);
          }
          if (_.includes(beforeNetworkNodeList, connection.nodeB)) {
            beforeNetworkNodeList.push(connection.nodeA);
          }
        }
        beforeNetworkNodeList = _.uniq(beforeNetworkNodeList);
        if (
          _.difference(beforeNetworkNodeList, selectedNetworkNodeList)
            .length === 0
        ) {
          break;
        }
        selectedNetworkNodeList = _.clone(beforeNetworkNodeList);
      }

      const selectedNetworkNodeInfoList = _.filter(
        this.props.chainNetworks!.nodes,
        (netowrkNode: NetworkNodeInfo) =>
          _.includes(selectedNetworkNodeList, netowrkNode.name) &&
          netowrkNode.name !== node.id
      );
      this.setState({ selectedNetworkNodeList: selectedNetworkNodeInfoList });
    });
    this.setState({ selectedNode });
  };
  private onDeselect = () => {
    this.setState({ selectedNode: undefined, selectedNetworkNodeList: [] });
  };

  private getNodeInfoElem = (node: NetworkNodeInfo) => {
    return (
      <div className="node-info-element">
        <ul
          className={`node-info-element-data list-unstyled ${node.status ===
            "UFO" && "mb-0"}`}
        >
          <li>
            <div>
              Status :{" "}
              <span className={getStatusClass(node.status)}>{node.status}</span>
            </div>
          </li>
          {node.name && (
            <li>
              <div>
                Name : <span>{node.name}</span>
              </div>
            </li>
          )}
          {node.address && (
            <li>
              <div>
                Address : <span>{node.address}</span>
              </div>
            </li>
          )}
          {node.bestBlockId && (
            <li>
              <div>
                Best block :{" "}
                <span>
                  {node.bestBlockId.blockNumber} (
                  {node.bestBlockId.hash
                    ? node.bestBlockId.hash.slice(0, 6)
                    : "Invalid hash"}
                  )
                </span>
              </div>
            </li>
          )}
          {node.version && (
            <li>
              <div>
                Version :{" "}
                <span>
                  {node.version.version} ({node.version.hash.slice(0, 6)})
                </span>
              </div>
            </li>
          )}
        </ul>
        {node.status !== "UFO" && (
          <div className="bottom-container">
            <Link className="view-details" to={`/nodelist/${node.name}`}>
              View details
            </Link>
          </div>
        )}
        {node.status === "UFO" && (
          <div className="bottom-container">
            <a
              href="https://github.com/codechain-io/codechain-agent"
              target="_blank"
            >
              <span className="view-details">Install agent</span>
            </a>
          </div>
        )}
      </div>
    );
  };
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
)(Dashboard);
