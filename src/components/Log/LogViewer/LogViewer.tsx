import { faArrowDown, faArrowUp } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import Table from "reactstrap/lib/Table";
import { changeOrder, loadMoreLog, setAutoRefresh } from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import { Log } from "../../../requests/types";
import LogItem from "./LogItem/LogItem";
import "./LogViewer.css";

interface StateProps {
  orderBy: "DESC" | "ASC";
  logs?: Log[] | null;
  isFetchingLog: boolean;
  noMoreData: boolean;
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
    const { orderBy, isFetchingLog, logs, noMoreData } = this.props;
    return (
      <div className="log-viewer animated fadeIn">
        <Table>
          <thead>
            <tr>
              <th
                style={{ width: "180px" }}
                onClick={this.toggleOrder}
                className="date-table-header text-center"
              >
                Date{" "}
                {orderBy === "DESC" ? (
                  <FontAwesomeIcon icon={faArrowDown} />
                ) : (
                  <FontAwesomeIcon icon={faArrowUp} />
                )}
              </th>
              <th style={{ width: "100px" }} className="text-center">
                Node
              </th>
              <th style={{ width: "80px" }} className="text-center">
                Level
              </th>
              <th style={{ width: "120px" }} className="text-center">
                Target
              </th>
              <th>Message</th>
            </tr>
          </thead>
          <tbody>
            {logs && _.map(logs, log => <LogItem key={log.id} log={log} />)}
            {isFetchingLog ? (
              <tr>
                <td colSpan={5} className="text-center">
                  Loading...
                </td>
              </tr>
            ) : noMoreData ? (
              <tr>
                <td colSpan={5} className="text-center">
                  No more log
                </td>
              </tr>
            ) : (
              <tr onClick={this.handleLoadMore}>
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
  private handleLoadMore = () => {
    this.props.dispatch(setAutoRefresh(false));
    this.props.dispatch(loadMoreLog());
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  logs: state.logReducer.logs,
  orderBy: state.logReducer.orderBy,
  isFetchingLog: state.logReducer.isFetchingLog,
  noMoreData: state.logReducer.noMoreData
});
export default connect(mapStateToProps)(LogViewer);
