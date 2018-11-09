import * as _ from "lodash";
import * as moment from "moment";
import { LogAction } from "../actions/log";
import Log from "../components/Log/Log";

export interface LogState {
  filter: {
    nodeNames: string[];
    levels: ("error" | "warn" | "info" | "debug" | "trace")[];
    targets: string[];
  };
  search: string;
  time: {
    fromTime: moment.Moment;
    toTime: moment.Moment;
  };
  page: number;
  itemPerPage: number;
  isFetchingLog: boolean;
  isFetchingTarget: boolean;
  targets?: string[] | null;
  lastUpdated?: number | null;
  logs?: Log[] | null;
  orderBy: "ASC" | "DESC";
  fetchingUUIDForLog?: string | null;
  nodeColor: {
    [nodeName: string]: string;
  };
  noMoreData: boolean;
  setAutoRefresh: boolean;
}

const initialState: LogState = {
  filter: {
    nodeNames: [],
    levels: ["error", "warn", "info", "debug", "trace"],
    targets: []
  },
  time: {
    fromTime: moment().subtract("days", 7),
    toTime: moment()
  },
  search: "",
  page: 1,
  itemPerPage: 15,
  isFetchingLog: false,
  isFetchingTarget: false,
  orderBy: "DESC",
  nodeColor: {},
  noMoreData: false,
  setAutoRefresh: false
};

export const logReducer = (state = initialState, action: LogAction) => {
  switch (action.type) {
    case "ChangeDate":
      return { ...state, time: action.data, noMoreData: false, page: 1 };
    case "ChagneSearchText":
      return { ...state, search: action.data, noMoreData: false, page: 1 };
    case "ChangeNodes": {
      const newFilter = {
        ...state.filter,
        nodeNames: action.data
      };
      return { ...state, filter: newFilter, noMoreData: false, page: 1 };
    }
    case "ChangeDebugLevel": {
      const newFilter = {
        ...state.filter,
        levels: action.data
      };
      return { ...state, filter: newFilter, noMoreData: false, page: 1 };
    }
    case "RequestTargets": {
      return { ...state, isFetchingTarget: true };
    }
    case "SetTargets": {
      return { ...state, targets: action.data, isFetchingTarget: false };
    }
    case "RequestLogs": {
      return { ...state, isFetchingLog: true, fetchingUUIDForLog: action.data };
    }
    case "SetLogs": {
      return { ...state, logs: action.data, isFetchingLog: false };
    }
    case "ChangeTargets": {
      const newFilter = {
        ...state.filter,
        targets: action.data
      };
      return { ...state, filter: newFilter, noMoreData: false, page: 1 };
    }
    case "ChangeOrder": {
      return { ...state, orderBy: action.data };
    }
    case "SetNodeColor": {
      const newNodeColor = {
        ...state.nodeColor,
        [action.data.nodeName]: action.data.color
      };
      return {
        ...state,
        nodeColor: newNodeColor
      };
    }
    case "LoadMore": {
      return {
        ...state,
        page: action.data
      };
    }
    case "SetNoMoreData": {
      return {
        ...state,
        noMoreData: true
      };
    }
    case "SetAutoRefresh": {
      return {
        ...state,
        setAutoRefresh: action.data
      };
    }
  }
  return state;
};
