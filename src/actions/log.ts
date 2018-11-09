import * as moment from "moment";
import Log from "../components/Log/Log";
import { ReducerConfigure } from "../reducers";
import { LogState } from "../reducers/log";
import RequestAgent from "../RequestAgent";
const uuidv1 = require("uuid/v1");

export type LogAction =
  | ChangeNodes
  | ChangeDebugLevel
  | ChangeDate
  | ChagneSearchText
  | ChangeOrder
  | ChangeTargets
  | SetTargets
  | RequestTargets
  | SetLogs
  | RequestLogs
  | SetNodeColor
  | LoadMore
  | SetNoMoreData
  | SetAutoRefresh;

export interface ChangeNodes {
  type: "ChangeNodes";
  data: string[];
}

export interface ChangeDebugLevel {
  type: "ChangeDebugLevel";
  data: string[];
}

export interface ChangeDate {
  type: "ChangeDate";
  data: {
    fromTime: moment.Moment;
    toTime: moment.Moment;
  };
}

export interface ChangeTargets {
  type: "ChangeTargets";
  data: string[];
}

export interface ChagneSearchText {
  type: "ChagneSearchText";
  data: string;
}

export interface ChangeOrder {
  type: "ChangeOrder";
  data: "DESC" | "ASC";
}

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

export const changeDate = (
  startDate: moment.Moment,
  endDate: moment.Moment
) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeDate",
      data: {
        fromTime: startDate,
        toTime: endDate
      }
    });
    dispatch(fetchLogsIfNeeded());
  };
};

export const changeSearchText = (search: string) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChagneSearchText",
      data: search
    });
    dispatch(fetchLogsIfNeeded());
  };
};

export const changeNodes = (nodes: string[]) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeNodes",
      data: nodes
    });
    dispatch(fetchLogsIfNeeded());
  };
};

export const changeDebugLevel = (levels: string[]) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeDebugLevel",
      data: levels
    });
    dispatch(fetchLogsIfNeeded());
  };
};

export const changeTypes = (types: string[]) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeTypes",
      data: types
    });
    dispatch(fetchLogsIfNeeded());
  };
};

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
      dispatch(changeTargets(response.targets));
    }
  };
};

export const changeTargets = (targets: string[]) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeTargets",
      data: targets
    });
    dispatch(fetchLogsIfNeeded());
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
        time: logReducer.time,
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

export const changeOrder = (orderBy: "DESC" | "ASC") => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    dispatch({
      type: "ChangeOrder",
      data: orderBy
    });
    dispatch(fetchLogsIfNeeded());
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
    if (isOn) {
      refresher = setInterval(() => {
        const logReducer = getState().logReducer;
        dispatch(changeDate(logReducer.time.fromTime, moment()));
      }, 3000);
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
