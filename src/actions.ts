import {
  ChainNetworks,
  ChainNetworksUpdate,
  NodeInfo,
  NodeUpdateInfo
} from "./requests/types";
export type Action =
  | SetChainNetworks
  | SetNodeInfo
  | UpdateNodeInfo
  | UpdateChainNetworks;

export interface SetChainNetworks {
  type: "SetChainNetworks";
  data: ChainNetworks;
}

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

export interface UpdateChainNetworks {
  type: "UpdateChainNetworks";
  data: ChainNetworksUpdate;
}

export const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data
});

const setNodeInfo = (name: string, data: NodeInfo) => ({
  type: "SetNodeInfo",
  name,
  data
});

const updateNodeInfo = (name: string, data: NodeUpdateInfo) => ({
  type: "UpdateNodeInfo",
  name,
  data
});

const updateChainNetworks = (data: ChainNetworksUpdate) => ({
  type: "UpdateChainNetworks",
  data
});

export const Actions = {
  setNodeInfo,
  setChainNetworks,
  updateNodeInfo,
  updateChainNetworks
};
