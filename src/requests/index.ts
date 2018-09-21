import RequestAgent from "../RequestAgent";
import { ChainNetworks, NodeInfo, SocketAddr } from "./types";

const getChainNetworks = async () => {
  return await RequestAgent.getInstance().call<ChainNetworks>(
    "dashboard_getNetwork",
    []
  );
};

const getNodeInfo = async (nodeAddress: SocketAddr) => {
  return await RequestAgent.getInstance().call<NodeInfo>("node_getInfo", [
    nodeAddress
  ]);
};

const startNode = async (
  nodeAddress: SocketAddr,
  env: string,
  args: string
) => {
  return await RequestAgent.getInstance().call<NodeInfo>("node_start", [
    nodeAddress,
    {
      env,
      args
    }
  ]);
};

const stopNode = async (nodeAddress: SocketAddr) => {
  return await RequestAgent.getInstance().call<NodeInfo>("node_stop", [
    nodeAddress
  ]);
};

export const Apis = {
  getChainNetworks,
  getNodeInfo,
  startNode,
  stopNode
};
