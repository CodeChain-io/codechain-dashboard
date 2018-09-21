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
  socketAddr: string;
  data: NodeInfo;
}

export interface UpdateNodeInfo {
  type: "UpdateNodeInfo";
  socketAddr: string;
  data: NodeInfo;
}

export interface UpdateChainNetworks {
  type: "UpdateChainNetworks";
  data: ChainNetworks;
}

export const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data
});

const setNodeInfo = (socketAddr: string, data: NodeInfo) => ({
  type: "SetNodeInfo",
  socketAddr,
  data
});

const updateNodeInfo = (socketAddr: string, data: NodeUpdateInfo) => ({
  type: "UpdateNodeInfo",
  socketAddr,
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
