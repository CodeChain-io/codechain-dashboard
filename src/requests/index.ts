import RequestAgent from "../RequestAgent";
import { ChainNetworks, NodeDetail, SocketAddr } from "./types";

const getChainNetworks = async () => {
  return await RequestAgent.getInstance().call<ChainNetworks>(
    "dashboard_getNetwork",
    []
  );
};

const getNodeInfo = async (nodeAddress: SocketAddr) => {
  return await RequestAgent.getInstance().call<NodeDetail>("node_getInfo", [
    nodeAddress
  ]);
};

export const Apis = {
  getChainNetworks,
  getNodeInfo
};
