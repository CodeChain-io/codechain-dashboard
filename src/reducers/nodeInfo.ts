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
  nodeInfo: {
    [name: string]: NodeInfo;
  };
}

const initialState: NodeState = {
  nodeInfo: {}
};

export const nodeInfoReducer = (
  state = initialState,
  action: NodeInfoAction
) => {
  switch (action.type) {
    case "SetNodeInfo": {
      const nodeInfo = {
        ...state.nodeInfo,
        [action.name]: action.data
      };
      return {
        ...state,
        nodeInfo
      };
    }
    case "UpdateNodeInfo":
      if (!state.nodeInfo[action.name]) {
        return {
          ...state
        };
      }
      const updatedNodeInfo = {
        ...state.nodeInfo,
        [action.name]: merge(state.nodeInfo[action.name], action.data, {
          arrayMerge: overwriteMerge
        })
      };
      return {
        ...state,
        nodeInfo: updatedNodeInfo
      };
  }
  return state;
};
