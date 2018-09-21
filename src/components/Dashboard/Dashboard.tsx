import { faCircle, faInfo } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect, DispatchProp } from "react-redux";
import { Dispatch } from "redux";
import { Actions } from "../../actions";
import { RootState } from "../../reducers";
import { Apis } from "../../requests";
import { ChainNetworks } from "../../requests/types";
import { ConnectionGraphContainer } from "./ConnectGraphContainer/ConnectionGraphContainer";
import "./Dashboard.css";

interface OwnProps {
  chainNetworks: ChainNetworks | undefined;
  getChainNetworks: () => void;
}
type Props = DispatchProp & OwnProps;
class Dashboard extends React.Component<Props> {
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
      <div className="dashboard d-flex">
        <div className="left-panel mr-3">
          <ConnectionGraphContainer
            chainNetworks={chainNetworks}
            className="animated fadeIn"
          />
        </div>
        <div className="right-panel">
          <div className="connection-graph-help animated fadeIn">
            <div className="connection-graph-help-header">
              <h5 className="mb-0">
                <FontAwesomeIcon className="mr-2" icon={faInfo} />
                Status
              </h5>
            </div>
            <div className="connection-graph-help-body">
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
        </div>
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
)(Dashboard);
