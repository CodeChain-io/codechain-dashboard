import { NodeInfo, NodeUpdateInfo } from "../requests/types";
export type NodeInfoAction = SetNodeInfo | UpdateNodeInfo;

export interface SetNodeInfo {
  type: "SetNodeInfo";
  name: string;
  data: NodeInfo;
}

export interface UpdateNodeInfo {
  type: "UpdateNodeInfo";
  name: string;
  data: NodeInfo;
}

export const setNodeInfo = (name: string, data: NodeInfo) => ({
  type: "SetNodeInfo",
  name,
  data
});

export const updateNodeInfo = (name: string, data: NodeUpdateInfo) => ({
  type: "UpdateNodeInfo",
  name,
  data
});
