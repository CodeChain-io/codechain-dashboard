import { ChainNetworks, NodeDetail } from "./requests/types";
export type Action = SetChainNetworks | SetNodeDetail;

export interface SetChainNetworks {
  type: "SetChainNetworks";
  data: ChainNetworks;
}

export interface SetNodeDetail {
  type: "SetNodeDetail";
  socketAddr: string;
  data: NodeDetail;
}

const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data
});

const setNodeDetail = (socketAddr: string, data: NodeDetail) => ({
  type: "SetNodeDetail",
  socketAddr,
  data
});

export const Actions = {
  setNodeDetail,
  setChainNetworks
};
