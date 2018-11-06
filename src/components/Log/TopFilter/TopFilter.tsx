import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import { connect } from "react-redux";
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

type Props = StateProps & DispatchProps;
class TopFilter extends React.Component<Props, any> {
  constructor(props: any) {
    super(props);
  }
  public render() {
    const { startDate, endDate, search } = this.props;
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
              className="date-picker"
              selected={endDate}
              onChange={this.handleChangeEndDate}
              showTimeSelect={true}
              dateFormat="YYYY-MM-DD HH:mm:ss"
            />
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
    this.props.dispatch(changeDate(this.props.startDate, date));
  };
  private handleChangeSearch = (event: any) => {
    this.props.dispatch(changeSearchText(event.target.value));
  };
}
const mapStateToProps = (state: ReducerConfigure) => ({
  startDate: state.logReducer.time.fromTime,
  endDate: state.logReducer.time.toTime,
  search: state.logReducer.search
});
export default connect(mapStateToProps)(TopFilter);
