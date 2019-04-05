import * as React from "react";
import LeftFilter from "./LeftFilter/LeftFilter";
import "./Log.css";
import LogViewer from "./LogViewer/LogViewer";
import TopFilter from "./TopFilter/TopFilter";

export default class Log extends React.Component<any, any> {
  public render() {
    return (
      <div className="log">
        <div className="mb-3">
          <TopFilter />
        </div>
        <div className="d-flex">
          <div className="left mr-3">
            <LeftFilter />
          </div>
          <div className="right">
            <LogViewer />
          </div>
        </div>
      </div>
    );
  }
}
