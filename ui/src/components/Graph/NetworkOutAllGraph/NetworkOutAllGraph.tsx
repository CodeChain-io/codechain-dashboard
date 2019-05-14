import * as _ from "lodash";
import * as moment from "moment";
import { PlotData } from "plotly.js";
import { Component } from "react";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import Plot from "react-plotly.js";
import { connect } from "react-redux";
import { Label } from "reactstrap";
import {
  changeNetworkOutAllFilters,
  fetchNetworkOutAllGraph
} from "../../../actions/graph";
import { ReducerConfigure } from "../../../reducers";
import { GraphNetworkOutAllRow } from "../../../requests/types";
import "./NetworkOutAllGraph.css";

interface StateProps {
  fromTime: number;
  toTime: number;
  data: GraphNetworkOutAllRow[];
}

interface DispatchProps {
  dispatch: any;
}

type Props = StateProps & DispatchProps;
class NetworkOutAllGraph extends Component<Props> {
  public constructor(props: any) {
    super(props);
  }

  public componentDidMount(): void {
    this.props.dispatch(fetchNetworkOutAllGraph());
  }

  public render() {
    const { fromTime, toTime } = this.props;
    const rowsByNodeName = _.groupBy(this.props.data, row => row.nodeName);
    return (
      <div className="network-out-all-graph">
        <div className="from-time">
          <Label className="form-check-label" for="from-time-check">
            From
          </Label>
          <DatePicker
            className="date-picker"
            selected={moment.unix(fromTime)}
            onChange={this.handleChangeFromTime}
            onChangeRaw={this.handleChangeFromTimeRawDate}
            timeIntervals={10}
            showTimeSelect={true}
            dateFormat="YYYY-MM-DD HH:mm:ssZ"
          />
        </div>
        <div className="to-time">
          <Label className="form-check-label" for="from-time-check">
            To
          </Label>
          <DatePicker
            className="date-picker"
            selected={moment.unix(toTime)}
            onChange={this.handleChangeToTime}
            onChangeRaw={this.handleChangeToTimeRawDate}
            timeIntervals={10}
            showTimeSelect={true}
            dateFormat="YYYY-MM-DD HH:mm:ssZ"
          />
        </div>
        <div className="plot">
          <Plot
            data={_.map<any, Partial<PlotData>>(
              rowsByNodeName,
              (rows, nodeName) => ({
                x: _.map(rows, row => row.time),
                y: _.map(rows, row => row.value),
                type: "scatter",
                mode: "lines+markers",
                name: nodeName,
                showlegend: true
              })
            )}
            layout={{ width: 1000, height: 600, title: "Network Out All" }}
          />
        </div>
      </div>
    );
  }

  private handleChangeFromTime = (date: moment.Moment) => {
    this.props.dispatch(
      changeNetworkOutAllFilters({
        time: {
          fromTime: date.unix(),
          toTime: this.props.toTime
        }
      })
    );
  };
  private handleChangeFromTimeRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutAllFilters({
          time: {
            fromTime: newDate.unix(),
            toTime: this.props.toTime
          }
        })
      );
    }
  };

  private handleChangeToTime = (date: moment.Moment) => {
    this.props.dispatch(
      changeNetworkOutAllFilters({
        time: {
          fromTime: this.props.fromTime,
          toTime: date.unix()
        }
      })
    );
  };
  private handleChangeToTimeRawDate = (event: any) => {
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutAllFilters({
          time: {
            fromTime: this.props.toTime,
            toTime: newDate.unix()
          }
        })
      );
    }
  };
}

const mapStateToProps = (state: ReducerConfigure) => {
  return {
    data: state.graphReducer.networkOutAllGraph.data,
    fromTime: state.graphReducer.networkOutAllGraph.time.fromTime,
    toTime: state.graphReducer.networkOutAllGraph.time.toTime
  };
};

export default connect(mapStateToProps)(NetworkOutAllGraph);
