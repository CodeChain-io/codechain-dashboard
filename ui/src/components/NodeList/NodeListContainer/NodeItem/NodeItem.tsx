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
            <div className="node-name">
              {nodeInfo.address
                ? `${nodeInfo.name} (${nodeInfo.address})`
                : nodeInfo.name}
            </div>
            <div className="node-info text-right">
              <a
                href="https://github.com/codechain-io/codechain-agent"
                target="_blank"
                rel="noopener noreferrer"
              >
                <button type="button" className="btn btn-secondary">
                  Install Agent
                </button>
              </a>
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
          <Link to={`/nodelist/${encodeURI(nodeInfo.name)}`}>
            <div className=" d-flex align-items-center">
              <div className="node-status text-center">
                <FontAwesomeIcon
                  className={getStatusClass(nodeInfo.status)}
                  icon={faCircle}
                />
              </div>
              <div className="node-name">
                {nodeInfo.address
                  ? `${nodeInfo.name} (${nodeInfo.address})`
                  : nodeInfo.name}
              </div>
              <div className="node-info text-right">
                <div>
                  Block:{" "}
                  {nodeInfo.bestBlockId
                    ? nodeInfo.bestBlockId.blockNumber
                    : "Unknown"}{" "}
                  (
                  {nodeInfo.bestBlockId
                    ? nodeInfo.bestBlockId.hash.substr(0, 6)
                    : "Unknown"}
                  )
                </div>
                <div>
                  Version:{" "}
                  {nodeInfo.version ? nodeInfo.version.version : "Unknown"} (
                  {nodeInfo.version
                    ? nodeInfo.version.hash.substr(0, 6)
                    : "Unknown"}
                  )
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
