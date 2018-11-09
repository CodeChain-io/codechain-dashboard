import { faArrowDown, faArrowUp } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as moment from "moment";
import * as React from "react";
import { connect } from "react-redux";
import Table from "reactstrap/lib/Table";
import { changeOrder } from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import { Log } from "../../../requests/types";
import "./LogViewer.css";

interface StateProps {
  orderBy: "DESC" | "ASC";
  logs?: Log[] | null;
  isFetchingLog: boolean;
  nodeColors: {
    [nodeName: string]: string;
  };
}

interface DispatchProps {
  dispatch: any;
}

type Props = StateProps & DispatchProps;
class LogViewer extends React.Component<Props, any> {
  public constructor(props: any) {
    super(props);
  }
  public render() {
    const { orderBy, isFetchingLog, logs, nodeColors } = this.props;
    return (
      <div className="log-viewer animated fadeIn">
        <Table>
          <thead>
            <tr>
              <th
                style={{ width: "180px" }}
                onClick={this.toggleOrder}
                className="date-table-header"
              >
                Date{" "}
                {orderBy === "DESC" ? (
                  <FontAwesomeIcon icon={faArrowDown} />
                ) : (
                  <FontAwesomeIcon icon={faArrowUp} />
                )}
              </th>
              <th style={{ width: "100px" }}>Node</th>
              <th style={{ width: "80px" }}>Status</th>
              <th style={{ width: "120px" }}>Target</th>
              <th>Message</th>
            </tr>
          </thead>
          <tbody>
            {logs &&
              _.map(logs, log => (
                <tr
                  key={log.id}
                  style={
                    nodeColors[log.nodeName]
                      ? {
                          backgroundColor: nodeColors[log.nodeName],
                          color: this.getColorByBackground(
                            nodeColors[log.nodeName]
                          )
                        }
                      : {
                          backgroundColor: "#ffffff",
                          color: "#000000"
                        }
                  }
                >
                  <td>{moment(log.timestamp).format("YYYY-MM-DD HH:mm:ss")}</td>
                  <td>{log.nodeName}</td>
                  <td>{log.level}</td>
                  <td>{log.target}</td>
                  <td className="message-col">{log.message}</td>
                </tr>
              ))}
            {isFetchingLog ? (
              <tr>
                <td colSpan={5} className="text-center">
                  Loading...
                </td>
              </tr>
            ) : (
              <tr>
                <td colSpan={5} className="text-center load-more">
                  Load More
                </td>
              </tr>
            )}
          </tbody>
        </Table>
      </div>
    );
  }
  private toggleOrder = () => {
    this.props.dispatch(
      changeOrder(this.props.orderBy === "DESC" ? "ASC" : "DESC")
    );
  };
  private getColorByBackground = (hexBackground: string) => {
    const brightness = this.getBrightness(this.hexToRgb(hexBackground));
    return brightness > 0.5 ? "#000000" : "#ffffff";
  };
  private hexToRgb = (hex: string) => {
    return hex
      .replace(
        /^#?([a-f\d])([a-f\d])([a-f\d])$/i,
        (m, r, g, b) => "#" + r + r + g + g + b + b
      )
      .substring(1)
      .match(/.{2}/g)!
      .map(x => parseInt(x, 16));
  };
  private getBrightness = (rgb: number[]) => {
    const R = rgb[0] / 255;
    const G = rgb[1] / 255;
    const B = rgb[2] / 255;
    return Math.sqrt(
      0.299 * Math.pow(R, 2.2) +
        0.587 * Math.pow(G, 2.2) +
        0.114 * Math.pow(B, 2.2)
    );
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  logs: state.logReducer.logs,
  orderBy: state.logReducer.orderBy,
  isFetchingLog: state.logReducer.isFetchingLog,
  nodeColors: state.logReducer.nodeColor
});
export default connect(mapStateToProps)(LogViewer);
