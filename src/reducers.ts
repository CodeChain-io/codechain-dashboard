import * as _ from "lodash";
import { Action } from "./actions";
import { ChainNetworks, NetworkNodeInfo, NodeInfo } from "./requests/types";
const merge = require("deepmerge").default;

export interface RootState {
  nodeInfo: {
    [name: string]: NodeInfo;
  };
  chainNetworks: ChainNetworks | undefined;
}

const initialState: RootState = {
  nodeInfo: {},
  chainNetworks: undefined
};

export const appReducer = (state = initialState, action: Action) => {
  switch (action.type) {
    case "SetChainNetworks": {
      const chainNetworks = action.data;
      return {
        ...state,
        chainNetworks
      };
    }
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
    case "UpdateChainNetworks": {
      if (!state.chainNetworks) {
        return {
          ...state
        };
      }
      const newNodes = _.filter(
        action.data.nodes,
        (updateNode: NetworkNodeInfo) =>
          !_.find(
            state.chainNetworks!.nodes,
            (node: NetworkNodeInfo) => node.name === updateNode.name
          )
      );
      let updatedNodes = _.concat(state.chainNetworks.nodes, newNodes);
      updatedNodes = _.map(updatedNodes, node => {
        const changedNode = _.find(
          action.data.nodes,
          (updateNode: NetworkNodeInfo) => updateNode.name === node.name
        );
        if (changedNode) {
          return merge(
            node,
            _.find(
              action.data.nodes,
              (updateNode: NetworkNodeInfo) => updateNode.name === node.name
            )
          );
        } else {
          return node;
        }
      });
      const connectionAdded = action.data.connectionsAdded
        ? _.concat(
            state.chainNetworks.connections,
            action.data.connectionsAdded
          )
        : _.clone(state.chainNetworks.connections);
      const connectionRemoved = action.data.connectionsRemoved
        ? _.difference(connectionAdded, action.data.connectionsRemoved)
        : connectionAdded;
      return {
        ...state,
        chainNetworks: {
          nodes: updatedNodes,
          connections: connectionRemoved
        }
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
        [action.name]: merge(state.nodeInfo[action.name], action.data)
      };
      return {
        ...state,
        nodeInfo: updatedNodeInfo
      };
  }
  return state;
};
