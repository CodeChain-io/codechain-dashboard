import {
  faAngleDoubleLeft,
  faAngleDoubleRight,
  faAngleLeft,
  faAngleRight
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { Col, Label, Row } from "reactstrap";
import { fetchChainNetworksIfNeeded } from "../../../actions/chainNetworks";
import { ReducerConfigure } from "../../../reducers";
import { ChainNetworks } from "../../../requests/types";
import "./RPCRightPanel.css";

interface OwnProps {
  className?: string;
  rpc?: {
    method: string;
    params: object[] | object;
  };
}

interface StateProps {
  chainNetworks: ChainNetworks | undefined;
}

interface State {
  checkedNodeList: string[];
  checkedSelectedNodeList: string[];
  selectedNodeList: string[];
  jsonRPCInput: string;
}

interface DispatchProps {
  getChainNetworks: () => void;
}

type Props = OwnProps & StateProps & DispatchProps;

class RPCRightPanel extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      checkedNodeList: [],
      checkedSelectedNodeList: [],
      selectedNodeList: [],
      jsonRPCInput: props.rpc ? this.createJSONRPCString(props.rpc) : ""
    };
  }
  public componentDidUpdate(prevProps: Props) {
    if (this.props.rpc && this.props.rpc !== prevProps.rpc) {
      this.setState({
        jsonRPCInput: this.createJSONRPCString(this.props.rpc)
      });
    }
  }
  public componentDidMount() {
    this.props.getChainNetworks();
  }
  public render() {
    const { className, chainNetworks } = this.props;
    const {
      selectedNodeList,
      checkedNodeList,
      checkedSelectedNodeList,
      jsonRPCInput
    } = this.state;
    return (
      <div className={`${className} rpc-right-panel`}>
        <div>
          <Label for="rpc-input">JSON-RPC</Label>
          <a
            href="https://github.com/CodeChain-io/codechain/blob/master/spec/JSON-RPC.md"
            target="_blank"
            rel="noopener noreferrer"
            className="float-right"
          >
            JSON-RPC List
          </a>
          <textarea
            className="form-control rpc-input"
            aria-label="RPC area"
            placeholder="Type json rpc"
            id="rpc-input"
            value={jsonRPCInput}
            onChange={this.handleChangeJSONInput}
          />
        </div>
        <div>
          <div className="form-group mt-3">
            <Row>
              <Col sm={5}>
                <Label for="rpc-node-list">Running node list</Label>
                <select
                  multiple={true}
                  className="form-control"
                  id="rpc-node-list"
                  value={checkedNodeList}
                  onChange={this.handleCheckOnNodeList}
                >
                  {chainNetworks ? (
                    _.map(this.getRemainAvailableNodeList(), node => (
                      <option key={node.name} value={node.name}>
                        {node.name}
                      </option>
                    ))
                  ) : (
                    <option>Loading...</option>
                  )}
                </select>
              </Col>
              <Col sm={2} className="d-flex align-items-end">
                <div className="w-100">
                  <div className="d-flex justify-content-center">
                    <button
                      type="button"
                      className="btn btn-secondary select-btn"
                      onClick={this.moveRightAll}
                    >
                      <FontAwesomeIcon icon={faAngleDoubleRight} />
                    </button>
                  </div>
                  <div className="d-flex mt-1 justify-content-center">
                    <button
                      type="button"
                      className="btn btn-secondary select-btn"
                      onClick={this.moveRight}
                    >
                      <FontAwesomeIcon icon={faAngleRight} />
                    </button>
                  </div>
                  <div className="d-flex mt-1 justify-content-center">
                    <button
                      type="button"
                      className="btn btn-secondary select-btn"
                      onClick={this.moveLeft}
                    >
                      <FontAwesomeIcon icon={faAngleLeft} />
                    </button>
                  </div>
                  <div className="d-flex mt-1 justify-content-center">
                    <button
                      type="button"
                      className="btn btn-secondary select-btn"
                      onClick={this.moveLeftAll}
                    >
                      <FontAwesomeIcon icon={faAngleDoubleLeft} />
                    </button>
                  </div>
                </div>
              </Col>
              <Col sm={5}>
                <Label for="rpc-selected-node">Selected node</Label>
                <select
                  multiple={true}
                  className="form-control"
                  id="rpc-selected-node"
                  value={checkedSelectedNodeList}
                  onChange={this.handleCheckOnSelectedNodeList}
                >
                  {_.map(selectedNodeList, node => (
                    <option key={node} value={node}>
                      {node}
                    </option>
                  ))}
                </select>
              </Col>
            </Row>
          </div>
        </div>
        <div className="d-flex justify-content-end">
          <button type="button" className="btn btn-primary">
            Send
          </button>
        </div>
        <div>
          <Label for="response-input">Response</Label>
          <div className="rpc-response-tab-container">
            <div className="rpc-response-tab d-inline-block active">
              <span>Agent 1 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 2 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 3 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 3 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 3 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 3 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 3 response</span>
            </div>
            <div className="rpc-response-tab d-inline-block">
              <span>Agent 5 response</span>
            </div>
          </div>
          <textarea
            className="form-control rpc-response"
            aria-label="response area"
            readOnly={true}
            id="response-input"
            placeholder="Send json rpc"
          />
        </div>
      </div>
    );
  }

  private createJSONRPCString = (rpc: {
    method: string;
    params: object[] | object;
  }) => {
    const jsonForamt = {
      jsonrpc: "2.0",
      method: rpc.method,
      params: rpc.params,
      id: null
    };
    return JSON.stringify(jsonForamt, null, 2);
  };

  private getAvailableNodeList = () => {
    if (!this.props.chainNetworks) {
      return [];
    }
    return _.filter(
      this.props.chainNetworks.nodes,
      node => node.status === "Run"
    );
  };

  private getRemainAvailableNodeList = () => {
    if (!this.props.chainNetworks) {
      return [];
    }
    return _.filter(
      this.props.chainNetworks.nodes,
      node =>
        !_.includes(this.state.selectedNodeList, node.name) &&
        node.status === "Run"
    );
  };

  private handleCheckOnNodeList = (e: any) => {
    const options = e.target.options;
    const value = [];
    for (let i = 0, l = options.length; i < l; i++) {
      if (options[i].selected) {
        value.push(options[i].value);
      }
    }
    this.setState({
      checkedNodeList: value
    });
  };

  private handleCheckOnSelectedNodeList = (e: any) => {
    const options = e.target.options;
    const value = [];
    for (let i = 0, l = options.length; i < l; i++) {
      if (options[i].selected) {
        value.push(options[i].value);
      }
    }
    this.setState({
      checkedSelectedNodeList: value
    });
  };

  private handleChangeJSONInput = (e: any) => {
    this.setState({ jsonRPCInput: e.target.value });
  };

  private moveRightAll = () => {
    const availableNodeNames = _.map(
      this.getAvailableNodeList(),
      node => node.name
    );
    this.setState({ selectedNodeList: availableNodeNames });
    this.clearSelection();
  };

  private moveLeftAll = () => {
    this.setState({ selectedNodeList: [] });
    this.clearSelection();
  };

  private moveRight = () => {
    this.setState({
      selectedNodeList: _.concat(
        this.state.selectedNodeList,
        this.state.checkedNodeList
      )
    });
    this.clearSelection();
  };

  private moveLeft = () => {
    this.setState({
      selectedNodeList: _.difference(
        this.state.selectedNodeList,
        this.state.checkedSelectedNodeList
      )
    });
    this.clearSelection();
  };

  private clearSelection = () => {
    this.setState({ checkedNodeList: [], checkedSelectedNodeList: [] });
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
)(RPCRightPanel);
