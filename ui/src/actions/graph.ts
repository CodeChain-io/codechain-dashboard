import moment from "moment";
import { ReducerConfigure } from "../reducers";
import RequestAgent from "../RequestAgent";
import {
  GraphNetworkOutAllAVGRow,
  GraphNetworkOutAllRow
} from "../requests/types";

export type GraphAction =
  | SetNetworkOutAllGraph
  | ChangeNetworkOutAllFilters
  | SetNetworkOutAllAVGGraph
  | ChangeNetworkOutAllAVGFilters;

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

export interface SetNetworkOutAllAVGGraph {
  type: "SetNetworkOutAllAVGGraph";
  data: GraphNetworkOutAllAVGRow[];
}

const setNetworkOutAllAVGGraph = (data: GraphNetworkOutAllAVGRow[]) => ({
  type: "SetNetworkOutAllAVGGraph",
  data
});

export interface ChangeNetworkOutAllAVGFilters {
  type: "ChangeNetworkOutAllAVGFilters";
  data: {
    time: {
      fromTime: number;
      toTime: number;
    };
  };
}

export const changeNetworkOutAllAVGFilters = (params: {
  time: {
    fromTime: number;
    toTime: number;
  };
}) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeNetworkOutAllAVGFilters",
      data: {
        time: params.time
      }
    });
    dispatch(fetchNetworkOutAllAVGGraph());
  };
};

export const fetchNetworkOutAllAVGGraph = () => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    const response = await RequestAgent.getInstance().call<{
      rows: GraphNetworkOutAllAVGRow[];
    }>("graph_network_out_all_node_avg", [
      {
        from: moment
          .unix(getState().graphReducer.networkOutAllAVGGraph.time.fromTime)
          .toISOString(),
        to: moment
          .unix(getState().graphReducer.networkOutAllAVGGraph.time.toTime)
          .toISOString(),
        period: "minutes5"
      }
    ]);
    dispatch(setNetworkOutAllAVGGraph(response.rows));
  };
};
