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
    console.log(this.props.nodeColors);
    return (
      <div className="log-viewer">
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
              <th style={{ width: "120px" }}>Node</th>
              <th style={{ width: "80px" }}>Status</th>
              <th style={{ width: "120px" }}>Target</th>
              <th>Body</th>
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
                          backgroundColor: nodeColors[log.nodeName]
                        }
                      : {
                          backgroundColor: "#ffffff"
                        }
                  }
                >
                  <td>{moment(log.timestamp).format("YYYY-MM-DD HH:mm:ss")}</td>
                  <td>{log.nodeName}</td>
                  <td>{log.level}</td>
                  <td>{log.target}</td>
                  <td>{log.message}</td>
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
}
const mapStateToProps = (state: ReducerConfigure) => ({
  logs: state.logReducer.logs,
  orderBy: state.logReducer.orderBy,
  isFetchingLog: state.logReducer.isFetchingLog,
  nodeColors: state.logReducer.nodeColor
});
export default connect(mapStateToProps)(LogViewer);
