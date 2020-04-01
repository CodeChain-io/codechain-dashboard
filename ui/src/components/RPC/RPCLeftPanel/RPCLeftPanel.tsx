import * as _ from "lodash";
import * as React from "react";
import "./RPCLeftPanel.css";

interface Props {
  className?: string;
  onClickHistoryItem: (rpc: {
    method: string;
    params: object[] | object;
  }) => void;
}
export class RPCLeftPanel extends React.Component<Props> {
  public render() {
    const { className } = this.props;
    return (
      <div className={`${className} rpc-left-panel d-flex`}>
        <div className="button-container d-flex align-items-bottom">
          <div className="history-button text-center active">History</div>
          <div className="collection-button text-center">Collections</div>
        </div>
        <div className="history-container">
          {_.map(_.range(10), index => {
            return (
              <div
                key={index}
                className="history-item"
                onClick={this.onClickItem}
              >
                <p className="history-item-name mb-0">
                  Dummy_getBestBlockNumber
                </p>
                <p className="history-item-params mb-0">
                  parameter1 parameter2 parameter3 parameter4
                </p>
                <p className="history-item-node-list mb-0">
                  agent1 agent2 agent3 agent4 agent5 agant6
                </p>
              </div>
            );
          })}
        </div>
      </div>
    );
  }

  private onClickItem = () => {
    this.props.onClickHistoryItem({
      method: "Dummy_getBestBlockNumber",
      params: ["DummyParam", 1, "number", "123"]
    });
  };
}

export default RPCLeftPanel;
