import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import { connect } from "react-redux";
import Label from "reactstrap/lib/Label";
import { changeFilters, setAutoRefresh } from "../../../actions/log";
import { ReducerConfigure } from "../../../reducers";
import "./TopFilter.css";

interface StateProps {
  startDate: number;
  endDate: number;
  search: string;
  autoRefresh: boolean;
  setFromTime: boolean;
  setToTime: boolean;
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
    this.props.dispatch(
      changeFilters({
        time: {
          fromTime: moment()
            .subtract("days", 7)
            .unix(),
          toTime: moment().unix()
        }
      })
    );
  }
  public render() {
    const {
      startDate,
      endDate,
      search,
      autoRefresh,
      setToTime,
      setFromTime
    } = this.props;
    return (
      <div className="top-filter">
        <div className="d-flex align-items-center">
          <div>
            <div className="mb-1">
              <div className="label-time form-check">
                <input
                  type="checkbox"
                  className="form-check-input"
                  id="from-time-check"
                  checked={setFromTime}
                  onChange={this.handleChangeFromTimeCheck}
                />
                <Label className="form-check-label" for="from-time-check">
                  From
                </Label>
              </div>
              <DatePicker
                className="date-picker"
                selected={moment(startDate)}
                onChange={this.handleChangeStartDate}
                onChangeRaw={this.handleChangeStartRawDate}
                timeIntervals={10}
                showTimeSelect={true}
                dateFormat="YYYY-MM-DD HH:mm:ssZ"
              />
            </div>
            <div>
              <div className="label-time  form-check">
                <input
                  type="checkbox"
                  className="form-check-input"
                  id="to-time-check"
                  checked={setToTime}
                  onChange={this.handleChangeToTimeCheck}
                />
                <Label className="form-check-label" for="to-time-check">
                  To
                </Label>
              </div>
              <DatePicker
                className={`date-picker`}
                selected={moment(endDate)}
                onChange={this.handleChangeEndDate}
                timeIntervals={10}
                disabledKeyboardNavigation={false}
                showTimeSelect={true}
                onChangeRaw={this.handleChangeEndRawDate}
                dateFormat="YYYY-MM-DD HH:mm:ssZ"
              />
            </div>
          </div>
          <div className="ml-4">
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
      this.props.dispatch(
        changeFilters({
          time: { fromTime: newDate.unix(), toTime: this.props.endDate }
        })
      );
    }
  };
  private handleChangeEndRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(setAutoRefresh(false));
      this.props.dispatch(
        changeFilters({
          time: { fromTime: this.props.startDate, toTime: newDate.unix() }
        })
      );
    }
  };
  private handleChangeStartDate = (date: moment.Moment) => {
    this.props.dispatch(
      changeFilters({
        time: { fromTime: date.unix(), toTime: this.props.endDate }
      })
    );
  };
  private handleChangeFromTimeCheck = (event: any) => {
    this.props.dispatch(changeFilters({ setFromTime: event.target.checked }));
  };
  private handleChangeToTimeCheck = (event: any) => {
    this.props.dispatch(changeFilters({ setToTime: event.target.checked }));
  };
  private handleChangeEndDate = (date: moment.Moment) => {
    this.props.dispatch(setAutoRefresh(false));
    this.props.dispatch(
      changeFilters({
        time: { fromTime: this.props.startDate, toTime: date.unix() }
      })
    );
  };
  private handleChangeAutoRefresh = (event: any) => {
    this.props.dispatch(setAutoRefresh(event.target.checked));
  };
  private handleChangeSearch = (event: any) => {
    this.props.dispatch(changeFilters({ search: event.target.value }));
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  startDate: state.logReducer.time.fromTime,
  endDate: state.logReducer.time.toTime,
  search: state.logReducer.search,
  autoRefresh: state.logReducer.setAutoRefresh,
  setFromTime: state.logReducer.setFromTime,
  setToTime: state.logReducer.setToTime
});
export default connect(mapStateToProps)(TopFilter);
