import * as _ from "lodash";
import * as React from "react";
import { Doughnut, HorizontalBar } from "react-chartjs-2";
import { JsonRPCError } from "../../../../RequestAgent";
import { Apis } from "../../../../requests";
import {
  NodeInfo,
  NodeStatus,
  UpdateCodeChainRequest
} from "../../../../requests/types";
import "./NodeDetail.css";
import StartNodeModal from "./StartNodeModal/StartNodeModal";
const { confirmAlert } = require("react-confirm-alert");
import { faSpinner } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { Parcel } from "codechain-sdk/lib/core/classes";
import { toast } from "react-toastify";
import { getStatusClass } from "../../../../utils/getStatusClass";
import UpgradeNodeModal from "../../UpgradeNodeModal/UpgradeNodeModal";

interface Props {
  nodeInfo: NodeInfo;
  className?: string;
}

interface State {
  isStartNodeModalOpen: boolean;
  isUpgradeNodeModalOpen: boolean;
}

enum NodeStartErrors {
  AlreadyRunning = -10001,
  EnvParseError = -10002
}

const getGBNumber = (byte: number) => {
  return Math.floor((byte / 1024 / 1024 / 1024) * 100) / 100;
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
    Math.floor((memoryUsage.available / 1024 / 1024 / 1024) * 100) / 100;
  const usedGB =
    Math.floor(
      ((memoryUsage.total - memoryUsage.available) / 1024 / 1024 / 1024) * 100
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
  private logFileHost = `${
    process.env.REACT_APP_LOG_SERVER_HOST
      ? process.env.REACT_APP_LOG_SERVER_HOST
      : "http://localhost:5012"
  }`;
  public constructor(props: Props) {
    super(props);
    this.state = {
      isStartNodeModalOpen: false,
      isUpgradeNodeModalOpen: false
    };
  }
  public render() {
    const { className, nodeInfo } = this.props;
    const { isStartNodeModalOpen, isUpgradeNodeModalOpen } = this.state;
    return (
      <div className={`node-detail d-flex ${className}`}>
        <StartNodeModal
          isOpen={isStartNodeModalOpen}
          onClose={this.handleOnClose}
          onAfterOpen={this.handleOnAfterOpen}
          onStartNode={this.handleOnStartNode}
          startOption={nodeInfo.startOption}
        />
        {nodeInfo.version && (
          <UpgradeNodeModal
            isOpen={isUpgradeNodeModalOpen}
            currentCommitHash={nodeInfo.version.hash}
            onClose={this.handleOnCloseUpgradeModal}
            onUpdateNode={this.handleOnUpdateNode}
          />
        )}
        <div className="left-panel">
          <div className="data-row mb-1">
            <div>
              <h4>
                Status:{" "}
                <span className={`mr-3 ${getStatusClass(nodeInfo.status)}`}>
                  {(nodeInfo.status === "Starting" ||
                    nodeInfo.status === "Updating") && (
                    <FontAwesomeIcon className="mr-1 spin" icon={faSpinner} />
                  )}
                  {nodeInfo.status}
                </span>
                {this.getButtonByStatus(nodeInfo.status)}
              </h4>
            </div>
            <div>
              <h5>
                {nodeInfo.address
                  ? `${nodeInfo.name}(${nodeInfo.address})`
                  : nodeInfo.name}
              </h5>
            </div>
          </div>
          {nodeInfo.startOption && (
            <div>
              <span>Environment variables</span>
              <div className="data-container mb-2">
                {nodeInfo.startOption.env}
              </div>
              <span>Arguments</span>
              <div className="data-container mb-2">
                {nodeInfo.startOption.args}
              </div>
            </div>
          )}
          <div className="text-right">
            <a
              target="_blank"
              className="link-text"
              href={`${this.logFileHost}/log/${nodeInfo.name}`}
            >
              Show logs
            </a>
          </div>
          <hr />
          <div className="data-row">
            <div>Version</div>
            <div>{nodeInfo.version ? nodeInfo.version.version : "Unknown"}</div>
          </div>
          <div className="data-row">
            <div>Hash</div>
            <div>
              {nodeInfo.version ? (
                <span className="link-text" onClick={this.openUpgradeNodeModal}>
                  {nodeInfo.version.hash.slice(0, 6)}
                </span>
              ) : (
                "Unknown"
              )}
            </div>
          </div>
          <div className="data-row mb-3">
            <div>Binary Checksum</div>
            <div>
              {nodeInfo.version ? (
                <span className="link-text" onClick={this.openUpgradeNodeModal}>
                  {nodeInfo.version.binaryChecksum.slice(0, 6)}
                </span>
              ) : (
                "Unknown"
              )}
            </div>
          </div>
          <hr />
          <div className="data-row">
            <div>Best block number</div>
            <div>
              {nodeInfo.bestBlockId
                ? nodeInfo.bestBlockId.blockNumber
                : "Unknown"}
            </div>
          </div>
          <div className="data-row">
            <div>Best block hash</div>
            <div>
              {nodeInfo.bestBlockId
                ? nodeInfo.bestBlockId.hash.slice(0, 8)
                : "Unknown"}
            </div>
          </div>
          <div className="data-row mb-1">
            <div>Pending parcels</div>
            <div>
              {nodeInfo.pendingParcels ? nodeInfo.pendingParcels.length : 0}
            </div>
          </div>
          <div className="data-container mb-3">
            {nodeInfo.pendingParcels
              ? _.map(
                  nodeInfo.pendingParcels,
                  pendingParcel => Parcel.fromJSON(pendingParcel).hash().value
                ).join(" ")
              : ""}
          </div>
          <hr />
          <div className="data-row">
            <div>Peer count</div>
            <div>{nodeInfo.peers ? nodeInfo.peers.length : "Unknown"}</div>
          </div>
          <div className="data-row mb-1">
            <div>Peer list</div>
          </div>
          <div className="data-container mb-3">
            {nodeInfo.peers ? nodeInfo.peers.join(" ") : ""}
          </div>
          <div className="data-row mb-1">
            <div>Whitelist</div>
            <div>
              {nodeInfo.whitelist
                ? nodeInfo.whitelist.enabled
                  ? "Enabled"
                  : "Disabled"
                : "Unknown"}
            </div>
          </div>
          <div className="data-container mb-3">
            {nodeInfo.whitelist
              ? _.map(nodeInfo.whitelist.list, whitelist =>
                  whitelist.join(",")
                ).join(" ")
              : ""}
          </div>
          <div className="data-row mb-1">
            <div>Blacklist</div>
            <div>
              {nodeInfo.blacklist
                ? nodeInfo.blacklist.enabled
                  ? "Enabled"
                  : "Disabled"
                : "Unknown"}
            </div>
          </div>
          <div className="data-container mb-3">
            {nodeInfo.blacklist
              ? _.map(nodeInfo.blacklist.list, blacklist =>
                  blacklist.join(",")
                ).join(" ")
              : ""}
          </div>
        </div>
        <div className="right-panel">
          <div className="text-right">Agent : v0.1.0</div>
          <div className="mt-5 mb-5 d-flex align-items-center">
            <div className="chart-data-container d-flex justify-content-center">
              <div className="chart-data">
                <h5>CPU usage</h5>
                {nodeInfo.hardware
                  ? _.map(nodeInfo.hardware.cpuUsage, (usage, index) => (
                      <p key={`cpu-usage-p-${index}`} className="mb-0">
                        {`CPU-${index} : ${Math.floor(usage * 100 * 100) /
                          100}`}
                        (%)
                      </p>
                    ))
                  : "Unknown"}
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
                  data={
                    nodeInfo.hardware
                      ? getCpuUsage(nodeInfo.hardware.cpuUsage)
                      : {
                          datasets: [
                            {
                              data: []
                            }
                          ],
                          labels: []
                        }
                  }
                />
              </div>
            </div>
          </div>
          <div className="mt-5 mb-5 d-flex align-items-center">
            <div className="chart-data-container d-flex justify-content-center">
              <div className="chart-data">
                <h5>Disk usage</h5>
                <p className="mb-0">
                  Total pace :{" "}
                  {nodeInfo.hardware
                    ? getGBNumber(nodeInfo.hardware.diskUsage.total)
                    : "Unknown"}
                  (GB)
                </p>
                <p className="mb-0">
                  Used space :{" "}
                  {nodeInfo.hardware
                    ? getGBNumber(
                        nodeInfo.hardware.diskUsage.total -
                          nodeInfo.hardware.diskUsage.available
                      )
                    : "Unknown"}
                  (GB)
                </p>
                <p className="mb-0">
                  Free space :{" "}
                  {nodeInfo.hardware
                    ? getGBNumber(nodeInfo.hardware.diskUsage.available)
                    : "Unknown"}
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
                  data={
                    nodeInfo.hardware
                      ? getDistUsage(nodeInfo.hardware.diskUsage)
                      : {
                          datasets: [
                            {
                              data: []
                            }
                          ],
                          labels: []
                        }
                  }
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
                  {nodeInfo.hardware
                    ? getGBNumber(nodeInfo.hardware.memoryUsage.total)
                    : "Unknown"}
                  (GB)
                </p>
                <p className="mb-0">
                  Used space :{" "}
                  {nodeInfo.hardware
                    ? getGBNumber(
                        nodeInfo.hardware.memoryUsage.total -
                          nodeInfo.hardware.memoryUsage.available
                      )
                    : "Unknown"}
                  (GB)
                </p>
                <p className="mb-0">
                  Free space :{" "}
                  {nodeInfo.hardware
                    ? getGBNumber(nodeInfo.hardware.memoryUsage.available)
                    : "Unknown"}
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
                  data={
                    nodeInfo.hardware
                      ? getMemoryUsage(nodeInfo.hardware.memoryUsage)
                      : {
                          datasets: [
                            {
                              data: []
                            }
                          ],
                          labels: []
                        }
                  }
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }
  private handleOnStartNode = async (env: string, args: string) => {
    const name = this.props.nodeInfo.name;
    try {
      await Apis.startNode(name, env, args);
      this.setState({ isStartNodeModalOpen: false });
    } catch (e) {
      const error = e as JsonRPCError;
      if (error.code === NodeStartErrors.AlreadyRunning) {
        toast.error("The node is already running.");
      } else if (error.code === NodeStartErrors.EnvParseError) {
        toast.error("Env parsing error.");
      }
    }
  };
  private handleOnAfterOpen = () => {
    console.log("Get starting env history");
  };
  private handleOnClose = () => {
    this.setState({ isStartNodeModalOpen: false });
  };
  private handleOnCloseUpgradeModal = () => {
    this.setState({ isUpgradeNodeModalOpen: false });
  };
  private openStartNodeModal = () => {
    this.setState({ isStartNodeModalOpen: true });
  };
  private openUpgradeNodeModal = () => {
    this.setState({ isUpgradeNodeModalOpen: true });
  };
  private handleOnUpdateNode = async (req: UpdateCodeChainRequest) => {
    const name = this.props.nodeInfo.name;
    await Apis.updateNode(name, req);
    this.setState({ isUpgradeNodeModalOpen: false });
  };
  private getButtonByStatus = (status: NodeStatus) => {
    switch (status) {
      case "Run":
      case "Starting":
        return (
          <button
            type="button"
            onClick={this.onStop}
            className="btn btn-secondary status-btn"
          >
            Stop
          </button>
        );
      case "Stop":
      case "Error":
        return (
          <button
            type="button"
            onClick={this.openStartNodeModal}
            className="btn btn-secondary status-btn"
          >
            Start
          </button>
        );
      case "Updating":
        return null;
    }
    throw Error("Invalid status");
  };

  private onStop = () => {
    confirmAlert({
      title: "Are you sure?",
      message: "The node will be shut down.",
      buttons: [
        {
          label: "Yes",
          onClick: async () => {
            try {
              await Apis.stopNode(this.props.nodeInfo.name);
            } catch (e) {
              console.log(e);
            }
          }
        },
        {
          label: "No"
        }
      ]
    });
  };
}
