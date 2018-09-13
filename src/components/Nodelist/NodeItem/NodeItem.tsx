import { faCircle, faCog } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as React from "react";
import "./NodeItem.css";
interface Props {
  className?: string;
}
const NodeItem = (props: Props) => {
  const { className } = props;
  return (
    <div className={`node-item d-flex ${className}`}>
      <div className="node-item-info-container d-flex align-items-center">
        <div className="node-status text-center text-success">
          <FontAwesomeIcon icon={faCircle} />
        </div>
        <div className="node-name">Node name</div>
        <div className="node-info text-right">
          <div>Best block number 20 (asibdb)</div>
          <div>v0.1.0 (bBsekf)</div>
        </div>
      </div>
      <div className="setting-btn-container d-flex justify-content-center">
        <FontAwesomeIcon className="align-self-center" icon={faCog} />
      </div>
    </div>
  );
};

export default NodeItem;
