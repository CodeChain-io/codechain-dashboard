import moment from "moment";
import Log from "../components/Log/Log";
import { ReducerConfigure } from "../reducers";
import { LogState } from "../reducers/log";
import RequestAgent from "../RequestAgent";
const uuidv1 = require("uuid/v1");

export type LogAction =
  | SetTargets
  | RequestTargets
  | SetLogs
  | RequestLogs
  | SetNodeColor
  | LoadMore
  | SetNoMoreData
  | SetAutoRefresh
  | ChangeFilters;

export interface RequestTargets {
  type: "RequestTargets";
}

export interface SetTargets {
  type: "SetTargets";
  data: string[];
}

export interface RequestLogs {
  type: "RequestLogs";
  data: string;
}

export interface SetLogs {
  type: "SetLogs";
  data: Log[];
}

export interface SetNodeColor {
  type: "SetNodeColor";
  data: {
    nodeName: string;
    color: string;
  };
}

export interface LoadMore {
  type: "LoadMore";
  data: number;
}

export interface SetNoMoreData {
  type: "SetNoMoreData";
}

export interface SetAutoRefresh {
  type: "SetAutoRefresh";
  data: boolean;
}

export interface ChangeFilters {
  type: "ChangeFilters";
  data: {
    time?: {
      fromTime?: number | null;
      toTime?: number | null;
    } | null;
    search?: string | null;
    filter?: {
      nodeNames?: string[] | null;
      levels?: ("error" | "warn" | "info" | "debug" | "trace")[] | null;
      targets?: string[] | null;
    } | null;
    itemPerPage?: number | null;
    orderBy?: ("ASC" | "DESC") | null;
    setFromTime?: boolean | null;
    setToTime?: boolean | null;
  };
}

const requestTargets = () => ({
  type: "RequestTargets"
});

const setTargets = (data: string[]) => ({
  type: "SetTargets",
  data
});

const shouldFetchTargets = (state: LogState) => {
  if (state.targets) {
    return true;
  } else if (state.isFetchingTarget) {
    return false;
  }
  return true;
};

export const fetchTargetsIfNeeded = () => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    if (shouldFetchTargets(getState().logReducer)) {
      dispatch(requestTargets());
      const response = await RequestAgent.getInstance().call<{
        targets: string[];
      }>("log_getTargets", []);
      dispatch(setTargets(response.targets));
      dispatch(changeFilters({ filter: { targets: response.targets } }));
    }
  };
};

const requestLogs = (uuid: string) => ({
  type: "RequestLogs",
  data: uuid
});

const setLogs = (data: Log[]) => ({
  type: "SetLogs",
  data
});

export const fetchLogsIfNeeded = (haveToAppend?: boolean) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    const uuid = uuidv1();
    dispatch(requestLogs(uuid));
    const logReducer = getState().logReducer;
    const response = await RequestAgent.getInstance().call<{
      logs: Log[];
    }>("log_get", [
      {
        filter: logReducer.filter,
        search: logReducer.search,
        time: {
          fromTime:
            logReducer.setFromTime &&
            moment.unix(logReducer.time.fromTime).toISOString(),
          toTime:
            logReducer.setToTime &&
            moment.unix(logReducer.time.toTime).toISOString()
        },
        page: logReducer.page,
        itemPerPage: logReducer.itemPerPage,
        orderBy: logReducer.orderBy
      }
    ]);
    if (getState().logReducer.fetchingUUIDForLog === uuid) {
      dispatch(
        setLogs(
          haveToAppend && logReducer.logs
            ? logReducer.logs.concat(response.logs)
            : response.logs
        )
      );
      if (response.logs.length < logReducer.itemPerPage) {
        dispatch(setNoMoreData());
      }
    }
  };
};

export const setNodeColor = (nodeName: string, color: string) => ({
  type: "SetNodeColor",
  data: {
    nodeName,
    color
  }
});

export const loadMoreLog = () => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    const logReducer = getState().logReducer;
    dispatch({
      type: "LoadMore",
      data: logReducer.page + 1
    });
    dispatch(fetchLogsIfNeeded(true));
  };
};

export const setNoMoreData = () => ({
  type: "SetNoMoreData"
});

let refresher: any;
export const setAutoRefresh = (isOn: boolean) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    const intervalFunc = () => {
      const logReducer = getState().logReducer;
      dispatch(
        changeFilters({
          time: {
            fromTime: logReducer.time.fromTime,
            toTime: moment.now()
          },
          orderBy: "DESC",
          setToTime: true
        })
      );
    };
    if (isOn) {
      intervalFunc();
      refresher = setInterval(intervalFunc, 3000);
    } else {
      if (refresher) {
        clearInterval(refresher);
      }
    }
    dispatch({
      type: "SetAutoRefresh",
      data: isOn
    });
  };
};

export const changeFilters = (params: {
  time?: {
    fromTime?: number | null;
    toTime?: number | null;
  } | null;
  search?: string | null;
  filter?: {
    nodeNames?: string[] | null;
    levels?: ("error" | "warn" | "info" | "debug" | "trace")[] | null;
    targets?: string[] | null;
  } | null;
  itemPerPage?: number | null;
  orderBy?: ("ASC" | "DESC") | null;
  setFromTime?: boolean | null;
  setToTime?: boolean | null;
}) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeFilters",
      data: params
    });
    dispatch(fetchLogsIfNeeded());
  };
};
