import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import { Label } from "reactstrap";
import { fetchChainNetworksIfNeeded } from "../../../actions/chainNetworks";
import {
  changeFilters,
  fetchTargetsIfNeeded,
  setNodeColor
} from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import { NodeInfo } from "../../../requests/types";
import ColorPicker from "./ColorPicker/ColorPicker";
import "./LeftFilter.css";

interface StateProps {
  selectedNodes: string[];
  selectedLevels: ("error" | "warn" | "info" | "debug" | "trace")[];
  selectedTargets: string[];
  nodes?: NodeInfo[] | null;
  targets?: string[] | null;
  nodeColors: {
    [nodeName: string]: string;
  };
}

interface DispatchProps {
  dispatch: any;
}

type Props = StateProps & DispatchProps;

class LeftFilter extends React.Component<Props, any> {
  public constructor(props: Props) {
    super(props);
  }

  public componentDidMount() {
    this.props.dispatch(fetchChainNetworksIfNeeded());
    this.props.dispatch(fetchTargetsIfNeeded());
  }

  public render() {
    const {
      nodes,
      selectedNodes,
      selectedLevels,
      targets,
      selectedTargets,
      nodeColors
    } = this.props;
    return (
      <div className="left-filter">
        <h5>Node</h5>
        <div>
          {nodes ? (
            <ul className="list-unstyled">
              {_.map(nodes, node => (
                <li key={node.name}>
                  <div className="form-check">
                    <div className="d-flex align-itmes-center">
                      <div>
                        <input
                          type="checkbox"
                          className="form-check-input"
                          id={`${node.name}-check-box`}
                          name={node.name}
                          checked={_.includes(selectedNodes, node.name)}
                          onChange={this.handleNodeCheck}
                        />
                        <Label
                          className="form-check-label"
                          for={`${node.name}-check-box`}
                        >
                          {node.name}
                        </Label>
                      </div>
                      <div className="ml-auto">
                        <ColorPicker
                          color={nodeColors[node.name] || "#ffffff"}
                          onColorSelected={_.partial(
                            this.handleColorChange,
                            node.name
                          )}
                        />
                      </div>
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            "Loading ... "
          )}
          <hr />
        </div>
        <h5>Debug level</h5>
        <div>
          <ul className="list-unstyled">
            {_.map(["error", "warn", "info", "debug", "trace"], level => (
              <li key={level}>
                <div className="form-check">
                  <input
                    type="checkbox"
                    className="form-check-input"
                    id={`${level}-check-box`}
                    name={level}
                    checked={_.includes(selectedLevels, level)}
                    onChange={this.handleLevelCheck}
                  />
                  <Label
                    className="form-check-label"
                    for={`${level}-check-box`}
                  >
                    {level}
                  </Label>
                </div>
              </li>
            ))}
          </ul>
          <hr />
        </div>
        <h5>Target</h5>
        <div>
          {targets ? (
            <ul className="list-unstyled">
              {_.map(targets, target => (
                <li key={target}>
                  <div className="form-check">
                    <input
                      type="checkbox"
                      className="form-check-input"
                      id={`${target}-check-box`}
                      name={target}
                      checked={_.includes(selectedTargets, target)}
                      onChange={this.handleTargetCheck}
                    />
                    <Label
                      className="form-check-label"
                      for={`${target}-check-box`}
                    >
                      {target}
                    </Label>
                  </div>
                </li>
              ))}
            </ul>
          ) : (
            "Loading..."
          )}
        </div>
      </div>
    );
  }
  private handleNodeCheck = (event: any) => {
    const { dispatch, selectedNodes } = this.props;
    const target = event.target;
    const name = target.name;
    if (target.checked) {
      dispatch(
        changeFilters({ filter: { nodeNames: _.concat(selectedNodes, name) } })
      );
    } else {
      dispatch(
        changeFilters({
          filter: { nodeNames: _.filter(selectedNodes, node => node !== name) }
        })
      );
    }
  };
  private handleLevelCheck = (event: any) => {
    const { dispatch, selectedLevels } = this.props;
    const target = event.target;
    const name = target.name;
    if (target.checked) {
      dispatch(
        changeFilters({ filter: { levels: _.concat(selectedLevels, name) } })
      );
    } else {
      dispatch(
        changeFilters({
          filter: { levels: _.filter(selectedLevels, level => level !== name) }
        })
      );
    }
  };
  private handleTargetCheck = (event: any) => {
    const { dispatch, selectedTargets } = this.props;
    const target = event.target;
    const name = target.name;
    if (target.checked) {
      dispatch(
        changeFilters({ filter: { targets: _.concat(selectedTargets, name) } })
      );
    } else {
      dispatch(
        changeFilters({
          filter: { targets: _.filter(selectedTargets, t => t !== name) }
        })
      );
    }
  };
  private handleColorChange = (nodeName: string, color: string) => {
    const { dispatch } = this.props;
    dispatch(setNodeColor(nodeName, color));
  };
}

const mapStateToProps = (state: ReducerConfigure) => ({
  selectedNodes: state.logReducer.filter.nodeNames,
  selectedLevels: state.logReducer.filter.levels,
  selectedTargets: state.logReducer.filter.targets,
  nodes:
    state.chainNetworksReducer.chainNetworks &&
    (state.chainNetworksReducer.chainNetworks.nodes as any[]),
  targets: state.logReducer.targets,
  nodeColors: state.logReducer.nodeColor
});
export default connect(mapStateToProps)(LeftFilter);
