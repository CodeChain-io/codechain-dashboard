import * as React from "react";
import "./RPC.css";
import { RPCLeftPanel } from "./RPCLeftPanel/RPCLeftPanel";
import RPCRightPanel from "./RPCRightPanel/RPCRightPanel";

interface State {
  selectedItem?: {
    method: string;
    params: object[] | object;
  };
}
export default class RPC extends React.Component<{}, State> {
  public constructor(props: {}) {
    super(props);
    this.state = {
      selectedItem: undefined
    };
  }
  public render() {
    const { selectedItem } = this.state;
    return (
      <div className="rpc-container d-flex">
        <RPCLeftPanel
          className="left-panel"
          onClickHistoryItem={this.handleItemSelect}
        />
        <RPCRightPanel className="right-panel" rpc={selectedItem} />
      </div>
    );
  }

  private handleItemSelect = (rpc: {
    method: string;
    params: object[] | object;
  }) => {
    this.setState({ selectedItem: rpc });
  };
}
