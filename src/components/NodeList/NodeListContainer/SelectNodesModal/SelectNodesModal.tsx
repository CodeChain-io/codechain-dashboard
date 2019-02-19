import * as _ from "lodash";
import * as React from "react";
import * as Modal from "react-modal";
import { ChainNetworks, NetworkNodeInfo } from "../../../../requests/types";
import SelectNodeCheckbox from "./SelectNodeCheckbox";

const customStyles = {
  content: {
    top: "50%",
    left: "50%",
    right: "auto",
    bottom: "auto",
    marginRight: "-50%",
    transform: "translate(-50%, -50%)"
  }
};

interface Props {
  onClose: () => void;
  isOpen: boolean;
  onSelectNodes: (selectedNodes: string[]) => void;
  chainNetworks: ChainNetworks;
}

interface State {
  selectedNodes: { [index: string]: boolean };
}

export default class SelectNodesModal extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      selectedNodes: _.chain(props.chainNetworks.nodes)
        .map((nodeInfo: NetworkNodeInfo) => {
          return [nodeInfo.name, false];
        })
        .fromPairs()
        .value()
    };
  }

  public render() {
    const { isOpen, onClose, chainNetworks } = this.props;
    const { selectedNodes } = this.state;
    return (
      <div>
        <Modal
          isOpen={isOpen}
          onRequestClose={onClose}
          style={customStyles}
          contentLabel="Select nodes to upgrade"
        >
          <div className="select-nodes-modal animated fadeIn">
            <div className={"select-nodes-modal-nodes"}>
              {_.map(chainNetworks.nodes, (nodeInfo: NetworkNodeInfo) => {
                return (
                  <SelectNodeCheckbox
                    name={nodeInfo.name}
                    checked={selectedNodes[nodeInfo.name]}
                    onChange={this.handleNodeCheckboxChange}
                  />
                );
              })}
            </div>
            <button onClick={this.handleSelectAllClick} className="btn mr-3">
              Select All
            </button>
            <button onClick={this.handleDeselectAllClick} className="btn mr-3">
              Deselect All
            </button>
            <button onClick={this.handleConfirmClick} className="btn">
              Upgrade
            </button>
          </div>
        </Modal>
      </div>
    );
  }

  private handleNodeCheckboxChange = (name: string) => {
    const prevState = this.state.selectedNodes[name];
    this.setState({
      selectedNodes: {
        ...this.state.selectedNodes,
        [name]: !prevState
      }
    });
  };

  private handleSelectAllClick = () => {
    const prevSelection = this.state.selectedNodes;
    this.setState({
      selectedNodes: _.mapValues(prevSelection, () => true)
    });
  };

  private handleDeselectAllClick = () => {
    const prevSelection = this.state.selectedNodes;
    this.setState({
      selectedNodes: _.mapValues(prevSelection, () => false)
    });
  };

  private handleConfirmClick = () => {
    const selectedNodeNames = _.chain(this.state.selectedNodes)
      .pickBy(selected => selected)
      .keys()
      .value();
    _.pickBy(this.state.selectedNodes);
    this.props.onSelectNodes(selectedNodeNames);
  };
}
