import * as _ from "lodash";
import moment from "moment";
import { PlotData } from "plotly.js";
import { Component } from "react";
import * as React from "react";
import DatePicker from "react-datepicker";
import "react-datepicker/dist/react-datepicker.css";
import Plot from "react-plotly.js";
import { connect } from "react-redux";
import { Label } from "reactstrap";
import { changeNetworkOutNodePeerFilters } from "../../../../actions/graph";
import { ReducerConfigure } from "../../../../reducers";
import { GraphNetworkOutNodePeerRow } from "../../../../requests/types";
import "./NetworkOutNodePeerGraph.css";

interface OwnProps {
  nodeId: string;
}

interface StateProps {
  fromTime: number;
  toTime: number;
  data: GraphNetworkOutNodePeerRow[];
}

interface DispatchProps {
  dispatch: any;
}

type Props = OwnProps & StateProps & DispatchProps;
class NetworkOutNodePeerGraph extends Component<Props> {
  public constructor(props: any) {
    super(props);
  }

  public componentDidMount(): void {
    console.log("componentDidMount of networkOutNodePeerGraph");
    this.props.dispatch(
      changeNetworkOutNodePeerFilters({
        nodeId: this.props.nodeId,
        time: {
          fromTime: this.props.fromTime,
          toTime: this.props.toTime
        }
      })
    );
  }

  public render() {
    const { fromTime, toTime } = this.props;
    const rowsByPeer = _.groupBy(this.props.data, row => row.peer);
    return (
      <div className="network-out-node-peer-graph">
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
            data={_.map<any, Partial<PlotData>>(rowsByPeer, (rows, peer) => ({
              x: _.map(rows, row => row.time),
              y: _.map(rows, row => row.value),
              type: "scatter",
              mode: "lines+markers",
              name: peer,
              showlegend: true
            }))}
            layout={{
              width: 1000,
              height: 600,
              title: "Network Out by Peer"
            }}
          />
        </div>
      </div>
    );
  }

  private handleChangeFromTime = (date: moment.Moment) => {
    const nodeId = this.props.nodeId;
    this.props.dispatch(
      changeNetworkOutNodePeerFilters({
        nodeId,
        time: {
          fromTime: date.unix(),
          toTime: this.props.toTime
        }
      })
    );
  };
  private handleChangeFromTimeRawDate = (event: any) => {
    const nodeId = this.props.nodeId;
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutNodePeerFilters({
          nodeId,
          time: {
            fromTime: newDate.unix(),
            toTime: this.props.toTime
          }
        })
      );
    }
  };

  private handleChangeToTime = (date: moment.Moment) => {
    const nodeId = this.props.nodeId;
    this.props.dispatch(
      changeNetworkOutNodePeerFilters({
        nodeId,
        time: {
          fromTime: this.props.fromTime,
          toTime: date.unix()
        }
      })
    );
  };
  private handleChangeToTimeRawDate = (event: any) => {
    const nodeId = this.props.nodeId;
    const newDate = moment(event.target.value);
    if (newDate.isValid()) {
      this.props.dispatch(
        changeNetworkOutNodePeerFilters({
          nodeId,
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
    data: state.graphReducer.networkOutNodePeerGraph.data,
    fromTime: state.graphReducer.networkOutNodePeerGraph.time.fromTime,
    toTime: state.graphReducer.networkOutNodePeerGraph.time.toTime
  };
};

export default connect(mapStateToProps)(NetworkOutNodePeerGraph);
