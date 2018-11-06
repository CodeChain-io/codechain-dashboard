import { faSearch } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import * as moment from "moment";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import "./TopFilter.css";

interface State {
  startDate: moment.Moment;
  endDate: moment.Moment;
}
export default class TopFilter extends React.Component<any, State> {
  constructor(props: any) {
    super(props);
    this.state = {
      startDate: moment().subtract("days", 7),
      endDate: moment()
    };
  }
  public render() {
    const { startDate, endDate } = this.state;
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
              <input type="text" />
            </div>
          </div>
        </div>
      </div>
    );
  }
  private handleChangeStartDate = (date: moment.Moment) => {
    this.setState({ startDate: date });
  };
  private handleChangeEndDate = (date: moment.Moment) => {
    this.setState({ endDate: date });
  };
}
