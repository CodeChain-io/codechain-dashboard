import { combineReducers } from "redux";
import { chainNetworksReducer, ChainNetworksState } from "./chainNetworks";
import { graphReducer, GraphState } from "./graph";
import { logReducer, LogState } from "./log";
import { nodeInfoReducer, NodeState } from "./nodeInfo";

export interface ReducerConfigure {
  nodeInfoReducer: NodeState;
  chainNetworksReducer: ChainNetworksState;
  logReducer: LogState;
  graphReducer: GraphState;
}

const rootReducer = combineReducers({
  nodeInfoReducer,
  chainNetworksReducer,
  logReducer,
  graphReducer
} as any);
export default rootReducer;
