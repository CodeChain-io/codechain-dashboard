import { faCircle, faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as React from "react";
import { Link } from "react-router-dom";
import { NetworkNodeInfo } from "../../../../requests/types";
import { getStatusClass } from "../../../../utils/getStatusClass";
import "./NodeItem.css";
interface Props {
  className?: string;
  nodeInfo: NetworkNodeInfo;
}

const NodeItem = (props: Props) => {
  const { className, nodeInfo } = props;
  if (nodeInfo.status === "UFO") {
    return (
      <div className={`node-item d-flex ${className}`}>
        <div className="node-item-info-container">
          <div className="d-flex align-items-center h-100">
            <div className="node-status text-center">
              <FontAwesomeIcon
                className={getStatusClass(nodeInfo.status)}
                icon={faCircle}
              />
            </div>
            <div className="node-name">{nodeInfo.address}</div>
            <div className="node-info text-right">
              <button type="button" className="btn btn-secondary">
                Install Agent
              </button>
            </div>
          </div>
        </div>
        <div className="setting-btn-container d-flex justify-content-center">
          <FontAwesomeIcon className="align-self-center" icon={faCog} />
        </div>
      </div>
    );
  } else {
    return (
      <div className={`node-item d-flex ${className}`}>
        <div className="node-item-info-container active">
          <Link to={`/nodelist/${encodeURI(nodeInfo.address)}`}>
            <div className=" d-flex align-items-center">
              <div className="node-status text-center">
                <FontAwesomeIcon
                  className={getStatusClass(nodeInfo.status)}
                  icon={faCircle}
                />
              </div>
              <div className="node-name">
                {nodeInfo.name
                  ? `${nodeInfo.name} (${nodeInfo.address})`
                  : nodeInfo.address}
              </div>
              <div className="node-info text-right">
                <div>
                  Best block number {nodeInfo.bestBlockId!.blockNumber} (
                  {nodeInfo.bestBlockId!.hash.value})
                </div>
                <div>
                  {nodeInfo.version!.version} (
                  {nodeInfo.version!.hash.substr(0, 6)})
                </div>
              </div>
            </div>
          </Link>
        </div>
        <div className="setting-btn-container d-flex justify-content-center">
          <FontAwesomeIcon className="align-self-center" icon={faCog} />
        </div>
      </div>
    );
  }
};

export default NodeItem;
