import { H256, Parcel } from "codechain-sdk/lib/core/classes";
export type NodeStatus = "Run" | "Starting" | "Stop" | "Error" | "UFO";
export type SocketAddr = string;
export type IpAddr = string;
export type Tag = string;
export type BlackList = WhiteList;
export interface WhiteList {
  list: [IpAddr, Tag][];
  enabled: boolean;
}
export enum CommonError {
  CodeChainIsNotRunning = 0,
  AgentNotFound = -1
}
export interface NetworkNodeInfo {
  status: NodeStatus;
  address?: SocketAddr;
  version?: { version: string; hash: string };
  bestBlockId?: { blockNumber: number; hash: H256 };
  name: string;
}
export interface ChainNetworks {
  nodes: NetworkNodeInfo[];
  connections: { nodeA: string; nodeB: string }[];
}
export interface ChainNetworksUpdate {
  nodes?: {
    status?: NodeStatus;
    address?: SocketAddr;
    version?: { version: string; hash: string };
    bestBlockId?: { blockNumber: number; hash: H256 };
    name: string;
  }[];
  connectionsAdded?: { nodeA: string; nodeB: string }[];
  connectionsRemoved?: { nodeA: string; nodeB: string }[];
}
export interface NodeInfo {
  name: string;
  startOption?: { env: string; args: string };
  address?: SocketAddr;
  agentVersion?: string;
  status: NodeStatus;
  version?: { version: string; hash: string };
  bestBlockId?: { blockNumber: number; hash: H256 };
  pendingParcels?: Parcel[];
  peers?: SocketAddr[];
  whitelist?: WhiteList;
  blacklist?: BlackList;
  hardware?: {
    cpuUsage: number[];
    diskUsage: { total: number; available: number; percentageUsed: number };
    memoryUsage: { total: number; available: number; percentageUsed: number };
  };
  events?: string[];
}
export interface NodeUpdateInfo {
  name: string;
  startOption?: { env: string; args: string };
  address?: SocketAddr;
  agentVersion?: string;
  status?: NodeStatus;
  version?: { version: string; hash: string };
  bestBlockId?: { blockNumber: number; hash: H256 };
  pendingParcels?: Parcel[];
  peers?: SocketAddr[];
  whitelist?: { list: SocketAddr[]; enabled: boolean };
  blacklist?: { list: SocketAddr[]; enabled: boolean };
  hardware?: {
    cpuUsage: number[];
    diskUsage: { total: number; available: number; percentageUsed: number };
    memoryUsage: { total: number; available: number; percentageUsed: number };
  };
  events?: string[];
}
