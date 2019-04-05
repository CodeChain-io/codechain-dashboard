import { combineReducers } from "redux";
import { chainNetworksReducer, ChainNetworksState } from "./chainNetworks";
import { logReducer, LogState } from "./log";
import { nodeInfoReducer, NodeState } from "./nodeInfo";

export interface ReducerConfigure {
  nodeInfoReducer: NodeState;
  chainNetworksReducer: ChainNetworksState;
  logReducer: LogState;
}

const rootReducer = combineReducers({
  nodeInfoReducer,
  chainNetworksReducer,
  logReducer
});
export default rootReducer;
