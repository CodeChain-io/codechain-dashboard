import { faCaretDown, faCaretUp } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as _ from "lodash";
import * as React from "react";
import { connect } from "react-redux";
import Table from "reactstrap/lib/Table";
import {
  changeFilters,
  loadMoreLog,
  setAutoRefresh
} from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import { Log } from "../../../requests/types";
import LogItem from "./LogItem/LogItem";
import "./LogViewer.css";

interface StateProps {
  orderBy: "DESC" | "ASC";
  logs?: Log[] | null;
  isFetchingLog: boolean;
  noMoreData: boolean;
  itemPerPage: number;
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
    const {
      orderBy,
      isFetchingLog,
      logs,
      noMoreData,
      itemPerPage
    } = this.props;
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
                  <FontAwesomeIcon icon={faCaretDown} />
                ) : (
                  <FontAwesomeIcon icon={faCaretUp} />
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
              <th>
                Message{" "}
                <span className="float-right">
                  Show{" "}
                  <select
                    value={itemPerPage}
                    onChange={this.handleChangeItemPerpage}
                  >
                    <option value={15}>15</option>
                    <option value={30}>30</option>
                    <option value={50}>50</option>
                    <option value={75}>75</option>
                    <option value={100}>100</option>
                  </select>{" "}
                  Items
                </span>
              </th>
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
                  Load More {itemPerPage} items
                </td>
              </tr>
            )}
          </tbody>
        </Table>
      </div>
    );
  }
  private handleChangeItemPerpage = (event: any) => {
    this.props.dispatch(
      changeFilters({ itemPerPage: parseInt(event.target.value, 10) })
    );
  };
  private toggleOrder = () => {
    this.props.dispatch(
      changeFilters({ orderBy: this.props.orderBy === "DESC" ? "ASC" : "DESC" })
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
  noMoreData: state.logReducer.noMoreData,
  itemPerPage: state.logReducer.itemPerPage
});
export default connect(mapStateToProps)(LogViewer);
