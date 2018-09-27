import RequestAgent from "../RequestAgent";
import { ChainNetworks, NodeInfo } from "./types";

const getChainNetworks = async () => {
  return await RequestAgent.getInstance().call<ChainNetworks>(
    "real_dashboard_getNetwork",
    []
  );
};

const getNodeInfo = async (nodeName: string) => {
  return await RequestAgent.getInstance().call<NodeInfo>("real_node_getInfo", [
    nodeName
  ]);
};

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

export const Apis = {
  getChainNetworks,
  getNodeInfo,
  startNode,
  stopNode
};
