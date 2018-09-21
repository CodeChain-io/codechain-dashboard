import {
  faBroadcastTower,
  faCircle,
  faInfo
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect, DispatchProp } from "react-redux";
import { Link } from "react-router-dom";
import { Dispatch } from "redux";
import { Actions } from "../../actions";
import { RootState } from "../../reducers";
import { Apis } from "../../requests";
import {
  ChainNetworks,
  NetworkNodeInfo,
  NodeStatus
} from "../../requests/types";
import { ConnectionGraphContainer } from "./ConnectGraphContainer/ConnectionGraphContainer";
import "./Dashboard.css";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
  getChainNetworks: () => void;
}
interface State {
  selectedNode?: NetworkNodeInfo;
}

const getStatusClass = (status: NodeStatus) => {
  switch (status) {
    case "Run":
      return "text-success";
    case "Stop":
      return "text-secondary";
    case "Error":
      return "text-danger";
    case "Starting":
      return "text-warning";
    case "UFO":
      return "text-info";
  }
  throw new Error("Invalid status");
};

type Props = DispatchProp & OwnProps;
class Dashboard extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      selectedNode: undefined
    };
  }
  public componentDidMount() {
    if (!this.props.chainNetworks) {
      this.props.getChainNetworks();
    }
  }
  public render() {
    const { chainNetworks } = this.props;
    const { selectedNode } = this.state;
    if (!chainNetworks) {
      return <div>Loading...</div>;
    }
    return (
      <div className="dashboard d-flex">
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
                  Node info
                </h5>
              </div>
              <div className="dashboard-item-body">
                <ul className="list-unstyled">
                  <li>
                    <div>
                      Status :{" "}
                      <span className={getStatusClass(selectedNode.status)}>
                        {selectedNode.status}
                      </span>
                    </div>
                  </li>
                  <li>
                    <div>
                      Address : <span>{selectedNode.address}</span>
                    </div>
                  </li>
                  {selectedNode.name && (
                    <li>
                      <div>
                        Name : <span>{selectedNode.name}</span>
                      </div>
                    </li>
                  )}
                  {selectedNode.bestBlockId && (
                    <li>
                      <div>
                        Best block :{" "}
                        <span>
                          {selectedNode.bestBlockId.blockNumber} (
                          {selectedNode.bestBlockId.hash.value
                            ? selectedNode.bestBlockId.hash.value.slice(0, 6)
                            : "Invalid hash"}
                          )
                        </span>
                      </div>
                    </li>
                  )}
                  {selectedNode.version && (
                    <li>
                      <div>
                        Version :{" "}
                        <span>
                          {selectedNode.version.version} (
                          {selectedNode.version.hash.slice(0, 6)})
                        </span>
                      </div>
                    </li>
                  )}
                </ul>
                <div>
                  <Link
                    className="view-details"
                    to={`/nodelist/${selectedNode.address}`}
                  >
                    View details
                  </Link>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    );
  }
  private onSelectNode = (node: { id: string; label: string }) => {
    const selectedNode = _.find(
      this.props.chainNetworks!.nodes,
      networkNodeInfo => networkNodeInfo.address === node.id
    );
    this.setState({ selectedNode });
  };

  private onDeselect = () => {
    this.setState({ selectedNode: undefined });
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
