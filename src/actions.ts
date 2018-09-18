import { ChainNetworks, NodeInfo } from "./requests/types";
export type Action = SetChainNetworks | SetNodeInfo;

export interface SetChainNetworks {
  type: "SetChainNetworks";
  data: ChainNetworks;
}

export interface SetNodeInfo {
  type: "SetNodeInfo";
  socketAddr: string;
  data: NodeInfo;
}

const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data
});

const setNodeInfo = (socketAddr: string, data: NodeInfo) => ({
  type: "SetNodeInfo",
  socketAddr,
  data
});

export const Actions = {
  setNodeInfo,
  setChainNetworks
};
