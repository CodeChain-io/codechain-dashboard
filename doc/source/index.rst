
==========================
CodeChain Agent Protocol
==========================

.. highlight:: typescript

.. contents:: :local:

***************
Term
***************

Agent
=====

An agent lives in the same machine with a CodeChain node. It queries
CodeChain nodes and get managerial information. It can stop/update/run a
CodeChain program.


Agent Hub
============

An agent server queries agents to get information. It saves the collected
data to its own DB.


Frontend
=========

A frontend provides the visualization of the information collected from agent
servers.


***************
Common
***************

JSON RPC
=========

This protocol is based on the JSON RPC 2.0 spec

::

  --> {"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}
  <-- {"jsonrpc": "2.0", "result": 19, "id": 1}

Types
=======

.. _type-parcel:

Parcel
-------

::

  interface Parcel {
    // ...
  }


.. _type-SocketAddr:

SocketAddr
------------

::

  type SocketAddr = string

.. _type-Block:

Block
--------

::

  interface Block {
    // ...
  }

.. _type-ISO8601:

ISO8601
-------

::

  type ISO8601 = string // "2018-09-03T23:05:48Z"

.. _type-ISO8601Duration:

ISO8601Duration
-----------------

::

  type ISO8601Duration = string // "P1YT30M3S"


.. _type-NodeStatus:

NodeStatus
----------

::

  type NodeStatus = "Run" | "Stop" | "Error" | "UFO";


.. _type-DashboardNodeInfo:

DashboardNodeInfo
------------------

::

  interface DashboardNodeInfo {
    status: NodeStatus;
    address: SocketAddr;
    version: { version: string, hash: string };
    bestBlockId:  { blockNumber: number, hash: H256 };
    name?: string;
  }

links: type-NodeStatus_, type-SocketAddr_

.. _type-DashboardUFONodeInfo:

DashboardUFONodeInfo
--------------------

::

  interface DashboardUFONodeInfo {
    status: NodeStatus;
    address: SocketAddr;
  }

.. _type-NodeInfo:

NodeInfo
--------

::

  interface NodeInfo {
    address: SocketAddr;
    name?: string;
    agentVersion: String;
    status: NodeStatus;
    startOptions?: { env: string, args: string };
    version: { version: string, hash: string };
    commitHash: string;
    bestBlockId: { blockNumber: number, hash: H256 };
    pendingParcels: Parcel[];
    peers: SocketAddr[];
    whitelist: { list: SocketAddr[], enabled: bool };
    blacklist: { list: SocketAddr[], enabled: bool };
    hardware: { 
      cpuUsage: number,
      diskUsage: { total: "{}GB", available:"{}GB", percentageUsed: "{}%"},
      memoryUsage: { total: "{}GB", available:"{}GB", percentageUsed: "{}%"}
    };
    // events from this node order by created time.
    events: Event[];
  }

.. _type-UFONodeInfo:

UFONodeInfo
-----------

::

  interface UFONodeInfo {
    address: SocketAddr;
    agentVersion: String;
    status: NodeStatus;
  }

Error
=======

JSON RPC error space
-----------------------

JSON RPC uses -32768 to -32000 as reserved pre-defined errors.

::

  namespace PredefinedErrors {
    const ParseError: number = -32700;
    const InvalidRequest: number = -32600;
    const MethodNotFound: number = -32601;
    const InvalidParams: number = -32602;
    const InternalError: number = -32603;
    const serverErrorStart: number = -32099;
    const serverErrorEnd: number = -32000;
    const ServerNotInitialized: number = -32002;
    const UnknownErrorCode: number = -32001;
  }

.. _common-error:

Common error space
--------------------

-9999 ~ 0 are reserved for common error codes.

::

  namespace CommonErrors {
    const CodeChainIsNotRunning = 0;
    const AgentNotFound = -1;
  }


Protocol error space
-----------------------

Easy protocol use -19999 ~ 10000 numbers as error code.
These error codes has different meaning according to which protocol returns.


**********************
Agent Hub -> Agent
**********************

Requests
=========

codechain_callRPC ➡️ ⬅️ 
------------------------

Run codechain RPC through agent. JSONRPC result will be included in innerResponse field.

.. _type-CodeChainCallRPCRequest:

Request
"""""""""

::

  type CodeChainCallRPCRequest = [
    string, // RPC's name
    any[] // RPC's arguments
  ];

.. _type-CodeChainCallRPCResponse:

Response
""""""""""

::

  interface CodeChainCallRPCResponse {
    innerResponse: any;
  }

Error
"""""""

::

  interface CodeChainCallRPCErrors {
    /**
     *  Some network error occured while sending RPC to CodeChain
     */
    const NetworkError = -10001
  }

hardware_get ➡️ ⬅️ 
------------------

Get hardware information of the computer which the CodeChain and agent lives.

Request
""""""""

No request arguments

Response
"""""""""

::

  interface HardwareGetResponse { 
    cpuUsage: number;
    diskUsage: { total: "{}GB", available: "{}GB", percentageUsed: "{}%"};
    memoryUsage: { total: "{}GB", available: "{}GB", percentageUsed: "{}%"};
  }
  
agent_getInfo ➡️ ⬅️ 
------------------

Get agent's status and CodeChain's address

Request
""""""""

No request arguments

Response
"""""""""

::

  interface AgentGetInfoResponse { 
    status: NodeStatus;
    address: SocketAddr;
  }

links: type-NodeStatus_, type-SocketAddr_

shell_startCodeChain ➡️ ⬅️ 
--------------------------

.. _type-ShellStartCodeChainRequest:

Request
""""""""

::

  type ShellStartCodeChainRequest = [
    {
      env: string; // "RUST_LOG=trace"
      args: string; // "-c husky"
    }
  ]

Response
"""""""""

``()``


Error
"""""""

::

  namespace ShellStartCodeChainErrors {
    /**
     *  There is a codechain instance already running.
     */
    const AlreadyRunning = -10001;
    /**
     *  The format of given `env` is wrong.
     */
    const EnvParseError = -10002;
  }


shell_stopCodeChain ➡️ ⬅️ 
--------------------------

Stop running codechain.

Request
"""""""""

No request arguments

Response
"""""""""

``()``

Error
""""""

Could return ``CodeChainIsNotRunning``

links: common-error_


shell_getUptime ➡️ ⬅️ 
---------------------

Get codechain's uptime. If codechain is not running now, it returns null.

Request
"""""""""

No request arguments

Response
"""""""""

::

  type ShellGetUptime = ISO8601Duration | null

links: type-ISO8601Duration_


shell_updateCodeChain ➡️ ⬅️ 
---------------------------

Update CodeChain source code to the given commit hash.

Request
"""""""""

Commit hash of the CodeChain repository

``string``

Response
"""""""""

``()``

Error
"""""""

::

  namespace ShellUpdateCodeChainErrors {
    /**
     *  Cannot find the given commit hash from the repository
     */
    const NoSuchCommitHash = -10001
  }

**********************
Agent -> Agent Hub
**********************

Notification
===============

event_connected ➡️ 
-------------------

This event fires when a node is connected to another node.

Arguments
"""""""""

Argument is the other node's socket address.
``SocketAddr``

links: type-SocketAddr_


event_disconnected ➡️ 
---------------------

This event fires when a node is disconnected from another node.

Arguments
"""""""""

Argument is the other node's socket address.
``SocketAddr``

links: type-SocketAddr_


event_parcelSent ➡️ 
-------------------

This event fires when a node propagate parcels to another node.

Arguments
""""""""""

First argument is the node's socket address which received the parcels.
Second argument is the content of the parcels.

``[SocketAddr, Parcel[]]``

links: type-SocketAddr_, type-Parcel_


event_parcelReceived ➡️ 
-------------------------

This event fires when a node receives parcels from another node.

Arguments
"""""""""

``[SocketAddr, Parcel[]]``

links: type-SocketAddr_, type-Parcel_


event_parcelRecevedByRPC ➡️ 
-----------------------------

This event fires when a node receives a parcel by `chain_sendSignedParcel` RPC.

Arguments
"""""""""

``[Parcel]``

links: type-Parcel_

event_blockSent ➡️ 
-------------------

This event fires when a node sent a block to another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: type-SocketAddr_, type-Block_


event_blockRequested ➡️ 
------------------------

This event fires when a node requests a block to another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: type-SocketAddr_, type-Block_


event_blockReceived ➡️ 
------------------------

This event fires when a node received a block from another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: type-SocketAddr_, type-Block_


event_miningStarted ➡️ 
-----------------------

This event fires when a node starts mining.

Arguments
"""""""""

First argument is the block which is will be mined.
Second argument is the target score.

``[Block, number]``


event_miningSucceeded ➡️ 
------------------------

This event fires when a node succeed mining.

Arguments
"""""""""

First argument is the block which is will be mined.
Second argument is the target score.
``[Block, targetScore]``


**************************
Frontend <-> Agent Hub
**************************

Dashboard Page
==============

dashboard_getNetwork ➡️ ⬅️ 
--------------------------

Frontend requests information to agent server to render dashboard page.

Request
"""""""""

No request arguments

Response
"""""""""

::

  interface DashboardGetNetworkResponse {
    nodes: (DashboardNodeInfo | DashboardUFONodeInfo)[];
    connections: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
  }

links: type-DashboardNodeInfo_, type-DashboardUFONodeInfo_

dashboard_updated ➡️ 
--------------------

Arguments
""""""""""
::

  type DashboardUpdatedArguments = [{
    nodes?: ({ address: SocketAddr; } | Partial<DashboardNodeInfo> | Partial<DashboardUFONodeInfo>)[];
    connectionsAdded?: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
    connectionsRemoved?: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
  }]

links: type-DashboardNodeInfo_, type-DashboardUFONodeInfo_

Node Page
==========

node_getInfo ➡️ ⬅️ 
------------------

Frontend requests information to agent server to render node page.

Request
"""""""""

First argument is the address of a node.

``[SocketAddr]``

Response
"""""""""

::

  type NodeGetInfoResponse = NodeInfo | UFONodeInfo

links: type-NodeInfo_, type-UFONodeInfo_

node_updated ➡️ 
----------------

Arguments
"""""""""

::

  type NodeUpdatedArguments = [{
    address: SocketAddr;
    name?: string;
    status?: NodeStatus;
    version?: { version: string, hash: string };
    bestBlockId?: { blockNumber: number, hash: H256 };
    pendingParcels?: Parcel[];
    peers?: SocketAddr[];
    whitelist?: { list: SocketAddr[], enabled: bool };
    blacklist?: { list: SocketAddr[], enabled: bool };
    hardware?: { 
      cpuUsage: number,
      diskUsage: { total: "{}GB", available:"{}GB", percentageUsed: "{}%"},
      memoryUsage: { total: "{}GB", available:"{}GB", percentageUsed: "{}%"}
    };
    eventsAdded?: Event[];
  }]

links: type-NodeStatus_

node_start ➡️ ⬅️ 
----------------

Request
"""""""""

::

  type NodeStartRequest = [
    SocketAddr,
    {
      env: string; // "RUST_LOG=trace"
      args: string; // "-c husky"
    }
  ]

links: type-ShellStartCodeChainRequest_

Response
"""""""""

``()``

Error
"""""""

::

  namespace NodeStartErrors {
    /**
     *  There is a codechain instance already running.
     */
    const AlreadyRunning = -10001;
    /**
     *  The format of given `env` is wrong.
     */
    const EnvParseError = -10002;
  }


node_stop ➡️ ⬅️ 
---------------

Request
"""""""""

No request arguments

Response
"""""""""

``[SocketAddr]``

Error
""""""

Could return ``CodeChainIsNotRunning``

links: common-error_

node_update ➡️ ⬅️ 
-----------------

Request
"""""""""

Commit hash of the CodeChain repository.

``string``

Response
"""""""""

``()``

Error
""""""

::

  namespace NodeUpdateErrors {
    /**
     *  Cannot find the given commit hash from the repository
     */
    const NoSuchCommitHash = -10001
  }


RPC Page
========

rpc_getHistory ➡️ ⬅️ 
--------------------

Request
"""""""""

::

  interface RPCGetHistoryRequest {
    from: number;
    count: number;
  }

Response
"""""""""

::

  interface RPCGetHistoryResponse {
    histories: {
      RPCArguments: string[];
      RPCResponse: string;
      sentTime: ISO8601;
    }[]
  }

links: type-ISO8601_

rpc_run ➡️ ⬅️ 
--------------

Request
"""""""""

::

  type RPCRunRequest = CodeChainCallRPCRequest

links: type-CodeChainCallRPCRequest_

Response
"""""""""

::

  type RPCRunResponse = CodeChainCallRPCResponse

links: type-CodeChainCallRPCResponse_


..
  rpc_name
  -----------

  Request
  """""""""

  ::

    x

  Response
  """""""""

  ::

    x

  rpc_name
  -----------

  Arguments
  """""""""

  ``[]``
  
**********************
Agent Hub web
**********************

Agent server serve codechain's log file using HTTP.

Someone could get Agent(127.0.0.1:3485)'s logfile using ``curl http://agenthub.com:5012/log/127.0.0.1:3485``
