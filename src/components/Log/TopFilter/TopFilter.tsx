import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import { connect } from "react-redux";
import Label from "reactstrap/lib/Label";
import {
  changeDate,
  changeSearchText,
  setAutoRefresh
} from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import "./TopFilter.css";

interface StateProps {
  startDate: moment.Moment;
  endDate: moment.Moment;
  search: string;
  autoRefresh: boolean;
}

interface DispatchProps {
  dispatch: any;
}

type Props = StateProps & DispatchProps;
class TopFilter extends React.Component<Props, any> {
  constructor(props: any) {
    super(props);
  }
  public componentWillUnmount() {
    this.props.dispatch(setAutoRefresh(false));
  }
  public componentDidMount() {
    this.props.dispatch(changeDate(moment().subtract("days", 7), moment()));
  }
  public render() {
    const { startDate, endDate, search, autoRefresh } = this.props;
    return (
      <div className="top-filter">
        <div className="d-flex align-items-center">
          <div>
            <DatePicker
              className="date-picker"
              selected={startDate}
              onChange={this.handleChangeStartDate}
              onChangeRaw={this.handleChangeStartRawDate}
              timeIntervals={10}
              showTimeSelect={true}
              dateFormat="YYYY-MM-DD HH:mm:ssZ"
            />
          </div>
          <div className="mr-3 ml-3">-</div>
          <div>
            <DatePicker
              className={`date-picker`}
              selected={endDate}
              onChange={this.handleChangeEndDate}
              timeIntervals={10}
              disabledKeyboardNavigation={false}
              showTimeSelect={true}
              onChangeRaw={this.handleChangeEndRawDate}
              dateFormat="YYYY-MM-DD HH:mm:ssZ"
            />
          </div>
          <div className="ml-3">
            <div className="form-check">
              <input
                type="checkbox"
                className="form-check-input"
                id="auto-refresh-check"
                checked={autoRefresh}
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
  private handleChangeStartRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(changeDate(newDate, this.props.endDate));
    }
  };
  private handleChangeEndRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(setAutoRefresh(false));
      this.props.dispatch(changeDate(this.props.startDate, newDate));
    }
  };
  private handleChangeStartDate = (date: moment.Moment) => {
    this.props.dispatch(changeDate(date, this.props.endDate));
  };
  private handleChangeEndDate = (date: moment.Moment) => {
    this.props.dispatch(setAutoRefresh(false));
    this.props.dispatch(changeDate(this.props.startDate, date));
  };
  private handleChangeAutoRefresh = (event: any) => {
    this.props.dispatch(setAutoRefresh(event.target.checked));
  };
  private handleChangeSearch = (event: any) => {
    this.props.dispatch(changeSearchText(event.target.value));
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  startDate: state.logReducer.time.fromTime,
  endDate: state.logReducer.time.toTime,
  search: state.logReducer.search,
  autoRefresh: state.logReducer.setAutoRefresh
});
export default connect(mapStateToProps)(TopFilter);
