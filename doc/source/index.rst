
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


Agent Server
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

  type NodeStatue = "Run" | "Stop" | "Error" | "UFO";

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

Common error space
--------------------

-9999 ~ 0 are reserved for common error codes.

::

  namespace CommonErrors {
    const CodeChainIsNotRunning = 0;
  }


Protocol error space
-----------------------

Easy protocol use -19999 ~ 10000 numbers as error code.
These error codes has different meaning according to which protocol returns.


**********************
Agent Server -> Agent
**********************

Requests
=========

codechain_callRPC ➡️ ⬅️ 
------------------------

Run codechain RPC through agent.

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

  interface RunCodeChainRPCErrors {
    /**
     *  RPC to the CodeChain has an error. Error object will be in the error's data field.
     */
    const InnerError = -10001
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


shell_startCodeChain ➡️ ⬅️ 
--------------------------

.. _type-ShellStartCodeChainRequest:

Request
""""""""

::

  type ShellStartCodeChainRequest = [
    {
  // FIXME: get json from setting file
    }
  ]

Response
"""""""""

``()``


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

links: :ref:`type-ISO8601Duration`


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
Agent -> Agent Server
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

links: :ref:`type-SocketAddr`


event_disconnected ➡️ 
---------------------

This event fires when a node is disconnected from another node.

Arguments
"""""""""

Argument is the other node's socket address.
``SocketAddr``

links: :ref:`type-SocketAddr`


event_parcelSent ➡️ 
-------------------

This event fires when a node propagate parcels to another node.

Arguments
""""""""""

First argument is the node's socket address which received the parcels.
Second argument is the content of the parcels.

``[SocketAddr, Parcel[]]``

links: :ref:`type-SocketAddr`, :ref:`type-Parcel`


event_parcelReceived ➡️ 
-------------------------

This event fires when a node receives parcels from another node.

Arguments
"""""""""

``[SocketAddr, Parcel[]]``

links: :ref:`type-SocketAddr`, :ref:`type-Parcel`


event_parcelRecevedByRPC ➡️ 
-----------------------------

This event fires when a node receives a parcel by `chain_sendSignedParcel` RPC.

Arguments
"""""""""

``[Parcel]``

links: :ref:`type-Parcel`

event_blockSent ➡️ 
-------------------

This event fires when a node sent a block to another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: :ref:`type-SocketAddr`, :ref:`type-Block`


event_blockRequested ➡️ 
------------------------

This event fires when a node requests a block to another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: :ref:`type-SocketAddr`, :ref:`type-Block`


event_blockReceived ➡️ 
------------------------

This event fires when a node received a block from another node.

Arguments
"""""""""

``[SocketAddr, Block]``

links: :ref:`type-SocketAddr`, :ref:`type-Block`


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
Frontend <-> Agent Server
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
    nodes: {
      status: NodeStatus;
      address: SocketAddr;
      version: string;
      bestBlockId:  { number: number, hash: H256 };
      pendingParcelCount: number;
    }[];
    connections: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
  }

links: :ref:`type-NodeStatus`

dashboard_updated ➡️ 
--------------------

Arguments
""""""""""
::

  type DashboardUpdatedArguments = [{
    nodes?: {
      address: SocketAddr;
      status?: NodeStatus;
      version?: string;
      bestBlockId?:  { number: number, hash: H256 };
      pendingParcelCount?: number;
    }[];
    connectionsAdded?: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
    connectionsRemoved?: { nodeA: SocketAddr; nodeB: SocketAddr; }[]
  }]

links: :ref:`type-NodeStatus`

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

  interface NodeGetInfoResponse {
    address: SocketAddr;
    status: NodeStatus;
    version: string;
    commitHash: string;
    bestBlockId: { number: number, hash: H256 };
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

links: :ref:`type-NodeStatus`

node_updated ➡️ 
----------------

Arguments
"""""""""

::

  type NodeUpdatedArguments = [{
    address: SocketAddr;
    status?: NodeStatus;
    version?: string;
    commitHash?: string;
    bestBlockId?: { number: number, hash: H256 };
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

links: :ref:`type-NodeStatus`

node_start ➡️ ⬅️ 
----------------

Request
"""""""""

::

  type NodeStartRequest = ShellStartCodeChainRequest

links: :ref:`type-ShellStartCodeChainRequest`

Response
"""""""""

``()``

node_stop ➡️ ⬅️ 
---------------

Request
"""""""""

No request arguments

Response
"""""""""

``()``

Error
""""""

Could return ``CodeChainIsNotRunning``

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

links: :ref:`type-ISO8601`

rpc_run ➡️ ⬅️ 
--------------

Request
"""""""""

::

  type RPCRunRequest = CodeChainCallRPCRequest

links: :ref:`type-CodeChainCallRPCRequest`

Response
"""""""""

::

  type RPCRunResponse = CodeChainCallRPCResponse

links: :ref:`type-CodeChainCallRPCResponse`


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
