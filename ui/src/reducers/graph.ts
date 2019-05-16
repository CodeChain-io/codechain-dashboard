import moment from "moment";
import { GraphAction } from "../actions/graph";
import NetworkOutAllGraph from "../components/Graph/NetworkOutAllGraph/NetworkOutAllGraph";
import {
  GraphNetworkOutAllAVGRow,
  GraphNetworkOutAllRow
} from "../requests/types";
const merge = require("deepmerge").default;

export interface GraphState {
  networkOutAllGraph: NetworkOutAllGraph;
  networkOutAllAVGGraph: NetworkOutAllAVGGraph;
}

export interface NetworkOutAllGraph {
  data: GraphNetworkOutAllRow[];
  time: {
    fromTime: number;
    toTime: number;
  };
}

export interface NetworkOutAllAVGGraph {
  data: GraphNetworkOutAllAVGRow[];
  time: {
    fromTime: number;
    toTime: number;
  };
}

const initialState: GraphState = {
  networkOutAllGraph: {
    data: [],
    time: {
      fromTime: moment()
        .subtract(7, "days")
        .unix(),
      toTime: moment().unix()
    }
  },
  networkOutAllAVGGraph: {
    data: [],
    time: {
      fromTime: moment()
        .subtract(7, "days")
        .unix(),
      toTime: moment().unix()
    }
  }
};

export const graphReducer = (state = initialState, action: GraphAction) => {
  switch (action.type) {
    case "ChangeNetworkOutAllFilters":
      return merge(state, { networkOutAllGraph: action.data });
    case "SetNetworkOutAllGraph":
      return {
        ...state,
        networkOutAllGraph: {
          ...state.networkOutAllGraph,
          data: action.data
        }
      };
    case "ChangeNetworkOutAllAVGFilters":
      return merge(state, { networkOutAllAVGGraph: action.data });
    case "SetNetworkOutAllAVGGraph":
      return {
        ...state,
        networkOutAllAVGGraph: {
          ...state.networkOutAllAVGGraph,
          data: action.data
        }
      };
  }
  return state;
};
