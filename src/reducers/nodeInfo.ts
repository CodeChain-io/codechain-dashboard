import * as _ from "lodash";
import { NodeInfoAction } from "../actions/nodeInfo";
import { NodeInfo } from "../requests/types";
const merge = require("deepmerge").default;
const overwriteMerge = (
  destinationArray: any,
  sourceArray: any,
  options: any
) => sourceArray;

export interface NodeState {
  nodeInfos: {
    [name: string]: {
      info?: NodeInfo | null;
      isFetching: boolean;
      lastUpdated?: number | null;
    };
  };
}

const initialState: NodeState = {
  nodeInfos: {}
};

export const nodeInfoReducer = (
  state = initialState,
  action: NodeInfoAction
) => {
  switch (action.type) {
    case "RequestNodeInfo": {
      const nodeInfos = {
        ...state.nodeInfos,
        [action.name]: {
          isFetching: true
        }
      };
      return {
        ...state,
        nodeInfos
      };
    }
    case "SetNodeInfo": {
      const nodeInfos = {
        ...state.nodeInfos,
        [action.name]: {
          info: action.data,
          isFetching: false,
          lastUpdated: action.receivedAt
        }
      };
      return {
        ...state,
        nodeInfos
      };
    }
    case "UpdateNodeInfo":
      if (!state.nodeInfos[action.name]) {
        return {
          ...state
        };
      }
      const updatedNodeInfos = {
        ...state.nodeInfos,
        [action.name]: {
          ...state.nodeInfos[action.name],
          info: merge(state.nodeInfos[action.name].info, action.data, {
            arrayMerge: overwriteMerge
          })
        }
      };
      return {
        ...state,
        nodeInfos: updatedNodeInfos
      };
  }
  return state;
};
