import { Action } from "./actions";
import { ChainNetworks, NodeDetail } from "./requests/types";
export interface RootState {
  nodeInfo: {
    [socketAddr: string]: NodeDetail;
  };
  chainNetworks: ChainNetworks | undefined;
}

const initialState: RootState = {
  nodeInfo: {},
  chainNetworks: undefined
};

export const appReducer = (state = initialState, action: Action) => {
  switch (action.type) {
    case "SetChainNetworks":
      const chainNetworks = action.data;
      return {
        ...state,
        chainNetworks
      };
    case "SetNodeDetail":
      const nodeInfo = {
        ...state.nodeInfo,
        [action.socketAddr]: action.data
      };
      return {
        ...state,
        nodeInfo
      };
  }
  return state;
};
