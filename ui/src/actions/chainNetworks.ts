import * as _ from "lodash";
import { ReducerConfigure } from "../reducers";
import { ChainNetworksState } from "../reducers/chainNetworks";
import RequestAgent from "../RequestAgent";
import { ChainNetworks, ChainNetworksUpdate } from "../requests/types";
import { changeFilters } from "./log";
export type ChainNetworksAction =
  | SetChainNetworks
  | UpdateChainNetworks
  | RequestChainNetworks;

export interface SetChainNetworks {
  type: "SetChainNetworks";
  data: ChainNetworks;
  receivedAt: number;
}

export interface UpdateChainNetworks {
  type: "UpdateChainNetworks";
  data: ChainNetworksUpdate;
}

export interface RequestChainNetworks {
  type: "RequestChainNetworks";
}

export const setChainNetworks = (data: ChainNetworks) => ({
  type: "SetChainNetworks",
  data,
  receivedAt: Date.now()
});

export const updateChainNetworks = (data: ChainNetworksUpdate) => ({
  type: "UpdateChainNetworks",
  data
});

export const requestChainNetworks = () => ({
  type: "RequestChainNetworks"
});

const shouldFetchChainNetworks = (state: ChainNetworksState) => {
  if (!state.chainNetworks) {
    return true;
  } else if (state.isFetching) {
    return false;
  }
  return true;
};

export const fetchChainNetworksIfNeeded = () => {
  return async (dispatch: any, getState: () => ReducerConfigure) => {
    if (shouldFetchChainNetworks(getState().chainNetworksReducer)) {
      dispatch(requestChainNetworks());
      const chainNetworks = await RequestAgent.getInstance().call<
        ChainNetworks
      >("dashboard_getNetwork", []);
      dispatch(setChainNetworks(chainNetworks));
      dispatch(
        changeFilters({
          filter: { nodeNames: _.map(chainNetworks.nodes, node => node.name) }
        })
      );
    }
  };
};
