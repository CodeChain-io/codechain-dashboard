import { ChainNetworks, ChainNetworksUpdate } from "../requests/types";
export type ChainNetworksAction = SetChainNetworks | UpdateChainNetworks;

export interface SetChainNetworks {
  type: "SetChainNetworks";
  data: ChainNetworks;
}

export interface UpdateChainNetworks {
  type: "UpdateChainNetworks";
  data: ChainNetworksUpdate;
}

export const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data
});

export const updateChainNetworks = (data: ChainNetworksUpdate) => ({
  type: "UpdateChainNetworks",
  data
});
