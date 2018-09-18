import { H256, Parcel } from "codechain-sdk/lib/core/classes";
export type NodeStatus = "Run" | "Stop" | "Error" | "UFO";
export type SocketAddr = string;
export interface NetworkNodeInfo {
  status: NodeStatus;
  address: SocketAddr;
  version?: { version: string; hash: string };
  bestBlockId?: { blockNumber: number; hash: H256 };
  name?: string;
}
export interface ChainNetworks {
  nodes: NetworkNodeInfo[];
  connections: { nodeA: SocketAddr; nodeB: SocketAddr }[];
}
export interface NodeInfo {
  address: SocketAddr;
  agentVersion: string;
  status: NodeStatus;
  version: string;
  commitHash: string;
  bestBlockId: { number: number; hash: H256 };
  pendingParcels: Parcel[];
  peers: SocketAddr[];
  whitelist: { list: SocketAddr[]; enabled: boolean };
  blacklist: { list: SocketAddr[]; enabled: boolean };
  hardware: {
    cpuUsage: number;
    diskUsage: { total: string; available: string; percentageUsed: string };
    memoryUsage: { total: string; available: string; percentageUsed: string };
  };
  events: string[];
}
