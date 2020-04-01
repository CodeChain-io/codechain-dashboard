import { NetworkNodeInfo } from "../../../requests/types";
import * as NLC from "./NodeListContainer";

describe("Test node ordering", () => {
  const alice: NetworkNodeInfo = {
    status: "Run",
    address: "119.202.81.99:8000",
    version: {
      version: "1.0.2",
      hash: ""
    },
    bestBlockId: {
      blockNumber: 100,
      hash: ""
    },
    name: "alice"
  };
  const bob: NetworkNodeInfo = {
    status: "Starting",
    address: "141.223.175.99:8001",
    version: {
      version: "1.0.1",
      hash: ""
    },
    bestBlockId: {
      blockNumber: 300,
      hash: ""
    },
    name: "bob"
  };
  const charlie: NetworkNodeInfo = {
    status: "Stop",
    address: "119.202.81.99:8002",
    version: {
      version: "1.1.4",
      hash: ""
    },
    bestBlockId: {
      blockNumber: 200,
      hash: ""
    },
    name: "charlie"
  };
  const david: NetworkNodeInfo = {
    status: "Updating",
    address: "110.202.81.27:9090",
    version: {
      version: "2.1.1",
      hash: ""
    },
    bestBlockId: {
      blockNumber: 320,
      hash: ""
    },
    name: "david"
  };
  const eve: NetworkNodeInfo = {
    status: "Error",
    address: "172.88.192.91:1010",
    version: {
      version: "1.2.7",
      hash: ""
    },
    bestBlockId: {
      blockNumber: 270,
      hash: ""
    },
    name: "eve"
  };
  const nodeArray = [eve, david, charlie, bob, alice];
  it("Test order by name", () => {
    const oracle = [alice, bob, charlie, david, eve];
    nodeArray.sort(NLC.nodeNameComparator);
    expect(nodeArray).toEqual(oracle);
  });
  it("Test order by socketAddress", () => {
    const oracle = [david, alice, charlie, bob, eve];
    nodeArray.sort(NLC.nodeSocketAddressComapartor);
    expect(nodeArray).toEqual(oracle);
  });
  it("Test order by blockNumber", () => {
    const oracle = [alice, charlie, eve, bob, david];
    nodeArray.sort(NLC.nodeBlockNumberComparator);
    expect(nodeArray).toEqual(oracle);
  });
  it("Test order by version", () => {
    const oracle = [bob, alice, charlie, eve, david];
    nodeArray.sort(NLC.nodeVersionComparator);
    expect(nodeArray).toEqual(oracle);
  });
  it("Test order by status", () => {
    const oracle = [eve, charlie, bob, david, alice];
    nodeArray.sort(NLC.nodeStatusComparator);
    expect(nodeArray).toEqual(oracle);
  });
});
