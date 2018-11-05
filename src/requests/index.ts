import RequestAgent from "../RequestAgent";
import { NodeInfo } from "./types";

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

const updateNode = async (nodeName: string, commitHash: string) => {
  return await RequestAgent.getInstance().call<void>("node_update", [
    nodeName,
    commitHash
  ]);
};

export const Apis = {
  startNode,
  stopNode,
  updateNode
};
