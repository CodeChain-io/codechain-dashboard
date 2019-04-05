import RequestAgent from "../RequestAgent";
import { NodeInfo, UpdateCodeChainRequest } from "./types";

const startNode = async (nodeName: string, env: string, args: string) => {
  return await RequestAgent.getInstance().call<NodeInfo>("node_start", [
    nodeName,
    {
      env,
      args
    }
  ]);
};

const stopNode = async (nodeName: string) => {
  return await RequestAgent.getInstance().call<NodeInfo>("node_stop", [
    nodeName
  ]);
};

const updateNode = async (nodeName: string, req: UpdateCodeChainRequest) => {
  return await RequestAgent.getInstance().call<void>("node_update", [
    nodeName,
    req
  ]);
};

export const Apis = {
  startNode,
  stopNode,
  updateNode
};
