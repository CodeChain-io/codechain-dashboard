import { combineReducers } from "redux";
import { chainNetworksReducer, ChainNetworksState } from "./chainNetworks";
import { nodeInfoReducer, NodeState } from "./nodeInfo";

export interface ReducerConfigure {
  nodeInfoReducer: NodeState;
  chainNetworksReducer: ChainNetworksState;
}

const rootReducer = combineReducers({
  nodeInfoReducer,
  chainNetworksReducer
});
export default rootReducer;
