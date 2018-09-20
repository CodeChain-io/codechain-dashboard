import * as _ from "lodash";
import * as React from "react";
import { Doughnut, HorizontalBar } from "react-chartjs-2";
import { JsonRPCError } from "../../../../RequestAgent";
import { Apis } from "../../../../requests";
import { NodeInfo, NodeStatus } from "../../../../requests/types";
import "./NodeDetail.css";
import StartNodeModal from "./StartNodeModal/StartNodeModal";
const { confirmAlert } = require("react-confirm-alert");

interface Props {
  nodeInfo: NodeInfo;
  className?: string;
}

interface State {
  isStartNodeModalOpen: boolean;
}

enum NodeStartErrors {
  AlreadyRunning = -10001,
  EnvParseError = -10002
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

const getGBNumber = (byte: number) => {
  return Math.floor((byte / 1000 / 1000 / 1000) * 100) / 100;
};

const getDistUsage = (diskUsage: {
  total: number;
  available: number;
  percentageUsed: number;
}) => {
  const availableGB = getGBNumber(diskUsage.available);
  const usedGB = getGBNumber(diskUsage.total - diskUsage.available);

  return {
    datasets: [
      {
        data: [usedGB, availableGB],
        backgroundColor: [
          "#CCCCCC",
          `${availableGB < 0.5 ? "#dc3545" : "#54A2E5"}`
        ]
      }
    ],
    // These labels appear in the legend and in the tooltips when hovering different arcs
    labels: ["Used Space", "Free Space"]
  };
};

const getMemoryUsage = (memoryUsage: {
  total: number;
  available: number;
  percentageUsed: number;
}) => {
  const availableGB =
    Math.floor((memoryUsage.available / 1000 / 1000 / 1000) * 100) / 100;
  const usedGB =
    Math.floor(
      ((memoryUsage.total - memoryUsage.available) / 1000 / 1000 / 1000) * 100
    ) / 100;
  return {
    datasets: [
      {
        data: [usedGB, availableGB],
        backgroundColor: [
          "#CCCCCC",
          `${availableGB < 1 ? "#dc3545" : "#54A2E5"}`
        ]
      }
    ],
    // These labels appear in the legend and in the tooltips when hovering different arcs
    labels: ["Used Space", "Free Space"]
  };
};

const getCpuUsage = (cpuUsage: number[]) => {
  const dataSet = _.map(cpuUsage, usage => Math.floor(usage * 100 * 100) / 100);
  const labels = _.map(cpuUsage, (usage, index) => `CPU-${index}`);
  const colors = _.map(
    cpuUsage,
    (usage, index) => (usage < 0.9 ? "#54A2E5" : "#dc3545")
  );
  return {
    datasets: [
      {
        data: dataSet,
        backgroundColor: colors
      }
    ],
    // These labels appear in the legend and in the tooltips when hovering different arcs
    labels
  };
};

export default class NodeDetail extends React.Component<Props, State> {
  public constructor(props: Props) {
    super(props);
    this.state = {
      isStartNodeModalOpen: false
    };
  }
  public render() {
    const { className, nodeInfo } = this.props;
    const { isStartNodeModalOpen } = this.state;
    return (
      <div className={`node-detail d-flex ${className}`}>
        <StartNodeModal
          isOpen={isStartNodeModalOpen}
          onClose={this.handleOnClose}
          onAfterOpen={this.handleOnAfterOpen}
          onStartNode={this.handleOnStartNode}
        />
        <div className="left-panel">
          <div className="data-row mb-1">
            <div>
              <h4>
                Status:{" "}
                <span className={`mr-3 ${getStatusClass(nodeInfo.status)}`}>
                  {nodeInfo.status}
                </span>
                {this.getButtonByStatus(nodeInfo.status)}
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
            Dummy - RUST_LOG="info,network=trace,miner=trace,sync=trace" cargo
            run -- --port 3485 --jsonrpc-port 8081 -c husky --author
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
          <div className="mt-5 mb-5 d-flex align-items-center">
            <div className="chart-data-container d-flex justify-content-center">
              <div className="chart-data">
                <h5>CPU usage</h5>
                {_.map(nodeInfo.hardware.cpuUsage, (usage, index) => (
                  <p key={`cpu-usage-p-${index}`} className="mb-0">
                    {`CPU-${index} : ${Math.floor(usage * 100 * 100) / 100}`}
                    (%)
                  </p>
                ))}
              </div>
            </div>
            <div className="chart-container d-flex justify-content-center">
              <div className="doughnut-chart">
                <HorizontalBar
                  options={{
                    legend: {
                      display: false
                    },
                    scales: {
                      xAxes: [
                        {
                          ticks: {
                            max: 100,
                            min: 0
                          }
                        }
                      ]
                    }
                  }}
                  data={getCpuUsage(nodeInfo.hardware.cpuUsage)}
                />
              </div>
            </div>
          </div>
          <div className="mt-5 mb-5 d-flex align-items-center">
            <div className="chart-data-container d-flex justify-content-center">
              <div className="chart-data">
                <h5>Disk usage</h5>
                <p className="mb-0">
                  Total pace : {getGBNumber(nodeInfo.hardware.diskUsage.total)}
                  (GB)
                </p>
                <p className="mb-0">
                  Used space :{" "}
                  {getGBNumber(
                    nodeInfo.hardware.diskUsage.total -
                      nodeInfo.hardware.diskUsage.available
                  )}
                  (GB)
                </p>
                <p className="mb-0">
                  Free space :{" "}
                  {getGBNumber(nodeInfo.hardware.diskUsage.available)}
                  (GB)
                </p>
              </div>
            </div>
            <div className="chart-container d-flex justify-content-center">
              <div className="doughnut-chart">
                <Doughnut
                  options={{
                    legend: {
                      position: "bottom"
                    }
                  }}
                  data={getDistUsage(nodeInfo.hardware.diskUsage)}
                />
              </div>
            </div>
          </div>
          <div className="d-flex align-items-center mb-5">
            <div className="chart-data-container d-flex justify-content-center">
              <div className="chart-data">
                <h5>Memory usage</h5>
                <p className="mb-0">
                  Total space :{" "}
                  {getGBNumber(nodeInfo.hardware.memoryUsage.total)}
                  (GB)
                </p>
                <p className="mb-0">
                  Used space :{" "}
                  {getGBNumber(
                    nodeInfo.hardware.memoryUsage.total -
                      nodeInfo.hardware.memoryUsage.available
                  )}
                  (GB)
                </p>
                <p className="mb-0">
                  Free space :{" "}
                  {getGBNumber(nodeInfo.hardware.memoryUsage.available)}
                  (GB)
                </p>
              </div>
            </div>
            <div className="chart-container d-flex justify-content-center">
              <div className="doughnut-chart">
                <Doughnut
                  options={{
                    legend: {
                      position: "bottom"
                    }
                  }}
                  data={getMemoryUsage(nodeInfo.hardware.memoryUsage)}
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
  private handleOnStartNode = async (env: string, args: string) => {
    const address = this.props.nodeInfo.address;
    try {
      await Apis.startNode(address, env, args);
    } catch (e) {
      const error = e as JsonRPCError;
      if (error.code === NodeStartErrors.AlreadyRunning) {
        console.log("Already running");
      } else if (error.code === NodeStartErrors.EnvParseError) {
        console.log("Env parsing error");
      }
    }
    this.setState({ isStartNodeModalOpen: false });
  };
  private handleOnAfterOpen = () => {
    console.log("Get starting env history");
  };
  private handleOnClose = () => {
    this.setState({ isStartNodeModalOpen: false });
  };
  private openStartNodeModal = () => {
    this.setState({ isStartNodeModalOpen: true });
  };
  private getButtonByStatus = (status: NodeStatus) => {
    switch (status) {
      case "Run":
        return (
          <button
            type="button"
            onClick={this.onStop}
            className="btn btn-secondary status-btn"
          >
            Stop
          </button>
        );
      case "Stop" || "Error":
        return (
          <button
            type="button"
            onClick={this.openStartNodeModal}
            className="btn btn-secondary status-btn"
          >
            Start
          </button>
        );
    }
    return "text-warning";
  };

  private onStop = () => {
    confirmAlert({
      title: "Are you sure?",
      message: "The node will be shut down.",
      buttons: [
        {
          label: "Yes",
          onClick: async () => {
            await Apis.stopNode(this.props.nodeInfo.address);
          }
        },
        {
          label: "No"
        }
      ]
    });
  };
}
