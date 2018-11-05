import { ReducerConfigure } from "../reducers";
import { NodeState } from "../reducers/nodeInfo";
import RequestAgent from "../RequestAgent";
import { NodeInfo, NodeUpdateInfo } from "../requests/types";
export type NodeInfoAction = SetNodeInfo | UpdateNodeInfo | RequestNodeInfo;

export interface SetNodeInfo {
  type: "SetNodeInfo";
  name: string;
  data: NodeInfo;
  receivedAt: number;
}

export interface UpdateNodeInfo {
  type: "UpdateNodeInfo";
  name: string;
  data: NodeInfo;
}

export interface RequestNodeInfo {
  type: "RequestNodeInfo";
  name: string;
}

export const setNodeInfo = (name: string, data: NodeInfo) => ({
  type: "SetNodeInfo",
  name,
  data,
  receivedAt: Date.now()
});

export const requestNodeInfo = (name: string) => ({
  type: "RequestNodeInfo",
  name
});

export const updateNodeInfo = (name: string, data: NodeUpdateInfo) => ({
  type: "UpdateNodeInfo",
  name,
  data
});

const shouldFetchNodeInfo = (state: NodeState, nodeName: string) => {
  const nodeInfo = state.nodeInfos[nodeName];
  if (!nodeInfo) {
    return true;
  } else if (nodeInfo.isFetching) {
    return false;
  }
  return true;
};

export const fetchNodeInfoIfNeeded = (nodeName: string) => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    if (shouldFetchNodeInfo(getState().nodeInfoReducer, nodeName)) {
      dispatch(requestNodeInfo(nodeName));
      const nodeInfo = await RequestAgent.getInstance().call<NodeInfo>(
        "real_node_getInfo",
        [nodeName]
      );
      dispatch(setNodeInfo(nodeName, nodeInfo));
    }
  };
};
