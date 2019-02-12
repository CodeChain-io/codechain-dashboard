import * as _ from "lodash";
import * as moment from "moment";
import { LogAction } from "../actions/log";
import Log from "../components/Log/Log";
import { getObjectFromStorage, saveObjectToStorage } from "../utils/storage";
const merge = require("deepmerge").default;
const overwriteMerge = (
  destinationArray: any,
  sourceArray: any,
  options: any
) => sourceArray;

export interface LogState {
  filter: {
    nodeNames: string[];
    levels: ("error" | "warn" | "info" | "debug" | "trace")[];
    targets: string[];
  };
  search: string;
  time: {
    fromTime: number;
    toTime: number;
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
  setFromTime: boolean;
  setToTime: boolean;
}

const initialState: LogState = {
  filter: {
    nodeNames: [],
    levels: ["error", "warn", "info", "debug", "trace"],
    targets: []
  },
  time: {
    fromTime: moment()
      .subtract("days", 7)
      .unix(),
    toTime: moment().unix()
  },
  search: "",
  page: 1,
  itemPerPage:
    ((getObjectFromStorage("itemPerPage") as { itemPerPage: number }) &&
      (getObjectFromStorage("itemPerPage") as { itemPerPage: number })
        .itemPerPage) ||
    15,
  isFetchingLog: false,
  isFetchingTarget: false,
  orderBy: "DESC",
  nodeColor: getObjectFromStorage("nodeColor") || {},
  noMoreData: false,
  setAutoRefresh: false,
  setFromTime: true,
  setToTime: true
};

export const logReducer = (state = initialState, action: LogAction) => {
  switch (action.type) {
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
    case "SetNodeColor": {
      const newNodeColor = {
        ...state.nodeColor,
        [action.data.nodeName]: action.data.color
      };
      saveObjectToStorage("nodeColor", newNodeColor);
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
    case "ChangeFilters": {
      if (action.data.itemPerPage) {
        saveObjectToStorage("itemPerPage", { itemPerPage: action.data.itemPerPage });
      }
      return merge({ ...state, noMoreData: false, page: 1 }, action.data, {
        arrayMerge: overwriteMerge
      });
    }
  }
  return state;
};
