import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import { connect } from "react-redux";
import Label from "reactstrap/lib/Label";
import { changeDate, changeSearchText } from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import "./TopFilter.css";

interface StateProps {
  startDate: moment.Moment;
  endDate: moment.Moment;
  search: string;
}

interface DispatchProps {
  dispatch: any;
}

interface State {
  setAutoRefresh: boolean;
}

type Props = StateProps & DispatchProps;
class TopFilter extends React.Component<Props, State> {
  private refresher: any;
  constructor(props: any) {
    super(props);
    this.state = {
      setAutoRefresh: false
    };
  }
  public componentWillUnmount() {
    if (this.refresher) {
      clearInterval(this.refresher);
    }
  }
  public render() {
    const { startDate, endDate, search } = this.props;
    const { setAutoRefresh } = this.state;
    return (
      <div className="top-filter">
        <div className="d-flex align-items-center">
          <div>
            <DatePicker
              className="date-picker"
              selected={startDate}
              onChange={this.handleChangeStartDate}
              showTimeSelect={true}
              dateFormat="YYYY-MM-DD HH:mm:ss"
            />
          </div>
          <div className="mr-3 ml-3">-</div>
          <div>
            <DatePicker
              className={`date-picker`}
              selected={endDate}
              onChange={this.handleChangeEndDate}
              showTimeSelect={true}
              dateFormat="YYYY-MM-DD HH:mm:ss"
            />
          </div>
          <div className="ml-3">
            <div className="form-check">
              <input
                type="checkbox"
                className="form-check-input"
                id="auto-refresh-check"
                checked={setAutoRefresh}
                onChange={this.handleChangeAutoRefresh}
              />
              <Label className="form-check-label" for="auto-refresh-check">
                Auto Refresh
              </Label>
            </div>
          </div>
          <div className="ml-auto search-container">
            <div className="d-flex align-items-center">
              <div className="mr-2">
                <FontAwesomeIcon icon={faSearch} />
              </div>
              <input
                type="text"
                value={search}
                onChange={this.handleChangeSearch}
              />
            </div>
          </div>
        </div>
      </div>
    );
  }
  private handleChangeStartDate = (date: moment.Moment) => {
    this.props.dispatch(changeDate(date, this.props.endDate));
  };
  private handleChangeEndDate = (date: moment.Moment) => {
    if (this.refresher) {
      clearInterval(this.refresher);
    }
    this.setState({ setAutoRefresh: false });
    this.props.dispatch(changeDate(this.props.startDate, date));
  };
  private handleChangeAutoRefresh = (event: any) => {
    if (event.target.checked) {
      this.setState({ setAutoRefresh: true });
      this.setEndtimeToCurrentTime();
      this.refresher = setInterval(this.setEndtimeToCurrentTime, 10000);
    } else {
      this.setState({ setAutoRefresh: false });
      if (this.refresher) {
        clearInterval(this.refresher);
      }
    }
  };
  private handleChangeSearch = (event: any) => {
    this.props.dispatch(changeSearchText(event.target.value));
  };
  private setEndtimeToCurrentTime = () => {
    this.props.dispatch(changeDate(this.props.startDate, moment()));
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  startDate: state.logReducer.time.fromTime,
  endDate: state.logReducer.time.toTime,
  search: state.logReducer.search
});
export default connect(mapStateToProps)(TopFilter);
