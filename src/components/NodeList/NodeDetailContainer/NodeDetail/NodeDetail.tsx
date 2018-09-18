import * as React from "react";
import { NodeInfo, NodeStatus } from "../../../../requests/types";
import "./NodeDetail.css";

interface Props {
  nodeInfo: NodeInfo;
  className?: string;
}

const getStatusClass = (status: NodeStatus) => {
  switch (status) {
    case "Run":
      return "text-success";
    case "Stop":
      return "text-secondary";
    case "Error":
      return "text-danger";
  }
  return "text-warning";
};

const getButtonByStatus = (status: NodeStatus) => {
  switch (status) {
    case "Run":
      return (
        <button type="button" className="btn btn-secondary status-btn">
          Stop
        </button>
      );
    case "Stop":
      return (
        <button type="button" className="btn btn-secondary status-btn">
          Start
        </button>
      );
    case "Error":
      return (
        <button type="button" className="btn btn-secondary status-btn">
          Start
        </button>
      );
  }
  return "text-warning";
};

export default (props: Props) => {
  const { className, nodeInfo } = props;
  return (
    <div className={`node-detail d-flex ${className}`}>
      <div className="left-panel">
        <div className="data-row mb-1">
          <div>
            <h4>
              Status:{" "}
              <span className={`mr-3 ${getStatusClass(nodeInfo.status)}`}>
                {nodeInfo.status}
              </span>
              {getButtonByStatus(nodeInfo.status)}
            </h4>
          </div>
          <div>
            <h4>
              {nodeInfo.name
                ? `${nodeInfo.name}(${nodeInfo.address})`
                : nodeInfo.address}
            </h4>
          </div>
        </div>
        <div className="data-container mb-3">
          Dummy - RUST_LOG="info,network=trace,miner=trace,sync=trace" cargo run
          -- --port 3485 --jsonrpc-port 8081 -c husky --author
          tccqplm67eps3yaryxu7fajdl9q7r3pgn8f0vcguptc --notify-work
          "http://127.0.0.1:3333" --whitelist-path whitelist.txt
          --bootstrap-addresses 13.124.101.76:3485 --force-sealing
          --reseal-min-period 4000
        </div>
        <hr />
        <div className="data-row">
          <div>Version</div>
          <div>{nodeInfo.version.version}</div>
        </div>
        <div className="data-row mb-3">
          <div>Hash</div>
          <div>{nodeInfo.version.hash}</div>
        </div>
        <hr />
        <div className="data-row">
          <div>Best block number</div>
          <div>{nodeInfo.bestBlockId.blockNumber}</div>
        </div>
        <div className="data-row">
          <div>Best block hash</div>
          <div>{nodeInfo.bestBlockId.hash}</div>
        </div>
        <div className="data-row mb-1">
          <div>Pending parcels</div>
          <div>{nodeInfo.pendingParcels.length}</div>
        </div>
        <div className="data-container mb-3">
          Dummy - Parcel1, Parcel2, Parcel3 ...
          <br />
          Dummy - Parcel1, Parcel2, Parcel3 ...
          <br />
          Dummy - Parcel1, Parcel2, Parcel3 ...
        </div>
        <hr />
        <div className="data-row">
          <div>Peer count</div>
          <div>{nodeInfo.peers.length}</div>
        </div>
        <div className="data-row mb-1">
          <div>Peer list</div>
        </div>
        <div className="data-container mb-3">{nodeInfo.peers.join(" ")}</div>
        <div className="data-row mb-1">
          <div>Whitelist</div>
          <div>{nodeInfo.whitelist.enabled ? "Enabled" : "Disabled"}</div>
        </div>
        <div className="data-container mb-3">
          {nodeInfo.whitelist.list.join(" ")}
        </div>
        <div className="data-row mb-1">
          <div>Blacklist</div>
          <div>{nodeInfo.blacklist.enabled ? "Enabled" : "Disabled"}</div>
        </div>
        <div className="data-container mb-3">
          {nodeInfo.blacklist.list.join(" ")}
        </div>
      </div>
      <div className="right-panel">
        <div>
          <div>Hardware info</div>
        </div>
      </div>
    </div>
  );
};
