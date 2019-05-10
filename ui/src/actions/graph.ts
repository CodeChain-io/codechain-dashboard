import * as moment from "moment";
import { ReducerConfigure } from "../reducers";
import RequestAgent from "../RequestAgent";
import { GraphNetworkOutAllRow } from "../requests/types";

export type GraphAction = SetNetworkOutAllGraph | ChangeNetworkOutAllFilters;

export interface SetNetworkOutAllGraph {
  type: "SetNetworkOutAllGraph";
  data: GraphNetworkOutAllRow[];
}

const setNetworkOutAllGraph = (data: GraphNetworkOutAllRow[]) => ({
  type: "SetNetworkOutAllGraph",
  data
});

export interface ChangeNetworkOutAllFilters {
  type: "ChangeNetworkOutAllFilters";
  data: {
    time: {
      fromTime: number;
      toTime: number;
    };
  };
}

export const changeNetworkOutAllFilters = (params: {
  time: {
    fromTime: number;
    toTime: number;
  };
}) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeNetworkOutAllFilters",
      data: {
        time: params.time
      }
    });
    dispatch(fetchNetworkOutAllGraph());
  };
};

export const fetchNetworkOutAllGraph = () => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    const response = await RequestAgent.getInstance().call<{
      rows: GraphNetworkOutAllRow[];
    }>("graph_network_out_all_node", [
      {
        from: moment
          .unix(getState().graphReducer.networkOutAllGraph.time.fromTime)
          .toISOString(),
        to: moment
          .unix(getState().graphReducer.networkOutAllGraph.time.toTime)
          .toISOString(),
        period: "minutes5"
      }
    ]);
    dispatch(setNetworkOutAllGraph(response.rows));
  };
};
