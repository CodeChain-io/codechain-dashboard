import * as React from "react";
import "./ConnectChart.css";

import { faCodeBranch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

interface Props {
  className?: string;
}
export const ConnectChart = (props: Props) => {
  const { className } = props;
  return (
    <div className={`connect-chart ${className}`}>
      <div className="connect-chart-header">
        <h5 className="mb-0">
          <FontAwesomeIcon className="mr-2" icon={faCodeBranch} />
          Node Activity Graph
        </h5>
      </div>
      <div className="connect-chart-body">body</div>
    </div>
  );
};
